use super::super::dto::FoundUserResponse;
use super::super::*;

use ratel_auth::{User, UserPhoneNumber};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum FindUserQueryType {
    Email,
    Username,
    PhoneNumber,
}

#[get("/api/users/find?user_type&value", user: ratel_auth::OptionalUser)]
pub async fn find_user_handler(
    user_type: FindUserQueryType,
    value: String,
) -> Result<FoundUserResponse> {
    let conf = super::super::config::get();
    let cli = conf.common.dynamodb();

    let user = match user_type {
        FindUserQueryType::Email => {
            let (users, _) = User::find_by_email(cli, &value, User::opt().limit(1)).await?;
            users.into_iter().next()
        }
        FindUserQueryType::Username => {
            let (users, _) = User::find_by_username(cli, &value, User::opt().limit(1)).await?;
            users.into_iter().next()
        }
        FindUserQueryType::PhoneNumber => {
            let (users, _) =
                UserPhoneNumber::find_by_phone_number(cli, &value, UserPhoneNumber::opt_one())
                    .await?;
            let user_phone = users.into_iter().next().unwrap_or_default();
            User::get(cli, &user_phone.pk, Some(EntityType::User)).await?
        }
    }
    .ok_or(Error::NotFound("User not found".to_string()))?;

    Ok(FoundUserResponse {
        pk: user.pk.to_string(),
        nickname: user.display_name,
        username: user.username,
        profile_url: user.profile_url,
    })
}
