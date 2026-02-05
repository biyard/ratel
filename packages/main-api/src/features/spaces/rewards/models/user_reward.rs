use crate::features::spaces::rewards::{
    RewardCondition, RewardPeriod, SpaceReward, SpaceRewardSk, UserRewardHistory,
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
/// UserReward: 유저가 특정 리워드를 획득한 통계
///
/// Key Structure:
/// - PK: USER#{user_pk}##SPACE#{space_pk} or TEAM#{team_pk}##SPACE#{space_pk}
/// - SK: SpaceReward##{EntityType}##{RewardUserBehavior}
///
/// Examples:
/// - PK: USER#{USER_ID}##SPACE#{SPACE_ID}
/// - SK: SpaceReward##Poll##CreatePoll
///
/// - PK: TEAM#{TEAM_ID}##SPACE#{SPACE_ID}
/// - SK: SpaceReward##Post##CreatePost
pub struct UserReward {
    pub pk: CompositePartition,
    pub sk: SpaceRewardSk,

    pub created_at: i64,
    pub updated_at: i64,

    pub total_claims: i64,
    pub total_points: i64,
}

impl UserReward {
    pub fn new(space_reward: SpaceReward, target_pk: Partition) -> Result<Self> {
        let (pk, sk) = Self::keys(target_pk, space_reward.pk.into(), space_reward.sk)?;
        let now = time::get_now_timestamp_millis();
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
        space_pk: SpacePartition,
        reward_key: SpaceRewardSk,
    ) -> Result<(CompositePartition, SpaceRewardSk)> {
        match target_pk {
            Partition::User(_) | Partition::Team(_) => {
                Ok((CompositePartition(target_pk, space_pk.into()), reward_key))
            }
            _ => {
                return Err(Error::InvalidPartitionKey(
                    "Biyard target_pk must be of Partition::User or Partition::Team type"
                        .to_string(),
                ));
            }
        }
    }

    pub async fn award(
        cli: &aws_sdk_dynamodb::Client,
        biyard: &Biyard,
        space_reward: SpaceReward,
        target_pk: Partition,        // Team Or User
        owner_pk: Option<Partition>, // Team Or User
    ) -> Result<Self> {
        let now = time::get_now_timestamp_millis();
        let space_pk = space_reward.pk.clone();

        let (user_reward_pk, user_reward_sk) = Self::keys(
            target_pk.clone(),
            space_pk.clone().into(),
            space_reward.sk.clone(),
        )?;
        let user_reward =
            Self::get(cli, user_reward_pk.clone(), Some(user_reward_sk.clone())).await?;

        let mut txs = vec![];

        // Check Reward Condition and Upsert UserReward
        let user_reward = if let Some(mut user_reward) = user_reward {
            match &space_reward.condition {
                RewardCondition::None => {}
                RewardCondition::MaxClaims(max) => {
                    if space_reward.total_claims >= *max {
                        return Err(Error::SpaceRewardMaxClaimsReached);
                    }
                }
                RewardCondition::MaxPoints(max) => {
                    if space_reward.total_points >= *max {
                        return Err(Error::SpaceRewardMaxPointsReached);
                    }
                }
                RewardCondition::MaxUserClaims(max) => {
                    if user_reward.total_claims >= *max {
                        return Err(Error::SpaceRewardMaxUserClaimsReached);
                    }
                }
                RewardCondition::MaxUserPoints(max) => {
                    if user_reward.total_points >= *max {
                        return Err(Error::SpaceRewardMaxUserPointsReached);
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
            let mut user_reward = Self::new(space_reward.clone(), target_pk.clone())?;
            user_reward.total_claims += 1;
            user_reward.total_points += space_reward.get_amount();
            txs.push(user_reward.create_transact_write_item());
            user_reward
        };

        // Update SpaceReward
        txs.push(
            SpaceReward::updater(&space_pk, &space_reward.sk)
                .increase_total_claims(1)
                .increase_total_points(space_reward.get_amount())
                .with_updated_at(now)
                .transact_write_item(),
        );

        // Create UserRewardHistory
        let mut history = UserRewardHistory::new(target_pk.clone(), space_reward.clone());

        let user_res = biyard
            .award_points(
                target_pk.clone().into(),
                space_reward.get_amount(),
                space_reward.description.clone(),
                None,
            )
            .await?;

        let owner_res = if let Some(ref owner_pk) = owner_pk {
            let owner_amount = space_reward.get_amount() * 10 / 100;
            match biyard
                .award_points(
                    owner_pk.clone().into(),
                    owner_amount,
                    space_reward.description.clone(),
                    None,
                )
                .await
            {
                Ok(res) => Some(res),
                Err(e) => {
                    // Rollback user points
                    let _ = biyard
                        .award_points(
                            target_pk.clone().into(),
                            space_reward.get_amount() * -1,
                            "Revert Points".to_string(),
                            Some(user_res.month.clone()),
                        )
                        .await;
                    return Err(e);
                }
            }
        } else {
            None
        };

        history.set_transaction(user_res.transaction_id.clone(), user_res.month.clone());
        txs.push(history.create_transact_write_item());

        // 3. Execute DB transaction
        if let Err(_) = transact_write_items!(cli, txs) {
            // Rollback user points
            let _ = biyard
                .award_points(
                    target_pk.clone().into(),
                    space_reward.get_amount() * -1,
                    "Revert Points".to_string(),
                    Some(user_res.month.clone()),
                )
                .await;

            // Rollback owner points (if awarded)
            if let (Some(owner_pk), Some(owner_res)) = (owner_pk, owner_res) {
                let _ = biyard
                    .award_points(
                        owner_pk.clone().into(),
                        (space_reward.get_amount() * 10 / 100) * -1,
                        "Revert Points".to_string(),
                        Some(owner_res.month.clone()),
                    )
                    .await;
            }

            return Err(Error::SpaceRewardAlreadyClaimedInPeriod);
        }

        Ok(user_reward)
    }
}
