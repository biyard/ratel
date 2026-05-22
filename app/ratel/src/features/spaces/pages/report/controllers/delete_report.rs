use crate::features::spaces::pages::report::models::SpaceReport;
use crate::features::spaces::pages::report::types::SpaceReportError;
use crate::features::spaces::pages::report::*;

/// `Default` is required by the `test_*!` macros — they fall back to
/// `<Resp>::default()` when a non-2xx response body fails to parse
/// into the typed shape.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteReportResponse {
    pub id: String,
}

/// Remove one saved report from a space. Auth: `SpaceReport::can_delete`
/// — only the space Creator (or a team Admin/Owner of a team-owned
/// space, which the extractor collapses to `Creator`) may delete.
///
/// The endpoint is idempotent — deleting a missing row is treated as
/// success because the UI flow happily fires the request again on
/// retries / double-clicks and we don't want a 4xx to leak through
/// the confirm modal.
#[delete("/v3/spaces/{space_pk}/reports/{report_id}", role: SpaceUserRole)]
pub async fn delete_report(
    space_pk: SpacePartition,
    report_id: SpaceReportEntityType,
) -> Result<DeleteReportResponse> {
    SpaceReport::can_delete(role)?;
    let space_partition: Partition = space_pk.into();
    let sk: EntityType = report_id.into();

    let conf = ServerConfig::default();
    let dynamo = conf.dynamodb();

    let id = match &sk {
        EntityType::SpaceReport(id) => id.clone(),
        _ => String::new(),
    };

    SpaceReport::delete(dynamo, &space_partition, Some(sk))
        .await
        .map_err(|e| {
            crate::error!("failed to delete report: {e:?}");
            SpaceReportError::ReportDeleteFailed
        })?;

    Ok(DeleteReportResponse { id })
}
