use crate::{
    AppState, Error,
    models::{TeamGroup, TeamOwner, TeamQueryOption, team::Team, team_owner, user::User},
    types::EntityType,
};
use bdk::prelude::*;
use by_axum::{
    aide::NoApi,
    axum::{
        Json,
        extract::{Query, State},
    },
};
use serde::{Deserialize, Serialize};

use super::dto::TeamResponse;

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct FindTeamQueryParams {
    #[schemars(description = "Search by username")]
    pub username: String,
}

pub async fn find_team_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Query(params): Query<FindTeamQueryParams>,
) -> Result<Json<TeamResponse>, Error> {
    let gsi2_sk_prefix = Team::compose_gsi2_sk(String::default());
    let team_query_option = Team::opt().sk(gsi2_sk_prefix);

    let (team, _) =
        Team::find_by_username_prefix(&dynamo.client, params.username.clone(), team_query_option)
            .await?;

    let team = team.into_iter().next().ok_or(Error::TeamNotFound)?;

    let permissions = if let Some(user) = user {
        let permissions = Team::get_permissions_by_team_pk(&dynamo.client, &team.pk, &user.pk)
            .await
            .unwrap_or_default();
        permissions.into()
    } else {
        0
    };

    Ok(Json(TeamResponse::from((team, permissions))))
}
