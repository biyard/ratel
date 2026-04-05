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
