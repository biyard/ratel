use bdk::prelude::*;
use dto::{
    AuthClient, Error, Result,
    by_axum::axum::{
        extract::{Query, State},
        response::{IntoResponse, Redirect},
    },
    sqlx::PgPool,
};

use crate::{
    config,
    models::oauth::{
        response_type::ResponseType,
        scope::{Scope, deserialize_scope_vec},
    },
};

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, aide::OperationIo, JsonSchema,
)]
pub struct AuthorizeQuery {
    pub response_type: ResponseType,
    pub client_id: String,
    pub redirect_uri: String,
    #[serde(deserialize_with = "deserialize_scope_vec")]
    pub scope: Vec<Scope>,
    pub state: Option<String>,
}

pub async fn authorize_handler(
    State(pool): State<PgPool>,
    Query(req): Query<AuthorizeQuery>,
) -> Result<impl IntoResponse> {
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
        if req.scope.is_empty() {
            "".to_string()
        } else {
            format!(
                "&scope={}",
                req.scope
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join("+")
            )
        },
    );

    Ok(Redirect::to(&url))
}
