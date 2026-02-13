use crate::{
    AppState, Error,
    models::{
        team::{TeamOwner, TeamOwnerQueryOption},
        user::{User, UserDetailResponse, UserEvmAddress, UserMetadata},
    },
    types::Theme,
    utils::validator::{validate_description, validate_image_url, validate_nickname},
};
use bdk::prelude::*;
use by_axum::{
    aide::NoApi,
    axum::{Json, extract::State},
};
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
    NoApi(user): NoApi<Option<User>>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<UpdateUserResponse>, Error> {
    let user = match user {
        Some(u) => u,
        None => return Err(Error::Unauthorized("Authentication required".into())),
    };

    match req {
        UpdateUserRequest::Theme { theme } => {
            User::updater(user.pk.clone(), user.sk)
                .with_theme(theme)
                .execute(&dynamo.client)
                .await?;
        }
        UpdateUserRequest::EvmAddress { evm_address } => {
            UserEvmAddress::new(user.pk.clone(), evm_address)
                .upsert(&dynamo.client)
                .await?;
        }
        UpdateUserRequest::Profile {
            nickname,
            profile_url,
            description,
        } => {
            validate_nickname(&nickname)?;
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
