use crate::features::membership::UserMembership;
use crate::features::spaces::rewards::RewardUserBehavior;
use crate::features::spaces::rewards::SpaceReward;
use crate::spaces::SpacePath;
use crate::spaces::SpacePathParam;
use crate::*;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct DeleteRewardRequest {
    entity_type: EntityType,
    behavior: RewardUserBehavior,
}

pub async fn delete_reward_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    NoApi(user): NoApi<User>,

    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<DeleteRewardRequest>,
) -> Result<Json<()>> {
    permissions.permitted(TeamGroupPermission::SpaceEdit)?;

    let space_reward = SpaceReward::get_by_action(
        &dynamo.client,
        space_pk.into(),
        req.entity_type,
        req.behavior,
    )
    .await?;

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
