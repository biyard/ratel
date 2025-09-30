use crate::{
    AppState, Error2,
    models::{
        team::{TeamOwner, TeamOwnerQueryOption},
        user::{User, UserDetailResponse, UserEvmAddress, UserMetadata},
    },
    types::Theme,
    utils::{
        dynamo_extractor::extract_user,
        validator::{validate_description, validate_image_url, validate_username},
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

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub enum UpdateUserRequest {
    Profile {
        #[schemars(description = "User Nickname to update")]
        nickname: String,
        #[schemars(description = "User profile URL to update")]
        profile_url: String,
        #[schemars(description = "User description to update")]
        description: String,
    },
    #[schemars(description = r#"User Theme ("light" or "dark") to update"#)]
    Theme {
        theme: Theme,
    },
    EvmAddress {
        evm_address: String,
    },
}

pub type UpdateUserResponse = UserDetailResponse;

pub async fn update_user_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(auth): Extension<Option<Authorization>>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<UpdateUserResponse>, Error2> {
    let user = extract_user(&dynamo.client, auth).await?;

    match req {
        UpdateUserRequest::Theme { theme } => {
            User::updater(user.pk.clone(), user.sk)
                .with_theme(theme)
                .execute(&dynamo.client)
                .await?;
        }
        UpdateUserRequest::EvmAddress { evm_address } => {
            UserEvmAddress::new(user.pk.clone(), evm_address)
                .create(&dynamo.client)
                .await?;
        }
        UpdateUserRequest::Profile {
            nickname,
            profile_url,
            description,
        } => {
            validate_username(&nickname)?;
            validate_image_url(&profile_url)?;
            validate_description(&description)?;
            User::updater(user.pk.clone(), user.sk)
                .with_display_name(nickname.clone())
                .with_profile_url(profile_url.clone())
                .with_description(description.clone())
                .execute(&dynamo.client)
                .await?;

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
                        .with_display_name(nickname.clone())
                        .with_profile_url(profile_url.clone())
                        .execute(&dynamo.client)
                        .await?;
                }

                if next.is_none() {
                    break;
                }
                bookmark = next;
            }
        }
    }

    let user = UserMetadata::query(&dynamo.client, user.pk).await?;
    let user: UserDetailResponse = UserDetailResponse::from(user);
    Ok(Json(user))
}
