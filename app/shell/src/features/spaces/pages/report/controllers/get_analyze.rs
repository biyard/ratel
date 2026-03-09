use crate::features::spaces::pages::report::models::SpaceAnalyze;
use crate::features::spaces::pages::report::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAnalyzeResponse {
    pub html_contents: Option<String>,
}

// FIXME: implement middleware and authorization
#[get("/v3/spaces/{space_pk}/analyze")]
pub async fn get_analyze(space_pk: SpacePartition) -> Result<GetAnalyzeResponse> {
    let partition = Partition::Space(space_pk.to_string());

    let conf = ServerConfig::default();
    let dynamo = conf.dynamodb();

    let analyze = SpaceAnalyze::get(dynamo, &partition, Some(EntityType::SpaceAnalyze))
        .await
        .map_err(|e| Error::InternalServerError(format!("failed to load analyze: {e}")))?;
    let html_contents = analyze.and_then(|item| item.html_contents);

    Ok(GetAnalyzeResponse { html_contents })
}
