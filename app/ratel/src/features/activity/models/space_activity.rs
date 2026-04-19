use crate::common::macros::DynamoEntity;
use crate::features::activity::*;
use crate::features::spaces::pages::actions::types::SpaceActionType;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
pub struct SpaceActivity {
    pub pk: CompositePartition<SpacePartition, AuthorPartition>,
    pub sk: EntityType,

    #[dynamo(prefix = "SACT", name = "find_by_space", index = "gsi1", pk)]
    pub space_pk: Partition,
    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub created_at: i64,

    pub user_pk: AuthorPartition,
    pub user_name: String,
    pub user_avatar: String,
    pub action_id: String,
    pub action_type: SpaceActionType,
    pub data: SpaceActivityData,

    pub base_score: i64,
    pub additional_score: i64,
    pub total_score: i64,
}

#[cfg(feature = "server")]
impl SpaceActivity {
    pub fn new(
        space_id: SpacePartition,
        author: AuthorPartition,
        action_id: String,
        action_type: SpaceActionType,
        data: SpaceActivityData,
        base_score: i64,
        additional_score: i64,
        user_name: String,
        user_avatar: String,
    ) -> Self {
        Self::new_with_dedup(
            space_id,
            author,
            action_id.clone(),
            action_type,
            data,
            base_score,
            additional_score,
            user_name,
            user_avatar,
            action_id,
        )
    }

    pub fn new_with_dedup(
        space_id: SpacePartition,
        author: AuthorPartition,
        action_id: String,
        action_type: SpaceActionType,
        data: SpaceActivityData,
        base_score: i64,
        additional_score: i64,
        user_name: String,
        user_avatar: String,
        dedup_key: String,
    ) -> Self {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        let space_pk: Partition = space_id.clone().into();
        let total_score = base_score + additional_score;
        let sk = EntityType::SpaceActivity(format!("{}#{}", dedup_key, now));

        Self {
            pk: CompositePartition(space_id, author.clone()),
            sk,
            space_pk,
            created_at: now,
            user_pk: author,
            user_name,
            user_avatar,
            action_id,
            action_type,
            data,
            base_score,
            additional_score,
            total_score,
        }
    }
}
