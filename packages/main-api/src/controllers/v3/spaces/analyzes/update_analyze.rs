use crate::features::spaces::analyzes::SpaceAnalyze;
use crate::spaces::SpacePath;
use crate::spaces::SpacePathParam;
use crate::*;

#[derive(Debug, Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
#[serde(untagged)]
pub enum UpdateAnalyzeRequest {
    Lda {
        topics: Vec<String>,
        keywords: Vec<Vec<String>>,
    },
    HtmlContents {
        html_contents: String,
    },
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

    let mut analyze = analyze.ok_or(Error::AnalyzeNotFound)?;

    match req {
        UpdateAnalyzeRequest::Lda { topics, keywords } => {
            let lda_topics = topics
                .iter()
                .zip(keywords.iter())
                .flat_map(|(topic, kws)| {
                    kws.iter().map(|kw| TopicRow {
                        topic: topic.clone(),
                        keyword: kw.clone(),
                    })
                })
                .collect::<Vec<TopicRow>>();

            SpaceAnalyze::updater(space_pk, EntityType::SpaceAnalyze)
                .with_lda_topics(lda_topics.clone())
                .execute(&dynamo.client)
                .await?;

            analyze.lda_topics = lda_topics;
        }
        UpdateAnalyzeRequest::HtmlContents { html_contents } => {
            SpaceAnalyze::updater(space_pk, EntityType::SpaceAnalyze)
                .with_html_contents(html_contents.clone())
                .execute(&dynamo.client)
                .await?;
            analyze.html_contents = Some(html_contents);
        }
    }

    Ok(Json(analyze))
}
