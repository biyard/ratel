use crate::common::models::auth::AdminUser;
use crate::features::admin::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackfillUserRewardHistoryGsiResponse {
    pub scanned: usize,
    pub updated: usize,
    pub skipped_already_set: usize,
    /// Rows whose description was resolved this run (subset of `updated`).
    /// Useful for telemetry — if this is consistently < `updated` it means
    /// the SpaceCommon/Post lookup keeps failing and the description is
    /// degrading to `"{space_pk}"` alone.
    pub description_resolved: usize,
}

/// One-shot migration: populate the `find_reward_by_user` GSI key
/// attributes (`gsi1_pk` / `gsi1_sk`) for every existing
/// `UserRewardHistory` row.
///
/// `DynamoEntity`'s `indexed_fields()` writes those attributes on every
/// new `create` / `upsert`, but rows written before the GSI annotation
/// was added lack them, so the GSI projection silently skips them. We
/// re-upsert each row so the populated keys land in the table and the
/// GSI catches up.
///
/// Uses the raw DynamoDB scan because `UserRewardHistory` shares its
/// partition with `UserReward` (aggregate row, sk = `RewardKey` with no
/// TimeKey) and we want to sweep every per-event row across every user.
/// `find_all` / `find_by_*` helpers assume a known pk, which is the
/// wrong shape for a table-wide migration — same reason
/// `backfill_user_team_role` drops to raw scan.
///
/// Idempotent: re-running on a row that already has `gsi1_pk` is a
/// no-op. Paginated via `bookmark` so each invocation stays under the
/// Lambda timeout regardless of table size.
#[post("/api/admin/migrations/user-reward-history-gsi?bookmark", _user: AdminUser)]
pub async fn backfill_user_reward_history_gsi(
    bookmark: Option<String>,
) -> Result<BackfillUserRewardHistoryGsiResponse> {
    use crate::common::models::reward::UserRewardHistory;
    use aws_sdk_dynamodb::types::AttributeValue;

    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let table_name = format!(
        "{}-main",
        std::env::var("DYNAMO_TABLE_PREFIX").unwrap_or_else(|_| "ratel-dev".to_string())
    );

    let mut response = BackfillUserRewardHistoryGsiResponse::default();

    let mut last_key: Option<std::collections::HashMap<String, AttributeValue>> =
        match bookmark.as_deref() {
            Some(bm) => Some(UserRewardHistory::decode_bookmark_all(bm)?),
            None => None,
        };

    loop {
        let mut scan = cli
            .scan()
            .table_name(&table_name)
            // Only per-event UserRewardHistory rows: sk = RewardKey###TimeKey.
            // Aggregate UserReward rows in the same partition have no `###`
            // separator and are filtered out so the deserialize step below
            // doesn't trip on them.
            .filter_expression("contains(sk, :sep)")
            .expression_attribute_values(":sep", AttributeValue::S("###".to_string()))
            .limit(100);

        if let Some(key) = last_key.take() {
            scan = scan.set_exclusive_start_key(Some(key));
        }

        let page = scan.send().await.map_err(|e| {
            crate::error!("scan failed: {e}");
            crate::common::Error::NotFound(format!("scan failed: {e}"))
        })?;

        let items = page.items.clone().unwrap_or_default();

        for item in items {
            // Skip rows that already have *both* GSI key and description.
            // Rows missing either one still go through the upsert path
            // because `description` was added in the same migration cycle
            // as the GSI keys — most legacy rows will be missing both.
            let has_gsi = item.contains_key("gsi1_pk");
            let has_description = item.contains_key("description");
            if has_gsi && has_description {
                response.scanned += 1;
                response.skipped_already_set += 1;
                continue;
            }

            let mut history: UserRewardHistory = match serde_dynamo::from_item(item) {
                Ok(h) => h,
                Err(e) => {
                    crate::error!("failed to deserialize UserRewardHistory: {e}");
                    continue;
                }
            };
            response.scanned += 1;

            // Resolve description if it's still empty. Two GetItems
            // (SpaceCommon → Post) per missing row — acceptable in a
            // one-shot migration, and degrades gracefully when either
            // lookup fails.
            if history.description.is_none() {
                let desc =
                    crate::common::models::reward::resolve_reward_description(cli, &history.sk.0)
                        .await;
                if !desc.is_empty() {
                    history.description = Some(desc);
                    response.description_resolved += 1;
                }
            }

            // `upsert` re-runs `indexed_fields()` so the new gsi1_pk /
            // gsi1_sk attributes get written, *and* the description we
            // just set goes to disk alongside. PutItem-based, idempotent.
            history.upsert(cli).await.map_err(|e| {
                crate::error!("failed to upsert UserRewardHistory: {e}");
                e
            })?;
            response.updated += 1;
        }

        match page.last_evaluated_key {
            Some(key) if !key.is_empty() => last_key = Some(key),
            _ => break,
        }
    }

    tracing::info!(
        scanned = response.scanned,
        updated = response.updated,
        skipped = response.skipped_already_set,
        description_resolved = response.description_resolved,
        "UserRewardHistory GSI + description backfill complete",
    );

    Ok(response)
}
