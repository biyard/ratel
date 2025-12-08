use crate::features::spaces::boards::models::space_post::SpacePost;
use crate::features::spaces::boards::models::space_post_comment::SpacePostComment;
use crate::models::{Post, SpaceCommon};
use crate::time::get_now_timestamp_millis;
use crate::*;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReportTargetKind {
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
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(index = "gsi1", sk)]
    #[dynamo(index = "gsi2", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    #[dynamo(prefix = "TARGET_PK", name = "find_by_target", index = "gsi1", pk)]
    pub target_pk: Partition,
    pub target_sk: Option<EntityType>,
    pub target_kind: ReportTargetKind,

    #[dynamo(prefix = "REPORTER_PK", name = "find_by_reporter", index = "gsi2", pk)]
    pub reporter_pk: Partition,
}

impl ContentReport {
    fn new_base(
        target_pk: Partition,
        target_sk: Option<EntityType>,
        target_kind: ReportTargetKind,
        reporter: &User,
    ) -> Self {
        let now = get_now_timestamp_millis();
        let id = crate::sorted_uuid();

        Self {
            pk: Partition::Report(id),
            sk: EntityType::ContentReport,
            created_at: now,
            updated_at: now,
            target_pk,
            target_sk,
            target_kind,
            reporter_pk: reporter.pk.clone(),
        }
    }

    pub fn from_post(post: &Post, reporter: &User) -> Self {
        ContentReport::new_base(
            post.pk.clone(),
            Some(post.sk.clone()),
            ReportTargetKind::Post,
            reporter,
        )
    }

    pub fn from_space(space: &SpaceCommon, reporter: &User) -> Self {
        ContentReport::new_base(
            space.pk.clone(),
            Some(space.sk.clone()),
            ReportTargetKind::Space,
            reporter,
        )
    }

    pub fn from_space_post(space_post: &SpacePost, reporter: &User) -> Self {
        ContentReport::new_base(
            space_post.pk.clone(),
            Some(space_post.sk.clone()),
            ReportTargetKind::SpacePost,
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
            ReportTargetKind::SpacePostComment,
            reporter,
        )
    }

    pub async fn submit(&self, cli: &aws_sdk_dynamodb::Client) -> Result<()> {
        self.create(cli).await
    }

    pub async fn is_reported_for_target_by_user(
        cli: &aws_sdk_dynamodb::Client,
        target_pk: &Partition,
        reporter_pk: &Partition,
    ) -> Result<bool> {
        let opt = ContentReport::opt_all();
        let (items, _) =
            ContentReport::find_by_reporter(cli, format!("{}", reporter_pk), opt).await?;

        Ok(items.iter().any(|r| &r.target_pk == target_pk))
    }

    pub async fn count_for_target(
        cli: &aws_sdk_dynamodb::Client,
        target_pk: &Partition,
    ) -> Result<i64> {
        let opt = ContentReport::opt_all();
        let (items, _) = ContentReport::find_by_target(cli, format!("{}", target_pk), opt).await?;
        Ok(items.len() as i64)
    }
}
