use crate::error::Error;
use crate::features::membership::UserMembership;
use crate::features::spaces::rewards::RewardKey;
use crate::features::spaces::rewards::RewardUserBehavior;
use crate::features::spaces::rewards::SpaceReward;
use crate::spaces::SpacePath;
use crate::spaces::SpacePathParam;
use crate::*;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct DeleteSpaceRewardRequest {
    pub sk: RewardKey,
}

pub async fn delete_space_reward_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    NoApi(user): NoApi<User>,

    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<DeleteSpaceRewardRequest>,
) -> Result<Json<()>> {
    permissions.permitted(TeamGroupPermission::SpaceEdit)?;
    tracing::info!(
        "delete_space_reward_handler: req: {} {}",
        req.sk.to_string(),
        space_pk.to_string()
    );
    let space_reward = SpaceReward::get(&dynamo.client, space_pk, Some(req.sk))
        .await?
        .ok_or(Error::SpaceRewardNotFound)?;

    // Refund Credit
    let mut user_membership = user.get_user_membership(&dynamo.client).await?;

    user_membership.use_credits(space_reward.credits * -1)?;
    let mut txs = vec![];
    txs.push(
        UserMembership::updater(user_membership.pk, user_membership.sk)
            .increase_remaining_credits(space_reward.credits)
            .transact_write_item(),
    );

    // Delete Reward
    txs.push(SpaceReward::delete_transact_write_item(
        space_reward.pk,
        space_reward.sk,
    ));

    transact_write_items!(&dynamo.client, txs)?;

    Ok(Json(()))
}
