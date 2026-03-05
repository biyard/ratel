use crate::*;

#[get("/api/spaces/{space_id}/dashboard-extensions", _space: common::models::space::SpaceCommon)]
pub async fn list_dashboard_extensions_handler(
    space_id: SpacePartition,
) -> std::result::Result<Vec<DashboardComponentData>, ServerFnError> {
    use common::Partition;
    use space_common::models::dashboard::aggregate::DashboardAggregate;
    use space_common::models::dashboard::recalculate::build_dashboard_components;

    let cfg = common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let space_pk: Partition = space_id.into();
    let agg = DashboardAggregate::get_or_default(cli, &space_pk)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to get aggregate: {e}")))?;

    Ok(build_dashboard_components(&agg, _space.participants))
}
