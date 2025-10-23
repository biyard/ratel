use crate::{
    Error2,
    constants::SESSION_KEY_USER_ID,
    models::user::User,
    types::{EntityType, Partition},
};

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
