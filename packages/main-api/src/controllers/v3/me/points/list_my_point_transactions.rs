use crate::{
    AppState, Error,
    models::{
        User,
        user::{UserDetailResponse, UserMetadata},
    },
    services::biyard::{UserPointBalanceResponse, UserPointTransactionResponse},
    types::{ListItemsQuery, ListItemsResponse, Pagination, Partition},
};
use aide::NoApi;
use axum::{
    Extension, Json,
    extract::{Query, State},
};
use bdk::prelude::*;

#[derive(
    serde::Deserialize,
    serde::Serialize,
    Debug,
    Clone,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct ListMyRewardsQuery {
    pub month: String, // e.g., "2024-06"
}

pub async fn list_my_point_transactions_handler(
    State(AppState { biyard, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Query(Pagination { bookmark }): ListItemsQuery,
    Query(ListMyRewardsQuery { month }): Query<ListMyRewardsQuery>,
) -> Result<Json<ListItemsResponse<UserPointTransactionResponse>>, Error> {
    let res = biyard
        .get_user_transactions(user.pk, month, bookmark, None)
        .await?;
    tracing::debug!("User Point Transactions {:?}", res.items);
    Ok(Json(res))
}
