use super::super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum UpdateUserRequest {
    Profile {
        nickname: String,
        profile_url: String,
        description: String,
    },
    Theme {
        theme: String,
    },
    EvmAddress {
        evm_address: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UserProfileResponse {
    pub username: String,
    pub display_name: String,
    pub profile_url: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UserDetailResponse {
    pub user: UserProfileResponse,
    pub evm_address: Option<String>,
}

#[get("/api/me", user: ratel_auth::User)]
pub async fn get_user_detail_handler() -> Result<UserDetailResponse> {
    let conf = common::config::ServerConfig::default();
    let cli = conf.dynamodb();

    let user = ratel_auth::User::get(cli, user.pk.clone(), Some(EntityType::User))
        .await?
        .ok_or(Error::NotFound("User not found".into()))?;

    let evm_address = ratel_auth::UserEvmAddress::get(
        cli,
        user.pk.clone(),
        Some(EntityType::UserEvmAddress),
    )
    .await?
    .map(|item| item.evm_address);

    Ok(UserDetailResponse {
        user: UserProfileResponse {
            username: user.username,
            display_name: user.display_name,
            profile_url: user.profile_url,
            description: user.description,
        },
        evm_address,
    })
}

#[post("/api/me", user: ratel_auth::User)]
pub async fn update_user_handler(body: UpdateUserRequest) -> Result<UserDetailResponse> {
    let conf = common::config::ServerConfig::default();
    let cli = conf.dynamodb();

    match body {
        UpdateUserRequest::Profile {
            nickname,
            profile_url,
            description,
        } => {
            let mut updater = ratel_auth::User::updater(user.pk.clone(), user.sk.clone())
                .with_display_name(nickname)
                .with_profile_url(profile_url)
                .with_description(description);
            updater.execute(cli).await?;
        }
        UpdateUserRequest::EvmAddress { evm_address } => {
            ratel_auth::UserEvmAddress::new(user.pk.clone(), evm_address)
                .upsert(cli)
                .await?;
        }
        UpdateUserRequest::Theme { .. } => {
            return Err(Error::NotSupported(
                "Theme update is not supported in settings".to_string(),
            ));
        }
    }

    let user = ratel_auth::User::get(cli, user.pk.clone(), Some(EntityType::User))
        .await?
        .ok_or(Error::NotFound("User not found".into()))?;

    let evm_address = ratel_auth::UserEvmAddress::get(
        cli,
        user.pk.clone(),
        Some(EntityType::UserEvmAddress),
    )
    .await?
    .map(|item| item.evm_address);

    Ok(UserDetailResponse {
        user: UserProfileResponse {
            username: user.username,
            display_name: user.display_name,
            profile_url: user.profile_url,
            description: user.description,
        },
        evm_address,
    })
}
