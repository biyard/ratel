use common::SpacePartition;
use dioxus::prelude::*;

#[get("/api/spaces/:space_id/dashboard-extensions", _space: common::models::space::SpaceCommon)]
pub async fn fetch_dashboard_extensions(
    space_id: SpacePartition,
) -> Result<Vec<space_common::types::dashboard::DashboardExtension>, ServerFnError> {
    use common::Partition;
    use space_common::models::dashboard_extension::DashboardExtensionEntity;
    use space_common::types::dashboard::*;

    let sk_prefix = common::EntityType::SpaceDashboardExtension(String::new()).to_string();

    let cli = crate::config::get().dynamodb();
    let space_pk: Partition = space_id.clone().into();
    let (items, _) = DashboardExtensionEntity::query(
        cli,
        space_pk,
        DashboardExtensionEntity::opt_all().sk(sk_prefix),
    )
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to query dashboard extensions: {e}")))?;

    let mut extensions: Vec<DashboardExtension> = items
        .into_iter()
        .filter_map(|item| serde_json::from_str::<DashboardExtension>(&item.data).ok())
        .collect();

    // Ensure TabChart always exists (lazy initialization)
    if !extensions.iter().any(|e| e.id == EXT_ID_TAB_CHART) {
        let tab_chart = DashboardComponentData::TabChart(TabChartData {
            icon: DashboardIcon::Participants,
            main_value: "0".to_string(),
            tabs: vec![],
        });
        let space_pk_ref: Partition = space_id.clone().into();
        let _ = DashboardExtensionEntity::upsert_extension(&space_pk_ref, EXT_ID_TAB_CHART, &tab_chart).await;
        extensions.push(DashboardExtension {
            id: EXT_ID_TAB_CHART.to_string(),
            data: tab_chart,
        });
    }

    extensions.sort_by_key(|ext| ext.order());
    Ok(extensions)
}
