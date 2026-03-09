use super::super::*;
use common::services::biyard::PointTransactionResponse;

use ratel_post::types::{TeamGroupPermission, TeamGroupPermissions};

pub type ListPointTransactionsResponse = ListResponse<PointTransactionResponse>;

#[get("/api/teams/:team_pk/points/transactions?month&bookmark", user: ratel_auth::User, permissions: TeamGroupPermissions)]
pub async fn list_team_point_transactions_handler(
    team_pk: TeamPartition,
    month: Option<String>,
    bookmark: Option<String>,
) -> Result<ListPointTransactionsResponse> {
    let cfg = common::CommonConfig::default();
    let team_pk: Partition = team_pk.into();
    let can_view = permissions.contains(TeamGroupPermission::TeamAdmin);
    if !can_view {
        return Err(Error::Unauthorized(
            "You don't have permission to view team rewards.".to_string(),
        ));
    }

    let month = month.unwrap_or_else(|| utils::time::current_month());

    let biyard_service = cfg.biyard();
    let res = biyard_service
        .list_user_transactions(team_pk.clone(), month, bookmark, Some(10))
        .await?;

    Ok(res)
}
