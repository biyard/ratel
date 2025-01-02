use by_axum::{
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Json,
    },
    log::root,
};
use dto::*;
use serde::{Deserialize, Serialize};
use slog::o;

#[derive(Clone, Debug)]
pub struct UserControllerV1 {
    log: slog::Logger,
}

#[derive(Debug, Serialize, PartialEq, Eq, Clone, Deserialize)]
pub struct SignupRequest {
    email: String,
    nickname: String,
    wallet_address: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserActionRequest {
    Signup(SignupRequest),
}

// NOTE: This is a real model and recommended to be moved to shared_models
#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct User {
    id: String,
    r#type: String,
    crated_at: u64,
    updated_at: u64,
    deleted_at: Option<u64>,

    name: Option<String>,

    // Indexes, if deleted_at is set, all values of indexes must be empty.
    gsi1: String,
    gsi2: String,
}

impl UserControllerV1 {
    pub fn route() -> Result<by_axum::axum::Router> {
        let log = root().new(o!("api-controller" => "UserControllerV1"));
        let ctrl = UserControllerV1 { log };

        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_user))
            .with_state(ctrl.clone()))
    }

    pub async fn act_user(
        State(ctrl): State<UserControllerV1>,
        Json(body): Json<UserActionRequest>,
    ) -> Result<Json<User>> {
        let log = ctrl.log.new(o!("api" => "create_user"));
        slog::debug!(log, "list_user {:?}", body);
        Ok(Json(User::default()))
    }
}
