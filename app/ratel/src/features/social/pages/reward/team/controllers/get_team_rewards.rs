use super::super::{dto::TeamRewardsResponse, *};
use crate::features::social::types::SocialError;

use crate::features::posts::models::Team;
use crate::features::social::pages::member::dto::TeamRole;

#[get("/api/teams/:team_pk/points?month", user: crate::features::auth::User, team: Team)]
pub async fn get_team_rewards_handler(
    team_pk: TeamPartition,
    month: Option<String>,
) -> Result<TeamRewardsResponse> {
    let _ = user;
    let _ = team_pk;

    let month = month.unwrap_or_else(|| utils::time::current_month());

    // Scope-A: team balance is the local `Team.points` (the `team`
    // extractor already loaded the row). No console (Biyard) reads.
    let team_points = team.points;

    Ok(TeamRewardsResponse {
        month,
        project_name: String::new(),
        token_symbol: "RATEL".to_string(),
        total_points: team_points.max(1),
        team_points,
        monthly_token_supply: 0,
    })
}
