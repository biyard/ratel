#[cfg(test)]
pub mod users_tests {
    use crate::controllers::v3::users::update_user::{UpdateUserRequest, update_user_handler};
    use dto::by_axum::axum::{
        Json,
        extract::{Extension, State},
    };
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
        use crate::tests::{create_app_state, create_auth, create_user_name, get_test_user};
        use crate::{
            controllers::v3::teams::{
                create_team::{CreateTeamRequest, create_team_handler},
                get_team::{GetTeamPathParams, get_team_handler},
            },
            types::Theme,
        };
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
}
