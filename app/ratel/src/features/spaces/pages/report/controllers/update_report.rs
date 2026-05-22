use crate::features::spaces::pages::report::models::SpaceReport;
use crate::features::spaces::pages::report::types::{ReportBlock, SpaceReportError};
use crate::features::spaces::pages::report::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateReportRequest {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub status: Option<ReportStatus>,
    #[serde(default)]
    pub blocks: Option<Vec<ReportBlock>>,
    #[serde(default)]
    pub html_contents: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateReportResponse {
    pub id: String,
    pub updated_at: i64,
}

#[patch("/v3/spaces/{space_pk}/reports/{report_id}", role: SpaceUserRole)]
pub async fn update_report(
    space_pk: SpacePartition,
    report_id: SpaceReportEntityType,
    req: UpdateReportRequest,
) -> Result<UpdateReportResponse> {
    SpaceReport::can_edit(role)?;
    let space_partition: Partition = space_pk.into();
    let sk: EntityType = report_id.into();

    let conf = ServerConfig::default();
    let dynamo = conf.dynamodb();

    let now = crate::common::utils::time::get_now_timestamp_millis();
    let mut updater =
        SpaceReport::updater(space_partition.clone(), sk.clone()).with_updated_at(now);
    if let Some(title) = req.title {
        updater = updater.with_title(title);
    }
    if let Some(description) = req.description {
        updater = updater.with_description(description);
    }
    if let Some(status) = req.status {
        updater = updater.with_status(status);
    }
    if let Some(blocks) = req.blocks {
        updater = updater.with_blocks(blocks);
    }
    if let Some(html) = req.html_contents {
        updater = updater.with_html_contents(html);
    }

    updater.execute(dynamo).await.map_err(|e| {
        crate::error!("failed to update report: {e:?}");
        SpaceReportError::ReportUpdateFailed
    })?;

    let id = match &sk {
        EntityType::SpaceReport(id) => id.clone(),
        _ => String::new(),
    };

    Ok(UpdateReportResponse {
        id,
        updated_at: now,
    })
}
