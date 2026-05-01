use super::helpers::*;
use crate::common::models::migration::LastBackfillVersion;

#[tokio::test]
async fn test_last_backfill_version_default_unset() {
    let ctx = TestContext::setup().await;
    let (pk, sk) = LastBackfillVersion::singleton_keys();
    let row = LastBackfillVersion::get(&ctx.ddb, &pk, Some(&sk)).await.unwrap();
    assert!(row.is_none(), "no migration row should exist initially");
}

#[tokio::test]
async fn test_advance_to_from_zero_inserts() {
    let ctx = TestContext::setup().await;
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
    std::env::remove_var("MIGRATE");
    crate::common::migrations::run_migrations(&ctx.ddb).await.unwrap();

    let (pk, sk) = LastBackfillVersion::singleton_keys();
    let row = LastBackfillVersion::get(&ctx.ddb, &pk, Some(&sk)).await.unwrap();
    assert!(row.is_none(), "MIGRATE unset must not advance version");
}

#[tokio::test]
async fn test_run_migrations_runs_m001() {
    let ctx = TestContext::setup().await;
    // Seed a SpaceScore so the backfill has work to do.
    use crate::features::activity::models::SpaceScore;
    use crate::features::activity::types::AuthorPartition;
    let user_pk = ctx.test_user.0.pk.clone();
    let space_part = SpacePartition("seed".into());
    let author = AuthorPartition::from(user_pk.clone());
    let mut s = SpaceScore::new(space_part, author, "u".into(), "".into());
    s.total_score = 5_000;
    s.create(&ctx.ddb).await.unwrap();

    let ddb = ctx.ddb.clone();
    run_with_env("MIGRATE", "true", move || async move {
        crate::common::migrations::run_migrations(&ddb).await.unwrap();
    })
    .await;

    // Verify version advanced.
    let (pk, sk) = LastBackfillVersion::singleton_keys();
    let row = LastBackfillVersion::get(&ctx.ddb, &pk, Some(&sk))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(row.version, 1);

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
    LastBackfillVersion::advance_to(&ctx.ddb, 0, 1).await.unwrap();

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
    assert_eq!(row.version, 1, "no further migrations to run");
}
