use bdk::prelude::by_axum::auth::Authorization;

use crate::{
    Error2,
    models::user::{User, UserMetadata},
    types::EntityType,
};

/// This function only uses in Signup / Login handler
pub fn get_principal_from_auth(auth: Option<Authorization>) -> Result<String, Error2> {
    let principal = match auth {
        Some(Authorization::UserSig(sig)) => sig.principal().map_err(|e| {
            tracing::error!("failed to get principal: {:?}", e);
            Error2::Unauthorized("Invalid signature".into())
        })?,
        _ => return Err(Error2::Unauthorized("Missing authorization".into())),
    };

    Ok(principal)
}

pub async fn extract_user_pk(auth: Option<Authorization>) -> Result<String, Error2> {
    match auth {
        Some(Authorization::DynamoSession(session)) => Ok(session.pk),
        _ => Err(Error2::Unauthorized("Missing authorization".into())),
    }
}

pub async fn extract_user(
    cli: &aws_sdk_dynamodb::Client,
    auth: Option<Authorization>,
) -> Result<User, Error2> {
    match auth {
        Some(Authorization::DynamoSession(session)) => {
            User::get(cli, &session.pk, Some(EntityType::User))
                .await?
                .ok_or(Error2::NotFound("User not found".into()))
        }

        _ => Err(Error2::Unauthorized("Missing authorization".into())),
    }
}

pub async fn extract_user_metadata(
    cli: &aws_sdk_dynamodb::Client,
    auth: Option<Authorization>,
) -> Result<Vec<UserMetadata>, Error2> {
    let user = match auth {
        Some(Authorization::DynamoSession(session)) => session,
        _ => return Err(Error2::Unauthorized("Missing authorization".into())),
    };
    let metadata = UserMetadata::query(&cli, user.pk).await?;
    Ok(metadata)
}
