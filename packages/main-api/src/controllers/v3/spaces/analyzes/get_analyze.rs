use crate::features::spaces::analyzes::{SpaceAnalyze, SpaceAnalyzeRequest, SpaceAnalyzeRequestQueryOption};
use crate::{
    controllers::v3::spaces::{SpacePath, SpacePathParam},
    *,
};

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema,
)]
pub struct SpaceAnalyzeResponse {
    #[serde(flatten)]
    pub analyze: SpaceAnalyze,
    pub analyze_finish: bool,
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

    let analyze_finish = if analyze.is_none() {
        true
    } else {
        let (requests, _) = SpaceAnalyzeRequest::find_by_analyze_finish(
            &dynamo.client,
            space_pk.clone(),
            SpaceAnalyzeRequestQueryOption::builder()
                .sk(SpaceAnalyzeRequest::pending_key()),
        )
        .await?;
        requests.is_empty()
    };

    Ok(Json(SpaceAnalyzeResponse {
        analyze: analyze.unwrap_or_default(),
        analyze_finish,
    }))
}
