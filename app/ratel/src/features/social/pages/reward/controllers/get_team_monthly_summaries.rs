use super::super::*;
use crate::common::services::MonthlySummariesResponse;
use crate::features::posts::models::Team;

#[get("/api/teams/:team_pk/points/monthly-summaries", user: crate::features::auth::User, team: Team)]
pub async fn get_team_monthly_summaries_handler(
    team_pk: TeamPartition,
) -> Result<MonthlySummariesResponse> {
    let cfg = crate::common::CommonConfig::default();
    let _ = user;
    let _ = team_pk;
    cfg.biyard().get_monthly_summaries(team.pk.clone()).await
}
