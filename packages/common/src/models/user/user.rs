#[cfg(feature = "server")]
use crate::{
    axum::{
        extract::{FromRef, FromRequest, FromRequestParts, Request},
        http::request::Parts,
    },
    middlewares::client_state::ClientState,
    utils::aws::dynamo::DynamoClient,
};
#[cfg(feature = "server")]
use tower_sessions::Session;

use crate::macros::dynamo_entity::DynamoEntity;
use crate::{
    models::user::{Theme, UserType},
    *,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct User {
    pub pk: Partition,
    #[dynamo(index = "gsi6", name = "find_by_follwers", pk)]
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi2", sk)]
    #[dynamo(prefix = "TS", index = "gsi3", sk)]
    #[dynamo(prefix = "TS", index = "gsi5", sk)]
    pub created_at: i64,
    #[dynamo(prefix = "USER_TYPE", index = "gsi4", sk)]
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
    #[dynamo(prefix = "PHONE", name = "find_by_phone", index = "gsi5", pk)]
    #[serde(default)]
    pub phone: Option<String>,

    pub term_agreed: bool,
    pub informed_agreed: bool,

    #[dynamo(prefix = "USER_TYPE", name = "find_by_user_type", index = "gsi4", pk)]
    pub user_type: UserType,

    #[dynamo(index = "gsi6", sk)]
    pub followers_count: i64,
    pub followings_count: i64,

    // profile contents
    pub description: String,
    #[dynamo(index = "gsi1", sk)]
    pub password: Option<String>,

    pub theme: Theme,
    pub points: i64,
}

pub const SESSION_KEY_USER_ID: &str = "user_id";

#[cfg(feature = "server")]
async fn extract_user_from_parts<S>(parts: &mut Parts, state: &S) -> Result<User>
where
    S: Send + Sync,
    ClientState: FromRef<S>,
    Session: FromRequestParts<S, Rejection: std::fmt::Debug>,
{
    if let Some(user) = parts.extensions.get::<User>() {
        return Ok(user.clone());
    }

    let clients = ClientState::from_ref(state);
    let dynamo_client = clients.dynamo;

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

    let user = user.unwrap();
    parts.extensions.insert(user.clone());
    Ok(user)
}

#[cfg(feature = "server")]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
    ClientState: FromRef<S>,
    Session: FromRequestParts<S, Rejection: std::fmt::Debug>,
{
    type Rejection = crate::Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        tracing::debug!("extracting user from request parts");
        extract_user_from_parts(parts, state).await
    }
}

/*
Note:
Case 1:
```
    impl FromRequestParts<ClientState> for Option<User> {
        type Rejection = crate::Error;

        async fn from_request_parts(parts: &mut Parts, state: &ClientState) -> Result<Self> {

            Ok(User::from_request_parts(parts, state).await.ok())
        }
    }
```
Dioxus Extractor required `impl<S> FromRequestParts<S> for Option<User>.`.
So when we use
#[get("/api/user", state: State<AppState>, user: Option<User>) ]
pub fn some_api() ...
it occurs error.

Case 2:
```
    impl<S> FromRequestParts<S> for Option<User>
    where
        S: Send + Sync,
        ClientState: FromRef<S>,
        Session: FromRequestParts<S, Rejection: std::fmt::Debug>,
    {
    ...
    }
```
Because of Rust Orphan Rule (https://doc.rust-lang.org/book/ch10-02-traits.html#orphan-rules),
we cannot implement like this.

So we need to use a wrapper struct to handle optional user.
#[get("/api/user", state: State<AppState>, user: OptionalUser) ]
So we use this struct to handle optional user.

TODO:
- Check if this is still true.
 */
pub struct OptionalUser(pub Option<User>);

impl From<OptionalUser> for Option<User> {
    fn from(value: OptionalUser) -> Self {
        value.0
    }
}

#[cfg(feature = "server")]
impl<S> FromRequestParts<S> for OptionalUser
where
    S: Send + Sync,
    ClientState: FromRef<S>,
    Session: FromRequestParts<S, Rejection: std::fmt::Debug>,
{
    type Rejection = crate::Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        Ok(OptionalUser(
            extract_user_from_parts(parts, state).await.ok(),
        ))
    }
}
