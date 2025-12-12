use std::collections::HashSet;

use crate::features::spaces::boards::models::space_post::SpacePost;
use crate::features::spaces::boards::models::space_post_comment::SpacePostComment;
use crate::models::{Post, SpaceCommon};
use crate::time::get_now_timestamp_millis;
use crate::types::*;
use crate::*;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReportTarget {
    #[default]
    Post,
    Space,
    SpacePost,
    SpacePostComment,
}

#[derive(
    Debug,
    Default,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    DynamoEntity,
    JsonSchema,
    aide::OperationIo,
)]
pub struct ContentReport {
    pub pk: CompositePartition,
    pub sk: EntityType,
    pub created_at: i64,
    pub updated_at: i64,

    pub target_pk: Partition,
    pub target_sk: Option<EntityType>,
    pub target: ReportTarget,

    pub reporter_pk: Partition,
}

impl ContentReport {
    fn new_base(
        target_pk: Partition,
        target_sk: Option<EntityType>,
        target: ReportTarget,
        reporter: &User,
    ) -> Self {
        let now = get_now_timestamp_millis();
        let reporter_pk = reporter.pk.clone();
        let pk = CompositePartition(reporter_pk.clone(), target_pk.clone());
        let sk = target_sk.clone().unwrap_or(EntityType::ContentReport);

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            target_pk,
            target_sk,
            target,
            reporter_pk,
        }
    }

    pub fn from_post(post: &Post, reporter: &User) -> Self {
        ContentReport::new_base(
            post.pk.clone(),
            Some(post.sk.clone()),
            ReportTarget::Post,
            reporter,
        )
    }

    pub fn from_space(space: &SpaceCommon, reporter: &User) -> Self {
        ContentReport::new_base(
            space.pk.clone(),
            Some(space.sk.clone()),
            ReportTarget::Space,
            reporter,
        )
    }

    pub fn from_space_post(space_post: &SpacePost, reporter: &User) -> Self {
        ContentReport::new_base(
            space_post.pk.clone(),
            Some(space_post.sk.clone()),
            ReportTarget::SpacePost,
            reporter,
        )
    }

    pub fn from_space_post_comment(
        comment: &SpacePostComment,
        space_post_pk: &Partition,
        reporter: &User,
    ) -> Self {
        ContentReport::new_base(
            space_post_pk.clone(),
            Some(comment.sk.clone()),
            ReportTarget::SpacePostComment,
            reporter,
        )
    }

    pub async fn submit(&self, cli: &aws_sdk_dynamodb::Client) -> Result<()> {
        self.create(cli).await
    }

    pub fn key_for_target(
        reporter_pk: &Partition,
        target_pk: &Partition,
        target_sk: Option<&EntityType>,
    ) -> (CompositePartition, EntityType) {
        let pk = CompositePartition(reporter_pk.clone(), target_pk.clone());
        let sk = target_sk.cloned().unwrap_or(EntityType::ContentReport);
        (pk, sk)
    }

    pub fn key_for_space_post_comment(
        reporter_pk: &Partition,
        space_post_pk: &Partition,
        comment: &SpacePostComment,
    ) -> (CompositePartition, EntityType) {
        Self::key_for_target(reporter_pk, space_post_pk, Some(&comment.sk))
    }

    pub async fn is_reported_for_target_by_user(
        cli: &aws_sdk_dynamodb::Client,
        target_pk: &Partition,
        target_sk: Option<&EntityType>,
        reporter_pk: &Partition,
    ) -> Result<bool> {
        let (pk, sk) = Self::key_for_target(reporter_pk, target_pk, target_sk);
        let reported = ContentReport::get(cli, &pk, Some(&sk)).await?.is_some();
        Ok(reported)
    }

    pub async fn reported_comment_ids_for_post_by_user(
        cli: &aws_sdk_dynamodb::Client,
        space_post_pk: &Partition,
        reporter_pk: &Partition,
        comments: &[SpacePostComment],
    ) -> Result<HashSet<String>> {
        let keys: Vec<_> = comments
            .iter()
            .map(|c| Self::key_for_space_post_comment(reporter_pk, space_post_pk, c))
            .collect();

        let reports = ContentReport::batch_get(cli, keys).await?;

        let set = reports
            .into_iter()
            .filter(|r| matches!(r.target, ReportTarget::SpacePostComment))
            .filter_map(|r| r.target_sk.map(|sk| sk.to_string()))
            .collect::<HashSet<_>>();

        Ok(set)
    }
}
