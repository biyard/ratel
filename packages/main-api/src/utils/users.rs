use bdk::prelude::by_axum::auth::Authorization;
use bdk::prelude::*;
use dto::*;

pub async fn extract_user_id(
    pool: &sqlx::Pool<sqlx::Postgres>,
    auth: Option<Authorization>,
) -> Result<i64> {
    let user_id = match auth {
        Some(Authorization::UserSig(sig)) => {
            let principal = sig.principal().map_err(|e| {
                tracing::error!("failed to get principal: {:?}", e);
                ServiceError::Unauthorized
            })?;
            User::query_builder()
                .principal_equals(principal)
                .query()
                .map(User::from)
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    tracing::error!("failed to get user: {:?}", e);
                    ServiceError::InvalideUser
                })?
                .id
        }
        Some(Authorization::Bearer { claims }) => claims.sub.parse::<i64>().map_err(|e| {
            tracing::error!("failed to parse user id: {:?}", e);
            ServiceError::Unauthorized
        })?,
        _ => return Err(ServiceError::Unauthorized),
    };

    Ok(user_id)
}

pub async fn extract_user_email(
    pool: &sqlx::Pool<sqlx::Postgres>,
    auth: Option<Authorization>,
) -> Result<String> {
    let email = match auth {
        Some(Authorization::UserSig(sig)) => {
            let principal = sig.principal().map_err(|e| {
                tracing::error!("failed to get principal: {:?}", e);
                ServiceError::Unauthorized
            })?;
            User::query_builder()
                .principal_equals(principal)
                .query()
                .map(User::from)
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    tracing::error!("failed to get user: {:?}", e);
                    ServiceError::InvalideUser
                })?
                .email
        }
        Some(Authorization::Bearer { claims }) => claims
            .custom
            .get("email")
            .unwrap_or(&"".to_string())
            .to_string(),
        _ => return Err(ServiceError::Unauthorized),
    };

    Ok(email)
}
