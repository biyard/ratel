// mod permission;
mod team_permission_verifier;

use crate::{Error2, types::TeamGroupPermission, utils::dynamo_extractor::extract_user_pk};
use bdk::prelude::{by_axum::auth::Authorization, *};
use team_permission_verifier::TeamPermissionVerifier;

#[derive(Debug, Clone, PartialEq)]
pub enum RatelResource {
    Team { team_pk: String },
    // Post { team_pk: Option<String> },
}

pub(super) fn permissions_mask(perms: &[TeamGroupPermission]) -> i64 {
    perms.iter().fold(0, |acc, p| acc | (1 << (*p as u8)))
}

pub trait PermissionVerifier {
    fn has_all_permission(&self, required_bit_mask: i64) -> bool;
    fn has_any_permissions(&self, required_bit_mask: i64) -> bool;
}

pub async fn check_permission(
    client: &aws_sdk_dynamodb::Client,
    auth: Option<Authorization>,
    rsc: RatelResource,
    permissions: Vec<TeamGroupPermission>,
) -> Result<(), Error2> {
    let user_pk = extract_user_pk(auth).await?;

    let verifier: Box<dyn PermissionVerifier> = match rsc {
        RatelResource::Team { team_pk } => {
            Box::new(TeamPermissionVerifier::new(client, user_pk, team_pk).await?)
        }
    };
    let required_mask = permissions_mask(&permissions);
    if !verifier.has_all_permission(required_mask) {
        return Err(Error2::Unauthorized(
            "You do not have permission to perform this action".into(),
        ));
    }

    Ok(())
}

pub async fn check_any_permission(
    client: &aws_sdk_dynamodb::Client,
    auth: Option<Authorization>,
    rsc: RatelResource,
    permissions: Vec<TeamGroupPermission>,
) -> Result<(), Error2> {
    let user_pk = extract_user_pk(auth).await?;

    let verifier: Box<dyn PermissionVerifier> = match rsc {
        RatelResource::Team { team_pk } => {
            Box::new(TeamPermissionVerifier::new(client, user_pk, team_pk).await?)
        }
    };
    let required_mask = permissions_mask(&permissions);
    if !verifier.has_any_permissions(required_mask) {
        return Err(Error2::Unauthorized(
            "You do not have permission to perform this action".into(),
        ));
    }

    Ok(())
}
