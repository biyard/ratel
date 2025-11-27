use crate::features::spaces::rewards::{RewardType, SpaceReward};
use crate::services::biyard::Biyard;
use crate::types::*;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default, JsonSchema, OperationIo)]
pub struct UserReward {
    // #[dynamo(index = "gsi1", name = "find_by_month", pk)]
    // USER#{USER_PK}#REWARDS#SPACE_PK#REWARD_TYPE
    pub pk: CompositePartition,
    pub sk: EntityType,
    // #[dynamo(index = "gsi1", sk, order = 1)]
    pub created_at: i64,

    // pub reward_label: String,
    // pub reward_amount: i64,
    // pub reward_description: Option<String>,

    // #[dynamo(index = "gsi1", sk, order = 0)]
    // pub month: String, // e.g., "2024-06"
    pub month: String,          // e.g., "2024-06"
    pub transaction_id: String, // Biyard Transaction ID
}

impl UserReward {
    pub fn new(
        user_pk: Partition,
        space_reward: SpaceReward,
        month: String,
        transaction_id: String,
    ) -> Self {
        let now = time::get_now_timestamp_millis();
        let (pk, sk) = Self::keys(&user_pk, &space_reward);
        Self {
            pk,
            sk,
            created_at: now,

            // SpaceReward Info
            // SpaceReward Info
            // reward_label: space_reward.label,
            // reward_description: space_reward.description,
            // reward_amount: space_reward.amount,

            // Biyard Response
            transaction_id,
            month,
        }
    }

    pub fn keys(
        user_pk: &Partition,
        space_reward: &SpaceReward,
    ) -> (CompositePartition, EntityType) {
        if !matches!(user_pk, Partition::User(_)) {
            panic!("UserReward pk must be of Partition::User type");
        }
        (
            CompositePartition(user_pk.clone(), Partition::Reward),
            EntityType::UserReward(
                space_reward.space_pk().to_string(),
                space_reward.reward_type().to_string(),
            ),
        )
    }

    pub async fn award(
        cli: &aws_sdk_dynamodb::Client,
        biyard: &Biyard,
        user_pk: Partition,
        space_reward: SpaceReward,
    ) -> Result<UserReward> {
        let res = biyard
            .award_points(
                user_pk.clone(),
                space_reward.amount,
                space_reward.description.clone(),
                None,
            )
            .await?;

        let user_reward = Self::new(user_pk, space_reward, res.month, res.transaction_id);
        user_reward.create(cli).await?;
        Ok(user_reward)
    }
}
