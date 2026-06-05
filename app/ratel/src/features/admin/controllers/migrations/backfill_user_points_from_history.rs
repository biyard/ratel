use crate::common::models::auth::AdminUser;
use crate::features::admin::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackfillUserPointsResponse {
    /// Total UserRewardHistory rows scanned (per-event log).
    pub history_rows_scanned: usize,
    /// Rows we couldn't deserialize into `UserRewardHistory`.
    pub history_rows_skipped_invalid: usize,
    /// Distinct user pks with a non-zero summed `point`.
    pub users_with_balance: usize,
    /// User rows whose `points` field was updated.
    pub updated: usize,
    /// Users whose `User.points` was already > 0 — left untouched unless
    /// `overwrite=true`.
    pub skipped_already_set: usize,
    /// Users whose pk has no matching ratel User row.
    pub skipped_user_not_found: usize,
    /// Users whose summed history balance came out to 0 or negative.
    pub skipped_zero_balance: usize,
    /// Sum of all point totals written across all users (sanity total).
    pub total_points_written: i64,
    /// When set, no writes actually happen — the response is what *would*
    /// have been written.
    pub dry_run: bool,
}

/// Backfill `User.points` from local `UserRewardHistory` rows.
///
/// Why this exists: with the Scope-A refactor, `User.points` became the
/// authoritative current balance on ratel — credited atomically with
/// every reward write, debited by Launchpad conversions. Users who
/// earned points BEFORE that refactor have their points only in the
/// event log (`UserRewardHistory`) — never propagated to `User.points`.
/// This migration sums every per-event `point` per user and writes the
/// total to `User.points`, so the ratel console can serve balances
/// without ever calling the (now-disabled) biyard console.
///
/// Why not biyard: the biyard monthly aggregate (`MONTH#{YYYY-MM}`) and
/// the local event log already share the same data — every Scope-A
/// award writes both rows in lockstep. The event log is what the
/// rewards page already renders ("REWARD HISTORY"), so summing it
/// guarantees the displayed balance equals the credited balance.
///
/// UserRewardHistory event-row layout:
/// - `pk    = USER#{uuid}##REWARD` (or `TEAM#{uuid}##REWARD`)
/// - `sk    = SPACE_REWARD##{...}###{period}`  ← `###` distinguishes
///                event rows from aggregate `UserReward` rows.
/// - `point: i64` — amount credited on this event.
///
/// Idempotent by default: users whose `User.points` is already > 0
/// (either Scope-A activity or a previous run) are skipped. Pass
/// `overwrite=true` only when you've reconciled that the recomputed
/// total is what should land — it WILL destroy any Launchpad-side
/// debits applied to `User.points` since the last reward award.
#[post(
    "/api/admin/migrations/backfill-user-points-from-history?dry_run&overwrite",
    _user: AdminUser
)]
pub async fn backfill_user_points_from_history(
    dry_run: Option<bool>,
    overwrite: Option<bool>,
) -> Result<BackfillUserPointsResponse> {
    use crate::common::models::auth::User;
    use crate::common::models::reward::UserRewardHistory;
    use crate::common::types::{EntityType, Partition};
    use aws_sdk_dynamodb::types::AttributeValue;
    use std::collections::HashMap;

    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let table_name = format!(
        "{}-main",
        std::env::var("DYNAMO_TABLE_PREFIX").unwrap_or_else(|_| "ratel-dev".to_string())
    );

    let dry_run = dry_run.unwrap_or(false);
    let overwrite = overwrite.unwrap_or(false);

    let mut response = BackfillUserPointsResponse {
        dry_run,
        ..Default::default()
    };

    // ---- Step 1: scan ratel main, sum `point` per pk ----------------
    //
    // Per-event UserRewardHistory rows have `###` somewhere in `sk`
    // (the period separator); the aggregate UserReward rows that share
    // the same partition do NOT. This is the exact filter the
    // `backfill_user_reward_history_gsi` migration uses to single out
    // event rows; reusing it keeps the two migrations in sync.
    //
    // We deserialize each item to `UserRewardHistory` so we get a
    // typed `pk: CompositePartition<Partition, Partition>` and `point:
    // i64` instead of fishing through raw `AttributeValue`s.
    //
    // Keyed by raw UUID string (not `Partition`) because `Partition`
    // doesn't derive `Hash`. We carry only USER-target rows forward;
    // TEAM rows are filtered out at insert time.
    let mut totals: HashMap<String, i64> = HashMap::new();
    let mut last_key: Option<HashMap<String, AttributeValue>> = None;

    loop {
        let mut scan = cli
            .scan()
            .table_name(&table_name)
            .filter_expression("contains(sk, :sep)")
            .expression_attribute_values(":sep", AttributeValue::S("###".to_string()))
            .limit(200);

        if let Some(k) = last_key.take() {
            scan = scan.set_exclusive_start_key(Some(k));
        }

        let page = scan.send().await.map_err(|e| {
            crate::error!("ratel main scan failed: {e}");
            crate::common::Error::NotFound(format!("ratel main scan failed: {e}"))
        })?;

        for item in page.items.unwrap_or_default() {
            response.history_rows_scanned += 1;

            match serde_dynamo::from_item::<_, UserRewardHistory>(item) {
                Ok(h) => {
                    // h.pk = CompositePartition(target_pk, Partition::Reward).
                    // We key totals by the *target* UUID (USER only —
                    // teams have their own balance migration when needed).
                    if let Partition::User(uid) = &h.pk.0 {
                        let slot = totals.entry(uid.clone()).or_insert(0);
                        *slot = slot.saturating_add(h.point);
                    }
                }
                Err(e) => {
                    response.history_rows_skipped_invalid += 1;
                    tracing::debug!(error = %e, "failed to deserialize UserRewardHistory");
                }
            }
        }

        match page.last_evaluated_key {
            Some(k) if !k.is_empty() => last_key = Some(k),
            _ => break,
        }
    }

    response.users_with_balance = totals.values().filter(|v| **v > 0).count();

    tracing::info!(
        scanned = response.history_rows_scanned,
        skipped_invalid = response.history_rows_skipped_invalid,
        unique_targets = response.users_with_balance,
        dry_run = dry_run,
        overwrite = overwrite,
        "UserRewardHistory rows aggregated for backfill",
    );

    // ---- Step 2: for each target pk, decide and (optionally) write --
    //
    // We only touch `User` rows here. Team rewards live on a separate
    // `Team.points` field and are populated via the same Scope-A path;
    // back-filling team balances is a sibling migration if/when it's
    // needed.
    let now = crate::common::utils::time::get_now_timestamp_millis();

    for (user_uuid, total) in totals.iter() {
        if *total <= 0 {
            response.skipped_zero_balance += 1;
            continue;
        }

        let target_pk = Partition::User(user_uuid.clone());

        let user = match User::get(cli, target_pk.clone(), Some(&EntityType::User)).await {
            Ok(Some(u)) => u,
            Ok(None) => {
                response.skipped_user_not_found += 1;
                tracing::debug!(target_pk = %target_pk, "user row not found in ratel");
                continue;
            }
            Err(e) => {
                crate::error!("User::get failed for {target_pk}: {e}");
                response.skipped_user_not_found += 1;
                continue;
            }
        };

        // Idempotency guard. A user with `points > 0` either (a) already
        // ran through this migration, (b) has Scope-A credits we'd be
        // about to clobber with a re-sum that includes those same
        // events again, or (c) has a Launchpad debit applied to a
        // historical balance. Re-running without `overwrite=true` is
        // safe and a no-op for them; with `overwrite=true` we set
        // points to the full history sum, which:
        //   - DOES correctly include every reward ever earned,
        //   - DOES undo any Launchpad debit since the corresponding
        //     award (because debits aren't written to history).
        if user.points > 0 && !overwrite {
            response.skipped_already_set += 1;
            continue;
        }

        if dry_run {
            response.updated += 1;
            response.total_points_written = response.total_points_written.saturating_add(*total);
            continue;
        }

        if let Err(e) = User::updater(target_pk.clone(), EntityType::User)
            .with_points(*total)
            .with_updated_at(now)
            .execute(cli)
            .await
        {
            crate::error!(
                "User::updater(with_points) failed for {target_pk}: {e}"
            );
            continue;
        }

        response.updated += 1;
        response.total_points_written = response.total_points_written.saturating_add(*total);
    }

    tracing::info!(
        updated = response.updated,
        skipped_already_set = response.skipped_already_set,
        skipped_user_not_found = response.skipped_user_not_found,
        skipped_zero_balance = response.skipped_zero_balance,
        total_points_written = response.total_points_written,
        dry_run = dry_run,
        "User.points backfill complete",
    );

    Ok(response)
}
