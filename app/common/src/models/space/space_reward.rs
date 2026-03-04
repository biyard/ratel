use crate::{types::*, utils::time::get_now_timestamp_millis, *};

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
    ) -> Result<Self> {
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
    ) -> Result<Vec<Self>> {
        let pk: Partition = space_pk.clone().into();
        let sk = RewardKey::get_space_reward_sk_prefix(space_pk, action);

        let opt = SpaceReward::opt_all().sk(sk);

        let (items, _) = Self::query(cli, pk, opt).await?;

        Ok(items)
    }
}

impl SpaceReward {
    pub fn can_edit(role: &SpaceUserRole) -> crate::Result<()> {
        match role {
            SpaceUserRole::Creator => Ok(()),
            _ => Err(Error::NoPermission),
        }
    }
}
