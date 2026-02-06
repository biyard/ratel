use crate::utils::time::get_now_timestamp_millis;
use crate::*;

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    DynamoEntity,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct SpaceIncentiveScore {
    pub pk: Partition,
    pub sk: EntityType,

    #[serde(default)]
    pub created_at: i64,
    #[serde(default)]
    pub updated_at: i64,

    #[serde(default)]
    pub space_pk: Partition,
    #[serde(default)]
    pub user_pk: Partition,

    #[serde(default)]
    pub pre_score: i64,
    #[serde(default)]
    pub post_score: i64,
}

impl SpaceIncentiveScore {
    pub fn keys(space_pk: &Partition, user_pk: &Partition) -> (Partition, EntityType) {
        (
            space_pk.clone(),
            EntityType::SpaceIncentiveScore(user_pk.to_string()),
        )
    }

    pub fn new(space_pk: Partition, user_pk: Partition) -> Self {
        let now = get_now_timestamp_millis();
        let sk = EntityType::SpaceIncentiveScore(user_pk.to_string());

        Self {
            pk: space_pk.clone(),
            sk,
            created_at: now,
            updated_at: now,
            space_pk,
            user_pk,
            pre_score: 0,
            post_score: 0,
        }
    }

    pub fn total_score(&self) -> i64 {
        self.pre_score + self.post_score
    }

    pub async fn get_by_user(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        user_pk: &Partition,
    ) -> Result<Option<Self>> {
        let (_, sk) = Self::keys(space_pk, user_pk);
        Self::get(cli, space_pk.clone(), Some(sk)).await
    }

    pub async fn add_pre_score(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        user_pk: &Partition,
        score: i64,
    ) -> Result<()> {
        let now = get_now_timestamp_millis();
        if let Some(mut item) = Self::get_by_user(cli, space_pk, user_pk).await? {
            item.pre_score += score;
            item.updated_at = now;
            item.upsert(cli).await?;
            return Ok(());
        }

        let mut item = Self::new(space_pk.clone(), user_pk.clone());
        item.pre_score = score;
        item.updated_at = now;
        item.upsert(cli).await?;
        Ok(())
    }

    pub async fn add_post_score(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        user_pk: &Partition,
        score: i64,
    ) -> Result<()> {
        let now = get_now_timestamp_millis();
        if let Some(mut item) = Self::get_by_user(cli, space_pk, user_pk).await? {
            item.post_score += score;
            item.updated_at = now;
            item.upsert(cli).await?;
            return Ok(());
        }

        let mut item = Self::new(space_pk.clone(), user_pk.clone());
        item.post_score = score;
        item.updated_at = now;
        item.upsert(cli).await?;
        Ok(())
    }

    pub async fn find_by_space(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        opt: SpaceIncentiveScoreQueryOption,
    ) -> Result<(Vec<Self>, Option<String>)> {
        let opt = opt.sk(EntityType::SpaceIncentiveScore(String::new()).to_string());
        Self::query(cli, space_pk.clone(), opt).await
    }
}
