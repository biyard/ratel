use crate::features::spaces::rewards::{
    RewardCondition, RewardKey, RewardPeriod, SpaceReward, UserRewardHistory,
};
use crate::services::biyard::Biyard;
use crate::types::*;
use crate::*;

#[derive(
    Debug,
    Default,
    Clone,
    DynamoEntity,
    JsonSchema,
    OperationIo,
    serde::Serialize,
    serde::Deserialize,
)]
/// SpaceReward: 스페이스에 설정한 리워드
///
/// Key Structure:
/// - PK: USER#{USER_pk}##SPACE#{SPACE_PK}
/// - SK: {EntityType}#{RewardKey}
///
/// Examples:
/// POLL RESPOND : PK: USER_REWARD#{USER_pk}##SPACE#{SPACE_PK}, SK: POLL#{POLL_ID}#RESPOND
/// SPACE BOARD Comment : PK: USER_REWARD#{USER_pk}##SPACE#{SPACE_PK}, SK: SPACE_BOARD#{SPACE_BOARD_ID}#Comment
/// SPACE BOARD LIKE : PK: USER_REWARD#{USER_pk}##SPACE#{SPACE_PK}, SK: SPACE_BOARD#{SPACE_BOARD_ID}#LIKE
/// - Get All Rewards: UserReward::query(pk)
/// - Get Specific Entity Reward: UserReward::query_begins_with_sk(EntityType)
pub struct UserReward {
    pub pk: CompositePartition,
    pub sk: RewardKey,

    pub created_at: i64,
    pub updated_at: i64,

    pub total_claims: i64,
    pub total_points: i64,
}

impl UserReward {
    pub fn new(space_reward: SpaceReward, user_pk: UserPartition) -> Self {
        let (pk, sk) = Self::keys(user_pk, space_reward.get_space_pk(), space_reward.sk);
        let now = time::get_now_timestamp_millis();
        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            total_claims: 0,
            total_points: 0,
        }
    }

    pub fn keys(
        user_pk: UserPartition,
        space_pk: SpacePartition,
        reward_key: RewardKey,
    ) -> (CompositePartition, RewardKey) {
        let user_reward: UserRewardPartition = UserRewardPartition(user_pk.0);

        (
            CompositePartition(user_reward.into(), space_pk.into()),
            reward_key,
        )
    }

    pub async fn award(
        cli: &aws_sdk_dynamodb::Client,
        biyard: &Biyard,
        user_pk: UserPartition,
        space_reward: SpaceReward,
    ) -> Result<Self> {
        let now = time::get_now_timestamp_millis();
        let space_pk = space_reward.get_space_pk();

        let (user_reward_pk, user_reward_sk) =
            Self::keys(user_pk.clone(), space_pk.clone(), space_reward.sk.clone());
        let user_reward =
            Self::get(cli, user_reward_pk.clone(), Some(user_reward_sk.clone())).await?;

        let mut txs = vec![];

        // Check Reward Condition and Upsert UserReward
        let user_reward = if let Some(mut user_reward) = user_reward {
            match &space_reward.condition {
                RewardCondition::None => {}
                RewardCondition::MaxClaims(max) => {
                    if space_reward.total_claims >= *max {
                        return Err(Error::RewardMaxClaimsReached);
                    }
                }
                RewardCondition::MaxPoints(max) => {
                    if space_reward.total_points >= *max {
                        return Err(Error::RewardMaxPointsReached);
                    }
                }
                RewardCondition::MaxUserClaims(max) => {
                    if user_reward.total_claims >= *max {
                        return Err(Error::RewardMaxUserClaimsReached);
                    }
                }
                RewardCondition::MaxUserPoints(max) => {
                    if user_reward.total_points >= *max {
                        return Err(Error::RewardMaxUserPointsReached);
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
            let mut user_reward = Self::new(space_reward.clone(), user_pk.clone());
            user_reward.total_claims += 1;
            user_reward.total_points += space_reward.get_amount();
            txs.push(user_reward.create_transact_write_item());
            user_reward
        };

        // Update SpaceReward
        txs.push(
            SpaceReward::updater(&space_reward.pk, &space_reward.sk)
                .increase_total_claims(1)
                .increase_total_points(space_reward.get_amount())
                .with_updated_at(now)
                .transact_write_item(),
        );

        // Create UserRewardHistory
        let mut history = UserRewardHistory::new(user_pk.clone(), space_reward.clone());

        let res = biyard
            .award_points(
                user_pk.clone().into(),
                space_reward.get_amount(),
                space_reward.description.clone(),
                None,
            )
            .await?;

        history.set_transaction(res.transaction_id.clone(), res.month.clone());
        txs.push(history.create_transact_write_item());
        if let Err(_) = transact_write_items!(cli, txs) {
            //When Transaction Failed. Revert Points
            biyard
                .award_points(
                    user_pk.clone().into(),
                    space_reward.get_amount() * -1,
                    "Revert Points".to_string(),
                    Some(res.month),
                )
                .await?;
            return Err(Error::RewardAlreadyClaimedInPeriod);
        }
        Ok(user_reward)
    }
}
