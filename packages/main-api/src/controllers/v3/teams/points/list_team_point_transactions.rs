use crate::{
    controllers::v3::teams::dto::TeamPathParam, services::biyard::UserPointTransactionResponse,
    types::*, utils::time as time_utils, *,
};
use aide::NoApi;
use axum::{Json, extract::Query, extract::State};
use bdk::prelude::*;

pub async fn list_team_point_transactions_handler(
    State(AppState { biyard, .. }): State<AppState>,
    NoApi(_user): NoApi<User>,
    Path(TeamPathParam { team_pk }): Path<TeamPathParam>,
    Query(TransactionsQuery { pagination, month }): Query<TransactionsQuery>,
) -> Result<Json<ListItemsResponse<UserPointTransactionResponse>>> {
    let month = month.unwrap_or_else(|| time_utils::current_month());
    let bookmark = pagination.bookmark;

    let res = biyard
        .list_user_transactions(team_pk.clone(), month, bookmark, None)
        .await?;

    tracing::debug!("Team Point Transactions {:?}", res.items);
    Ok(Json(res))
}
