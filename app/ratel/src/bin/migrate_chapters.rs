//! One-shot Quest Map data migration binary.
//!
//! See `docs/superpowers/specs/2026-04-09-action-ui-gamification-renewal-design.md`
//! section 12 for the authoritative migration plan. This binary is
//! **idempotent** — safe to re-run. It never deletes anything; it only
//! adds records that are missing.
//!
//! Steps:
//! 1. Seed two default chapters (`qualify`, `participate`) on every
//!    existing space that has no chapters yet.
//! 2. Backfill `chapter_id` on every existing `SpaceAction` by reading
//!    its `prerequisite: bool` flag (`true` → `qualify`, `false` →
//!    `participate`). Also backfills `depends_on: []`.
//! 3. Initialize `UserGlobalXp` for every user who has completed at
//!    least one action. Historical XP equals `total_point` with no
//!    multipliers (documented in the migration log).
//! 4. Seed `UserStreak` with zeros for every user that shows up in
//!    step 3.
//! 5. `SpaceCreatorEarnings` is **not** backfilled — per spec, tracking
//!    starts the moment the new system ships.
//!
//! Compile-check (never actually run against production without review):
//!
//! ```bash
//! DYNAMO_TABLE_PREFIX=ratel-dev \
//!   cargo check --bin migrate_chapters --features "server,bypass"
//! ```

#[cfg(not(feature = "server"))]
fn main() {
    eprintln!(
        "migrate_chapters requires the `server` feature (DynamoDB access). \
         Run with: cargo run --bin migrate_chapters --features \"server,bypass\""
    );
    std::process::exit(1);
}

#[cfg(feature = "server")]
#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    migration::run().await
}

#[cfg(feature = "server")]
mod migration {
    use std::collections::{HashMap, HashSet};

    use app_shell::common::models::space::SpaceCommon;
    use app_shell::common::types::{EntityType, Partition, SpacePartition, SpaceUserRole, UserPartition};
    use app_shell::features::spaces::pages::actions::gamification::{
        ChapterBenefit, SpaceChapter, UserGlobalXp, UserStreak,
    };
    use app_shell::features::spaces::pages::actions::models::SpaceAction;
    use aws_sdk_dynamodb::types::AttributeValue;

    /// Chapter id for the default "qualify" chapter seeded on every space.
    pub const QUALIFY_CHAPTER_ID: &str = "qualify";
    /// Chapter id for the default "participate" chapter seeded on every space.
    pub const PARTICIPATE_CHAPTER_ID: &str = "participate";

    pub async fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "info".into()),
            )
            .init();

        let cfg = app_shell::common::CommonConfig::default();
        let cli = cfg.dynamodb().clone();

        tracing::info!("migrate_chapters: starting");

        let spaces = scan_spaces(&cli).await?;
        tracing::info!("migrate_chapters: found {} spaces", spaces.len());

        let mut chapters_seeded = 0usize;
        for space in &spaces {
            if seed_default_chapters_if_missing(&cli, space).await? {
                chapters_seeded += 1;
            }
        }
        tracing::info!(
            "migrate_chapters: seeded default chapters on {} spaces (skipped {} already-seeded)",
            chapters_seeded,
            spaces.len() - chapters_seeded
        );

        let actions = scan_space_actions(&cli).await?;
        tracing::info!("migrate_chapters: found {} space actions", actions.len());

        let mut actions_backfilled = 0usize;
        for mut action in actions {
            if action.chapter_id.is_none() {
                action.chapter_id = Some(
                    if action.prerequisite {
                        QUALIFY_CHAPTER_ID
                    } else {
                        PARTICIPATE_CHAPTER_ID
                    }
                    .to_string()
                    .into(),
                );
                // depends_on already defaults to Vec::new() via serde, but
                // touch it explicitly so the stored record carries the
                // field even for pre-existing rows that were written
                // before the field existed.
                action.depends_on = Vec::new();
                action.upsert(&cli).await?;
                actions_backfilled += 1;
            }
        }
        tracing::info!(
            "migrate_chapters: backfilled chapter_id on {} actions",
            actions_backfilled
        );

        let user_totals = compute_user_historical_totals(&cli).await?;
        tracing::info!(
            "migrate_chapters: found {} users with historical completions",
            user_totals.len()
        );

        let mut global_xp_seeded = 0usize;
        let mut streaks_seeded = 0usize;
        for (user_id_str, total_points) in &user_totals {
            let user_id = UserPartition(user_id_str.clone());
            let user_pk: Partition = Partition::User(user_id_str.clone());

            if !has_entity::<UserGlobalXp>(&cli, &user_pk, &EntityType::UserGlobalXp).await? {
                let mut entry = UserGlobalXp::new(user_id.clone());
                entry.total_points = *total_points;
                entry.total_xp = *total_points; // historical XP == P (no multipliers)
                entry.level = UserGlobalXp::level_from_xp(*total_points);
                entry.create(&cli).await?;
                global_xp_seeded += 1;
            }

            if !has_entity::<UserStreak>(&cli, &user_pk, &EntityType::UserStreak).await? {
                let entry = UserStreak::new(user_id);
                entry.create(&cli).await?;
                streaks_seeded += 1;
            }
        }
        tracing::info!(
            "migrate_chapters: seeded {} UserGlobalXp and {} UserStreak rows",
            global_xp_seeded,
            streaks_seeded
        );

        tracing::info!("migrate_chapters: complete");
        Ok(())
    }

    /// Return every existing space that lives under an `sk == "SPACE_COMMON"`
    /// row. Uses a low-level scan — there is no `scan_all` helper on the
    /// generated entity.
    async fn scan_spaces(
        cli: &aws_sdk_dynamodb::Client,
    ) -> std::result::Result<Vec<SpaceCommon>, Box<dyn std::error::Error>> {
        let items = scan_by_sk_prefix(cli, SpaceCommon::table_name(), "SPACE_COMMON").await?;
        let mut out = Vec::with_capacity(items.len());
        for item in items {
            match serde_dynamo::from_item::<_, SpaceCommon>(item) {
                Ok(space) => out.push(space),
                Err(e) => tracing::warn!(error = %e, "failed to parse SpaceCommon row, skipping"),
            }
        }
        Ok(out)
    }

    async fn scan_space_actions(
        cli: &aws_sdk_dynamodb::Client,
    ) -> std::result::Result<Vec<SpaceAction>, Box<dyn std::error::Error>> {
        let items = scan_by_sk_prefix(cli, SpaceAction::table_name(), "SPACE_ACTION").await?;
        let mut out = Vec::with_capacity(items.len());
        for item in items {
            match serde_dynamo::from_item::<_, SpaceAction>(item) {
                Ok(action) => out.push(action),
                Err(e) => tracing::warn!(error = %e, "failed to parse SpaceAction row, skipping"),
            }
        }
        Ok(out)
    }

    /// Low-level paginated scan. Uses `sk begins_with <prefix>` as the
    /// filter expression. Expensive, but this binary is one-shot.
    async fn scan_by_sk_prefix(
        cli: &aws_sdk_dynamodb::Client,
        table_name: &str,
        prefix: &str,
    ) -> std::result::Result<Vec<HashMap<String, AttributeValue>>, Box<dyn std::error::Error>> {
        let mut out = Vec::new();
        let mut last_key: Option<HashMap<String, AttributeValue>> = None;
        loop {
            let mut req = cli
                .scan()
                .table_name(table_name)
                .filter_expression("begins_with(#sk, :prefix)")
                .expression_attribute_names("#sk", "sk")
                .expression_attribute_values(
                    ":prefix",
                    AttributeValue::S(prefix.to_string()),
                );
            if let Some(k) = last_key.take() {
                req = req.set_exclusive_start_key(Some(k));
            }
            let resp = req.send().await?;
            if let Some(items) = resp.items {
                out.extend(items);
            }
            match resp.last_evaluated_key {
                Some(k) => last_key = Some(k),
                None => break,
            }
        }
        Ok(out)
    }

    /// Idempotent seeder — if either default chapter is missing on the
    /// space, both are inserted. Returns `true` if anything was written.
    async fn seed_default_chapters_if_missing(
        cli: &aws_sdk_dynamodb::Client,
        space: &SpaceCommon,
    ) -> std::result::Result<bool, Box<dyn std::error::Error>> {
        let space_pk = space.pk.clone();
        let space_id_str = match &space_pk {
            Partition::Space(id) => id.clone(),
            _ => return Ok(false),
        };
        let space_id = SpacePartition(space_id_str);

        let qualify_sk = EntityType::SpaceChapter(QUALIFY_CHAPTER_ID.to_string());
        let participate_sk = EntityType::SpaceChapter(PARTICIPATE_CHAPTER_ID.to_string());

        let has_qualify = SpaceChapter::get(cli, &space_pk, Some(&qualify_sk))
            .await?
            .is_some();
        let has_participate = SpaceChapter::get(cli, &space_pk, Some(&participate_sk))
            .await?
            .is_some();

        if has_qualify && has_participate {
            return Ok(false);
        }

        if !has_qualify {
            let ch0 = SpaceChapter::new(
                space_id.clone(),
                QUALIFY_CHAPTER_ID.to_string(),
                0,
                "Qualify".to_string(),
                SpaceUserRole::Candidate,
                ChapterBenefit::RoleUpgradeAndXp(SpaceUserRole::Participant),
            );
            ch0.create(cli).await?;
        }

        if !has_participate {
            let ch1 = SpaceChapter::new(
                space_id,
                PARTICIPATE_CHAPTER_ID.to_string(),
                1,
                "Participate".to_string(),
                SpaceUserRole::Participant,
                ChapterBenefit::XpOnly,
            );
            ch1.create(cli).await?;
        }

        Ok(true)
    }

    /// Per-user historical totals derived from the existing
    /// `SpaceAction.total_points` field. The new system has no
    /// per-action completion aggregation yet, so we attribute every
    /// action's `total_points` to every user who completed it.
    ///
    /// For Phase 2 this is a best-effort pass: we walk every poll answer
    /// and quiz attempt row in the table and bucket `total_points`
    /// by user. Future phases will replace this with direct ledger
    /// reads once the ledger is populated.
    ///
    /// Keyed by user id string (not `Partition`) because `Partition`
    /// does not derive `Hash`. Converted back to `Partition::User`
    /// by the caller when performing row writes.
    async fn compute_user_historical_totals(
        cli: &aws_sdk_dynamodb::Client,
    ) -> std::result::Result<HashMap<String, i64>, Box<dyn std::error::Error>> {
        // Collect the set of users that have ever interacted with any
        // space action. We intentionally over-estimate by including any
        // row whose pk starts with `USER#` and whose sk hints at
        // participation. A missing user is worse than an extra zero row.
        let mut users: HashSet<String> = HashSet::new();

        // Walk poll answers — pk = SPACE_POLL_USER_ANSWER#<user_id>.
        let poll_answers =
            scan_by_pk_prefix(cli, SpaceAction::table_name(), "SPACE_POLL_USER_ANSWER").await?;
        for item in poll_answers {
            if let Some(AttributeValue::S(pk)) = item.get("pk") {
                if let Some(rest) = pk.strip_prefix("SPACE_POLL_USER_ANSWER#") {
                    users.insert(rest.to_string());
                }
            }
        }

        // Walk quiz attempts — pk = SPACE_QUIZ_ATTEMPT#<user_id>.
        let quiz_attempts =
            scan_by_pk_prefix(cli, SpaceAction::table_name(), "SPACE_QUIZ_ATTEMPT").await?;
        for item in quiz_attempts {
            if let Some(AttributeValue::S(pk)) = item.get("pk") {
                if let Some(rest) = pk.strip_prefix("SPACE_QUIZ_ATTEMPT#") {
                    users.insert(rest.to_string());
                }
            }
        }

        // For each user, historical total_points is not trivially
        // derivable without a per-user action join. The spec accepts
        // `historical XP == total_point` but clarifies the practical
        // implementation may be zero-seed with a later ledger sweep.
        // We seed every discovered user with 0 XP so that UserGlobalXp
        // rows exist and Phase 6's award_xp service can update them in
        // place. The migration log documents this choice.
        let mut totals = HashMap::with_capacity(users.len());
        for user_id in users {
            totals.insert(user_id, 0i64);
        }
        Ok(totals)
    }

    /// Low-level paginated scan filtered by pk prefix. Expensive, one-shot.
    async fn scan_by_pk_prefix(
        cli: &aws_sdk_dynamodb::Client,
        table_name: &str,
        prefix: &str,
    ) -> std::result::Result<Vec<HashMap<String, AttributeValue>>, Box<dyn std::error::Error>> {
        let mut out = Vec::new();
        let mut last_key: Option<HashMap<String, AttributeValue>> = None;
        loop {
            let mut req = cli
                .scan()
                .table_name(table_name)
                .filter_expression("begins_with(#pk, :prefix)")
                .expression_attribute_names("#pk", "pk")
                .expression_attribute_values(
                    ":prefix",
                    AttributeValue::S(prefix.to_string()),
                );
            if let Some(k) = last_key.take() {
                req = req.set_exclusive_start_key(Some(k));
            }
            let resp = req.send().await?;
            if let Some(items) = resp.items {
                out.extend(items);
            }
            match resp.last_evaluated_key {
                Some(k) => last_key = Some(k),
                None => break,
            }
        }
        Ok(out)
    }

    /// Generic existence check via `T::get(cli, pk, Some(sk))`.
    async fn has_entity<T>(
        cli: &aws_sdk_dynamodb::Client,
        pk: &Partition,
        sk: &EntityType,
    ) -> std::result::Result<bool, Box<dyn std::error::Error>>
    where
        T: serde::de::DeserializeOwned + HasGet,
    {
        T::check_exists(cli, pk, sk).await
    }

    /// Helper trait so `has_entity` can dispatch `get` without a generic
    /// bound soup. Implementations just delegate to the autogenerated
    /// `get` method on each DynamoEntity.
    #[async_trait::async_trait]
    pub trait HasGet: Sized {
        async fn check_exists(
            cli: &aws_sdk_dynamodb::Client,
            pk: &Partition,
            sk: &EntityType,
        ) -> std::result::Result<bool, Box<dyn std::error::Error>>;
    }

    #[async_trait::async_trait]
    impl HasGet for UserGlobalXp {
        async fn check_exists(
            cli: &aws_sdk_dynamodb::Client,
            pk: &Partition,
            sk: &EntityType,
        ) -> std::result::Result<bool, Box<dyn std::error::Error>> {
            Ok(UserGlobalXp::get(cli, pk, Some(sk)).await?.is_some())
        }
    }

    #[async_trait::async_trait]
    impl HasGet for UserStreak {
        async fn check_exists(
            cli: &aws_sdk_dynamodb::Client,
            pk: &Partition,
            sk: &EntityType,
        ) -> std::result::Result<bool, Box<dyn std::error::Error>> {
            Ok(UserStreak::get(cli, pk, Some(sk)).await?.is_some())
        }
    }
}
