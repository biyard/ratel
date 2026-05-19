use crate::common::models::auth::AdminUser;
use crate::features::admin::*;
use crate::features::auth::UserTeam;
use crate::features::posts::models::TeamOwner;
use crate::features::social::pages::member::dto::TeamRole;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackfillTeamOwnerRoleResponse {
    pub team_owners_scanned: usize,
    pub user_team_missing: usize,
    pub skipped_already_set: usize,
    pub updated: usize,
}

/// Backfill `UserTeam.role = Owner` for every team's recorded owner.
///
/// Why TeamOwner-driven instead of UserTeam-driven:
/// `TeamOwner` exists once per team and is the single source of truth for
/// ownership. Scanning these rows is far cheaper than sweeping the entire
/// membership table, and the lookup direction (owner → membership) cannot
/// produce false positives — every TeamOwner row implies exactly one
/// `UserTeam` row that should carry `role = Owner`.
///
/// Idempotency:
/// Per-row `GetItem` checks whether the `role` attribute is *present* on
/// disk (independent of serde's default-on-missing behaviour). Any row
/// that already carries an explicit `role` value — Owner / Admin / Member
/// — is skipped, so a manually-demoted owner is never silently re-promoted.
#[post("/api/admin/migrations/backfill-team-owner-role", _user: AdminUser)]
pub async fn backfill_team_owner_role() -> Result<BackfillTeamOwnerRoleResponse> {
    use aws_sdk_dynamodb::types::AttributeValue;
    use std::collections::HashMap;

    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let table_name = format!(
        "{}-main",
        std::env::var("DYNAMO_TABLE_PREFIX").unwrap_or_else(|_| "ratel-dev".to_string())
    );

    let mut response = BackfillTeamOwnerRoleResponse::default();
    let mut last_key: Option<HashMap<String, AttributeValue>> = None;

    loop {
        // Scan filter: every TeamOwner row has `sk = "TEAM_OWNER"` (unit
        // EntityType variant). DynamoDB doesn't accept reserved word `sk`
        // directly in filter_expression, so alias it via #s.
        let mut scan = cli
            .scan()
            .table_name(&table_name)
            .filter_expression("#s = :owner_sk")
            .expression_attribute_names("#s", "sk")
            .expression_attribute_values(
                ":owner_sk",
                AttributeValue::S("TEAM_OWNER".to_string()),
            )
            .limit(100);
        if let Some(key) = last_key.take() {
            scan = scan.set_exclusive_start_key(Some(key));
        }

        let page = scan.send().await.map_err(|e| {
            crate::error!("scan failed: {e}");
            crate::common::Error::NotFound(format!("scan failed: {e}"))
        })?;

        for item in page.items.unwrap_or_default() {
            response.team_owners_scanned += 1;

            let team_owner: TeamOwner = match serde_dynamo::from_item(item) {
                Ok(t) => t,
                Err(e) => {
                    crate::error!("failed to deserialize TeamOwner: {e}");
                    continue;
                }
            };

            // Derive the membership row's sort key from the team's pk.
            // The UserTeam sk format is `USER_TEAM#{team_pk_string}`.
            let user_team_sk = crate::common::types::EntityType::UserTeam(
                team_owner.pk.to_string(),
            );

            // Raw GetItem so we can inspect whether `role` is physically
            // present on the stored row — `UserTeam` model deserialization
            // would mask this via `#[serde(default)]`.
            let get_result = cli
                .get_item()
                .table_name(&table_name)
                .key("pk", AttributeValue::S(team_owner.user_pk.to_string()))
                .key("sk", AttributeValue::S(user_team_sk.to_string()))
                .send()
                .await
                .map_err(|e| {
                    crate::error!(
                        "UserTeam get_item failed for user={} team={}: {e}",
                        team_owner.user_pk,
                        team_owner.pk,
                    );
                    crate::common::Error::NotFound(format!("get failed: {e}"))
                })?;

            let raw = match get_result.item {
                Some(it) => it,
                None => {
                    // TeamOwner exists but no membership row — data
                    // inconsistency from legacy. Log + skip rather than
                    // upserting a partial row.
                    tracing::warn!(
                        user_pk = %team_owner.user_pk,
                        team_pk = %team_owner.pk,
                        "TeamOwner has no matching UserTeam row — skipping"
                    );
                    response.user_team_missing += 1;
                    continue;
                }
            };

            let has_role_attr = raw
                .get("role")
                .map(|v| !matches!(v, AttributeValue::Null(_)))
                .unwrap_or(false);
            if has_role_attr {
                response.skipped_already_set += 1;
                continue;
            }

            UserTeam::updater(&team_owner.user_pk, &user_team_sk)
                .with_role(TeamRole::Owner)
                .execute(cli)
                .await
                .map_err(|e| {
                    crate::error!(
                        "UserTeam update failed for user={} team={}: {e}",
                        team_owner.user_pk,
                        team_owner.pk,
                    );
                    crate::common::Error::NotFound(format!("update failed: {e}"))
                })?;
            response.updated += 1;
        }

        match page.last_evaluated_key {
            Some(key) if !key.is_empty() => last_key = Some(key),
            _ => break,
        }
    }

    tracing::info!(
        team_owners_scanned = response.team_owners_scanned,
        updated = response.updated,
        skipped_already_set = response.skipped_already_set,
        user_team_missing = response.user_team_missing,
        "TeamOwner-driven role backfill complete"
    );

    Ok(response)
}
