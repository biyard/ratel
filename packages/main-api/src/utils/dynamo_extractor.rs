use bdk::prelude::by_axum::auth::Authorization;

use crate::{
    Error2,
    constants::SESSION_KEY_USER_ID,
    models::user::{User, UserMetadata},
    types::{EntityType, Partition},
};

// #[deprecated(note = "use extract_user_from_session instead")]
pub async fn extract_user_pk(auth: Option<Authorization>) -> Result<String, Error2> {
    match auth {
        Some(Authorization::DynamoSession(session)) => Ok(session.pk),
        _ => Err(Error2::Unauthorized("Missing authorization".into())),
    }
}

pub async fn extract_user_from_session(
    cli: &aws_sdk_dynamodb::Client,
    session: &tower_sessions::Session,
) -> Result<User, Error2> {
    let user_pk: Partition = session
        .get(SESSION_KEY_USER_ID)
        .await?
        .ok_or(Error2::Unauthorized("no session".to_string()))?;

    let user = User::get(cli, &user_pk, Some(EntityType::User))
        .await?
        .ok_or(Error2::NotFound("User not found".into()))?;

    Ok(user)
}

// #[deprecated(note = "use extract_user_from_session instead")]
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

// #[deprecated(note = "use extract_user_from_session instead")]
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
