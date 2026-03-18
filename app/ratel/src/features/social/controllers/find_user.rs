use crate::features::social::controllers::dto::UserResponse;
use crate::features::social::*;
use crate::features::auth::*;

#[get("/api/users/find?username", user: OptionalUser)]
pub async fn find_user_by_username_handler(username: String) -> Result<UserResponse> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let (users, _) = crate::features::auth::User::find_by_username(cli, &username, Default::default()).await?;
    let user = users
        .into_iter()
        .find(|u| u.username == username)
        .ok_or(Error::NotFound("User not found".to_string()))?;

    Ok(UserResponse::from(user))
}
