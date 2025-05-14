mod general_permission_verifier;
mod team_permission_verifier;

pub use general_permission_verifier::*;
pub use team_permission_verifier::*;

use bdk::prelude::{by_axum::auth::Authorization, *};
use dto::*;

pub trait PermissionVerifier {
    fn has_permission(&self, user: &User, perm: GroupPermission) -> bool;
}

pub async fn check_perm(
    pool: &sqlx::Pool<sqlx::Postgres>,
    auth: Option<Authorization>,
    rsc: RatelResource,
    perm: GroupPermission,
) -> Result<User> {
    let user = match auth {
        Some(Authorization::UserSig(sig)) => {
            let principal = sig.principal().map_err(|e| {
                tracing::error!("failed to get principal: {:?}", e);
                Error::Unauthorized
            })?;
            let user = User::query_builder()
                .principal_equals(principal)
                .query()
                .map(User::from)
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    tracing::error!("failed to get user: {:?}", e);
                    Error::InvalidUser
                })?;
            user
        }
        Some(Authorization::Bearer { claims }) => {
            let user_id = claims.sub.parse::<i64>().map_err(|e| {
                tracing::error!("failed to parse user id: {:?}", e);
                Error::Unauthorized
            })?;
            tracing::debug!("extracted user_id: {:?}", user_id);

            let user = User::query_builder()
                .id_equals(user_id)
                .query()
                .map(User::from)
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    tracing::error!("failed to get user: {:?}", e);
                    Error::InvalidUser
                })?;
            tracing::debug!("extracted user: {:?}", user);
            user
        }
        _ => return Err(Error::Unauthorized),
    };

    let verifier: Box<dyn PermissionVerifier> = match rsc {
        RatelResource::Post { team_id } => Box::new(TeamPermissionVerifier::new(team_id)),
        RatelResource::Reply { team_id } => Box::new(TeamPermissionVerifier::new(team_id)),
        RatelResource::News => Box::new(GeneralPermissionVerifier::new()),
    };

    if !verifier.has_permission(&user, perm) {
        return Err(Error::Unauthorized);
    }

    tracing::debug!("authorized user_id: {:?}", user);

    Ok(user)
}
