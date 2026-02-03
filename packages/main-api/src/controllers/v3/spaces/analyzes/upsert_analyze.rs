use crate::features::spaces::analyzes::SpaceAnalyze;
use crate::features::spaces::analyzes::SpaceAnalyzeRequest;
use crate::features::spaces::boards::models::space_post::SpacePost;
use crate::features::spaces::boards::models::space_post_comment::SpacePostComment;
use crate::models::SpaceCommon;
use crate::spaces::SpacePath;
use crate::spaces::SpacePathParam;
use crate::utils::aws::PollScheduler;
use crate::utils::aws::get_aws_config;
use crate::utils::reports::LdaConfigV1;
use crate::utils::reports::NetworkConfigV1;
use crate::utils::reports::TfidfConfigV1;
use crate::utils::reports::run_lda;
use crate::utils::reports::run_network;
use crate::utils::reports::run_tfidf;
use crate::*;
use futures::future::try_join_all;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema,
)]
pub struct UpsertAnalyzeRequest {
    pub lda_topics: usize,
    pub tf_idf_keywords: usize,
    pub network_top_nodes: usize,

    pub remove_topics: Vec<String>,
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

    let sdk_config = get_aws_config();
    let scheduler = PollScheduler::new(&sdk_config);

    scheduler
        .schedule_upsert_analyze(
            space_pk.clone(),
            req.lda_topics,
            req.tf_idf_keywords,
            req.network_top_nodes,
            req.remove_topics.clone(),
        )
        .await?;

    let request = SpaceAnalyzeRequest::new(
        space_pk.clone(),
        req.lda_topics,
        req.tf_idf_keywords,
        req.network_top_nodes,
        req.remove_topics,
    );
    request.create(&dynamo.client).await?;
    Ok(Json(SpaceAnalyze::default()))
}
