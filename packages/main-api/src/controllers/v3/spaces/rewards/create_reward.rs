use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::membership::{UserMembership, user_membership};
use crate::features::spaces::rewards::{RewardType, SpaceReward, SpaceRewardResponse};
use crate::models::space::SpaceCommon;
use crate::types::{EntityType, SpacePublishState};
use crate::{
    AppState, Error, Permissions,
    models::user::User,
    types::{Partition, TeamGroupPermission},
};
use crate::{transact_write_all_items, transact_write_items};

use axum::{
    Json,
    extract::{Extension, Path, State},
};
use bdk::prelude::*;

use aide::NoApi;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema, Default)]
pub struct CreateRewardSpaceRequest {
    reward_type: RewardType,
    label: String,
    description: String,
    credit: i64,
}

pub async fn create_reward_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<CreateRewardSpaceRequest>,
) -> Result<Json<SpaceRewardResponse>, Error> {
    permissions.permitted(TeamGroupPermission::SpaceWrite)?;
    let mut updater_txs = vec![];

    let (mut user_membership, _membership) = user.get_membership(&dynamo.client).await?;

    user_membership.use_credits(req.credit)?;

    updater_txs.push(user_membership.upsert_transact_write_item());

    let amount = req.reward_type.point() * req.credit;
    let space_reward = SpaceReward::new(
        space_pk.clone(),
        req.reward_type,
        req.label,
        req.description,
        amount,
    );

    updater_txs.push(space_reward.create_transact_write_item());

    transact_write_items!(&dynamo.client, updater_txs)?;
    Ok(Json(space_reward.into()))
}
