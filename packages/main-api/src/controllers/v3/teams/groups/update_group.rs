use crate::models::team::TeamGroup;
use crate::types::{TeamGroupPermission, TeamGroupPermissions};
use crate::utils::security::{RatelResource, check_any_permission};
use crate::{AppState, Error2};
use dto::by_axum::{
    auth::Authorization,
    axum::{
        Extension,
        extract::{Json, Path, State},
    },
};
use dto::{JsonSchema, aide, schemars};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, aide::OperationIo, JsonSchema)]
pub struct UpdateGroupPathParams {
    #[schemars(description = "Team PK to be updated")]
    pub team_pk: String,
    #[schemars(description = "Group SK to be updated")]
    pub group_sk: String,
}

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct UpdateGroupRequest {
    #[schemars(description = "Group name to update")]
    pub name: Option<String>,
    #[schemars(description = "Group description to update")]
    pub description: Option<String>,
    #[schemars(description = "Group permissions to update")]
    pub permissions: Option<Vec<TeamGroupPermission>>,
}

pub async fn update_group_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(auth): Extension<Option<Authorization>>,
    Path(params): Path<UpdateGroupPathParams>,
    Json(req): Json<UpdateGroupRequest>,
) -> Result<(), Error2> {
    let required_permissions = if req.permissions.is_some() {
        vec![TeamGroupPermission::TeamAdmin]
    } else {
        vec![
            TeamGroupPermission::TeamAdmin,
            TeamGroupPermission::GroupEdit,
            TeamGroupPermission::TeamEdit,
        ]
    };
    check_any_permission(
        &dynamo.client,
        auth,
        RatelResource::Team {
            team_pk: params.team_pk.clone(),
        },
        required_permissions,
    )
    .await?;

    let mut need_update_user_permissions = false;

    let mut updater = TeamGroup::updater(params.team_pk.clone(), params.group_sk.clone());

    if let Some(name) = req.name {
        updater = updater.with_name(name);
    }
    if let Some(description) = req.description {
        updater = updater.with_description(description);
    }
    if let Some(permissions) = req.permissions {
        //FIXME: Permission change should be restricted to team owners only
        updater = updater.with_permissions(TeamGroupPermissions(permissions).into());
        need_update_user_permissions = true;
    }
    updater.execute(&dynamo.client).await?;

    if need_update_user_permissions {
        //FIXME: Update user permissions
    }
    Ok(())
}

#[cfg(test)]
pub mod update_group_tests {
    use super::*;
    use crate::{
        controllers::v3::teams::{
            create_team::{CreateTeamRequest, create_team_handler},
            groups::{
                add_member::{AddMemberPathParams, AddMemberRequest, add_member_handler},
                create_group::{CreateGroupPathParams, CreateGroupRequest, create_group_handler},
            },
        },
        tests::{create_app_state, create_auth, get_test_user},
    };

    #[tokio::test]
    async fn test_update_group_handler() {
        let app_state = create_app_state();
        let user = get_test_user(app_state.clone()).await;
        let auth = create_auth(user.clone()).await;

        let team_username = format!("TEAM{}", uuid::Uuid::new_v4().to_string());
        // Create a team
        let team = create_team_handler(
            State(app_state.clone()),
            Extension(Some(auth.clone())),
            Json(CreateTeamRequest {
                username: team_username.clone(),
                nickname: format!("{}'s Team", team_username),
                profile_url: "https://example.com/profile.png".into(),
                description: "This is a test team".into(),
            }),
        )
        .await;

        assert!(team.is_ok(), "Failed to create team: {:?}", team.err());
        let team = team.unwrap().0;

        // Create a team group
        let team_group = create_group_handler(
            State(app_state.clone()),
            Extension(Some(auth.clone())),
            Path(CreateGroupPathParams {
                team_pk: team.team_pk.clone(),
            }),
            Json(CreateGroupRequest {
                name: "Test Group".into(),
                description: "A group for testing".into(),
                image_url: "https://example.com/image.png".into(),
                permissions: vec![TeamGroupPermission::GroupEdit],
            }),
        )
        .await;
        assert!(
            team_group.is_ok(),
            "Failed to create team group: {:?}",
            team_group.err()
        );

        let team_group = team_group.unwrap().0;

        let res = update_group_handler(
            State(app_state.clone()),
            Extension(Some(auth.clone())),
            Path(UpdateGroupPathParams {
                team_pk: team.team_pk.clone(),
                group_sk: team_group.group_sk.clone(),
            }),
            Json(UpdateGroupRequest {
                name: Some("Updated Group Name".into()),
                description: Some("Updated description".into()),
                permissions: Some(vec![
                    TeamGroupPermission::GroupEdit,
                    TeamGroupPermission::TeamEdit,
                ]),
            }),
        )
        .await;

        assert!(res.is_ok(), "Failed to update group: {:?}", res.err());
    }
    #[tokio::test]
    async fn test_update_with_permisison() {
        let app_state = create_app_state();
        let user = get_test_user(app_state.clone()).await;
        let auth = create_auth(user.clone()).await;

        let team_username = format!("TEAM{}", uuid::Uuid::new_v4().to_string());
        // Create a team
        let team = create_team_handler(
            State(app_state.clone()),
            Extension(Some(auth.clone())),
            Json(CreateTeamRequest {
                username: team_username.clone(),
                nickname: format!("{}'s Team", team_username),
                profile_url: "https://example.com/profile.png".into(),
                description: "This is a test team".into(),
            }),
        )
        .await;

        assert!(team.is_ok(), "Failed to create team: {:?}", team.err());
        let team = team.unwrap().0;

        // Create a team group
        let team_group = create_group_handler(
            State(app_state.clone()),
            Extension(Some(auth.clone())),
            Path(CreateGroupPathParams {
                team_pk: team.team_pk.clone(),
            }),
            Json(CreateGroupRequest {
                name: "Test Group".into(),
                description: "A group for testing".into(),
                image_url: "https://example.com/image.png".into(),
                permissions: vec![TeamGroupPermission::GroupEdit],
            }),
        )
        .await;
        assert!(
            team_group.is_ok(),
            "Failed to create team group: {:?}",
            team_group.err()
        );

        let team_group = team_group.unwrap().0;

        let user2 = get_test_user(app_state.clone()).await;

        let res = add_member_handler(
            State(app_state.clone()),
            Extension(Some(auth.clone())),
            Path(AddMemberPathParams {
                team_pk: team.team_pk.clone(),
                group_sk: team_group.group_sk.clone(),
            }),
            Json(AddMemberRequest {
                user_pks: vec![user2.pk.to_string()],
            }),
        )
        .await;
        assert!(
            res.is_ok(),
            "Failed to add member to group: {:?}",
            res.err()
        );
        let res = res.unwrap().0;

        assert!(
            res.total_added == 1,
            "Expected total_added to be 1 but got: {:?}",
            res.total_added
        );

        let auth2 = create_auth(user2.clone()).await;

        // Try to update permission with user2 (should fail)
        let res = update_group_handler(
            State(app_state.clone()),
            Extension(Some(auth2.clone())),
            Path(UpdateGroupPathParams {
                team_pk: team.team_pk.clone(),
                group_sk: team_group.group_sk.clone(),
            }),
            Json(UpdateGroupRequest {
                name: Some("Updated Group Name".into()),
                description: Some("Updated description".into()),
                permissions: Some(vec![
                    TeamGroupPermission::GroupEdit,
                    TeamGroupPermission::TeamEdit,
                ]),
            }),
        )
        .await;
        assert!(
            res.is_err(),
            "Expected error reason: without Permission but got: {:?}",
            res.ok()
        );
        // Update permission with user2 (should succeed)
        let res = update_group_handler(
            State(app_state.clone()),
            Extension(Some(auth2.clone())),
            Path(UpdateGroupPathParams {
                team_pk: team.team_pk.clone(),
                group_sk: team_group.group_sk.clone(),
            }),
            Json(UpdateGroupRequest {
                name: Some("Updated Group Name".into()),
                description: Some("Updated description".into()),
                permissions: None, // No permission change
            }),
        )
        .await;

        assert!(res.is_ok(), "Failed to update group: {:?}", res.err());
    }
}
