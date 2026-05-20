use crate::common::models::space::SpaceCommon;
use crate::features::spaces::pages::report::models::SpaceReport;
use crate::features::spaces::pages::report::types::{ReportBlock, SpaceReportError};
use crate::features::spaces::pages::report::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetReportResponse {
    pub id: String,
    pub status: ReportStatus,
    pub title: String,
    pub description: String,
    pub html_contents: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub blocks: Vec<ReportBlock>,
    pub author: String,
}

#[get("/v3/spaces/{space_pk}/reports/{report_id}", role: SpaceUserRole)]
pub async fn get_report(
    space_pk: SpacePartition,
    report_id: SpaceReportEntityType,
) -> Result<GetReportResponse> {
    SpaceReport::can_view(role)?;
    let space_partition: Partition = space_pk.into();
    let sk: EntityType = report_id.into();

    let conf = ServerConfig::default();
    let dynamo = conf.dynamodb();

    let report = SpaceReport::get(dynamo, &space_partition, Some(sk.clone()))
        .await
        .map_err(|e| {
            crate::error!("failed to load report: {e:?}");
            SpaceReportError::ReportLoadFailed
        })?;
    let report = report.ok_or(SpaceReportError::ReportNotFound)?;

    let id = match &report.sk {
        EntityType::SpaceReport(id) => id.clone(),
        _ => String::new(),
    };

    let author = SpaceCommon::get(dynamo, &space_partition, Some(EntityType::SpaceCommon))
        .await
        .ok()
        .flatten()
        .map(|s| s.author_display_name)
        .unwrap_or_default();

    Ok(GetReportResponse {
        id,
        status: report.status,
        title: report.title,
        description: report.description,
        html_contents: report.html_contents,
        created_at: report.created_at,
        updated_at: report.updated_at,
        blocks: report.blocks,
        author,
    })
}
