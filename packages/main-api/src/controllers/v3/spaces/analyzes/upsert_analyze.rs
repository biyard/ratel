use crate::features::spaces::analyzes::SpaceAnalyze;
use crate::features::spaces::boards::models::space_post::SpacePost;
use crate::features::spaces::boards::models::space_post_comment::SpacePostComment;
use crate::models::SpaceCommon;
use crate::spaces::SpacePath;
use crate::spaces::SpacePathParam;
use crate::utils::reports::LdaConfigV1;
use crate::utils::reports::TfidfConfigV1;
use crate::utils::reports::run_lda;
use crate::utils::reports::run_tfidf;
use crate::*;
use futures::future::try_join_all;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema,
)]
pub struct UpsertAnalyzeRequest {
    pub lda_topics: usize,
}

pub async fn upsert_analyze_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<UpsertAnalyzeRequest>,
) -> Result<Json<SpaceAnalyze>> {
    if !matches!(space_pk.clone(), Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    permissions.permitted(TeamGroupPermission::SpaceEdit)?;

    let posts =
        SpacePost::find_by_space_ordered(&dynamo.client, space_pk.clone(), SpacePost::opt_all())
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
    let mut lda_config = LdaConfigV1::default();
    lda_config.num_topics = req.lda_topics;
    let lda = run_lda(&post_comments, lda_config)?;

    let mut tfidf_config = TfidfConfigV1::default();
    //FIXME: fix to params
    tfidf_config.max_features = 10;
    let tf_idf = run_tfidf(&post_comments, tfidf_config)?;

    let analyze = SpaceAnalyze::new(space_pk, Some(lda), None, Some(tf_idf));
    analyze.upsert(&dynamo.client).await?;

    Ok(Json(analyze))
}
