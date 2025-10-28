use crate::{
    AppState, Error,
    models::{
        team::{Team, TeamMetadata},
        user::User,
    },
};
use bdk::prelude::*;
use by_axum::{
    aide::NoApi,
    axum::{
        Json,
        extract::{Path, State},
    },
};
use serde::Deserialize;

use super::dto::TeamDetailResponse;

#[derive(Debug, Clone, Deserialize, aide::OperationIo, JsonSchema)]
pub struct GetTeamPathParams {
    pub team_pk: String,
}

pub type GetTeamResponse = TeamDetailResponse;

pub async fn get_team_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(path): Path<GetTeamPathParams>,
) -> Result<Json<GetTeamResponse>, Error> {
    let team = TeamMetadata::query(&dynamo.client, path.team_pk.clone()).await?;
    if team.is_empty() {
        return Err(Error::NotFound("Team not found".into()));
    }

    let mut team_response = TeamDetailResponse::from(team);

    // Add user's permissions if authenticated
    if let Some(user) = user {
        let team_pk = crate::types::Partition::Team(
            path.team_pk
                .strip_prefix("TEAM#")
                .unwrap_or(&path.team_pk)
                .to_string(),
        );
        let permissions = Team::get_permissions_by_team_pk(&dynamo.client, &team_pk, &user.pk)
            .await
            .unwrap_or_default();
        team_response.permissions = Some(permissions.into());
    }

    Ok(Json(team_response))
}
