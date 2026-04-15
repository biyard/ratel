use super::super::*;
use crate::common::services::biyard::PointTransactionResponse;
use crate::features::social::types::SocialError;

use crate::features::posts::models::Team;
use crate::features::social::pages::member::dto::TeamRole;

pub type ListPointTransactionsResponse = ListResponse<PointTransactionResponse>;

#[get("/api/teams/:team_pk/points/transactions?month&bookmark", user: crate::features::auth::User, team: Team)]
pub async fn list_team_point_transactions_handler(
    team_pk: TeamPartition,
    month: Option<String>,
    bookmark: Option<String>,
) -> Result<ListPointTransactionsResponse> {
    let cfg = crate::common::CommonConfig::default();
    let team_pk: Partition = team_pk.into();
    let _ = team;
    let _ = user;

    let month = month.unwrap_or_else(|| utils::time::current_month());

    let biyard_service = cfg.biyard();
    let res = biyard_service
        .list_user_transactions(team_pk.clone(), month, bookmark, Some(10))
        .await?;

    Ok(res)
}
