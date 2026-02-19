use crate::models::SpaceAnalyze;
use crate::*;
use percent_encoding::percent_decode_str;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAnalyzeHtmlRequest {
    pub html_contents: String,
}

// FIXME: implement middleware and authorization
#[patch("/v3/spaces/{space_pk}/analyze")]
pub async fn update_analyze(
    space_pk: String,
    req: UpdateAnalyzeHtmlRequest,
) -> Result<SpaceAnalyze> {
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
        .map_err(|e| {
            error!("update_analyze: failed to load analyze: {:?}", e);
            ServerFnError::new(format!("failed to load analyze: {e}"))
        })?;
    let mut analyze = analyze.unwrap_or_default();
    analyze.pk = partition.clone();
    analyze.sk = EntityType::SpaceAnalyze;

    SpaceAnalyze::updater(partition.clone(), EntityType::SpaceAnalyze)
        .with_html_contents(req.html_contents.clone())
        .execute(dynamo)
        .await
        .map_err(|e| ServerFnError::new(format!("failed to update analyze: {e:?}")))?;
    analyze.html_contents = Some(req.html_contents);

    Ok(analyze)
}
