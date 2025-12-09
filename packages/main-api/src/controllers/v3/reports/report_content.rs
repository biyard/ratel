use crate::features::report::ContentReport;
use crate::features::spaces::boards::models::space_post::SpacePost;
use crate::features::spaces::boards::models::space_post_comment::SpacePostComment;
use crate::models::Post;
use crate::models::SpaceCommon;
use crate::*;

#[derive(Debug, Deserialize, Serialize, Default, aide::OperationIo, JsonSchema)]
#[serde(untagged)]
pub enum ReportContentRequest {
    #[default]
    Empty,

    Post {
        post_pk: Partition,
    },

    Space {
        space_pk: Partition,
    },

    SpacePost {
        space_pk: Partition,
        space_post_pk: Partition,
    },

    SpacePostComment {
        space_post_pk: Partition,
        comment_sk: EntityType,
    },
}

#[derive(Debug, Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct ReportContentResponse {
    pub reported: bool,
}

pub async fn report_content_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Json(req): Json<ReportContentRequest>,
) -> Result<Json<ReportContentResponse>> {
    let cli = &dynamo.client;

    tracing::debug!(
        "report_content_handler: user_pk={:?}, req={:?}",
        user.pk,
        req
    );

    match req {
        ReportContentRequest::Post { post_pk } => {
            let post = Post::get(cli, post_pk.clone(), Some(EntityType::Post))
                .await?
                .ok_or_else(|| Error::BadRequest("post not found".into()))?;

            if ContentReport::is_reported_for_target_by_user(cli, &post.pk, &user.pk).await? {
                tracing::info!(
                    "report_content_handler: post already reported by user, user_pk={}, post_pk={}",
                    user.pk,
                    post.pk
                );
                return Ok(Json(ReportContentResponse { reported: false }));
            }

            let report = ContentReport::from_post(&post, &user);
            report.submit(cli).await?;

            Ok(Json(ReportContentResponse { reported: true }))
        }

        ReportContentRequest::Space { space_pk } => {
            let space = SpaceCommon::get(cli, space_pk.clone(), Some(EntityType::SpaceCommon))
                .await?
                .ok_or_else(|| Error::BadRequest("space not found".into()))?;

            if ContentReport::is_reported_for_target_by_user(cli, &space.pk, &user.pk).await? {
                tracing::info!(
                    "report_content_handler: space already reported by user, user_pk={}, space_pk={}",
                    user.pk,
                    space.pk
                );
                return Ok(Json(ReportContentResponse { reported: false }));
            }

            let report = ContentReport::from_space(&space, &user);
            report.submit(cli).await?;

            Ok(Json(ReportContentResponse { reported: true }))
        }

        ReportContentRequest::SpacePost {
            space_pk,
            space_post_pk,
        } => {
            let (pk, sk) = SpacePost::keys(&space_pk, &space_post_pk);
            let space_post = SpacePost::get(cli, pk.clone(), Some(sk.clone()))
                .await?
                .ok_or_else(|| Error::BadRequest("space_post not found".into()))?;

            if ContentReport::is_reported_for_target_by_user(cli, &space_post.pk, &user.pk).await? {
                tracing::info!(
                    "report_content_handler: space_post already reported by user, user_pk={}, space_pk={}, space_post_pk={}",
                    user.pk,
                    space_post.pk,
                    space_post_pk,
                );
                return Ok(Json(ReportContentResponse { reported: false }));
            }

            let report = ContentReport::from_space_post(&space_post, &user);
            report.submit(cli).await?;

            Ok(Json(ReportContentResponse { reported: true }))
        }

        ReportContentRequest::SpacePostComment {
            space_post_pk,
            comment_sk,
        } => {
            let comment =
                SpacePostComment::get(cli, space_post_pk.clone(), Some(comment_sk.clone()))
                    .await?
                    .ok_or_else(|| Error::BadRequest("space_post_comment not found".into()))?;

            if ContentReport::is_reported_for_target_by_user(cli, &space_post_pk, &user.pk).await? {
                tracing::info!(
                    "report_content_handler: space_post_comment already reported by user, user_pk={}, space_post_pk={}",
                    user.pk,
                    space_post_pk
                );
                return Ok(Json(ReportContentResponse { reported: false }));
            }

            let report = ContentReport::from_space_post_comment(&comment, &space_post_pk, &user);
            report.submit(cli).await?;

            Ok(Json(ReportContentResponse { reported: true }))
        }

        ReportContentRequest::Empty {} => Err(Error::BadRequest("invalid report request".into())),
    }
}
