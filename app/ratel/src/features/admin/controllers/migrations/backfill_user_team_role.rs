use crate::common::models::auth::AdminUser;
use crate::features::admin::*;
use crate::features::auth::{UserTeam, UserTeamGroup};
use crate::features::posts::models::TeamOwner;
use crate::features::posts::types::TeamGroupPermissions;
use crate::features::social::pages::member::dto::TeamRole;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackfillUserTeamRoleResponse {
    pub scanned: usize,
    pub updated: usize,
    pub owners_set: usize,
    pub admins_set: usize,
    pub members_set: usize,
    pub skipped_already_set: usize,
}

/// One-shot migration that populates `UserTeam.role` for every membership.
///
/// - If the user is the team's `TeamOwner` → `TeamRole::Owner`
/// - Else combine all of the user's `UserTeamGroup.team_group_permissions`
///   for this team:
///     - `TeamAdmin` bit set → `Owner` (defensive; TeamOwner should cover this)
///     - `TeamEdit` bit set → `Admin`
///     - otherwise          → `Member`
///
/// Idempotent: if a record already has a non-default role the migration
/// does not overwrite it unless `overwrite=true` is passed.
#[post("/api/admin/migrations/user-team-role?overwrite", _user: AdminUser)]
pub async fn backfill_user_team_role(
    overwrite: Option<bool>,
) -> Result<BackfillUserTeamRoleResponse> {
    use crate::features::posts::types::TeamGroupPermission;

    let overwrite = overwrite.unwrap_or(false);
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let table_name = format!(
        "{}-main",
        std::env::var("DYNAMO_TABLE_PREFIX").unwrap_or_else(|_| "ratel-dev".to_string())
    );

    let mut response = BackfillUserTeamRoleResponse::default();

    // Scan the entire table for UserTeam records (sk begins_with USER_TEAM#).
    // We use the raw DynamoDB scan because DynamoEntity query helpers assume a
    // known pk, but here we need to sweep every row.
    let mut last_key: Option<std::collections::HashMap<String, aws_sdk_dynamodb::types::AttributeValue>> = None;
    loop {
        let mut scan = cli
            .scan()
            .table_name(&table_name)
            .filter_expression("begins_with(sk, :prefix)")
            .expression_attribute_values(
                ":prefix",
                aws_sdk_dynamodb::types::AttributeValue::S("USER_TEAM#".to_string()),
            )
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
            // Deserialize into UserTeam via the DynamoEntity helper — if the
            // record was written before `role` existed, serde will default to
            // Member.
            let user_team: UserTeam = match serde_dynamo::from_item(item.clone()) {
                Ok(ut) => ut,
                Err(e) => {
                    crate::error!("failed to deserialize UserTeam: {e}");
                    continue;
                }
            };
            response.scanned += 1;

            if !overwrite && !matches!(user_team.role, TeamRole::Member) {
                // Already has a non-default role; skip.
                response.skipped_already_set += 1;
                continue;
            }

            // Extract team_pk from sk (EntityType::UserTeam(team_pk_str)).
            let team_pk_str = match &user_team.sk {
                crate::common::types::EntityType::UserTeam(s) => s.clone(),
                _ => continue,
            };
            let team_pk: Partition = team_pk_str.parse().unwrap_or_default();

            // Owner check.
            let mut role = TeamRole::Member;
            if let Ok(Some(owner)) =
                TeamOwner::get(cli, &team_pk, Some(&EntityType::TeamOwner)).await
            {
                if owner.user_pk == user_team.pk {
                    role = TeamRole::Owner;
                }
            }

            // If not owner, combine UserTeamGroup permissions to derive role.
            // Owner is authoritative only via the TeamOwner record — the
            // TeamAdmin permission bit was granted to every member of the
            // legacy `admin_group`, so treating that bit as Owner would
            // incorrectly promote every admin_group member.
            if matches!(role, TeamRole::Member) {
                let opt = UserTeamGroup::opt().sk(user_team.pk.to_string()).limit(50);
                if let Ok((groups, _)) =
                    UserTeamGroup::find_by_team_pk(cli, team_pk.clone(), opt).await
                {
                    let mut mask = 0i64;
                    for g in groups {
                        mask |= g.team_group_permissions;
                    }
                    let perms: TeamGroupPermissions = mask.into();
                    if perms.contains(TeamGroupPermission::TeamAdmin)
                        || perms.contains(TeamGroupPermission::TeamEdit)
                        || perms.contains(TeamGroupPermission::GroupEdit)
                    {
                        role = TeamRole::Admin;
                    }
                }
            }

            if matches!(role, TeamRole::Member) && matches!(user_team.role, TeamRole::Member) {
                // Nothing to change.
                continue;
            }

            UserTeam::updater(&user_team.pk, &user_team.sk)
                .with_role(role)
                .execute(cli)
                .await
                .map_err(|e| {
                    crate::error!("failed to update UserTeam.role: {e}");
                    crate::common::Error::NotFound(format!("update failed: {e}"))
                })?;

            response.updated += 1;
            match role {
                TeamRole::Owner => response.owners_set += 1,
                TeamRole::Admin => response.admins_set += 1,
                TeamRole::Member => response.members_set += 1,
            }
        }

        match page.last_evaluated_key {
            Some(key) if !key.is_empty() => last_key = Some(key),
            _ => break,
        }
    }

    Ok(response)
}
