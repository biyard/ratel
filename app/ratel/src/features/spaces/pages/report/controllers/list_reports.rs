use crate::common::ListResponse;
use crate::features::spaces::pages::report::models::SpaceReport;
use crate::features::spaces::pages::report::types::SpaceReportError;
use crate::features::spaces::pages::report::*;

#[get("/v3/spaces/{space_pk}/reports?bookmark&status", role: SpaceUserRole)]
pub async fn list_reports(
    space_pk: SpacePartition,
    bookmark: Option<String>,
    status: Option<ReportStatus>,
) -> Result<ListResponse<ReportListItem>> {
    SpaceReport::can_view(role)?;

    // Members can only ever see Published reports — any other request
    // (unfiltered or `status=Draft`) is silently coerced to the
    // Published filter so drafts can't leak through the API. Admins
    // keep the full filter spectrum (None / Draft / Published) for
    // their list page chips.
    let effective_status = if SpaceReport::can_edit(role).is_ok() {
        status
    } else {
        Some(ReportStatus::Published)
    };

    let space_partition: Partition = space_pk.into();
    let conf = ServerConfig::default();
    let dynamo = conf.dynamodb();

    let (reports, next_bookmark) = match effective_status {
        Some(s) => {
            let mut opt = SpaceReport::opt()
                .sk(s.to_string())
                .scan_index_forward(false)
                .limit(20);
            if let Some(bm) = bookmark {
                opt = opt.bookmark(bm);
            }
            SpaceReport::find_by_status(dynamo, &space_partition, opt)
                .await
                .map_err(|e| {
                    crate::error!("failed to list reports by status {s}: {e:?}");
                    SpaceReportError::ReportListFailed
                })?
        }
        None => {
            let mut opt = SpaceReport::opt()
                .sk(EntityType::SpaceReport(String::default()).to_string())
                .scan_index_forward(false)
                .limit(20);
            if let Some(bm) = bookmark {
                opt = opt.bookmark(bm);
            }
            SpaceReport::query(dynamo, space_partition, opt)
                .await
                .map_err(|e| {
                    crate::error!("failed to list reports: {e:?}");
                    SpaceReportError::ReportListFailed
                })?
        }
    };

    let items: Vec<ReportListItem> = reports
        .into_iter()
        .map(|r| ReportListItem {
            id: report_id_from_sk(&r.sk),
            status: r.status,
            title: r.title,
            description: r.description,
            created_at: r.created_at,
        })
        .collect();

    Ok(ListResponse {
        items,
        bookmark: next_bookmark,
    })
}

fn report_id_from_sk(sk: &EntityType) -> String {
    match sk {
        EntityType::SpaceReport(id) => id.clone(),
        _ => String::new(),
    }
}
