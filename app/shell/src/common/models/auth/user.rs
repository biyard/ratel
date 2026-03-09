#[cfg(feature = "server")]
use tower_sessions::Session;

use crate::common::types::UserType;
use crate::common::*;

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

    pub points: i64,
}

impl User {
    pub fn did(&self) -> String {
        format!("did:web:ratel.foundation:{}", self.username)
    }

    pub fn id(&self) -> String {
        match &self.pk {
            Partition::User(uid) => uid.clone(),
            _ => "".to_string(),
        }
    }
}

#[cfg(feature = "server")]
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
        let now = utils::time::now();

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

    pub fn new_phone(phone: String) -> Self {
        let uid = uuid::Uuid::new_v4().to_string();
        let pk = Partition::User(uid);
        let sk = EntityType::User;

        let now = utils::time::now();
        let display_name = names::Generator::with_naming(names::Name::Numbered)
            .next()
            .unwrap()
            .replace('-', " ");

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            display_name: display_name.clone(),
            email: phone.to_string(),
            profile_url: "".to_string(),
            term_agreed: true,
            informed_agreed: false,
            user_type: UserType::Individual,
            username: display_name,
            password: None,
            ..Default::default()
        }
    }
}

pub const SESSION_KEY_USER_ID: &str = "user_id";

#[cfg(feature = "server")]
async fn extract_user_from_parts<S>(parts: &mut Parts, state: &S) -> Result<User>
where
    S: Send + Sync,
    Session: FromRequestParts<S, Rejection: std::fmt::Debug>,
{
    if let Some(user) = parts.extensions.get::<User>() {
        return Ok(user.clone());
    }

    let conf = ServerConfig::default();
    let dynamo_client = conf.dynamodb();

    let session = Session::from_request_parts(parts, state)
        .await
        .map_err(|e| {
            tracing::error!("no session found from request: {:?}", e);
            Error::NoSessionFound
        })?;

    let user_pk: Partition = session
        .get(SESSION_KEY_USER_ID)
        .await
        .map_err(|e| {
            tracing::error!("no user id found from session: {:?}", e);
            Error::NoSessionFound
        })?
        .ok_or(Error::NoSessionFound)?;

    let user = User::get(dynamo_client, user_pk, Some(EntityType::User))
        .await
        .map_err(|e| {
            tracing::error!("failed to get user from db: {:?}", e);
            Error::NoSessionFound
        });

    if user.is_err() {
        tracing::error!("no user found: {:?}", user);
        if let Err(e) = session.flush().await {
            tracing::error!("failed to flush session: {:?}", e);
        }
        return Err(Error::NoSessionFound);
    }

    let user = user.unwrap();

    if user.is_none() {
        if let Err(e) = session.flush().await {
            tracing::error!("failed to flush session: {:?}", e);
        }
        return Err(Error::NoSessionFound);
    }

    let user = user.unwrap();
    parts.extensions.insert(user.clone());
    Ok(user)
}

#[cfg(feature = "server")]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
    Session: FromRequestParts<S, Rejection: std::fmt::Debug>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        tracing::debug!("extracting user from request parts. Path: {:?}", parts.uri);

        extract_user_from_parts(parts, state).await
    }
}

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
    Session: FromRequestParts<S, Rejection: std::fmt::Debug>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        Ok(OptionalUser(
            extract_user_from_parts(parts, state).await.ok(),
        ))
    }
}
