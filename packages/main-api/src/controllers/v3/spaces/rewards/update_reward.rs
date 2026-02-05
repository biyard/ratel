use crate::features::membership::UserMembership;
use crate::features::spaces::rewards::RewardUserBehavior;
use crate::features::spaces::rewards::SpaceReward;
use crate::features::spaces::rewards::SpaceRewardResponse;
use crate::spaces::SpacePath;
use crate::spaces::SpacePathParam;
use crate::utils::time::get_now_timestamp_millis;
use crate::*;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct UpdateSpaceRewardRequest {
    pub action_key: EntityType,
    pub behavior: RewardUserBehavior,

    #[serde(default)]
    pub description: String,
    pub credits: i64,
}

pub async fn update_space_reward_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<UpdateSpaceRewardRequest>,
) -> Result<Json<SpaceRewardResponse>> {
    permissions.permitted(TeamGroupPermission::SpaceEdit)?;

    let mut space_reward = SpaceReward::get_by_action(
        &dynamo.client,
        space_pk.into(),
        req.action_key,
        req.behavior,
    )
    .await?;

    let credit_delta = space_reward.credits - req.credits;

    let mut user_membership = user.get_user_membership(&dynamo.client).await?;

    user_membership.use_credits(credit_delta)?;
    let mut updater_txs = vec![];

    updater_txs.push(
        UserMembership::updater(user_membership.pk, user_membership.sk)
            .increase_remaining_credits(credit_delta)
            .transact_write_item(),
    );

    let updater = SpaceReward::updater(&space_reward.pk, &space_reward.sk)
        .with_description(req.description.clone())
        .with_credits(req.credits)
        .with_updated_at(now())
        .transact_write_item();

    updater_txs.push(updater);

    transact_write_items!(&dynamo.client, updater_txs)?;
    space_reward.updated_at = now();
    space_reward.description = req.description;
    space_reward.credits = req.credits;
    Ok(Json(space_reward.into()))
}
