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
#[serde(rename_all = "camelCase")]
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
        UserTelegram::new(user.pk.clone(), telegram_user.id, telegram_raw)
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

#[tokio::test]
async fn test_update_user_handler() {
    use crate::{
        tests::{create_app_state, create_auth, get_test_user},
        types::Theme,
    };

    let app_state = create_app_state();
    let user = get_test_user(app_state.clone()).await;
    let auth = create_auth(user.clone()).await;

    let now = chrono::Utc::now().timestamp();
    let new_nickname = format!("updated_nickname_{}", now);
    let new_profile_url = format!("https://new.url/profile_{}.png", now);
    let new_description = "This is the updated description.".to_string();
    let new_theme = Theme::Dark;
    let new_evm_address = "0x1234567890123456789012345678901234567890".to_string();

    let req = UpdateUserRequest {
        nickname: Some(new_nickname.clone()),
        profile_url: Some(new_profile_url.clone()),
        description: Some(new_description.clone()),
        theme: Some(new_theme.clone()),
        evm_address: Some(new_evm_address.clone()),
        telegram_raw: None,
    };

    let res = update_user_handler(State(app_state), Extension(Some(auth)), Json(req)).await;

    assert!(res.is_ok(), "Failed to update user: {:?}", res.err());
    let updated_user_response = res.unwrap().0;
    let user_detail = updated_user_response.user;

    assert_eq!(
        user_detail.nickname, new_nickname,
        "Nickname was not updated."
    );
    assert_eq!(
        user_detail.profile_url, new_profile_url,
        "Profile URL was not updated."
    );
    assert_eq!(
        user_detail.description, new_description,
        "Description was not updated."
    );

    assert_eq!(user_detail.theme, new_theme as u8, "Theme was not updated.");

    let has_evm_address = updated_user_response
        .evm_address
        .is_some_and(|v| v == new_evm_address);
    assert!(has_evm_address, "EVM address was not added or updated.");
}

#[cfg(test)]
pub mod update_user_tests {
    use super::*;
    use crate::controllers::v3::teams::{
        create_team::{CreateTeamRequest, create_team_handler},
        get_team::{GetTeamPathParams, get_team_handler},
    };
    use crate::tests::{create_app_state, create_auth, create_user_name, get_test_user};
    use dto::by_axum::axum::extract::Path;
    #[tokio::test]
    async fn test_update_user_with_team_handler() {
        let app_state = create_app_state();
        let user = get_test_user(app_state.clone()).await;
        let auth = create_auth(user.clone()).await;
        let username = create_user_name();
        let team_display_name = format!("test_team_{}", username);
        let team_username = format!("test_username_{}", username);

        // Create Team
        let create_res = create_team_handler(
            State(app_state.clone()),
            Extension(Some(auth.clone())),
            Json(CreateTeamRequest {
                nickname: team_display_name.clone(),
                username: team_username.clone(),
                description: "This is a test team".into(),
                profile_url: "https://example.com/profile.png".into(),
            }),
        )
        .await;
        assert!(
            create_res.is_ok(),
            "Failed to create team {:?}",
            create_res.err()
        );
        let team = create_res.unwrap().0;

        // Update User
        let new_username = create_user_name();
        let new_nickname = format!("updated_nickname_{}", new_username);
        let new_profile_url = format!("https://new.url/profile_{}.png", new_username);
        let new_description = "This is the updated description.".to_string();
        let new_theme = Theme::Dark;

        let update_user_res = update_user_handler(
            State(app_state.clone()),
            Extension(Some(auth.clone())),
            Json(UpdateUserRequest {
                nickname: Some(new_nickname.clone()),
                profile_url: Some(new_profile_url.clone()),
                description: Some(new_description.clone()),
                theme: Some(new_theme.clone()),
                evm_address: None,
                telegram_raw: None,
            }),
        )
        .await;
        assert!(
            update_user_res.is_ok(),
            "Failed to update user {:?}",
            update_user_res.err()
        );

        let team = get_team_handler(
            State(app_state.clone()),
            Extension(Some(auth.clone())),
            Path(GetTeamPathParams {
                team_pk: team.team_pk.clone(),
            }),
        )
        .await;
        assert!(team.is_ok(), "Failed to get team {:?}", team.err());
        let team_owner = team.unwrap().0.owner;
        assert!(team_owner.is_some(), "Team owner should exist");
        let team_owner = team_owner.unwrap();
        assert_eq!(
            team_owner.display_name, new_nickname,
            "Team owner display name was not updated"
        );
    }

    // Additional tests can be added here if needed
}
