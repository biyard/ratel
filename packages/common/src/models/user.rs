#[cfg(feature = "server")]
use crate::axum::{
    extract::{FromRef, FromRequest, FromRequestParts, Request},
    http::request::Parts,
};
#[cfg(feature = "server")]
use tower_sessions::Session;

use crate::*;

// FIXME: DO NOT USE THIS STRUCT. IT IS A TEMPORARY SOLUTION.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq)]
#[cfg_attr(
    feature = "server",
    derive(DynamoEntity, schemars::JsonSchema, aide::OperationIo)
)]
pub struct User {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    pub display_name: String,
}

pub const SESSION_KEY_USER_ID: &str = "user_id";

#[cfg(feature = "server")]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
    Session: FromRequestParts<S, Rejection: std::fmt::Debug>,
{
    type Rejection = crate::Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        use aws_sdk_dynamodb::Client;

        tracing::debug!("extracting user from request parts");

        if let Some(user) = parts.extensions.get::<User>() {
            return Ok(user.clone());
        }

        let dynamo_client = parts
            .extensions
            .get::<Client>()
            .ok_or_else(|| {
                tracing::error!("DynamoClient not found in extensions");
                crate::Error::NoSessionFound
            })?
            .clone();

        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(|e| {
                tracing::error!("no session found from request: {:?}", e);
                crate::Error::NoSessionFound
            })?;

        let user_pk: Partition = session
            .get(SESSION_KEY_USER_ID)
            .await
            .map_err(|e| {
                tracing::error!("no user id found from session: {:?}", e);
                crate::Error::NoSessionFound
            })?
            .ok_or(crate::Error::NoSessionFound)?;

        let user = User::get(&dynamo_client, user_pk, Some(EntityType::User))
            .await
            .map_err(|e| {
                tracing::error!("failed to get user from db: {:?}", e);
                crate::Error::NoSessionFound
            });

        if user.is_err() {
            tracing::error!("no user found: {:?}", user);
            if let Err(e) = session.flush().await {
                tracing::error!("failed to flush session: {:?}", e);
            }
            return Err(crate::Error::NoSessionFound);
        }

        let user = user.unwrap();

        if user.is_none() {
            if let Err(e) = session.flush().await {
                tracing::error!("failed to flush session: {:?}", e);
            }
            return Err(crate::Error::NoSessionFound);
        }

        parts.extensions.insert(user.as_ref().unwrap().clone());
        Ok(user.unwrap())
    }
}
