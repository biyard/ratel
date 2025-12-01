use crate::features::spaces::rewards::RewardType;
use crate::features::spaces::rewards::SpaceReward;
use crate::features::spaces::rewards::SpaceRewardResponse;
use crate::spaces::SpacePath;
use crate::spaces::SpacePathParam;
use crate::utils::time::get_now_timestamp_millis;
use crate::*;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema, Default)]
pub struct UpdateRewardRequest {
    pub reward_type: RewardType,
    pub label: String,
    pub description: String,
    pub amount: i64,
}

pub async fn update_reward_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<UpdateRewardRequest>,
) -> Result<Json<SpaceRewardResponse>> {
    permissions.permitted(TeamGroupPermission::SpaceEdit)?;

    let (pk, sk) = SpaceReward::keys(&space_pk, &req.reward_type);
    let now = get_now_timestamp_millis();
    let updater = SpaceReward::updater(&pk, &sk)
        .with_amount(req.amount)
        .with_label(req.label)
        .with_description(req.description)
        .with_updated_at(now);

    let reward = updater.execute(&dynamo.client).await.map_err(|e| {
        tracing::debug!("Failed to update reward: {:?}", e);
        Error::InternalServerError(e.to_string())
    })?;
    Ok(Json(reward.into()))
}
