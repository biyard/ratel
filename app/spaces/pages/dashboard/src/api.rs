use common::SpacePartition;
use dioxus::prelude::*;

#[get("/api/spaces/:space_id/dashboard-extensions", space: ratel_auth::space::SpaceCommon)]
pub async fn fetch_dashboard_extensions(
    space_id: SpacePartition,
) -> Result<Vec<crate::types::DashboardExtension>, ServerFnError> {
    use common::Partition;

    let _space = space;
    let sk_prefix = common::EntityType::SpaceDashboardExtension(String::new()).to_string();

    let cli = crate::config::get().dynamodb();
    let space_pk: Partition = space_id.into();
    let (items, _) = crate::models::DashboardExtensionEntity::query(
        cli,
        space_pk,
        crate::models::DashboardExtensionEntity::opt_all().sk(sk_prefix),
    )
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to query dashboard extensions: {e}")))?;

    let mut extensions: Vec<crate::types::DashboardExtension> = items
        .into_iter()
        .map(|item| {
            serde_json::from_str::<crate::types::DashboardExtension>(&item.data).map_err(|e| {
                ServerFnError::new(format!("Failed to parse dashboard extension data: {e}"))
            })
        })
        .collect::<std::result::Result<_, _>>()?;

    extensions.sort_by_key(|ext: &crate::types::DashboardExtension| ext.order());
    Ok(extensions)
}
