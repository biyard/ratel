use super::helpers::*;
use crate::common::models::migration::LastBackfillVersion;

#[tokio::test]
async fn test_last_backfill_version_default_unset() {
    let ctx = TestContext::setup().await;
    reset_migration_state(&ctx.ddb).await;
    let (pk, sk) = LastBackfillVersion::singleton_keys();
    let row = LastBackfillVersion::get(&ctx.ddb, &pk, Some(&sk)).await.unwrap();
    assert!(row.is_none(), "no migration row should exist initially");
}

#[tokio::test]
async fn test_advance_to_from_zero_inserts() {
    let ctx = TestContext::setup().await;
    reset_migration_state(&ctx.ddb).await;
    LastBackfillVersion::advance_to(&ctx.ddb, 0, 1).await.unwrap();
    let (pk, sk) = LastBackfillVersion::singleton_keys();
    let row = LastBackfillVersion::get(&ctx.ddb, &pk, Some(&sk))
        .await
        .unwrap()
        .expect("row should exist after advance");
    assert_eq!(row.version, 1);
}

#[tokio::test]
async fn test_advance_to_with_correct_expected_succeeds() {
    let ctx = TestContext::setup().await;
    reset_migration_state(&ctx.ddb).await;
    LastBackfillVersion::advance_to(&ctx.ddb, 0, 1).await.unwrap();
    LastBackfillVersion::advance_to(&ctx.ddb, 1, 2).await.unwrap();
    let (pk, sk) = LastBackfillVersion::singleton_keys();
    let row = LastBackfillVersion::get(&ctx.ddb, &pk, Some(&sk))
        .await
        .unwrap()
        .expect("row should exist");
    assert_eq!(row.version, 2);
}

#[tokio::test]
async fn test_advance_to_with_wrong_expected_fails() {
    let ctx = TestContext::setup().await;
    reset_migration_state(&ctx.ddb).await;
    LastBackfillVersion::advance_to(&ctx.ddb, 0, 1).await.unwrap();
    let res = LastBackfillVersion::advance_to(&ctx.ddb, 0, 2).await;
    assert!(res.is_err(), "advancing with stale expected should be rejected");
    let (pk, sk) = LastBackfillVersion::singleton_keys();
    let row = LastBackfillVersion::get(&ctx.ddb, &pk, Some(&sk))
        .await
        .unwrap()
        .expect("row should still be at 1");
    assert_eq!(row.version, 1, "version must not advance on conflict");
}

#[tokio::test]
async fn test_run_migrations_skips_when_migrate_unset() {
    let ctx = TestContext::setup().await;
    reset_migration_state(&ctx.ddb).await;
    std::env::remove_var("MIGRATE");
    crate::common::migrations::run_migrations(&ctx.ddb).await.unwrap();

    let (pk, sk) = LastBackfillVersion::singleton_keys();
    let row = LastBackfillVersion::get(&ctx.ddb, &pk, Some(&sk)).await.unwrap();
    assert!(row.is_none(), "MIGRATE unset must not advance version");
}

#[tokio::test]
async fn test_run_migrations_runs_m001() {
    let ctx = TestContext::setup().await;
    reset_migration_state(&ctx.ddb).await;
    // Seed a SpaceScore so the backfill has work to do.
    use crate::features::activity::models::SpaceScore;
    use crate::features::activity::types::AuthorPartition;
    let user_pk = ctx.test_user.0.pk.clone();
    let space_part = SpacePartition("seed".into());
    let author = AuthorPartition::from(user_pk.clone());
    let mut s = SpaceScore::new(space_part, author, "u".into(), "".into());
    s.total_score = 5_000;
    s.create(&ctx.ddb).await.unwrap();

    // Exercise the real `run_migrations` entry point so this test actually
    // covers the runner's "stored < 1 → run m001" branch. m002 runs in the
    // same pass and demands `M002_CSV_PATH`; feed it a header-only CSV so it
    // walks zero rows and returns Ok without affecting what we assert here.
    let csv = unique_csv_path("m001-runs-noop");
    std::fs::write(&csv, "user_id,amount,created_at_ms,reward_key\n").unwrap();
    let path = csv.to_string_lossy().to_string();

    let ddb = ctx.ddb.clone();
    run_with_envs(
        &[("MIGRATE", "true"), ("M002_CSV_PATH", &path)],
        move || async move {
            crate::common::migrations::run_migrations(&ddb).await.unwrap();
        },
    )
    .await;

    let _ = std::fs::remove_file(&csv);

    // Verify the runner advanced past m001. m002 ran on a header-only CSV
    // (no rows) and bumps the version to 2 as well — that's expected, the
    // signal we care about is "m001 executed", confirmed via CharacterXp below.
    let (pk, sk) = LastBackfillVersion::singleton_keys();
    let row = LastBackfillVersion::get(&ctx.ddb, &pk, Some(&sk))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(row.version, 2);

    // Verify CharacterXp seeded.
    use crate::features::character::models::CharacterXp;
    let (xpk, xsk) = CharacterXp::user_keys(&user_pk);
    let xp = CharacterXp::get(&ctx.ddb, &xpk, Some(&xsk))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(xp.total_xp, 5_000);
    assert_eq!(xp.level, 4);
}

#[tokio::test]
async fn test_run_migrations_idempotent_at_version() {
    let ctx = TestContext::setup().await;
    reset_migration_state(&ctx.ddb).await;
    // Pre-advance to the latest known migration version so `run_migrations`
    // has nothing to do. Bumping this whenever a new migration lands keeps
    // the "already at HEAD = no-op" invariant honest.
    LastBackfillVersion::advance_to(&ctx.ddb, 0, 1).await.unwrap();
    LastBackfillVersion::advance_to(&ctx.ddb, 1, 2).await.unwrap();

    let ddb = ctx.ddb.clone();
    run_with_env("MIGRATE", "true", move || async move {
        crate::common::migrations::run_migrations(&ddb).await.unwrap();
    })
    .await;

    let (pk, sk) = LastBackfillVersion::singleton_keys();
    let row = LastBackfillVersion::get(&ctx.ddb, &pk, Some(&sk))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(row.version, 2, "no further migrations to run");
}

// ── m002 (backfill_pending_rewards) ──────────────────────────────

fn unique_csv_path(label: &str) -> std::path::PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("m002_{label}_{nanos}.csv"))
}

#[tokio::test]
async fn test_run_migrations_runs_m002() {
    let ctx = TestContext::setup().await;
    reset_migration_state(&ctx.ddb).await;
    // Skip past m001 so this test isolates m002 behavior.
    LastBackfillVersion::advance_to(&ctx.ddb, 0, 1).await.unwrap();

    let csv = unique_csv_path("runs");
    std::fs::write(
        &csv,
        "user_id,amount,created_at_ms,reward_key\n\
         test-m002-runs-a,30000,1776590280000,SPACE_REWARD##019d70df-dfc0-7222-be71-e55c2bd8121a##ref-a##QUIZ_ANSWER\n\
         test-m002-runs-b,20000,1776590281000,SPACE_REWARD##019d70df-dfc0-7222-be71-e55c2bd8121a##ref-b##RESPOND_POLL\n",
    )
    .unwrap();
    let path = csv.to_string_lossy().to_string();

    let ddb = ctx.ddb.clone();
    run_with_envs(
        &[("MIGRATE", "true"), ("M002_CSV_PATH", &path)],
        move || async move {
            crate::common::migrations::run_migrations(&ddb).await.unwrap();
        },
    )
    .await;

    // Version advanced to 2.
    let (pk, sk) = LastBackfillVersion::singleton_keys();
    let row = LastBackfillVersion::get(&ctx.ddb, &pk, Some(&sk))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(row.version, 2);

    // Both rows landed under the outage-backfill description. The shared
    // PendingReward table can carry rows from sibling tests (e.g. the
    // idempotency test below), so filter on *this* test's user_ids — not
    // just `description == "outage-backfill"`.
    use crate::common::models::reward::PendingReward;
    let (items, _) = PendingReward::find_by_status(
        &ctx.ddb,
        PendingRewardStatus::Pending,
        PendingReward::opt().limit(100),
    )
    .await
    .unwrap();
    let mine: Vec<_> = items
        .iter()
        .filter(|r| r.description == "outage-backfill")
        .filter(|r| matches!(&r.target_pk,
            Partition::User(u) if u == "test-m002-runs-a" || u == "test-m002-runs-b"))
        .collect();
    assert_eq!(mine.len(), 2);

    let _ = std::fs::remove_file(&csv);
}

#[tokio::test]
async fn test_m002_aborts_when_csv_path_unset() {
    let ctx = TestContext::setup().await;
    reset_migration_state(&ctx.ddb).await;
    LastBackfillVersion::advance_to(&ctx.ddb, 0, 1).await.unwrap();

    let ddb = ctx.ddb.clone();
    run_with_env("MIGRATE", "true", move || async move {
        // Ensure no prior test left M002_CSV_PATH lying around.
        std::env::remove_var("M002_CSV_PATH");
        let res = crate::common::migrations::run_migrations(&ddb).await;
        assert!(res.is_err(), "m002 must fail when M002_CSV_PATH is unset");
    })
    .await;

    // Version must not advance past m001.
    let (pk, sk) = LastBackfillVersion::singleton_keys();
    let row = LastBackfillVersion::get(&ctx.ddb, &pk, Some(&sk))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(row.version, 1, "failed m002 must not bump version");
}

#[tokio::test]
async fn test_m002_idempotent_on_rerun() {
    let ctx = TestContext::setup().await;
    reset_migration_state(&ctx.ddb).await;
    LastBackfillVersion::advance_to(&ctx.ddb, 0, 1).await.unwrap();

    let csv = unique_csv_path("idem");
    std::fs::write(
        &csv,
        "user_id,amount,created_at_ms,reward_key\n\
         test-m002-idem,30000,1776590280000,SPACE_REWARD##019d70df-dfc0-7222-be71-e55c2bd8121a##ref-i##QUIZ_ANSWER\n",
    )
    .unwrap();
    let path = csv.to_string_lossy().to_string();

    // First run.
    let ddb1 = ctx.ddb.clone();
    let p1 = path.clone();
    run_with_envs(
        &[("MIGRATE", "true"), ("M002_CSV_PATH", &p1)],
        move || async move {
            crate::common::migrations::m002_backfill_pending_rewards::run(&ddb1)
                .await
                .unwrap();
        },
    )
    .await;

    // Second run on the same CSV — conditional PutItem must skip every row.
    let ddb2 = ctx.ddb.clone();
    let p2 = path.clone();
    run_with_envs(
        &[("MIGRATE", "true"), ("M002_CSV_PATH", &p2)],
        move || async move {
            crate::common::migrations::m002_backfill_pending_rewards::run(&ddb2)
                .await
                .unwrap();
        },
    )
    .await;

    // Still exactly one row for our test user.
    use crate::common::models::reward::PendingReward;
    let (items, _) = PendingReward::find_by_status(
        &ctx.ddb,
        PendingRewardStatus::Pending,
        PendingReward::opt().limit(100),
    )
    .await
    .unwrap();
    let mine: Vec<_> = items
        .iter()
        .filter(|r| r.target_pk == Partition::User("test-m002-idem".into()))
        .collect();
    assert_eq!(mine.len(), 1, "rerun must not duplicate");

    let _ = std::fs::remove_file(&csv);
}
