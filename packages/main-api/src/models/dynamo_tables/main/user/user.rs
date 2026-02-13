use crate::features::did::VerifiedAttributes;
use crate::features::membership::Membership;
use crate::features::membership::UserMembership;
use crate::utils::time::get_now_timestamp_millis;
use crate::*;
use names::Generator;
use names::Name;
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

    pub fn new_phone(phone: String) -> Self {
        let uid = uuid::Uuid::new_v4().to_string();
        let pk = Partition::User(uid.clone());
        let sk = EntityType::User;

        let now = get_now_timestamp_millis();
        let display_name = Generator::with_naming(Name::Numbered)
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
            username: display_name.clone(),
            password: None,
            ..Default::default()
        }
    }

    /// Check if the user is an admin
    pub fn is_admin(&self) -> bool {
        self.user_type == UserType::Admin
    }

    pub async fn get_attributes(
        &self,
        cli: &aws_sdk_dynamodb::Client,
    ) -> Result<VerifiedAttributes> {
        let (pk, sk) = VerifiedAttributes::keys(&self.pk);

        Ok(VerifiedAttributes::get(cli, pk, Some(sk))
            .await?
            .unwrap_or_default())
    }

    pub async fn get_user_membership(
        &self,
        cli: &aws_sdk_dynamodb::Client,
    ) -> Result<UserMembership> {
        let user_membership = UserMembership::get(cli, &self.pk, Some(EntityType::UserMembership))
            .await?
            .ok_or_else(|| crate::Error::NoUserMembershipFound)?;

        Ok(user_membership)
    }

    pub async fn get_membership(
        &self,
        cli: &aws_sdk_dynamodb::Client,
    ) -> Result<(UserMembership, Membership)> {
        let user_membership = self.get_user_membership(cli).await?;
        let pk: Partition = user_membership.membership_pk.clone().into();
        let membership = Membership::get(cli, pk, Some(EntityType::Membership))
            .await?
            .ok_or_else(|| crate::Error::NoMembershipFound)?;
        Ok((user_membership, membership))
    }
}

impl FromRequestParts<AppState> for Option<User> {
    type Rejection = crate::Error;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self> {
        Ok(User::from_request_parts(parts, state).await.ok())
    }
}

// For authenticated routes where User must be present
impl FromRequestParts<AppState> for User {
    type Rejection = crate::Error;

    async fn from_request_parts(parts: &mut Parts, _state: &AppState) -> Result<Self> {
        tracing::debug!("extracting user from request parts");

        if let Some(user) = parts.extensions.get::<User>() {
            return Ok(user.clone());
        }

        let session = Session::from_request_parts(parts, _state)
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

        let user = User::get(&(_state.dynamo.client), user_pk, Some(EntityType::User))
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
            return Err(crate::Error::NoUserFound);
        }

        parts.extensions.insert(user.as_ref().unwrap().clone());

        Ok(user.unwrap())
    }
}

#[async_trait::async_trait]
impl EntityPermissions for User {
    async fn get_permissions_for(
        &self,
        _cli: &aws_sdk_dynamodb::Client,
        requester: &Partition,
    ) -> Permissions {
        if &self.pk == requester {
            Permissions::all()
        } else {
            Permissions::empty()
        }
    }
}
