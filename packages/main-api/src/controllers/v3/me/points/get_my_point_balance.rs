use crate::{
    AppState, Error,
    models::{
        User,
        user::{UserDetailResponse, UserMetadata},
    },
    services::biyard::UserPointBalanceResponse,
    types::Partition,
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
pub struct GetMyRewardsQuery {
    pub month: String, // e.g., "2024-06"
}

pub async fn get_my_point_balance_handler(
    State(AppState { biyard, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Query(GetMyRewardsQuery { month }): Query<GetMyRewardsQuery>,
) -> Result<Json<UserPointBalanceResponse>, Error> {
    let res = biyard.get_user_balance(user.pk, month).await?;
    tracing::debug!("User Balance {:?}", res);
    Ok(Json(res))
}
