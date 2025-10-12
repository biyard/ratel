use crate::{
    AppState, Error2,
    models::{
        team::{Team, TeamGroup},
        user::{User, UserTeam, UserTeamGroup},
    },
    types::{EntityType, TeamGroupPermission},
    utils::security::{RatelResource, check_any_permission_with_user},
};
use dto::by_axum::{
    aide::NoApi,
    axum::{
        Json,
        extract::{Path, State},
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
    NoApi(user): NoApi<Option<User>>,
    Path(params): Path<AddMemberPathParams>,
    Json(req): Json<AddMemberRequest>,
) -> Result<Json<AddMemberResponse>, Error2> {
    let user = user.ok_or(Error2::Unauthorized("Authentication required".into()))?;
    
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
    let team_group = TeamGroup::get(&dynamo.client, &params.team_pk, Some(params.group_sk)).await?;

    let team = team.ok_or(Error2::NotFound("Team not found".into()))?;
    let team_group = team_group.ok_or(Error2::NotFound("Team group not found".into()))?;

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
        .increase_members(success_count)
        .execute(&dynamo.client)
        .await?;
    Ok(Json(AddMemberResponse {
        total_added: success_count,
        failed_pks,
    }))
}
