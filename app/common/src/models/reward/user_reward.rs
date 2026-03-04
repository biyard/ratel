use crate::{
    models::space::SpaceReward, types::*, utils::time::get_now_timestamp_millis, *,
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

    pub fn from_space_reward(space_reward: &SpaceReward, target_pk: Partition) -> Result<Self> {
        Self::available_partition(&target_pk)?;
        let (pk, sk) = Self::keys(target_pk, space_reward.sk.clone())?;
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

    #[cfg(feature = "server")]
    pub async fn award(
        cli: &aws_sdk_dynamodb::Client,
        space_reward: &SpaceReward,
        target_pk: Partition,
    ) -> Result<Self> {
        use super::UserRewardHistory;

        let now = get_now_timestamp_millis();
        let space_pk = space_reward.pk.clone();

        let (user_reward_pk, user_reward_sk) =
            Self::keys(target_pk.clone(), space_reward.sk.clone())?;
        let user_reward =
            Self::get(cli, user_reward_pk.clone(), Some(user_reward_sk.clone())).await?;

        let mut txs = vec![];

        let user_reward = if let Some(mut user_reward) = user_reward {
            match &space_reward.condition {
                RewardCondition::None => {}
                RewardCondition::MaxClaims(max) => {
                    if space_reward.total_claims >= *max {
                        return Err(SpaceRewardError::MaxClaimsReached.into());
                    }
                }
                RewardCondition::MaxPoints(max) => {
                    if space_reward.total_points >= *max {
                        return Err(SpaceRewardError::MaxPointsReached.into());
                    }
                }
                RewardCondition::MaxUserClaims(max) => {
                    if user_reward.total_claims >= *max {
                        return Err(SpaceRewardError::MaxUserClaimsReached.into());
                    }
                }
                RewardCondition::MaxUserPoints(max) => {
                    if user_reward.total_points >= *max {
                        return Err(SpaceRewardError::MaxUserPointsReached.into());
                    }
                }
            }
            txs.push(
                UserReward::updater(&user_reward.pk, &user_reward.sk)
                    .increase_total_points(space_reward.get_amount())
                    .increase_total_claims(1)
                    .with_updated_at(now)
                    .transact_write_item(),
            );
            user_reward.total_claims += 1;
            user_reward.total_points += space_reward.get_amount();
            user_reward
        } else {
            let mut user_reward = Self::from_space_reward(space_reward, target_pk.clone())?;
            user_reward.total_claims += 1;
            user_reward.total_points += space_reward.get_amount();
            txs.push(user_reward.create_transact_write_item());
            user_reward
        };

        // Update SpaceReward totals
        txs.push(
            SpaceReward::updater(&space_pk, &space_reward.sk)
                .increase_total_claims(1)
                .increase_total_points(space_reward.get_amount())
                .with_updated_at(now)
                .transact_write_item(),
        );

        // Create UserRewardHistory
        let history = UserRewardHistory::new(target_pk.clone(), space_reward);
        txs.push(history.create_transact_write_item());

        // Execute DB transaction
        if let Err(_) = transact_write_items!(cli, txs) {
            return Err(SpaceRewardError::AlreadyClaimedInPeriod.into());
        }

        Ok(user_reward)
    }
}
