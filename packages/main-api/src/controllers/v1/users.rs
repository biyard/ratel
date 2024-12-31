use by_axum::{
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Json,
    },
    log::root,
};
use common_query_response::CommonQueryResponse;
use dto::*;
use serde::{Deserialize, Serialize};
use slog::o;

#[derive(Clone, Debug)]
pub struct UserControllerV1 {
    log: slog::Logger,
}

// NOTE: if already have other pagination, please remove this and use defined one.
#[derive(Debug, serde::Deserialize)]
pub struct Pagination {
    page: Option<usize>,
    size: Option<usize>,
    bookmark: Option<usize>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateUserRequest {
    name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateUserRequest {
    name: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum ActionUserRequest {
    Action1(String),
    Action2(String),
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
            .route("/:id", get(Self::get_user).post(Self::act_user_by_id))
            .with_state(ctrl.clone())
            .route("/", post(Self::act_user).get(Self::list_user))
            .with_state(ctrl.clone()))
    }

    pub async fn act_user(
        State(ctrl): State<UserControllerV1>,
        Json(body): Json<ActionUserRequest>,
    ) -> Result<Json<User>> {
        let log = ctrl.log.new(o!("api" => "create_user"));
        slog::debug!(log, "list_user {:?}", body);
        Ok(Json(User::default()))
    }

    pub async fn update_user(
        State(ctrl): State<UserControllerV1>,
        Path(id): Path<String>,
        Json(body): Json<UpdateUserRequest>,
    ) -> Result<()> {
        let log = ctrl.log.new(o!("api" => "update_user"));
        slog::debug!(log, "list_user {:?} {:?}", id, body);
        Ok(())
    }

    pub async fn act_user_by_id(
        State(ctrl): State<UserControllerV1>,
        Path(id): Path<String>,
        Json(body): Json<ActionUserRequest>,
    ) -> Result<()> {
        let log = ctrl.log.new(o!("api" => "update_user"));
        slog::debug!(log, "list_user {:?} {:?}", id, body);
        Ok(())
    }

    pub async fn get_user(
        State(ctrl): State<UserControllerV1>,
        Path(id): Path<String>,
    ) -> Result<Json<User>> {
        let log = ctrl.log.new(o!("api" => "get_user"));
        slog::debug!(log, "get_user {:?}", id);
        Ok(Json(User::default()))
    }

    pub async fn delete_user(
        State(ctrl): State<UserControllerV1>,
        Path(id): Path<String>,
    ) -> Result<()> {
        let log = ctrl.log.new(o!("api" => "delete_user"));
        slog::debug!(log, "delete_user {:?}", id);
        Ok(())
    }

    pub async fn list_user(
        State(ctrl): State<UserControllerV1>,
        Query(pagination): Query<Pagination>,
    ) -> Result<Json<CommonQueryResponse<User>>> {
        let log = ctrl.log.new(o!("api" => "list_user"));
        slog::debug!(log, "list_user {:?}", pagination);

        Ok(Json(CommonQueryResponse::default()))
    }
}
