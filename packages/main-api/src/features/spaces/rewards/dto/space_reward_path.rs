use bdk::prelude::{axum::extract::Path, *};

use crate::{
    features::spaces::rewards::RewardType,
    types::{EntityType, Partition},
};

pub(crate) type SpaceRewardPath = Path<SpaceRewardPathParam>;

#[derive(Debug, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema)]
pub struct SpaceRewardPathParam {
    pub space_pk: Partition,
    pub reward_sk: RewardType,
}
