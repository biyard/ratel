use crate::features::social::pages::member::dto::FoundUserResponse;
use crate::features::social::pages::member::{MemberError};
use crate::features::social::*;
use crate::features::auth::{User, UserPhoneNumber};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum FindUserQueryType {
    Email,
    Username,
    PhoneNumber,
}

#[get("/api/users/find?user_type&value", _user: crate::features::auth::OptionalUser)]
pub async fn find_user_handler(
    user_type: FindUserQueryType,
    value: String,
) -> Result<FoundUserResponse> {
    let conf = crate::features::social::pages::member::config::get();
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
    .ok_or(MemberError::UserNotFound)?;

    Ok(FoundUserResponse {
        pk: user.pk.to_string(),
        nickname: user.display_name,
        username: user.username,
        profile_url: user.profile_url,
        description: user.description,
    })
}
