use crate::{dto::TeamRewardsResponse, *};
use common::services::BiyardService;

use ratel_post::models::Team;
use ratel_post::types::{TeamGroupPermission, TeamGroupPermissions};

#[get("/api/teams/:team_pk/points?month", user: ratel_auth::User)]
pub async fn get_team_rewards_handler(
    team_pk: TeamPartition,
    month: Option<String>,
) -> Result<TeamRewardsResponse> {
    let cfg = common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let team_pk: Partition = team_pk.into();

    let permissions = Team::get_permissions_by_team_pk(cli, &team_pk, &user.pk)
        .await
        .unwrap_or_else(|_| TeamGroupPermissions::empty());
    let can_view = permissions.contains(TeamGroupPermission::TeamAdmin);
    if !can_view {
        return Err(Error::Unauthorized(
            "You don't have permission to view team rewards.".to_string(),
        ));
    }

    let month = month.unwrap_or_else(|| utils::time::current_month());

    let biyard_service = cfg.biyard();
    let balance = biyard_service
        .get_user_balance(team_pk.clone(), month.clone())
        .await?;
    let token = biyard_service.get_project_info().await?;

    Ok(TeamRewardsResponse {
        month,
        project_name: token.name,
        token_symbol: token.symbol,
        total_points: balance.project_total_points,
        team_points: balance.balance,
        monthly_token_supply: balance.monthly_token_supply,
    })
}
