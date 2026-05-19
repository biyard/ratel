use crate::common::models::auth::AdminUser;
use crate::features::admin::*;

#[cfg(feature = "server")]
fn is_present_string(
    item: &std::collections::HashMap<String, aws_sdk_dynamodb::types::AttributeValue>,
    key: &str,
) -> bool {
    matches!(item.get(key), Some(aws_sdk_dynamodb::types::AttributeValue::S(s)) if !s.is_empty())
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackfillUserRewardHistoryResponse {
    pub scanned: usize,
    pub updated: usize,
    pub skipped_already_complete: usize,
    pub description_resolved: usize,
    pub action_name_resolved: usize,
    pub transaction_matched: usize,
    pub no_biyard_match: usize,
    pub biyard_scanned: usize,
    pub biyard_indexed: usize,
}

#[post(
    "/api/admin/migrations/user-reward-history-gsi?biyard_table&project_id",
    _user: AdminUser
)]
pub async fn backfill_user_reward_history_gsi(
    biyard_table: Option<String>,
    project_id: Option<String>,
) -> Result<BackfillUserRewardHistoryResponse> {
    use crate::common::models::reward::UserRewardHistory;
    use aws_sdk_dynamodb::types::AttributeValue;
    use std::collections::HashMap;

    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let biyard_table = biyard_table.unwrap_or_else(|| "biyard-prod-main".to_string());
    // Default the project_id from the configured `BiyardService`
    // (which loaded it from `BIYARD_PROJECT_ID` at build time). Admin
    // can still override via `?project_id=...` for cross-project
    // sweeps if ever needed.
    let project_id_raw = project_id
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| common_config.biyard().project_id().to_string());
    if project_id_raw.is_empty() {
        return Err(crate::common::Error::NotFound(
            "project_id is required (set BIYARD_PROJECT_ID or pass ?project_id=...)".to_string(),
        ));
    }
    // Biyard stores `project_id` as `PROJECT#{uuid}` in the
    // PointTransaction rows, but env / ratel side usually carries the
    // bare uuid (e.g. `BIYARD_PROJECT_ID=019d6c77-...`). Normalize so
    // either form passes through the filter; idempotent if the prefix
    // is already present.
    let project_id_value = if project_id_raw.starts_with("PROJECT#") {
        project_id_raw
    } else {
        format!("PROJECT#{}", project_id_raw)
    };

    let table_name = format!(
        "{}-main",
        std::env::var("DYNAMO_TABLE_PREFIX").unwrap_or_else(|_| "ratel-dev".to_string())
    );

    let mut response = BackfillUserRewardHistoryResponse::default();

    // ---- Step 1: build the Biyard index ----------------------------
    //
    // meta_user_id → Vec<(created_at, sk, month)>
    //
    // We store the full Biyard sk verbatim because that's what callers
    // (e.g. the rewards page list) expect as `transaction_id`. The
    // shape is `POINT_TRANSACTION#{YYYY-MM}#{uuid_v7}`.
    let mut user_transactions: HashMap<String, Vec<(i64, String, String)>> = HashMap::new();
    let mut biyard_last_key: Option<HashMap<String, AttributeValue>> = None;

    loop {
        let mut scan = cli
            .scan()
            .table_name(&biyard_table)
            .filter_expression("project_id = :pid AND begins_with(sk, :sk_prefix)")
            .expression_attribute_values(":pid", AttributeValue::S(project_id_value.clone()))
            .expression_attribute_values(
                ":sk_prefix",
                AttributeValue::S("POINT_TRANSACTION#".to_string()),
            )
            .limit(500);

        if let Some(k) = biyard_last_key.take() {
            scan = scan.set_exclusive_start_key(Some(k));
        }

        let page = scan.send().await.map_err(|e| {
            crate::error!("biyard scan failed: {e}");
            crate::common::Error::NotFound(format!("biyard scan failed: {e}"))
        })?;

        for item in page.items.unwrap_or_default() {
            response.biyard_scanned += 1;

            let meta_user_id = item
                .get("meta_user_id")
                .and_then(|v| v.as_s().ok())
                .cloned();
            let created_at = item
                .get("created_at")
                .and_then(|v| v.as_n().ok())
                .and_then(|s| s.parse::<i64>().ok());
            let sk = item.get("sk").and_then(|v| v.as_s().ok()).cloned();
            let month = item.get("month").and_then(|v| v.as_s().ok()).cloned();

            if let (Some(uid), Some(ts), Some(tx_sk), Some(m)) =
                (meta_user_id, created_at, sk, month)
            {
                user_transactions
                    .entry(uid)
                    .or_default()
                    .push((ts, tx_sk, m));
                response.biyard_indexed += 1;
            }
        }

        match page.last_evaluated_key {
            Some(k) if !k.is_empty() => biyard_last_key = Some(k),
            _ => break,
        }
    }

    // Sort each user's Biyard tx list ascending by created_at — required
    // for the 1:1 zip match in step 3 (matching ratel's nth-oldest row
    // with biyard's nth-oldest transaction).
    for txs in user_transactions.values_mut() {
        txs.sort_by_key(|(ts, _, _)| *ts);
    }

    tracing::info!(
        biyard_scanned = response.biyard_scanned,
        biyard_indexed = response.biyard_indexed,
        unique_users = user_transactions.len(),
        "Biyard PointTransaction index built",
    );

    // ---- Step 2: collect every UserRewardHistory row in memory ------
    //
    // We can't 1:1 match while paginating because a user's history rows
    // can land across multiple scan pages — we need all rows for a user
    // in hand before we can sort & zip them against the user's Biyard
    // tx list. Rows that are already fully populated are skipped here so
    // they don't take up memory.
    let mut pending_histories: Vec<UserRewardHistory> = Vec::new();
    let mut last_key: Option<HashMap<String, AttributeValue>> = None;
    loop {
        let mut scan = cli
            .scan()
            .table_name(&table_name)
            // Per-event UserRewardHistory rows only — aggregate
            // UserReward rows share the partition but have no `###`
            // separator in sk.
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

        for item in page.items.unwrap_or_default() {
            response.scanned += 1;

            match serde_dynamo::from_item::<_, UserRewardHistory>(item) {
                Ok(h) => pending_histories.push(h),
                Err(e) => {
                    crate::error!("failed to deserialize UserRewardHistory: {e}");
                }
            }
        }

        match page.last_evaluated_key {
            Some(key) if !key.is_empty() => last_key = Some(key),
            _ => break,
        }
    }

    tracing::info!(
        ratel_scanned = response.scanned,
        ratel_pending = pending_histories.len(),
        skipped_already_complete = response.skipped_already_complete,
        "UserRewardHistory rows collected for backfill",
    );

    let mut histories_by_user: HashMap<String, Vec<UserRewardHistory>> = HashMap::new();
    let mut non_user_target: Vec<UserRewardHistory> = Vec::new();
    for h in pending_histories.into_iter() {
        match &h.pk.0 {
            Partition::User(u) | Partition::Team(u) => {
                histories_by_user.entry(u.clone()).or_default().push(h);
            }
            _ => non_user_target.push(h),
        }
    }

    for (user, hist_list) in histories_by_user.iter_mut() {
        hist_list.sort_by_key(|h| h.created_at);

        let tx_list = user_transactions.get(user);
        let tx_count = tx_list.map(|v| v.len()).unwrap_or(0);

        for (idx, history) in hist_list.iter_mut().enumerate() {
            // Always overwrite — every history row gets re-matched on
            // each run. Idempotent for stable data, and lets a re-run
            // correct any drift from earlier partial backfills.
            if idx < tx_count {
                let (_, tx_sk, month) = &tx_list.unwrap()[idx];
                history.transaction_id = Some(tx_sk.clone());
                history.month = Some(month.clone());
                response.transaction_matched += 1;
            } else {
                response.no_biyard_match += 1;
            }
        }
    }

    // ---- Step 4: resolve description + upsert -----------------------
    let mut to_upsert: Vec<UserRewardHistory> = histories_by_user
        .into_values()
        .flatten()
        .chain(non_user_target.into_iter())
        .collect();

    for history in to_upsert.iter_mut() {
        if history.description.is_none() {
            let desc =
                crate::common::models::reward::resolve_reward_description(cli, &history.sk.0).await;
            if !desc.is_empty() {
                history.description = Some(desc);
                response.description_resolved += 1;
            }
        }

        // Behavior label (e.g. "투표 응답", "토론 댓글") rendered from
        // `sk.0.behavior` — no DB hit, no `is_none` guard. Earlier
        // backfill passes wrote `SpaceAction.title` here (e.g. the
        // post title for a quiz row); each run now reseeds the field
        // to the canonical behavior label so stale strings get
        // overwritten.
        let name = crate::common::models::reward::resolve_action_name(&history.sk.0);
        if !name.is_empty() {
            history.action_name = Some(name);
            response.action_name_resolved += 1;
        }

        // `upsert` re-runs `indexed_fields()` so any missing
        // `gsi1_pk` / `gsi1_sk` get populated at the same time as
        // description / transaction_id / month.
        history.upsert(cli).await.map_err(|e| {
            crate::error!("failed to upsert UserRewardHistory: {e}");
            e
        })?;
        response.updated += 1;
    }

    tracing::info!(
        scanned = response.scanned,
        updated = response.updated,
        skipped = response.skipped_already_complete,
        description_resolved = response.description_resolved,
        action_name_resolved = response.action_name_resolved,
        transaction_matched = response.transaction_matched,
        no_biyard_match = response.no_biyard_match,
        "UserRewardHistory backfill (GSI + description + action_name + transaction_id) complete",
    );

    Ok(response)
}
