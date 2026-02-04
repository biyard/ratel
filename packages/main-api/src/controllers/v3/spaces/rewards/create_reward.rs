use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::membership::{UserMembership, user_membership};
use crate::features::spaces::rewards::{
    Reward, RewardAction, RewardKey, RewardTypeRequest, SpaceReward, SpaceRewardResponse,
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
pub struct CreateRewardSpaceRequest {
    reward: RewardTypeRequest,
    #[serde(default)]
    description: String,
    credits: i64,
}

pub async fn create_reward_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<CreateRewardSpaceRequest>,
) -> Result<Json<SpaceRewardResponse>, Error> {
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
    let reward_action: RewardAction = req.reward.clone().into();
    let reward_key = RewardKey::from(req.reward);
    let reward = Reward::get_by_reward_action(&dynamo.client, &reward_action).await?;

    let space_reward = SpaceReward::new(
        space_pk.into(),
        reward_key,
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
