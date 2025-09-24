use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    RatelResource, Result, User,
    by_axum::auth::Authorization,
    sqlx::{Pool, Postgres},
};

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct DeleteTeamRequest {
    #[schemars(description = "Deleted Team ID")]
    pub team_id: i64,
}

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct DeleteTeamResponse {
    pub team_id: i64,
}

use crate::security::check_perm;

pub async fn delete_team_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<Pool<Postgres>>,
    Json(req): Json<DeleteTeamRequest>,
) -> Result<Json<DeleteTeamResponse>> {
    let team_id = req.team_id;
    let repo = User::get_repository(pool.clone());

    let _ = check_perm(
        &pool,
        auth,
        RatelResource::Team { team_id: team_id },
        dto::GroupPermission::DeleteGroup,
    )
    .await?;

    let _ = repo.delete(team_id).await?;

    Ok(Json(DeleteTeamResponse { team_id: team_id }))
}
