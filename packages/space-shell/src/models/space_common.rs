use crate::models::File;
use crate::*;
use ratel_post::types::{BoosterType, SpacePublishState, SpaceStatus, SpaceType, SpaceVisibility};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(DynamoEntity))]
pub struct SpaceCommon {
    pub pk: Partition,
    pub sk: EntityType,
    #[cfg_attr(feature = "server", dynamo(index = "gsi2", sk))]
    #[cfg_attr(feature = "server", dynamo(index = "gsi1", sk))]
    #[cfg_attr(feature = "server", dynamo(index = "gsi6", sk))]
    pub created_at: i64,
    pub updated_at: i64,
    pub status: Option<SpaceStatus>,
    #[cfg_attr(
        feature = "server",
        dynamo(
            prefix = "SPACE_COMMON_VIS",
            name = "find_by_visibility",
            index = "gsi6",
            order = 2,
            pk
        )
    )]
    pub visibility: SpaceVisibility,
    #[cfg_attr(feature = "server", dynamo(index = "gsi6", order = 1, pk))]
    pub publish_state: SpacePublishState,
    #[cfg_attr(
        feature = "server",
        dynamo(prefix = "POST_PK", name = "find_by_post_pk", index = "gsi2", pk)
    )]
    pub post_pk: Partition,
    pub space_type: SpaceType,
    #[serde(default)]
    pub content: String,
    #[cfg_attr(
        feature = "server",
        dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)
    )]
    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,
    pub started_at: Option<i64>,
    pub ended_at: Option<i64>,
    pub booster: BoosterType,
    pub custom_booster: Option<i64>,
    pub rewards: Option<i64>,
    #[serde(default)]
    pub reports: i64,
    #[serde(default)]
    pub anonymous_participation: bool,
    #[deprecated(note = "Use Visibility variant instead")]
    #[serde(default)]
    pub change_visibility: bool,
    #[serde(default)]
    pub participants: i64,
    pub files: Option<Vec<File>>,
    #[serde(default)]
    pub block_participate: bool,
    #[serde(default = "max_quota")]
    pub quota: i64,
    #[serde(default = "max_quota")]
    pub remains: i64,
}

fn max_quota() -> i64 {
    1_000_000
}
