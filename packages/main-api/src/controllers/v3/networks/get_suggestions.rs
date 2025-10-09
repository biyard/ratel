use crate::AppState;
use crate::models::team::{Team, TeamQueryOption};
use crate::models::user::{User, UserQueryOption};
use crate::types::EntityType;
use axum::extract::*;
use bdk::prelude::*;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct GetSuggestionsResponse {
    pub suggested_teams: Vec<Follower>,
    pub suggested_users: Vec<Follower>,
}

pub async fn get_suggestions_handler(
    State(AppState { dynamo, .. }): State<AppState>,
) -> Result<Json<GetSuggestionsResponse>, crate::Error2> {
    let (users, _) = User::find_by_follwers(
        &dynamo.client,
        EntityType::User.to_string(),
        UserQueryOption::builder().limit(3),
    )
    .await
    .unwrap_or((vec![], None));

    let (teams, _) = Team::find_by_follwers(
        &dynamo.client,
        EntityType::User.to_string(),
        TeamQueryOption::builder().limit(3),
    )
    .await
    .unwrap_or((vec![], None));

    Ok(Json(GetSuggestionsResponse {
        suggested_teams: teams.into_iter().map(Follower::from).collect(),
        suggested_users: users.into_iter().map(Follower::from).collect(),
    }))
}

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct Follower {
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
    pub followers: i64,
    pub followings: i64,
}

impl From<Team> for Follower {
    fn from(team: Team) -> Self {
        Self {
            display_name: team.display_name,
            profile_url: team.profile_url,
            username: team.username,
            followers: team.followers,
            followings: team.followings,
        }
    }
}

impl From<User> for Follower {
    fn from(user: User) -> Self {
        Self {
            display_name: user.display_name,
            profile_url: user.profile_url,
            username: user.username,
            followers: user.followers_count,
            followings: user.followings_count,
        }
    }
}
