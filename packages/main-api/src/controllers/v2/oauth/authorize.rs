use bdk::prelude::*;
use dto::{
    AuthClient, Error, Result,
    by_axum::axum::{
        extract::{Query, State},
        response::{IntoResponse, Redirect},
    },
    sqlx::PgPool,
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
pub struct AuthorizeQuery {
    pub response_type: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub scope: Option<String>,
    pub state: Option<String>,
}

pub async fn authorize_handler(
    State(pool): State<PgPool>,
    Query(req): Query<AuthorizeQuery>,
) -> Result<impl IntoResponse> {
    match req.response_type.as_str() {
        "code" => {
            tracing::debug!("handling code response type");
        }
        _ => {
            tracing::error!("unsupported response type: {}", req.response_type);
            return Err(Error::BadRequest);
        }
    }
    let client = AuthClient::query_builder()
        .client_id_contains(req.client_id.clone())
        .query()
        .map(AuthClient::from)
        .fetch_one(&pool)
        .await?;
    if client.redirect_uris.contains(&req.redirect_uri) {
        return Err(Error::BadRequest);
    }
    let client_url = "http://localhost:8080/oauth";
    let url = format!(
        "{}?client_id={}&redirect_uri={}&scope={}&state={}",
        client_url,
        req.client_id,
        req.redirect_uri,
        req.scope.unwrap_or_default(),
        req.state.unwrap_or_default()
    );

    let url = serde_urlencoded::from_str(&url)?;

    Ok(Redirect::to(url))
}
