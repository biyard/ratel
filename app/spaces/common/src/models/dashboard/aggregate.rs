use common::{DynamoEntity, EntityType, Partition};
use serde::{Deserialize, Serialize};

/// Single aggregate row per space storing all dashboard counters.
/// Each action only does `increase_X(1)` or `decrease_X(1)` — one atomic DynamoDB update.
#[derive(Debug, Clone, Default, Serialize, Deserialize, DynamoEntity)]
pub struct DashboardAggregate {
    pub pk: Partition,
    pub sk: EntityType,

    pub poll_count: i64,
    pub post_count: i64,
    pub poll_response_count: i64,
    pub comment_count: i64,
    pub like_count: i64,
    pub total_points: i64,
}

impl DashboardAggregate {
    pub fn keys(space_pk: &Partition) -> (Partition, EntityType) {
        (
            space_pk.clone(),
            EntityType::SpaceDashboardExtension("aggregate".to_string()),
        )
    }
}

#[cfg(feature = "server")]
impl DashboardAggregate {
    pub async fn get_or_create(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
    ) -> common::Result<Self> {
        let aggregate = Self::get_or_default(cli, space_pk).await?;
        let (agg_pk, agg_sk) = Self::keys(space_pk);

        if Self::get(cli, &agg_pk, Some(agg_sk)).await?.is_none() {
            cli.transact_write_items()
                .set_transact_items(Some(vec![aggregate.create_transact_write_item()]))
                .send()
                .await
                .map_err(|e| {
                    crate::Error::Unknown(format!("Failed to create dashboard aggregate: {e}"))
                })?;
        }

        Ok(aggregate)
    }

    pub async fn get_or_default(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
    ) -> common::Result<Self> {
        let (pk, sk) = Self::keys(space_pk);
        match Self::get(cli, &pk, Some(sk)).await? {
            Some(agg) => Ok(agg),
            None => Ok(Self {
                pk: space_pk.clone(),
                sk: EntityType::SpaceDashboardExtension("aggregate".to_string()),
                ..Default::default()
            }),
        }
    }

    /// Atomic increment helpers — return a TransactWriteItem for use in existing transactions.
    pub fn inc_polls(
        space_pk: &Partition,
        delta: i64,
    ) -> aws_sdk_dynamodb::types::TransactWriteItem {
        let (pk, sk) = Self::keys(space_pk);
        Self::updater(&pk, sk)
            .increase_poll_count(delta)
            .transact_write_item()
    }

    pub fn inc_posts(
        space_pk: &Partition,
        delta: i64,
    ) -> aws_sdk_dynamodb::types::TransactWriteItem {
        let (pk, sk) = Self::keys(space_pk);
        Self::updater(&pk, sk)
            .increase_post_count(delta)
            .transact_write_item()
    }

    pub fn inc_poll_responses(
        space_pk: &Partition,
        delta: i64,
    ) -> aws_sdk_dynamodb::types::TransactWriteItem {
        let (pk, sk) = Self::keys(space_pk);
        Self::updater(&pk, sk)
            .increase_poll_response_count(delta)
            .transact_write_item()
    }

    pub fn inc_comments(
        space_pk: &Partition,
        delta: i64,
    ) -> aws_sdk_dynamodb::types::TransactWriteItem {
        let (pk, sk) = Self::keys(space_pk);
        Self::updater(&pk, sk)
            .increase_comment_count(delta)
            .transact_write_item()
    }

    pub fn inc_likes(
        space_pk: &Partition,
        delta: i64,
    ) -> aws_sdk_dynamodb::types::TransactWriteItem {
        let (pk, sk) = Self::keys(space_pk);
        Self::updater(&pk, sk)
            .increase_like_count(delta)
            .transact_write_item()
    }

    pub fn inc_points(
        space_pk: &Partition,
        delta: i64,
    ) -> aws_sdk_dynamodb::types::TransactWriteItem {
        let (pk, sk) = Self::keys(space_pk);
        Self::updater(&pk, sk)
            .increase_total_points(delta)
            .transact_write_item()
    }
}
