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
pub struct AddMemberPathParams {}

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct AddMemberRequest {}

pub async fn add_member_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(auth): Extension<Option<Authorization>>,
    Path(params): Path<AddMemberPathParams>,
    Json(req): Json<AddMemberRequest>,
) -> Result<(), Error2> {
    Err(Error2::InternalServerError("Not implemented".into()))
}
