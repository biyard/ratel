use crate::features::spaces::analyzes::SpaceAnalyze;
use crate::spaces::SpacePath;
use crate::spaces::SpacePathParam;
use crate::*;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema,
)]
pub struct UpdateAnalyzeRequest {
    pub topics: Vec<String>,
    pub keywords: Vec<Vec<String>>,
}

pub async fn update_analyze_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<UpdateAnalyzeRequest>,
) -> Result<Json<SpaceAnalyze>> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    permissions.permitted(TeamGroupPermission::SpaceEdit)?;

    let analyze = SpaceAnalyze::get(
        &dynamo.client,
        space_pk.clone(),
        Some(EntityType::SpaceAnalyze),
    )
    .await?;

    if analyze.is_none() {
        return Err(Error::AnalyzeNotFound);
    }

    let mut analyze = analyze.unwrap();

    let lda_topics = req
        .topics
        .iter()
        .zip(req.keywords.iter())
        .map(|(topic, keywords)| {
            keywords
                .iter()
                .map(|keyword| TopicRow {
                    topic: topic.clone(),
                    keyword: keyword.clone(),
                })
                .collect::<Vec<TopicRow>>()
        })
        .flatten()
        .collect::<Vec<TopicRow>>();

    let _ = SpaceAnalyze::updater(space_pk, EntityType::SpaceAnalyze)
        .with_lda_topics(lda_topics.clone())
        .execute(&dynamo.client)
        .await?;

    analyze.lda_topics = lda_topics;

    Ok(Json(analyze))
}
