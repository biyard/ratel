use super::super::{dto::TeamRewardsResponse, *};
use crate::features::social::types::SocialError;

use crate::features::posts::models::Team;
use crate::features::social::pages::member::dto::TeamRole;

#[get("/api/teams/:team_pk/points?month", user: crate::features::auth::User, team: Team)]
pub async fn get_team_rewards_handler(
    team_pk: TeamPartition,
    month: Option<String>,
) -> Result<TeamRewardsResponse> {
    let cfg = crate::common::CommonConfig::default();
    let team_pk: Partition = team_pk.into();
    let _ = team;
    let _ = user;

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
