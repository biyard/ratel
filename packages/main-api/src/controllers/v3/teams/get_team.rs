use crate::{
    AppState, Error2,
    models::team::{TeamDetailResponse, TeamMetadata},
};
use dto::by_axum::{
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, State},
    },
};
use dto::{JsonSchema, aide, schemars};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, aide::OperationIo, JsonSchema)]
pub struct GetTeamPathParams {
    pub team_pk: String,
}

pub type GetTeamResponse = TeamDetailResponse;

pub async fn get_team_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(_auth): Extension<Option<Authorization>>,
    Path(path): Path<GetTeamPathParams>,
) -> Result<Json<GetTeamResponse>, Error2> {
    let team = TeamMetadata::query(&dynamo.client, path.team_pk).await?;
    if team.is_empty() {
        return Err(Error2::NotFound("Team not found".into()));
    }
    let team = TeamDetailResponse::from(team);
    Ok(Json(team))
}

#[tokio::test]
async fn test_get_team_handler() {
    use super::create_team::{CreateTeamRequest, create_team_handler};
    use crate::tests::{create_app_state, create_auth, get_test_user};

    let app_state = create_app_state();
    let user = get_test_user(app_state.clone()).await;
    let auth = create_auth(user.clone()).await;
    let now = chrono::Utc::now().timestamp();
    let team_display_name = format!("test_team_{}", now);
    let team_username = format!("test_username_{}", now);

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
    let team_pk = team.team_pk;

    // Get Team
    let get_res = get_team_handler(
        State(app_state),
        Extension(Some(auth)),
        Path(GetTeamPathParams { team_pk }),
    )
    .await;
    assert!(get_res.is_ok(), "Failed to get team {:?}", get_res.err());
    let team_detail = get_res.unwrap().0;
    let team = &team_detail.team;
    let owner = team_detail.owner.as_ref().expect("Owner should exist");

    assert_eq!(
        team.nickname, team_display_name,
        "Failed to match team nickname"
    );
    assert_eq!(
        team.username, team_username,
        "Failed to match team username"
    );
    assert_eq!(
        owner.user_pk.to_string(),
        user.pk.to_string(),
        "Failed to match `owner pk`"
    );
}
