use crate::{
    AppState, Error,
    models::{
        team::{Team, TeamGroup},
        user::{User, UserTeam, UserTeamGroup},
    },
    services::fcm_notification::FCMService,
    types::{EntityType, Partition, TeamGroupPermission},
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
use std::collections::HashSet;

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct AddMemberPathParams {
    #[schemars(description = "Team PK to be updated")]
    pub team_pk: String,
    #[schemars(description = "Group SK to be updated")]
    pub group_sk: String,
}

#[derive(Debug, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct AddMemberRequest {
    #[schemars(description = "User PKs to add to the group")]
    pub user_pks: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct AddMemberResponse {
    pub total_added: i64,
    pub failed_pks: Vec<String>,
}

pub async fn add_member_handler(
    State(AppState { dynamo, ses, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(params): Path<AddMemberPathParams>,
    Json(req): Json<AddMemberRequest>,
) -> Result<Json<AddMemberResponse>, Error> {
    let user = user.ok_or(Error::Unauthorized("Authentication required".into()))?;

    // Check permissions
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

    // Get the team and group
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
    let mut invite_emails: Vec<String> = Vec::new();
    let mut invite_pks: Vec<Partition> = Vec::new();

    let mut seen_pks = HashSet::new();

    for member in &req.user_pks {
        if !seen_pks.insert(member.as_str()) {
            failed_pks.push(member.to_string());
            continue;
        }

        let user = User::get(&dynamo.client, member, Some(EntityType::User)).await?;
        if user.is_none() {
            failed_pks.push(member.to_string());
            continue;
        }
        let user = user.unwrap();

        // Check if UserTeam already exists, if not create it
        let user_team_sk = EntityType::UserTeam(team.pk.to_string());
        let existing_user_team =
            UserTeam::get(&dynamo.client, &user.pk, Some(&user_team_sk)).await?;

        if existing_user_team.is_none() {
            UserTeam::new(user.pk.clone(), team.clone())
                .create(&dynamo.client)
                .await?;

            invite_emails.push(user.email.clone());
            invite_pks.push(user.pk.clone());
        }

        let user_team_group_sk = EntityType::UserTeamGroup(team_group.sk.to_string());
        let existing_user_team_group =
            UserTeamGroup::get(&dynamo.client, &user.pk, Some(&user_team_group_sk)).await?;

        if existing_user_team_group.is_some() {
            continue;
        }

        // Create UserTeamGroup only if the user is not already in the group.
        UserTeamGroup::new(user.pk, team_group.clone())
            .create(&dynamo.client)
            .await?;
        success_count += 1;
    }

    // Bulk send team invite emails (only for newly-linked users)
    if !invite_emails.is_empty() {
        let _ = UserTeam::send_email(&dynamo, &ses, team.clone(), invite_emails).await?;

        // FIXME: fix to one call code
        let mut fcm = FCMService::new().await?;
        let _ = UserTeam::send_notification(&dynamo, &mut fcm, invite_pks, &team).await?;
    }

    TeamGroup::updater(team_group.pk, team_group.sk)
        .increase_members(success_count)
        .execute(&dynamo.client)
        .await?;

    Ok(Json(AddMemberResponse {
        total_added: success_count,
        failed_pks,
    }))
}
