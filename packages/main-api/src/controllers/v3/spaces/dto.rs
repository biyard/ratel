use bdk::prelude::{axum::extract::Path, *};
use serde::Deserialize;

use crate::{
    models::space::SpaceCommon,
    types::{BoosterType, EntityType, Partition, SpacePublishState, SpaceStatus, SpaceVisibility},
};

#[derive(Debug, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct TimeRange(pub i64, pub i64); // (started_at, ended_at)

impl TimeRange {
    pub fn is_valid(&self) -> bool {
        self.0 < self.1
    }
}

pub type SpacePath = Path<SpacePathParam>;

#[derive(Debug, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema)]
pub struct SpacePathParam {
    pub space_pk: Partition,
}

pub type SpaceDiscussionPath = Path<SpaceDiscussionPathParam>;

#[derive(Debug, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema)]
pub struct SpaceDiscussionPathParam {
    pub space_pk: Partition,
    pub discussion_pk: Partition,
}

pub type SpacePanelPath = Path<SpacePanelPathParam>;

#[derive(Debug, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema)]
pub struct SpacePanelPathParam {
    pub space_pk: Partition,
    pub panel_pk: Partition,
}

pub type SpacePostCommentPath = Path<SpacePostCommentPathParam>;

#[derive(Debug, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema)]
pub struct SpacePostCommentPathParam {
    pub space_pk: Partition,
    pub space_post_pk: Partition,
    pub space_post_comment_sk: EntityType,
}

pub type SpacePostPath = Path<SpacePostPathParam>;

#[derive(Debug, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema)]
pub struct SpacePostPathParam {
    pub space_pk: Partition,
    pub space_post_pk: Partition,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema, Default)]
pub struct SpaceCommonResponse {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    pub status: Option<SpaceStatus>,
    pub publish_state: SpacePublishState,
    pub visibility: SpaceVisibility,
    pub post_pk: Partition,
    pub content: String,

    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,

    pub started_at: Option<i64>,
    pub ended_at: Option<i64>,

    pub booster: BoosterType,
    pub custom_booster: Option<i64>,
    pub rewards: Option<i64>,

    pub quota: i64,
    pub remains: i64,
}

impl From<SpaceCommon> for SpaceCommonResponse {
    fn from(value: SpaceCommon) -> Self {
        Self {
            pk: value.pk,
            sk: value.sk,
            created_at: value.created_at,
            updated_at: value.updated_at,
            status: value.status,
            publish_state: value.publish_state,
            visibility: value.visibility,
            post_pk: value.post_pk,
            user_pk: value.user_pk,
            author_display_name: value.author_display_name,
            author_profile_url: value.author_profile_url,
            author_username: value.author_username,
            started_at: value.started_at,
            ended_at: value.ended_at,
            booster: value.booster,
            custom_booster: value.custom_booster,
            rewards: value.rewards,
            content: value.content,
            quota: value.quota,
            remains: value.remains,
        }
    }
}
