use crate::features::spaces::rewards::RewardType;
use crate::types::*;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default, JsonSchema, OperationIo)]
pub struct SpaceReward {
    pub pk: CompositePartition,
    pub sk: RewardType,

    pub created_at: i64,
    pub updated_at: i64,
    pub label: String,
    pub description: String,
    pub amount: i64,
}

impl SpaceReward {
    pub fn new(
        space_pk: Partition,
        reward_type: RewardType,
        label: String,
        description: String,
        amount: i64,
    ) -> Self {
        let (pk, sk) = Self::keys(&space_pk, &reward_type);
        let now = time::get_now_timestamp_millis();

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,

            label,
            amount,
            description,
        }
    }

    pub fn keys(
        space_pk: &Partition,
        reward_type: &RewardType,
    ) -> (CompositePartition, RewardType) {
        if !matches!(space_pk, Partition::Space(_)) {
            panic!("SpaceReward pk must be of Partition::Space type");
        }
        (
            CompositePartition(space_pk.clone(), Partition::Reward),
            reward_type.clone(),
        )
    }
    pub fn space_pk(&self) -> Partition {
        match &self.pk.0 {
            Partition::Space(v) => Partition::Space(v.clone()),
            _ => panic!("SpaceReward pk must be of Partition::Space type"),
        }
    }
    pub fn reward_type(&self) -> RewardType {
        self.sk.clone()
    }
}

impl SpaceReward {
    pub async fn list_by_space(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        limit: Option<i32>,
        bookmark: Option<String>,
    ) -> Result<(Vec<SpaceReward>, Option<String>)> {
        let (pk, _) = SpaceReward::keys(space_pk, &RewardType::None);

        let mut options = SpaceRewardQueryOption::builder().limit(limit.unwrap_or(50));
        if let Some(next) = bookmark {
            options = options.bookmark(next);
        };

        Self::query(cli, pk, options).await
    }

    pub async fn get_reward(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        reward_type: &RewardType,
    ) -> Result<Option<Self>> {
        let (pk, sk) = Self::keys(space_pk, reward_type);
        Self::get(cli, pk, Some(sk)).await
    }
}
