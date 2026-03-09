use crate::features::spaces::pages::report::models::SpaceAnalyze;
use crate::features::spaces::pages::report::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAnalyzeHtmlRequest {
    pub html_contents: String,
}

// FIXME: implement middleware and authorization
#[patch("/v3/spaces/{space_pk}/analyze")]
pub async fn update_analyze(
    space_pk: SpacePartition,
    req: UpdateAnalyzeHtmlRequest,
) -> Result<SpaceAnalyze> {
    let partition = Partition::Space(space_pk.to_string());

    let conf = ServerConfig::default();
    let dynamo = conf.dynamodb();

    let analyze = SpaceAnalyze::get(dynamo, &partition, Some(EntityType::SpaceAnalyze))
        .await
        .map_err(|e| {
            error!("update_analyze: failed to load analyze: {:?}", e);
            Error::InternalServerError(format!("failed to load analyze: {e}"))
        })?;
    let mut analyze = analyze.unwrap_or_default();
    analyze.pk = partition.clone();
    analyze.sk = EntityType::SpaceAnalyze;

    SpaceAnalyze::updater(partition.clone(), EntityType::SpaceAnalyze)
        .with_html_contents(req.html_contents.clone())
        .execute(dynamo)
        .await
        .map_err(|e| Error::InternalServerError(format!("failed to update analyze: {e:?}")))?;
    analyze.html_contents = Some(req.html_contents);

    Ok(analyze)
}
