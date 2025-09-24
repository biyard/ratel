// TODO: After migrate to DYNAMODB, remove these modules

mod general_permission_verifier;
mod space_permission_verifier;
mod team_permission_verifier;

pub use general_permission_verifier::*;
use space_permission_verifier::SpacePermissionVerifier;
pub use team_permission_verifier::*;

use bdk::prelude::{by_axum::auth::Authorization, *};
use dto::*;

use crate::utils::users::extract_user_with_options;

pub trait PermissionVerifier {
    fn has_permission(&self, user: &User, perm: GroupPermission) -> bool;
}

pub trait MainGroupPermissionVerifier {
    fn has_group_permission(&self, perm: GroupPermission) -> bool;
}

pub async fn check_perm_without_error(
    pool: &sqlx::Pool<sqlx::Postgres>,
    auth: Option<Authorization>,
    rsc: RatelResource,
    perm: GroupPermission,
) -> Result<User> {
    let user = extract_user_with_options(pool, auth, true).await?;

    let verifier: Box<dyn PermissionVerifier> = match rsc {
        RatelResource::Post { team_id } => {
            Box::new(TeamPermissionVerifier::new(team_id, pool).await)
        }
        RatelResource::Reply { team_id } => {
            Box::new(TeamPermissionVerifier::new(team_id, pool).await)
        }
        RatelResource::News | RatelResource::Promotions | RatelResource::Oracles => {
            Box::new(GeneralPermissionVerifier::new())
        }
        RatelResource::Space { space_id } => {
            Box::new(SpacePermissionVerifier::new(user.id, space_id, pool).await)
        }
        RatelResource::Team { team_id } => {
            Box::new(TeamPermissionVerifier::new(team_id, pool).await)
        }
    };

    if !verifier.has_permission(&user, perm) {
        return Ok(User::default());
    }

    Ok(user)
}

pub async fn check_perm(
    pool: &sqlx::Pool<sqlx::Postgres>,
    auth: Option<Authorization>,
    rsc: RatelResource,
    perm: GroupPermission,
) -> Result<User> {
    let user = extract_user_with_options(pool, auth, true).await?;

    let verifier: Box<dyn PermissionVerifier> = match rsc {
        RatelResource::Post { team_id } => {
            Box::new(TeamPermissionVerifier::new(team_id, pool).await)
        }
        RatelResource::Reply { team_id } => {
            Box::new(TeamPermissionVerifier::new(team_id, pool).await)
        }
        RatelResource::News | RatelResource::Promotions | RatelResource::Oracles => {
            Box::new(GeneralPermissionVerifier::new())
        }
        RatelResource::Space { space_id } => {
            Box::new(SpacePermissionVerifier::new(user.id, space_id, pool).await)
        }
        RatelResource::Team { team_id } => {
            Box::new(TeamPermissionVerifier::new(team_id, pool).await)
        }
    };

    if !verifier.has_permission(&user, perm) {
        return Err(Error::Unauthorized);
    }

    tracing::debug!("authorized user_id: {:?}", user);

    Ok(user)
}
