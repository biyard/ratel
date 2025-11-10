use ssi::claims::ResourceProvider;

use crate::*;
use crate::{
    Error,
    features::spaces::members::SpaceEmailVerification,
    models::{User, UserTeamGroup, UserTeamGroupQueryOption, team::Team, *},
    types::*,
    utils::time::get_now_timestamp_millis,
};

use super::SpaceParticipant;

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    DynamoEntity,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
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

    pub status: Option<SpaceStatus>, // Waiting, InProgress, Started, Finished

    #[dynamo(
        prefix = "SPACE_COMMON_VIS",
        name = "find_by_visibility",
        index = "gsi6",
        order = 2,
        pk
    )]
    pub visibility: SpaceVisibility, // Private, Public, Team(team_pk)
    #[dynamo(index = "gsi6", order = 1, pk)]
    pub publish_state: SpacePublishState, // Draft, Published
    #[dynamo(prefix = "POST_PK", name = "find_by_post_pk", index = "gsi2", pk)]
    pub post_pk: Partition,
    pub space_type: SpaceType,
    #[serde(default)]
    pub content: String,

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

    #[serde(default)]
    pub anonymous_participation: bool,
    #[serde(default)]
    // participants is the number of participants. It is incremented when a user participates in the space.
    // It is only used for spaces enabling explicit participation such as anonymous participation.
    pub participants: i64,

    // space pdf files
    pub files: Vec<File>,
}

impl SpaceCommon {
    pub fn new(
        Post {
            pk: post_pk,
            user_pk,
            author_display_name,
            author_profile_url,
            author_username,
            ..
        }: Post,
    ) -> Self {
        let now = get_now_timestamp_millis();
        Self {
            pk: post_pk
                .clone()
                .to_space_pk()
                .expect("post_pk must be Partition::Feed"),
            sk: EntityType::SpaceCommon,
            created_at: now,
            updated_at: now,
            post_pk,
            publish_state: SpacePublishState::Draft,
            status: None,
            visibility: SpaceVisibility::Private,
            user_pk,
            author_display_name,
            author_profile_url,
            author_username,
            ..Default::default()
        }
    }

    pub fn should_explicit_participation(&self) -> bool {
        self.anonymous_participation
    }

    pub fn is_published(&self) -> bool {
        self.publish_state == SpacePublishState::Published
    }

    pub async fn is_space_admin(&self, cli: &aws_sdk_dynamodb::Client, user: &User) -> bool {
        if matches!(&self.user_pk, Partition::User(_)) {
            &self.user_pk == &user.pk
        } else if matches!(&self.user_pk, Partition::Team(_)) {
            Team::has_permission(cli, &self.user_pk, &user.pk, TeamGroupPermission::TeamAdmin)
                .await
                .unwrap_or(false)
        } else {
            false
        }
    }
}

impl SpaceCommon {
    pub fn permissions_for_guest(&self) -> TeamGroupPermissions {
        if self.visibility == SpaceVisibility::Public
            && self.publish_state == SpacePublishState::Published
        {
            return TeamGroupPermissions::read();
        }

        TeamGroupPermissions::empty()
    }

    pub fn validate_editable(&self) -> bool {
        self.publish_state == SpacePublishState::Draft
            || (self.publish_state == SpacePublishState::Published
                && (self.status == Some(SpaceStatus::Waiting) || self.status.is_none()))
    }
}

#[async_trait::async_trait]
impl ResourcePermissions for SpaceCommon {
    fn viewer_permissions(&self) -> Permissions {
        if self.visibility == SpaceVisibility::Public
            && self.publish_state == SpacePublishState::Published
        {
            return Permissions::read();
        }

        Permissions::empty()
    }

    fn participant_permissions(&self) -> Permissions {
        Permissions::read()
    }

    fn resource_owner(&self) -> ResourceOwnership {
        self.user_pk.clone().into()
    }

    async fn is_participant(&self, cli: &aws_sdk_dynamodb::Client, requester: &Partition) -> bool {
        let (pk, sk) = SpaceParticipant::keys(self.pk.clone(), requester.clone());

        SpaceParticipant::get(cli, &pk, Some(&sk))
            .await
            .map(|sp| sp.is_some())
            .unwrap_or(false)
    }
}
