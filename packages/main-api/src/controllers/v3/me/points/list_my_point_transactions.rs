use crate::{
    AppState, Error,
    models::User,
    services::biyard::UserPointTransactionResponse,
    types::{ListItemsResponse, TransactionsQuery},
    utils::time as time_utils,
};
use aide::NoApi;
use axum::{
    Json,
    extract::{Query, State},
};
use bdk::prelude::*;

pub async fn list_my_point_transactions_handler(
    State(AppState { biyard, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Query(TransactionsQuery { pagination, month }): Query<TransactionsQuery>,
) -> Result<Json<ListItemsResponse<UserPointTransactionResponse>>, Error> {
    let month = month.unwrap_or_else(|| time_utils::current_month());
    let bookmark = pagination.bookmark;

    let res = biyard
        .list_user_transactions(user.pk, month, bookmark, None)
        .await?;

    tracing::debug!("User Point Transactions {:?}", res.items);
    Ok(Json(res))
}
