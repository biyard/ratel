use crate::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UpdateUserProfileRequest {
    pub nickname: Option<String>,
    pub profile_url: Option<String>,
    pub description: Option<String>,
}

#[post("/api/auth/profile", user: User)]
pub async fn update_user_profile_handler(
    body: UpdateUserProfileRequest,
) -> Result<User> {
    let conf = crate::config::get();
    let cli = conf.dynamodb();

    let mut updater = User::updater(user.pk.clone(), user.sk.clone());
    let mut has_update = false;

    if let Some(nickname) = &body.nickname {
        updater = updater.with_display_name(nickname.clone());
        has_update = true;
    }

    if let Some(profile_url) = &body.profile_url {
        updater = updater.with_profile_url(profile_url.clone());
        has_update = true;
    }

    if let Some(description) = &body.description {
        updater = updater.with_description(description.clone());
        has_update = true;
    }

    if has_update {
        updater.execute(cli).await?;
    }

    let updated_user = User::get(cli, user.pk.clone(), Some(EntityType::User))
        .await?
        .ok_or(Error::NotFound("User not found".into()))?;

    Ok(updated_user)
}
