use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{Extension, Json, extract::State, routing::get},
};
use dto::*;

use crate::utils::users::extract_user;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct UserPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct AuthController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl AuthController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route(
                "/hasura",
                get(Self::get_hasura_auth).post(Self::post_hasura_auth),
            )
            .with_state(self.clone()))
    }

    pub async fn get_hasura_auth(
        State(ctrl): State<AuthController>,
        Extension(auth): Extension<Option<Authorization>>,
    ) -> Result<Json<HasuraAuthResponse>> {
        let user = extract_user(&ctrl.pool, auth).await;

        if user.is_err() {
            return Ok(Json(HasuraAuthResponse {
                hasura_user_id: "anonymous".to_string(),
                hasura_role: HasuraRole::User,
                hasura_is_owner: HasuraIsOwner::False,
            }));
        }

        let user = user.unwrap();
        let is_admin = user.is_admin();

        Ok(Json(HasuraAuthResponse {
            hasura_user_id: user.id.to_string(),
            hasura_role: if is_admin {
                HasuraRole::Admin
            } else {
                HasuraRole::User
            },
            hasura_is_owner: if is_admin {
                HasuraIsOwner::True
            } else {
                HasuraIsOwner::False
            },
        }))
    }

    pub async fn post_hasura_auth(
        State(ctrl): State<AuthController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(req): Json<HasuraAuthRequest>,
    ) -> Result<Json<HasuraAuthResponse>> {
        tracing::debug!("Post Hasura Auth: {:?}", req);
        // FIXME: refactroing to use authorization in req instead of header.
        //        post request will be empty of Authorization header.
        let user = extract_user(&ctrl.pool, auth).await;

        if user.is_err() {
            return Ok(Json(HasuraAuthResponse {
                hasura_user_id: "anonymous".to_string(),
                hasura_role: HasuraRole::User,
                hasura_is_owner: HasuraIsOwner::False,
            }));
        }

        let user = user.unwrap();
        let is_admin = user.is_admin();

        Ok(Json(HasuraAuthResponse {
            hasura_user_id: user.id.to_string(),
            hasura_role: if is_admin {
                HasuraRole::Admin
            } else {
                HasuraRole::User
            },
            hasura_is_owner: if is_admin {
                HasuraIsOwner::True
            } else {
                HasuraIsOwner::False
            },
        }))
    }
}

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct HasuraAuthRequest {
    pub headers: std::collections::HashMap<String, String>,
    pub request: HasuraAuthInnerRequest,
}

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct HasuraAuthInnerRequest {
    pub variables: std::collections::HashMap<String, serde_json::Value>,
    pub operation_name: String,
    pub query: String,
}

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct HasuraAuthResponse {
    #[serde(rename = "X-Hasura-User-Id")]
    pub hasura_user_id: String,
    #[serde(rename = "X-Hasura-Role")]
    pub hasura_role: HasuraRole,
    #[serde(rename = "X-Hasura-Is-Owner")]
    pub hasura_is_owner: HasuraIsOwner,
}

#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    aide::OperationIo,
)]
#[serde(rename_all = "snake_case")]
pub enum HasuraRole {
    Anonymous,
    User,
    Admin,
}

#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    aide::OperationIo,
)]
#[serde(rename_all = "snake_case")]
pub enum HasuraIsOwner {
    True,
    False,
}
