use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::membership::{UserMembership, user_membership};
use crate::features::spaces::rewards::{
    Reward, RewardAction, RewardUserBehavior, SpaceReward, SpaceRewardResponse,
};
use crate::models::space::SpaceCommon;
use crate::types::{EntityType, SpacePublishState};
use crate::{
    AppState, Error, Permissions,
    models::user::User,
    types::{Partition, TeamGroupPermission},
};
use crate::{config, transact_write_all_items, transact_write_items};

use axum::{
    Json,
    extract::{Extension, Path, State},
};
use bdk::prelude::*;

use aide::NoApi;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct CreateSpaceRewardRequest {
    action_key: EntityType,
    behavior: RewardUserBehavior,
    #[serde(default)]
    description: String,
    credits: i64,
}

pub async fn create_space_reward_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<CreateSpaceRewardRequest>,
) -> Result<Json<SpaceRewardResponse>, Error> {
    let action = RewardAction::try_from(&req.action_key)?;
    if action != req.behavior.action() {
        return Err(Error::BehaviorNotMatchAction);
    }
    permissions.permitted(TeamGroupPermission::SpaceEdit)?;
    let mut updater_txs = vec![];

    let mut user_membership = user.get_user_membership(&dynamo.client).await?;

    /* FIXME:
       There is a concurrency problem.
       When Use Credit API is called, the remaining credits should be updated.
       But the remaining credits is updated after the reward is created.
       If two APIs are requested simultaneously,
       there's a risk that Credit will become negative.

       We need to add `conditional function` or `condition` to DynamoEntity.
    */
    user_membership.use_credits(req.credits)?;
    updater_txs.push(
        UserMembership::updater(user_membership.pk, user_membership.sk)
            .decrease_remaining_credits(req.credits)
            .transact_write_item(),
    );
    let reward = Reward::get(&dynamo.client, Partition::Reward, Some(&req.behavior))
        .await?
        .ok_or(Error::RewardNotFound)?;

    let space_reward = SpaceReward::new(
        space_pk.into(),
        req.action_key,
        req.behavior,
        req.description,
        req.credits,
        reward.point,
        reward.period,
        reward.condition,
    );

    updater_txs.push(space_reward.create_transact_write_item());

    transact_write_items!(&dynamo.client, updater_txs)?;
    Ok(Json(space_reward.into()))
}
