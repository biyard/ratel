#![allow(warnings)]
use crate::{
    AppState, Error2, constants::SESSION_KEY_USER_ID, types::*,
    utils::time::get_now_timestamp_millis,
};
// use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::{body::Body, extract::State, http::Request};
use bdk::prelude::*;
use tower_sessions::Session;

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    DynamoEntity,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct User {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi2", sk)]
    #[dynamo(prefix = "TS", index = "gsi3", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    pub display_name: String,
    pub profile_url: String,
    #[dynamo(
        prefix = "EMAIL#PASSWORD",
        name = "find_by_email_and_password",
        index = "gsi1",
        pk
    )]
    #[dynamo(prefix = "EMAIL", name = "find_by_email", index = "gsi3", pk)]
    pub email: String,
    // NOTE: username is linked with gsi2-index of team model.
    #[dynamo(prefix = "USERNAME", name = "find_by_username", index = "gsi2", pk)]
    pub username: String,

    pub term_agreed: bool,
    pub informed_agreed: bool,

    pub user_type: UserType,

    pub followers_count: i64,
    pub followings_count: i64,

    // profile contents
    pub description: String,
    #[dynamo(index = "gsi1", sk)]
    pub password: Option<String>,

    pub membership: Membership,
    pub theme: Theme,
    pub points: i64,
}

impl User {
    pub fn new(
        display_name: String,
        email: String,
        profile_url: String,
        term_agreed: bool,
        informed_agreed: bool,
        user_type: UserType,
        username: String,
        password: Option<String>,
    ) -> Self {
        let uid = uuid::Uuid::new_v4().to_string();
        let pk = Partition::User(uid);
        let sk = EntityType::User;

        let now = get_now_timestamp_millis();

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            display_name,
            email,
            profile_url,
            term_agreed,
            informed_agreed,
            user_type,
            username,
            password,
            ..Default::default()
        }
    }
}

impl FromRequestParts<AppState> for Option<User> {
    type Rejection = crate::Error2;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, _state).await;

        if let Err(e) = &session {
            return Ok(None);
        }

        let session = session.unwrap();

        let user_pk: Partition = if let Ok(Some(u)) = session.get(SESSION_KEY_USER_ID).await {
            u
        } else {
            let _ = session.flush().await;
            return Ok(None);
        };

        let user = if let Ok(Some(u)) =
            User::get(&(_state.dynamo.client), user_pk, Some(EntityType::User)).await
        {
            u
        } else {
            let _ = session.flush().await;
            return Ok(None);
        };

        Ok(Some(user))
    }
}

// For authenticated routes where User must be present
impl FromRequestParts<AppState> for User {
    type Rejection = crate::Error2;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, _state)
            .await
            .map_err(|e| crate::Error2::NoSessionFound)?;

        let user_pk: Partition = session
            .get(SESSION_KEY_USER_ID)
            .await
            .map_err(|e| crate::Error2::NoSessionFound)?
            .ok_or(crate::Error2::NoSessionFound)?;

        let user = User::get(&(_state.dynamo.client), user_pk, Some(EntityType::User))
            .await
            .map_err(|e| crate::Error2::NoSessionFound);

        if user.is_err() {
            let _ = session.flush().await;
            return Err(crate::Error2::NoSessionFound);
        }

        let user = user.unwrap();

        if user.is_none() {
            let _ = session.flush().await;
            return Err(crate::Error2::NoUserFound);
        }

        Ok(user.unwrap())
    }
}
