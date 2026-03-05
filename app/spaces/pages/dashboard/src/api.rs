use common::SpacePartition;
use dioxus::prelude::*;

#[get("/api/spaces/:space_id/dashboard-extensions", space: common::models::space::SpaceCommon)]
pub async fn fetch_dashboard_extensions(
    space_id: SpacePartition,
) -> Result<Vec<space_common::types::dashboard::DashboardComponentData>, ServerFnError> {
    use common::Partition;
    use space_common::models::dashboard::aggregate::DashboardAggregate;
    use space_common::models::dashboard::recalculate::build_dashboard_components;

    let cli = crate::config::get().dynamodb();
    let space_pk: Partition = space_id.into();

    let agg = DashboardAggregate::get_or_default(cli, &space_pk)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to get aggregate: {e}")))?;

    Ok(build_dashboard_components(&agg, space.participants))
}
