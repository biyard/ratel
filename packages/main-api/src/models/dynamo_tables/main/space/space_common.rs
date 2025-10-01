use crate::{types::*, utils::time::get_now_timestamp_millis};
use bdk::prelude::*;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default, schemars::JsonSchema,
)]

/*
PUBLISH_STATE: 유저의 게시물 상태
    Draft: 작성중
    Published: 게시됨

STATUS: Space 의 진행 상태(Only time limited space use this field based on started_at and ended_at)
    None: for Draft or Time Unlimited space
    Waiting: for Published but not started yet
    InProgress: User Can respond or doing some actions for space
    Finished: User

VISIBILITY: 유저가 글을 볼 수 있는 범위
    Private: only author can read
    Public: anyone can read
    Team(team_pk): only team members can read

---
PERMISSION RULES:

READ: Based on VISIBILITY
    Private: only author can read
    Public: anyone can read
    Team(team_pk): only team members can read

EDIT(UPDATE): Based on PUBLISH_STATE and STATUS
    Only Draft publish_state or Waiting status can be edited
    Once Published, cannot revert to Draft
    Once InProgress, cannot revert to Waiting
    Once Finished, cannot revert to InProgress

ACTION(e.g., Respond to Poll): Based on STATUS
    Only InProgress status can perform actions
    Cannot perform actions in Waiting or Finished status

*/
pub struct SpaceCommon {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(index = "gsi2", sk)]
    #[dynamo(index = "gsi1", sk)]
    #[dynamo(index = "gsi6", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    pub status: Option<SpaceStatus>, // Waiting, InProgress, Finished
    pub publish_state: SpacePublishState, // Draft, Published
    #[dynamo(prefix = "VIS", name = "find_by_visibility", index = "gsi6", pk)]
    pub visibility: SpaceVisibility, // Private, Public, Team(team_pk)
    #[dynamo(prefix = "POST_PK", name = "find_by_post_pk", index = "gsi2", pk)]
    pub post_pk: Partition,

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,

    pub started_at: Option<i64>,
    pub ended_at: Option<i64>,

    pub booster: BoosterType,
    pub custom_booster: Option<i64>,
    pub rewards: Option<i64>,
}

impl SpaceCommon {
    pub fn new<A: Into<Author>>(pk: Partition, post_pk: Partition, author: A) -> Self {
        let now = get_now_timestamp_millis();
        let Author {
            pk: user_pk,
            display_name,
            profile_url,
            username,
        } = author.into();
        Self {
            pk,
            sk: EntityType::SpaceCommon,
            created_at: now,
            updated_at: now,
            post_pk,
            publish_state: SpacePublishState::Draft,
            status: None,
            visibility: SpaceVisibility::Private,
            user_pk,
            author_display_name: display_name,
            author_profile_url: profile_url,
            author_username: username,
            ..Default::default()
        }
    }

    pub fn with_time(mut self, started_at: i64, ended_at: i64) -> Self {
        self.started_at = Some(started_at);
        self.ended_at = Some(ended_at);
        self
    }
    pub fn with_booster(mut self, booster: BoosterType) -> Self {
        self.booster = booster;
        self
    }
    // pub fn with_visibility(mut self, visibility: SpaceVisibility) -> Self {
    //     self.visibility = Some(visibility);
    //     self
    // }
}
