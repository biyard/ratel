use crate::features::spaces::rewards::RewardType;
use crate::features::spaces::rewards::SpaceReward;
use crate::spaces::SpacePath;
use crate::spaces::SpacePathParam;
use crate::*;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema, Default)]
pub struct DeleteRewardRequest {
    pub reward_sk: RewardType,
}

pub async fn delete_reward_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<DeleteRewardRequest>,
) -> Result<Json<()>> {
    permissions.permitted(TeamGroupPermission::SpaceDelete)?;

    let (pk, sk) = SpaceReward::keys(&space_pk, &req.reward_sk);
    SpaceReward::delete(&dynamo.client, &pk, Some(&sk))
        .await
        .map_err(|e| {
            tracing::debug!("Failed to delete reward: {:?}", e);
            Error::InternalServerError(e.to_string())
        })?;

    Ok(Json(()))
}
