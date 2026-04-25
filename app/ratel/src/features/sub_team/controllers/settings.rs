use crate::common::*;
use crate::features::posts::models::Team;
use crate::features::social::pages::member::dto::TeamRole;
use crate::features::sub_team::types::{SubTeamError, SubTeamSettingsResponse, UpdateSubTeamSettingsRequest};

#[patch("/api/teams/:team_pk/sub-teams/settings", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn update_sub_team_settings_handler(
    team_pk: TeamPartition,
    body: UpdateSubTeamSettingsRequest,
) -> Result<SubTeamSettingsResponse> {
    let _ = team_pk;
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }

    let _ = user;

    let mut team = team;
    let now = crate::common::utils::time::get_now_timestamp_millis();

    let mut updater = Team::updater(&team.pk, EntityType::Team).with_updated_at(now);
    let mut changed = false;

    if let Some(flag) = body.is_parent_eligible {
        updater = updater.with_is_parent_eligible(flag);
        team.is_parent_eligible = flag;
        changed = true;
    }

    if let Some(min_members) = body.min_sub_team_members {
        let clamped = min_members.max(0);
        updater = updater.with_min_sub_team_members(clamped);
        team.min_sub_team_members = clamped;
        changed = true;
    }

    if changed {
        updater.execute(cli).await.map_err(|e| {
            crate::error!("update_sub_team_settings execute failed: {e}");
            SubTeamError::ApplicationStateMismatch
        })?;
    }

    Ok(SubTeamSettingsResponse {
        is_parent_eligible: team.is_parent_eligible,
        min_sub_team_members: team.min_sub_team_members,
    })
}
