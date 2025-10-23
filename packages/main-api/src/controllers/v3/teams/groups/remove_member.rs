use crate::{
    AppState, Error,
    models::{
        team::{Team, TeamGroup},
        user::{User, UserTeam, UserTeamGroup},
    },
    types::{EntityType, TeamGroupPermission},
    utils::security::{RatelResource, check_any_permission_with_user},
};
use bdk::prelude::*;
use by_axum::{
    aide::NoApi,
    axum::{
        Json,
        extract::{Path, State},
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct RemoveMemberPathParams {
    #[schemars(description = "Team PK to be updated")]
    pub team_pk: String,
    #[schemars(description = "Group SK to be updated")]
    pub group_sk: String,
}

#[derive(Debug, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct RemoveMemberRequest {
    #[schemars(description = "User PKs to remove from the group")]
    pub user_pks: Vec<String>,
}

#[derive(Debug, Serialize, Default, aide::OperationIo, JsonSchema)]
pub struct RemoveMemberResponse {
    pub total_removed: i64,
    pub failed_pks: Vec<String>,
}

//
pub async fn remove_member_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(params): Path<RemoveMemberPathParams>,
    Json(req): Json<RemoveMemberRequest>,
) -> Result<Json<RemoveMemberResponse>, Error> {
    let user = user.ok_or(Error::Unauthorized("Authentication required".into()))?;

    check_any_permission_with_user(
        &dynamo.client,
        &user,
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
    let team_group = TeamGroup::get(
        &dynamo.client,
        &params.team_pk,
        Some(EntityType::TeamGroup(params.group_sk.clone())),
    )
    .await?;

    let team = team.ok_or(Error::NotFound("Team not found".into()))?;
    let team_group = team_group.ok_or(Error::NotFound("Team group not found".into()))?;

    let mut success_count = 0;
    let mut failed_pks = vec![];

    for member in &req.user_pks {
        let user = User::get(&dynamo.client, member, Some(EntityType::User)).await?;
        if user.is_none() {
            failed_pks.push(member.to_string());
            continue;
        }
        let user = user.unwrap();
        //TODO: implement batch delete
        UserTeamGroup::delete(
            &dynamo.client,
            &user.pk,
            Some(EntityType::UserTeamGroup(team_group.sk.to_string())),
        )
        .await?;
        let (user_team_group, _) =
            UserTeamGroup::find_by_team_group_pk(&dynamo.client, &user.pk, Default::default())
                .await?;
        if user_team_group.is_empty() {
            // If user is not part of any other groups in the team, remove from UserTeam as well
            UserTeam::delete(
                &dynamo.client,
                &user.pk,
                Some(EntityType::UserTeam(team.pk.to_string())),
            )
            .await?;
        }
        success_count += 1;
    }

    TeamGroup::updater(team_group.pk, team_group.sk)
        .decrease_members(success_count)
        .execute(&dynamo.client)
        .await?;

    Ok(Json(RemoveMemberResponse {
        total_removed: success_count,
        failed_pks,
    }))
}
