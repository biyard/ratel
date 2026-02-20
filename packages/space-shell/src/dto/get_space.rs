use crate::models::File;
use common::types::{EntityType, Partition};
use ratel_auth::models::user::UserType;
use ratel_post::types::{BoosterType, SpacePublishState, SpaceStatus, SpaceType, SpaceVisibility};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetSpaceResponse {
    pub pk: Partition,
    pub sk: EntityType,
    pub title: String,
    pub content: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub urls: Vec<String>,
    pub space_type: SpaceType,
    pub features: Vec<String>,
    pub status: Option<SpaceStatus>,
    pub permissions: i64,
    pub author_type: UserType,
    pub author_display_name: String,
    pub author_username: String,
    pub author_profile_url: String,
    pub certified: bool,
    pub likes: i64,
    pub comments: i64,
    pub shares: i64,
    pub reports: i64,
    pub rewards: Option<i64>,
    pub visibility: SpaceVisibility,
    pub publish_state: SpacePublishState,
    pub booster: BoosterType,
    pub files: Option<Vec<File>>,
    pub anonymous_participation: bool,
    pub can_participate: bool,
    pub participated: bool,
    pub participant_display_name: Option<String>,
    pub participant_profile_url: Option<String>,
    pub participant_username: Option<String>,
    pub remains: i64,
    pub quota: i64,
    pub is_report: bool,
}
