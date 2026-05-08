use super::super::*;
use super::dto::{UserDetailResponse, UserProfileResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UpdateProfileRequest {
    pub nickname: String,
    pub profile_url: String,
    pub description: String,
}

#[post("/api/me/profile", user: crate::features::auth::User)]
pub async fn update_profile_handler(body: UpdateProfileRequest) -> Result<UserDetailResponse> {
    let conf = crate::common::config::ServerConfig::default();
    let cli = conf.dynamodb();

    crate::features::auth::User::updater(user.pk.clone(), user.sk.clone())
        .with_display_name(body.nickname)
        .with_profile_url(body.profile_url)
        .with_description(body.description)
        .execute(cli)
        .await?;

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
