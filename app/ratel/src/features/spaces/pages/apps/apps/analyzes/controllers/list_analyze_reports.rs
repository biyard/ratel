use crate::common::ListResponse;
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::features::spaces::pages::apps::models::SpaceApp;

#[get("/api/spaces/{space_id}/apps/analyzes/reports?bookmark", role: SpaceUserRole)]
pub async fn list_analyze_reports(
    space_id: SpacePartition,
    bookmark: Option<String>,
) -> Result<ListResponse<AnalyzeReport>> {
    SpaceApp::can_edit(role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.into();

    let mut opt = SpaceAnalyzeReport::opt()
        .sk(EntityType::SpaceAnalyzeReport(String::default()).to_string())
        .scan_index_forward(false)
        .limit(20);
    if let Some(bookmark) = bookmark {
        opt = opt.bookmark(bookmark);
    }

    let (reports, bookmark) = SpaceAnalyzeReport::query(cli, space_pk, opt).await?;

    let items = reports
        .into_iter()
        .map(|report| {
            let id = match &report.sk {
                EntityType::SpaceAnalyzeReport(id) => id.clone(),
                _ => String::new(),
            };
            AnalyzeReport {
                id,
                name: report.name,
                status: report.status,
                created_at: report.created_at,
                filters: report.filters,
            }
        })
        .collect();

    Ok(ListResponse { items, bookmark })
}
