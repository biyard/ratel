use crate::common::macros::DynamoEntity;
use crate::features::activity::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
pub struct SpaceScore {
    pub pk: CompositePartition<SpacePartition, AuthorPartition>,
    pub sk: EntityType,

    #[dynamo(prefix = "SCSP", name = "find_by_space", index = "gsi1", pk)]
    pub space_pk: Partition,
    #[dynamo(prefix = "SCR", index = "gsi1", sk)]
    pub total_score: i64,

    pub user_pk: AuthorPartition,
    pub user_name: String,
    pub user_avatar: String,

    pub poll_score: i64,
    pub quiz_score: i64,
    pub follow_score: i64,
    pub discussion_score: i64,

    pub updated_at: i64,
}

#[cfg(feature = "server")]
impl SpaceScore {
    pub fn new(
        space_id: SpacePartition,
        author: AuthorPartition,
        user_name: String,
        user_avatar: String,
    ) -> Self {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        let space_pk: Partition = space_id.clone().into();

        Self {
            pk: CompositePartition(space_id, author.clone()),
            sk: EntityType::SpaceScore,
            space_pk,
            total_score: 0,
            user_pk: author,
            user_name,
            user_avatar,
            poll_score: 0,
            quiz_score: 0,
            follow_score: 0,
            discussion_score: 0,
            updated_at: now,
        }
    }

    pub fn keys(
        space_id: &SpacePartition,
        author: &AuthorPartition,
    ) -> (CompositePartition<SpacePartition, AuthorPartition>, EntityType) {
        (
            CompositePartition(space_id.clone(), author.clone()),
            EntityType::SpaceScore,
        )
    }
}
