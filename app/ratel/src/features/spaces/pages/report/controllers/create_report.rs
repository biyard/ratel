use crate::features::spaces::pages::report::models::SpaceReport;
use crate::features::spaces::pages::report::types::SpaceReportError;
use crate::features::spaces::pages::report::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateReportRequest {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateReportResponse {
    pub id: String,
}

#[post("/v3/spaces/{space_pk}/reports", role: SpaceUserRole)]
pub async fn create_report(
    space_pk: SpacePartition,
    req: CreateReportRequest,
) -> Result<CreateReportResponse> {
    SpaceReport::can_edit(role)?;
    let conf = ServerConfig::default();
    let dynamo = conf.dynamodb();

    let title = req.title.unwrap_or_default();
    let description = req.description.unwrap_or_default();

    let report = SpaceReport::new(space_pk, title, description);
    let id = match &report.sk {
        EntityType::SpaceReport(id) => id.clone(),
        _ => String::new(),
    };

    report.create(dynamo).await.map_err(|e| {
        crate::error!("failed to create report: {e:?}");
        SpaceReportError::ReportCreateFailed
    })?;

    Ok(CreateReportResponse { id })
}
