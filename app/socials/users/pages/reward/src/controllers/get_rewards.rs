use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct RewardsResponse {
    pub points: i64,
    pub display_name: String,
    pub username: String,
}

#[get("/api/me/rewards", user: ratel_auth::User)]
pub async fn get_rewards_handler() -> Result<RewardsResponse> {
    Ok(RewardsResponse {
        points: user.points,
        display_name: user.display_name.clone(),
        username: user.username.clone(),
    })
}
