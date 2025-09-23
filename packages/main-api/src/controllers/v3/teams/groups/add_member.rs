use crate::{
    AppState, Error2,
    models::{
        team::{Team, TeamGroup},
        user::{User, UserTeam, UserTeamGroup},
    },
    types::{EntityType, TeamGroupPermission},
    utils::security::{RatelResource, check_any_permission},
};
use dto::by_axum::{
    auth::Authorization,
    axum::{
        Extension,
        extract::{Json, Path, State},
    },
};
use dto::{JsonSchema, aide, schemars};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct AddMemberPathParams {
    #[schemars(description = "Team PK to be updated")]
    pub team_pk: String,
    #[schemars(description = "Group SK to be updated")]
    pub group_sk: String,
}

#[derive(Debug, Deserialize, Default, aide::OperationIo, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddMemberRequest {
    #[schemars(description = "User PKs to add to the group")]
    pub user_pks: Vec<String>,
}

#[derive(Debug, Serialize, Default, aide::OperationIo, JsonSchema)]
pub struct AddMemberResponse {
    pub total_added: i64,
    pub failed_pks: Vec<String>,
}

pub async fn add_member_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(auth): Extension<Option<Authorization>>,
    Path(params): Path<AddMemberPathParams>,
    Json(req): Json<AddMemberRequest>,
) -> Result<Json<AddMemberResponse>, Error2> {
    check_any_permission(
        &dynamo.client,
        auth,
        RatelResource::Team {
            team_pk: params.team_pk.clone(),
        },
        vec![
            TeamGroupPermission::GroupEdit,
            TeamGroupPermission::TeamAdmin,
            TeamGroupPermission::TeamEdit,
        ],
    )
    .await?;

    let team = Team::get(&dynamo.client, &params.team_pk, Some(EntityType::Team)).await?;
    if team.is_none() {
        return Err(Error2::NotFound("Team not found".into()));
    }
    let team_group = TeamGroup::get(&dynamo.client, &params.team_pk, Some(params.group_sk)).await?;
    if team_group.is_none() {
        return Err(Error2::NotFound("Team group not found".into()));
    }
    let team = team.unwrap();
    let team_group = team_group.unwrap();

    let mut success_count = 0;
    let mut failed_pks = vec![];
    for member in &req.user_pks {
        let user = User::get(&dynamo.client, member, Some(EntityType::User)).await?;
        if user.is_none() {
            failed_pks.push(member.to_string());
            continue;
        }
        let user = user.unwrap();
        UserTeam::new(user.pk.clone(), team.clone())
            .create(&dynamo.client)
            .await?;
        UserTeamGroup::new(user.pk, team_group.clone())
            .create(&dynamo.client)
            .await?;
        success_count += 1;
    }
    TeamGroup::updater(team_group.pk, team_group.sk)
        .with_members(success_count)
        .execute(&dynamo.client)
        .await?;
    Ok(Json(AddMemberResponse {
        total_added: success_count,
        failed_pks,
    }))
}

#[cfg(test)]
pub mod add_member_tests {
    use super::*;
    use crate::{
        controllers::v3::{
            teams::{
                create_team::{CreateTeamRequest, create_team_handler},
                get_team::{GetTeamPathParams, get_team_handler},
                groups::create_group::{
                    CreateGroupPathParams, CreateGroupRequest, create_group_handler,
                },
            },
            users::get_user_info::get_user_info_handler,
        },
        tests::{create_app_state, create_auth, get_test_user},
        types::TeamGroupPermission,
    };

    #[tokio::test]
    async fn test_add_member_handler() {
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
                team_id: team.team_pk.clone(),
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

        // Create Some users to be added
        let user2 = get_test_user(app_state.clone()).await;
        let auth2 = create_auth(user2.clone()).await;

        let user3 = get_test_user(app_state.clone()).await;

        // Call add_member_handler
        let add_member_res = add_member_handler(
            State(app_state.clone()),
            Extension(Some(auth.clone())),
            Path(AddMemberPathParams {
                team_pk: team.team_pk.clone(),
                group_sk: team_group.group_sk.clone(),
            }),
            Json(AddMemberRequest {
                user_pks: vec![user2.pk.to_string(), user3.pk.to_string()],
            }),
        )
        .await;

        assert!(
            add_member_res.is_ok(),
            "Failed to add members: {:?}",
            add_member_res.err()
        );

        let team = get_team_handler(
            State(app_state.clone()),
            Extension(Some(auth.clone())),
            Path(GetTeamPathParams {
                team_pk: team.team_pk.clone(),
            }),
        )
        .await;

        assert!(
            team.is_ok(),
            "Failed to get team after adding members: {:?}",
            team.err()
        );

        let team = team.unwrap().0;
        let res = team.groups.unwrap_or_default();
        let team_group = res
            .into_iter()
            .find(|g| g.sk == team_group.group_sk)
            .expect("Team group should exist");

        assert_eq!(team_group.members, 2, "Team group members should be 2");

        let user2 =
            get_user_info_handler(State(app_state.clone()), Extension(Some(auth2.clone()))).await;

        assert!(user2.is_ok(), "Failed to get user2 info: {:?}", user2.err());
        let user2 = user2.unwrap().0;
        let user2_teams = user2.teams.unwrap_or_default();

        assert_eq!(user2_teams.len(), 1, "User2 should be in 1 team");
    }
}
