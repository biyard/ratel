use crate::features::spaces::analyzes::{
    SpaceAnalyze, SpaceAnalyzeRequest, SpaceAnalyzeRequestQueryOption,
};
use crate::{
    controllers::v3::spaces::{SpacePath, SpacePathParam},
    *,
};

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema,
)]
pub struct SpaceAnalyzeResponse {
    #[serde(flatten)]
    pub analyze: SpaceAnalyzePayload,
    pub analyze_finish: bool,
}

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema,
)]
pub struct SpaceAnalyzePayload {
    #[serde(flatten)]
    pub analyze: SpaceAnalyze,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lda_count: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tf_idf_count: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_count: Option<usize>,
}

pub async fn get_analyze_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
) -> Result<Json<SpaceAnalyzeResponse>> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    permissions.permitted(TeamGroupPermission::SpaceRead)?;

    let analyze =
        SpaceAnalyze::get(&dynamo.client, &space_pk, Some(EntityType::SpaceAnalyze)).await?;

    let (requests, _) = SpaceAnalyzeRequest::find_by_analyze_finish(
        &dynamo.client,
        space_pk.clone(),
        SpaceAnalyzeRequestQueryOption::builder().sk(SpaceAnalyzeRequest::pending_key()),
    )
    .await?;

    let pending_request = requests.into_iter().max_by_key(|r| r.created_at);
    let analyze_finish = pending_request.is_none();

    let analyze = if let Some(request) = pending_request {
        let mut base = analyze.unwrap_or_default();
        base.pk = space_pk;
        base.sk = EntityType::SpaceAnalyze;
        base.remove_topics = request.remove_topics.clone();
        SpaceAnalyzePayload {
            analyze: base,
            tf_idf_count: Some(request.tf_idf_keywords),
            network_count: Some(request.network_top_nodes),
            lda_count: Some(request.lda_topics),
        }
    } else {
        let analyze = analyze.unwrap_or_default();
        let lda_count = {
            let mut topics = std::collections::HashSet::new();
            for row in &analyze.lda_topics {
                topics.insert(row.topic.as_str());
            }
            topics.len()
        };
        SpaceAnalyzePayload {
            analyze: analyze.clone(),
            tf_idf_count: Some(analyze.tf_idf.len()),
            network_count: Some(analyze.network.nodes.len()),
            lda_count: Some(lda_count),
        }
    };

    Ok(Json(SpaceAnalyzeResponse {
        analyze,
        analyze_finish,
    }))
}
