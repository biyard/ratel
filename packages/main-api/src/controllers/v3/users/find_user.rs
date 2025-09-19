use crate::{
    AppState, Error2,
    models::user::{User, UserDetailResponse, UserMetadata, UserPhoneNumber},
};
use bdk::prelude::*;
use dto::{
    JsonSchema, aide,
    by_axum::axum::{
        Json,
        extract::{Query, State},
    },
};
use serde::Deserialize;

use validator::Validate;

#[derive(Debug, Clone, Deserialize, aide::OperationIo, JsonSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FindUserQueryParams {
    #[schemars(description = "email address")]
    #[validate(email)]
    pub email: Option<String>,
    #[schemars(description = "username")]
    pub username: Option<String>,
    #[schemars(description = "User's phone number")]
    pub phone_number: Option<String>,
}

pub struct UserDetail {
    pub pk: String,
    pub email: String,
    pub display_name: String,
    pub profile_url: String,
    pub content: String,
    pub user_type: String,

    pub theme: Option<String>,

    pub principal: Option<String>,
    pub evm_address: Option<String>,
    pub phone_number: Option<String>,
    pub telegram: Option<String>,
    pub referral_code: Option<String>,
}
pub type FindUserResponse = UserDetailResponse;

pub async fn find_user_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Query(params): Query<FindUserQueryParams>,
) -> Result<Json<FindUserResponse>, Error2> {
    let user_pk = match (params.email, params.username, params.phone_number) {
        (Some(email), None, None) => {
            let (users, _bookmark) =
                User::find_by_email(&dynamo.client, &email, Default::default()).await?;
            if users.len() == 0 {
                return Err(Error2::NotFound(format!(
                    "User not found with email: {}",
                    email
                )));
            }
            users.into_iter().nth(0).unwrap().pk
        }
        (None, Some(username), None) => {
            let (users, _bookmark) =
                User::find_by_username(&dynamo.client, &username, Default::default()).await?;
            if users.len() == 0 {
                return Err(Error2::NotFound(format!(
                    "User not found with username: {}",
                    username
                )));
            }
            users.into_iter().nth(0).unwrap().pk
        }
        (None, None, Some(phone_number)) => {
            let (users, _bookmark) = UserPhoneNumber::find_by_phone_number(
                &dynamo.client,
                &phone_number,
                Default::default(),
            )
            .await?;
            if users.len() == 0 {
                return Err(Error2::NotFound(format!(
                    "User not found with phone number: {}",
                    phone_number
                )));
            }
            users.into_iter().nth(0).unwrap().pk
        }
        _ => {
            return Err(Error2::BadRequest(
                "One and only one of email, username, or phoneNumber must be provided".to_string(),
            ));
        }
    };
    let res = UserMetadata::query(&dynamo.client, user_pk).await?;
    Ok(Json(UserDetailResponse::from(res)))
}
