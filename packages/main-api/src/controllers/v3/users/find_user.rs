use crate::{
    AppState, Error,
    models::user::{User, UserDetailResponse, UserMetadata, UserPhoneNumber},
};
use bdk::prelude::*;
use by_axum::axum::{
    Json,
    extract::{Query, State},
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, aide::OperationIo, JsonSchema)]
pub struct FindUserQueryParams {
    #[schemars(description = "query type")]
    pub r#type: FindUserQueryType,
    #[schemars(description = "query value")]
    pub value: String,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum FindUserQueryType {
    Email,
    Username,
    PhoneNumber,
}
pub type FindUserResponse = UserDetailResponse;

pub async fn find_user_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Query(params): Query<FindUserQueryParams>,
) -> Result<Json<FindUserResponse>, Error> {
    let user_pk = match params.r#type {
        FindUserQueryType::Email => {
            let email = params.value;
            let (users, _bookmark) =
                User::find_by_email(&dynamo.client, &email, Default::default()).await?;
            if users.len() == 0 {
                return Err(Error::NotFound(format!(
                    "User not found with email: {}",
                    email
                )));
            }
            users.into_iter().nth(0).unwrap().pk
        }
        FindUserQueryType::Username => {
            let username = params.value;
            let (users, _bookmark) =
                User::find_by_username(&dynamo.client, &username, Default::default()).await?;
            if users.len() == 0 {
                return Err(Error::NotFound(format!(
                    "User not found with username: {}",
                    username
                )));
            }
            users.into_iter().nth(0).unwrap().pk
        }
        FindUserQueryType::PhoneNumber => {
            let phone_number = params.value;
            let (users, _bookmark) = UserPhoneNumber::find_by_phone_number(
                &dynamo.client,
                &phone_number,
                Default::default(),
            )
            .await?;
            if users.len() == 0 {
                return Err(Error::NotFound(format!(
                    "User not found with phone number: {}",
                    phone_number
                )));
            }
            users.into_iter().nth(0).unwrap().pk
        }
    };

    let res = UserMetadata::query(&dynamo.client, user_pk).await?;
    Ok(Json(UserDetailResponse::from(res)))
}
