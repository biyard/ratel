use crate::common::{
    types::*, utils::time::get_now_timestamp_millis, *,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity)]
pub struct UserReward {
    pub pk: CompositePartition,
    pub sk: RewardKey,

    pub created_at: i64,
    pub updated_at: i64,

    pub total_claims: i64,
    pub total_points: i64,
}

#[cfg(feature = "server")]
impl UserReward {
    fn available_partition(target_pk: &Partition) -> Result<()> {
        match &target_pk {
            Partition::User(_) | Partition::Team(_) => Ok(()),
            _ => Err(Error::InvalidPartitionKey(
                "Must be User or Team".to_string(),
            )),
        }
    }

    pub fn from_reward(behavior: RewardUserBehavior, target_pk: Partition) -> Result<Self> {
        Self::available_partition(&target_pk)?;
        let reward_key = RewardKey::from(behavior);
        let (pk, sk) = Self::keys(target_pk, reward_key)?;
        let now = get_now_timestamp_millis();
        Ok(Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            total_claims: 0,
            total_points: 0,
        })
    }

    pub fn from_reward_key(reward_key: RewardKey, target_pk: Partition) -> Result<Self> {
        Self::available_partition(&target_pk)?;
        let (pk, sk) = Self::keys(target_pk, reward_key)?;
        let now = get_now_timestamp_millis();
        Ok(Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            total_claims: 0,
            total_points: 0,
        })
    }

    pub fn keys(
        target_pk: Partition,
        reward_key: RewardKey,
    ) -> Result<(CompositePartition, RewardKey)> {
        match &target_pk {
            Partition::User(_) | Partition::Team(_) => {
                Ok((CompositePartition(target_pk, Partition::Reward), reward_key))
            }
            _ => Err(Error::InvalidPartitionKey(
                "Must be User or Team".to_string(),
            )),
        }
    }
}
