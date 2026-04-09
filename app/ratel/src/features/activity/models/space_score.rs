use crate::common::macros::DynamoEntity;
use crate::features::activity::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
pub struct SpaceScore {
    pub pk: CompositePartition<SpacePartition, AuthorPartition>,
    pub sk: EntityType,

    // FIXME: remove gsi1 field when space score table is migrated, currently it is used for querying scores by space, but it is redundant since we can query by space_pk
    #[dynamo(prefix = "SCSP", name = "find_by_space", index = "gsi1", pk)]
    #[dynamo(prefix = "SCSP", name = "find_by_space_rank", index = "gsi2", pk)]
    pub space_pk: Partition,
    #[dynamo(prefix = "SCR", index = "gsi1", sk)]
    pub total_score: i64,
    #[serde(default)]
    #[dynamo(prefix = "SCR", index = "gsi2", order = 1, sk)]
    pub rank_total_score: i64,

    #[dynamo(index = "gsi2", order = 3, sk)]
    pub user_pk: AuthorPartition,
    pub user_name: String,
    pub user_avatar: String,

    pub poll_score: i64,
    pub quiz_score: i64,
    pub follow_score: i64,
    pub discussion_score: i64,

    #[dynamo(index = "gsi2", order = 2, sk)]
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
            rank_total_score: 0,
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
    ) -> (
        CompositePartition<SpacePartition, AuthorPartition>,
        EntityType,
    ) {
        (
            CompositePartition(space_id.clone(), author.clone()),
            EntityType::SpaceScore,
        )
    }
}
