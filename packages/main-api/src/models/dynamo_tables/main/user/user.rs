use crate::{
    AppState, constants::SESSION_KEY_USER_ID, types::*, utils::time::get_now_timestamp_millis,
};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
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
    #[dynamo(index = "gsi6", name = "find_by_follwers", pk)]
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

// impl User {
//     pub async fn migrate(cli: &aws_sdk_dynamodb::Client) -> Result<(), crate::Error2> {
//         use aws_sdk_dynamodb::operation::get_item::GetItemOutput;

//         let GetItemOutput { item, .. } = cli
//             .get_item()
//             .table_name(Self::table_name())
//             .key(
//                 "pk",
//                 aws_sdk_dynamodb::types::AttributeValue::S("MIGRATE#USER".to_string()),
//             )
//             .key(
//                 "sk",
//                 aws_sdk_dynamodb::types::AttributeValue::S("MIGRATE".to_string()),
//             )
//             .send()
//             .await
//             .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

//         let last_migrated_version = if let Some(item) = item {
//             item.get("last_migrated_version")
//                 .and_then(|v| v.as_n().ok())
//                 .and_then(|v| v.parse::<i64>().ok())
//                 .unwrap_or(0)
//         } else {
//             0
//         };

//         if last_migrated_version >= 1 {
//             tracing::info!("User table already migrated to the latest version.");
//             return Ok(());
//         }

//         let resp = cli
//             .query()
//             .table_name(Self::table_name())
//             .index_name("type-index")
//             .expression_attribute_names("#pk", "sk")
//             .expression_attribute_values(
//                 ":pk",
//                 aws_sdk_dynamodb::types::AttributeValue::S("USER".to_string()),
//             )
//             .key_condition_expression("#pk = :pk")
//             .send()
//             .await
//             .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

//         let items: Vec<Self> = resp
//             .items
//             .unwrap_or_default()
//             .into_iter()
//             .map(|item| serde_dynamo::from_item(item))
//             .collect::<Result<Vec<_>, _>>()?;

//         let bookmark = if let Some(ref last_evaluated_key) = resp.last_evaluated_key {
//             Some(Self::encode_lek_all(last_evaluated_key)?)
//         } else {
//             None
//         };

//         items
//             .iter()
//             .map(|user| {
//                 let item = serde_dynamo::to_item(user)?;
//                 let item = user.indexed_fields(item);

//                 let req = aws_sdk_dynamodb::types::Put::builder()
//                     .table_name(Self::table_name())
//                     .set_item(Some(item))
//                     .build()
//                     .unwrap();
//             })
//             .collect::<Result<Vec<_>, _>>()?;

//         Ok(())
//     }
// }

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

    /// Check if the user is an admin
    pub fn is_admin(&self) -> bool {
        self.user_type == UserType::Admin
    }
}

impl FromRequestParts<AppState> for Option<User> {
    type Rejection = crate::Error;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        tracing::debug!("extracting optional user from request parts");
        let session = Session::from_request_parts(parts, _state).await;

        if let Err(_e) = &session {
            return Ok(None);
        }

        let session = session.unwrap();

        let user_pk: Partition = if let Ok(Some(u)) = session.get(SESSION_KEY_USER_ID).await {
            tracing::debug!("found user id in session: {:?}", u);
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
    type Rejection = crate::Error;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        tracing::debug!("extracting user from request parts");
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

        Ok(user.unwrap())
    }
}
