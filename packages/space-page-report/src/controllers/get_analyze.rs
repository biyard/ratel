use crate::models::SpaceAnalyze;
use crate::*;
use percent_encoding::percent_decode_str;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAnalyzeResponse {
    pub html_contents: Option<String>,
}

// FIXME: implement middleware and authorization
#[get("/v3/spaces/{space_pk}/analyze")]
pub async fn get_analyze(space_pk: String) -> Result<GetAnalyzeResponse> {
    let decoded = percent_decode_str(&space_pk)
        .decode_utf8()
        .map_err(|e| Error::InternalServerError(format!("invalid space_pk encoding: {e}")))?;
    let partition = Partition::from_str(&decoded)
        .map_err(|e| Error::InternalServerError(format!("invalid space_pk: {e}")))?;
    if !matches!(partition, Partition::Space(_)) {
        return Err(Error::InvalidPartitionKey(format!(
            "invalid space_pk partition"
        )));
    }

    let conf = ServerConfig::default();
    let dynamo = conf.dynamodb();

    let analyze = SpaceAnalyze::get(dynamo, &partition, Some(EntityType::SpaceAnalyze))
        .await
        .map_err(|e| Error::InternalServerError(format!("failed to load analyze: {e}")))?;
    let html_contents = analyze.and_then(|item| item.html_contents);

    Ok(GetAnalyzeResponse { html_contents })
}
