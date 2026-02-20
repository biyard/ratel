use crate::{types::*, utils::time::get_now_timestamp_millis, *};

#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity)]
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
    // pub space_type: SpaceType,
    #[serde(default)]
    pub content: String,

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,

    pub started_at: Option<i64>,
    pub ended_at: Option<i64>,

    // pub booster: BoosterType,
    pub custom_booster: Option<i64>,
    pub rewards: Option<i64>,

    #[serde(default)]
    pub reports: i64,

    #[serde(default)]
    pub anonymous_participation: bool,

    #[serde(default)]
    pub participants: i64,

    #[serde(default = "max_quota")]
    pub quota: i64,
    #[serde(default = "max_quota")]
    pub remains: i64,
}

fn max_quota() -> i64 {
    1_000_000
}

impl SpaceCommon {
    pub fn new(
        post_id: FeedPartition,
        user_id: UserPartition,
        author_display_name: String,
        author_profile_url: String,
        author_username: String,
    ) -> Self {
        let now = get_now_timestamp_millis();
        let post_pk: Partition = post_id.into();
        let user_pk: Partition = user_id.into();

        Self {
            pk: post_pk
                .clone()
                .to_space_pk()
                .expect("Failed to convert post_pk to space_pk"),
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
}
