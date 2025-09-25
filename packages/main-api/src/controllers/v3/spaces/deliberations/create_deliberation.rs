#![allow(unused)]
use crate::{
    AppState, Error2, models::space::DeliberationSpace, utils::dynamo_extractor::extract_user,
};
use dto::by_axum::{
    auth::Authorization,
    axum::{
        Extension,
        extract::{Json, State},
    },
};
use dto::{JsonSchema, aide, schemars};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema, Validate)]
pub struct CreateDeliberationRequest {}

#[derive(Debug, Clone, Serialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreateDeliberationResponse {
    pub is_successed: bool,
}

//FIXME: implement this handler
pub async fn create_deliberation_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(auth): Extension<Option<Authorization>>,
    Json(req): Json<CreateDeliberationRequest>,
) -> Result<Json<CreateDeliberationResponse>, Error2> {
    let user = extract_user(&dynamo.client, auth).await?;

    let deliberation = DeliberationSpace::new(user);
    deliberation.create(&dynamo.client).await?;

    Ok(Json(CreateDeliberationResponse { is_successed: true }))
}
