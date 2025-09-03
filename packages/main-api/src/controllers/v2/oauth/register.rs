use bdk::prelude::*;
use dto::{
    AuthClient,
    by_axum::axum::{Json, extract::State, http::StatusCode, response::IntoResponse},
    sqlx::PgPool,
};
use uuid::Uuid;

use crate::{
    controllers::v2::oauth::approve::generate_random_string,
    models::oauth::{
        grant_type::GrantType,
        response_type::ResponseType,
        scope::{Scope, deserialize_scope_vec},
    },
};

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]

pub struct ClientRegistrationRequest {
    pub client_name: String,
    pub redirect_uris: Vec<String>,
    pub grant_types: Vec<GrantType>,
    pub token_endpoint_auth_method: String,
    pub response_types: Vec<ResponseType>,
    #[serde(deserialize_with = "deserialize_scope_vec")]
    pub scope: Vec<Scope>,
}

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct ClientRegistrationResponse {
    pub client_id: String,
    pub client_secret: Option<String>,
    pub client_name: String,
    pub redirect_uris: Vec<String>,
}

pub async fn register_handler(
    State(pool): State<PgPool>,
    Json(req): Json<ClientRegistrationRequest>,
) -> impl IntoResponse {
    if req.redirect_uris.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "invalid_request",
                "error_description": "at least one redirect uri is required"
            })),
        )
            .into_response();
    }

    // generate client id and secret
    let client_id = format!("client-{}", Uuid::new_v4());
    let client_secret = if let Ok(v) = generate_random_string() {
        v
    } else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "server_error",
                "error_description": "failed to generate client secret"
            })),
        )
            .into_response();
    };

    let repo = AuthClient::get_repository(pool)
        .insert(
            client_id.clone(),
            client_secret.clone(),
            req.redirect_uris.clone(),
            vec![],
        )
        .await;
    if repo.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "server_error",
                "error_description": "failed to register client"
            })),
        )
            .into_response();
    }

    let response = ClientRegistrationResponse {
        client_id,
        client_secret: Some(client_secret),
        client_name: req.client_name,
        redirect_uris: req.redirect_uris,
    };

    (StatusCode::CREATED, Json(response)).into_response()
}
