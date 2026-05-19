use crate::common::{types::*, utils::time::get_now_timestamp_millis, *};

#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity)]
pub struct UserRewardHistory {
    #[dynamo(
        prefix = "URH_BY_TARGET",
        name = "find_reward_by_user",
        index = "gsi1",
        pk
    )]
    pub pk: CompositePartition,

    pub sk: UserRewardHistoryKey,

    pub point: i64,

    #[dynamo(name = "find_reward_by_user", index = "gsi1", sk)]
    pub created_at: i64,

    pub transaction_id: Option<String>,
    pub month: Option<String>,

    #[serde(default)]
    pub description: Option<String>,
}

#[cfg(feature = "server")]
impl UserRewardHistory {
    pub fn from_params(
        target_pk: Partition,
        reward_key: RewardKey,
        period: &RewardPeriod,
        amount: i64,
    ) -> Self {
        let now = get_now_timestamp_millis();
        let time_key = period.to_time_key(now);
        let pk = CompositePartition(target_pk, Partition::Reward);
        let sk = UserRewardHistoryKey(reward_key, time_key);

        Self {
            pk,
            sk,
            point: amount,
            created_at: now,
            ..Default::default()
        }
    }

    pub fn set_transaction(&mut self, transaction_id: String, month: String) -> &mut Self {
        self.transaction_id = Some(transaction_id);
        self.month = Some(month);
        self
    }
}

/// Resolves `"{space_pk}#{space_title}"` for a UserRewardHistory row.
///
/// The space title lives on the `Post` (every Space is backed by a Post
/// referenced via `SpaceCommon.post_pk`), so this is a two-step lookup.
/// Failures degrade gracefully: a missing space_pk segment in the reward
/// key, a deleted SpaceCommon, or a deleted Post all fall back to
/// `"{space_pk}"` alone so the row still identifies its source space.
/// Used by `SpaceReward::award` at write time and by the GSI/description
/// backfill migration.
#[cfg(feature = "server")]
pub async fn resolve_reward_description(
    cli: &aws_sdk_dynamodb::Client,
    reward_key: &RewardKey,
) -> String {
    use crate::common::models::space::SpaceCommon;
    use crate::features::posts::models::Post;

    let Some(space_partition) = reward_key.space_pk.clone() else {
        return String::new();
    };

    let space_pk_str = space_partition.to_string();
    let space_pk: Partition = space_partition.into();

    let Ok(Some(common)) = SpaceCommon::get(cli, &space_pk, Some(&EntityType::SpaceCommon)).await
    else {
        return space_pk_str;
    };

    let Ok(Some(post)) = Post::get(cli, &common.post_pk, Some(&EntityType::Post)).await else {
        return space_pk_str;
    };

    if post.title.is_empty() {
        space_pk_str
    } else {
        format!("{}#{}", space_pk_str, post.title)
    }
}
