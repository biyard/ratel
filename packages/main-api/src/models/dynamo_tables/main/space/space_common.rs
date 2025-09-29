use crate::{models::user::User, types::*, utils::time::get_now_timestamp_millis};
use bdk::prelude::*;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default, schemars::JsonSchema,
)]
pub struct SpaceCommon {
    pub pk: Partition,

    #[dynamo(index = "gsi2", sk)]
    #[dynamo(index = "gsi1", sk)]
    #[dynamo(index = "gsi6", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    pub sk: EntityType,

    // // Space statistics
    // pub participants: i64,
    #[dynamo(prefix = "VIS", name = "find_by_visibility", index = "gsi6", pk)]
    pub visibility: SpaceVisibility,
    #[dynamo(prefix = "POST_PK", name = "find_by_post_pk", index = "gsi2", pk)]
    pub post_pk: Partition,

    // Author info
    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,

    pub started_at: Option<i64>,
    pub ended_at: Option<i64>,
}

impl SpaceCommon {
    pub fn new(
        pk: Partition,
        post_pk: Partition,
        User {
            pk: user_pk,
            display_name,
            profile_url,
            username,
            ..
        }: User,
    ) -> Self {
        let now = get_now_timestamp_millis();

        Self {
            pk,
            sk: EntityType::SpaceCommon,
            post_pk,
            created_at: now,
            updated_at: now,
            visibility: SpaceVisibility::Public,
            user_pk,
            author_display_name: display_name,
            author_profile_url: profile_url,
            author_username: username,
            ..Default::default()
        }
    }

    pub fn with_visibility(mut self, visibility: SpaceVisibility) -> Self {
        self.visibility = visibility;
        self
    }
}
