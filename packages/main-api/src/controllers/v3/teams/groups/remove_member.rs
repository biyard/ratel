#![allow(unused)]
use crate::{AppState, Error2};
use dto::by_axum::{
    auth::Authorization,
    axum::{
        Extension,
        extract::{Json, Path, State},
    },
};
use dto::{JsonSchema, aide, schemars};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, aide::OperationIo, JsonSchema)]
pub struct RemoveMemberPathParams {}

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct RemoveMemberRequest {}

#[derive(Debug, Clone, Serialize, Default, aide::OperationIo, JsonSchema)]
pub struct RemoveMemberResponse {}

pub async fn remove_member_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(auth): Extension<Option<Authorization>>,
    Path(params): Path<RemoveMemberPathParams>,
    Json(req): Json<RemoveMemberRequest>,
) -> Result<Json<RemoveMemberResponse>, Error2> {
    Err(Error2::InternalServerError("Not implemented".into()))
}
