use crate::{
    AppState, Error2,
    models::{
        team::{TeamOwner, TeamOwnerQueryOption},
        user::{User, UserDetailResponse, UserEvmAddress, UserMetadata, UserTelegram},
    },
    types::Theme,
    utils::{
        dynamo_extractor::extract_user,
        telegram::parse_telegram_raw,
        validator::{validate_description, validate_image_url, validate_nickname},
    },
};
use dto::by_axum::{
    auth::Authorization,
    axum::{
        Extension,
        extract::{Json, State},
    },
};
use dto::{JsonSchema, aide, schemars};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Default, aide::OperationIo, JsonSchema, Validate)]
pub struct UpdateUserRequest {
    #[schemars(description = "User Nickname to update")]
    #[validate(custom(function = "validate_nickname"))]
    pub nickname: Option<String>,
    #[schemars(description = "User profile URL to update")]
    #[validate(custom(function = "validate_image_url"))]
    pub profile_url: Option<String>,
    #[schemars(description = "User description to update")]
    #[validate(custom(function = "validate_description"))]
    pub description: Option<String>,
    #[schemars(description = r#"User Theme ("light" or "dark") to update"#)]
    pub theme: Option<Theme>,
    pub evm_address: Option<String>,
    pub telegram_raw: Option<String>,
}

pub type UpdateUserResponse = UserDetailResponse;

pub async fn update_user_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(auth): Extension<Option<Authorization>>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<UpdateUserResponse>, Error2> {
    let mut user = extract_user(&dynamo.client, auth).await?;
    let mut updater = User::updater(user.pk.clone(), user.sk);
    let mut need_update = false;
    if let Some(nickname) = req.nickname {
        updater = updater.with_display_name(nickname.clone());
        user.display_name = nickname;
        need_update = true;
    }
    if let Some(profile_url) = req.profile_url {
        updater = updater.with_profile_url(profile_url.clone());
        user.profile_url = profile_url;
        need_update = true;
    }
    if let Some(description) = req.description {
        updater = updater.with_description(description.clone());
        user.description = description;
        need_update = true;
    }
    if let Some(theme) = req.theme {
        updater = updater.with_theme(theme);
    }

    updater.execute(&dynamo.client).await?;

    if let Some(evm_address) = req.evm_address {
        UserEvmAddress::new(user.pk.clone(), evm_address)
            .create(&dynamo.client)
            .await?;
    }

    if let Some(telegram_raw) = req.telegram_raw {
        let telegram_user = parse_telegram_raw(telegram_raw.clone())?;
        UserTelegram::new(user.pk.clone(), telegram_user.id)
            .create(&dynamo.client)
            .await?;
    }

    // Update Team Owner Update

    if need_update {
        let mut bookmark = None;
        loop {
            let mut option = TeamOwnerQueryOption::builder();
            if bookmark.is_some() {
                option = option.bookmark(bookmark.unwrap());
            };
            let (team_owners, next) =
                TeamOwner::find_by_user_pk(&dynamo.client, &user.pk, option).await?;
            for team_owner in &team_owners {
                tracing::debug!("Found team owner: {:?}", team_owner);
                TeamOwner::updater(&team_owner.pk, &team_owner.sk)
                    .with_display_name(user.display_name.clone())
                    .with_profile_url(user.profile_url.clone())
                    .execute(&dynamo.client)
                    .await?;
            }

            if next.is_none() {
                break;
            }
            bookmark = next;
        }
    }
    let user = UserMetadata::query(&dynamo.client, user.pk).await?;
    let user: UserDetailResponse = UserDetailResponse::from(user);
    Ok(Json(user))
}
