use super::super::*;
use super::dto::{UserDetailResponse, UserProfileResponse};

#[get("/api/me", user: crate::features::auth::User)]
pub async fn get_user_detail_handler() -> Result<UserDetailResponse> {
    let conf = crate::common::config::ServerConfig::default();
    let cli = conf.dynamodb();

    let user = crate::features::auth::User::get(cli, user.pk.clone(), Some(EntityType::User))
        .await?
        .ok_or(Error::NotFound("User not found".into()))?;

    let evm_address = crate::features::auth::UserEvmAddress::get(
        cli,
        user.pk.clone(),
        Some(EntityType::UserEvmAddress),
    )
    .await?
    .map(|item| item.evm_address);

    Ok(UserDetailResponse {
        user: UserProfileResponse {
            username: user.username.clone(),
            display_name: user.display_name.clone(),
            profile_url: user.profile_url.clone(),
            description: user.description.clone(),
        },
        email: user.email.clone(),
        evm_address,
    })
}
