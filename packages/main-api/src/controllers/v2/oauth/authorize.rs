use bdk::prelude::*;
use dto::{
    AuthClient, Error, Result,
    by_axum::axum::{
        extract::{Query, State},
        response::{IntoResponse, Redirect},
    },
    sqlx::PgPool,
};

use crate::config;

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
        .client_id_equals(req.client_id.clone())
        .query()
        .map(AuthClient::from)
        .fetch_one(&pool)
        .await?;
    let redirect_uri = url::Url::parse(&req.redirect_uri)?;
    if !client.redirect_uris.contains(&redirect_uri.to_string()) {
        return Err(Error::BadRequest);
    }

    let client_url = format!("https://{}/oauth-login", config::get().signing_domain);
    let url = format!(
        "{}?client_id={}&redirect_uri={}{}{}",
        client_url,
        req.client_id,
        req.redirect_uri,
        if let Some(state) = &req.state {
            format!("&state={}", state)
        } else {
            "".to_string()
        },
        if let Some(scope) = &req.scope {
            format!("&scope={}", scope)
        } else {
            "".to_string()
        },
    );

    Ok(Redirect::to(&url))
}
