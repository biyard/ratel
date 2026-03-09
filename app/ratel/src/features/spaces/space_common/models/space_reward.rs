use crate::common::{
    models::reward::{UserReward, UserRewardHistory},
    types::*, utils::time::get_now_timestamp_millis, *,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity)]
pub struct SpaceReward {
    pub pk: Partition,
    pub sk: RewardKey,

    pub created_at: i64,
    pub updated_at: i64,

    #[serde(default)]
    pub behavior: RewardUserBehavior,
    #[serde(default)]
    pub description: String,

    pub credits: i64,
    pub point: i64,

    pub total_points: i64,
    pub total_claims: i64,

    pub period: RewardPeriod,
    pub condition: RewardCondition,
}

#[cfg(feature = "server")]
impl SpaceReward {
    pub fn new(
        space_pk: SpacePartition,
        entity_type: EntityType,
        behavior: RewardUserBehavior,
        description: String,
        credits: i64,
        point: i64,
        period: RewardPeriod,
        condition: RewardCondition,
    ) -> Self {
        let sk = RewardKey::from((space_pk.clone(), entity_type, behavior.clone()));
        let now = get_now_timestamp_millis();

        Self {
            pk: space_pk.into(),
            sk,
            behavior,
            created_at: now,
            updated_at: now,
            credits,
            point,
            description,
            period,
            condition,
            total_points: 0,
            total_claims: 0,
        }
    }

    pub fn get_amount(&self) -> i64 {
        self.point * self.credits
    }

    #[cfg(feature = "server")]
    pub async fn get_by_action(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: SpacePartition,
        action: EntityType,
        behavior: RewardUserBehavior,
    ) -> crate::common::Result<Self> {
        let pk: Partition = space_pk.clone().into();
        let sk = RewardKey::from((space_pk, action, behavior));

        Self::get(cli, pk, Some(sk))
            .await?
            .ok_or(SpaceRewardError::NotFound.into())
    }

    #[cfg(feature = "server")]
    pub async fn list_by_action(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: SpacePartition,
        action: Option<EntityType>,
    ) -> crate::common::Result<Vec<Self>> {
        let pk: Partition = space_pk.clone().into();
        let sk = RewardKey::get_space_reward_sk_prefix(space_pk, action);

        let opt = SpaceReward::opt_all().sk(sk);

        let (items, _) = Self::query(cli, pk, opt).await?;

        Ok(items)
    }
}

impl SpaceReward {
    pub fn can_edit(role: &SpaceUserRole) -> crate::common::Result<()> {
        match role {
            SpaceUserRole::Creator => Ok(()),
            _ => Err(Error::NoPermission),
        }
    }
}

#[cfg(feature = "server")]
impl SpaceReward {
    pub async fn award(
        cli: &aws_sdk_dynamodb::Client,
        space_reward: &SpaceReward,
        target_pk: Partition,
    ) -> crate::common::Result<UserReward> {
        let now = get_now_timestamp_millis();
        let space_pk = space_reward.pk.clone();

        let (user_reward_pk, user_reward_sk) =
            UserReward::keys(target_pk.clone(), space_reward.sk.clone())?;
        let user_reward =
            UserReward::get(cli, user_reward_pk.clone(), Some(user_reward_sk.clone())).await?;

        let mut txs = vec![];
        let (current_user_claims, current_user_points) = user_reward
            .as_ref()
            .map(|reward| (reward.total_claims, reward.total_points))
            .unwrap_or((0, 0));

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
                if current_user_claims >= *max {
                    return Err(SpaceRewardError::MaxUserClaimsReached.into());
                }
            }
            RewardCondition::MaxUserPoints(max) => {
                if current_user_points >= *max {
                    return Err(SpaceRewardError::MaxUserPointsReached.into());
                }
            }
        }

        let user_reward = if let Some(mut user_reward) = user_reward {
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
            let mut user_reward =
                UserReward::from_reward_key(space_reward.sk.clone(), target_pk.clone())?;
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
        let history = UserRewardHistory::from_params(
            target_pk.clone(),
            space_reward.sk.clone(),
            &space_reward.period,
            space_reward.get_amount(),
        );
        txs.push(history.create_transact_write_item());

        // Execute DB transaction
        if let Err(err) = crate::transact_write_items!(cli, txs) {
            if let aws_sdk_dynamodb::Error::TransactionCanceledException(tx_err) = &err {
                let is_conditional_failure = tx_err
                    .cancellation_reasons()
                    .iter()
                    .any(|reason| reason.code() == Some("ConditionalCheckFailed"));

                if is_conditional_failure {
                    return Err(SpaceRewardError::AlreadyClaimedInPeriod.into());
                }
            }

            return Err(Error::Unknown(format!(
                "failed to write reward transaction: {err}"
            )));
        }

        Ok(user_reward)
    }
}
