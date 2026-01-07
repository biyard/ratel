use crate::features::spaces::boards::models::space_post::SpacePost;
use crate::features::spaces::boards::models::space_post_comment::SpacePostComment;
use crate::models::SpaceCommon;
use crate::utils::reports::{
    LdaConfigV1, NetworkConfigV1, TfidfConfigV1, run_lda, run_network, run_tfidf,
};
use crate::*;
use axum::{Extension, Json, extract::State};
use futures::future::try_join_all;

#[derive(
    Debug,
    Clone,
    Default,
    serde::Serialize,
    serde::Deserialize,
    aide::OperationIo,
    schemars::JsonSchema,
)]
pub struct GetSpaceResultResponse {
    pub lda: Vec<TopicRow>,
    pub network: Vec<NetworkCentralityRow>,
    pub tf_idf: Vec<TfidfRow>,
}

pub async fn get_space_result_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_perms): NoApi<Permissions>,
    Extension(space): Extension<SpaceCommon>,
) -> Result<Json<GetSpaceResultResponse>> {
    use crate::utils::reports::build_space_report_pdf;
    use crate::utils::reports::upload_report_pdf_to_s3;

    // FIXME: add space results permission
    // perms.permitted(TeamGroupPermission::SpaceRead)?;
    let posts =
        SpacePost::find_by_space_ordered(&dynamo.client, space.clone().pk, SpacePost::opt_all())
            .await?
            .0;

    let comment_futs = posts.iter().filter_map(|post| {
        let space_post_pk = match &post.sk {
            EntityType::SpacePost(pk) => Partition::SpacePost(pk.to_string()),
            _ => return None,
        };

        Some(async {
            let (comments, _) = SpacePostComment::find_by_post_order_by_likes(
                &dynamo.client,
                space_post_pk,
                SpacePostComment::opt_all(),
            )
            .await?;
            Ok::<Vec<SpacePostComment>, crate::Error>(comments)
        })
    });

    let comments_per_post: Vec<Vec<SpacePostComment>> = try_join_all(comment_futs).await?;

    let mut post_comments: Vec<String> = Vec::new();
    for comments in comments_per_post {
        for c in comments {
            post_comments.push(c.content);
        }
    }

    tracing::debug!("total comments: {}", post_comments.len());

    let lda = run_lda(&post_comments, LdaConfigV1::default())?;
    let network = run_network(&post_comments, NetworkConfigV1::default())?;
    let tf_idf = run_tfidf(&post_comments, TfidfConfigV1::default())?;

    let pdf_bytes = build_space_report_pdf(&space, &lda, &network, &tf_idf)?;
    let (key, uri) = upload_report_pdf_to_s3(pdf_bytes).await?;
    tracing::debug!("space_result: pdf uploaded: {:?} {:?}", key, uri);

    Ok(Json(GetSpaceResultResponse {
        lda,
        network,
        tf_idf,
    }))
}
