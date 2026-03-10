use super::*;

#[get("/api/spaces/{space_id}/panels", role: SpaceUserRole)]
pub async fn list_panels(
    space_id: SpacePartition,
) -> crate::common::Result<Vec<SpacePanelQuotaResponse>> {
    SpacePanelQuota::can_view(role)?;

    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.into();

    let (panels, _) = SpacePanelQuota::query(
        cli,
        CompositePartition(space_pk, Partition::PanelAttribute),
        SpacePanelQuota::opt_all()
            .sk("SPACE_PANEL_ATTRIBUTE#".to_string())
            .scan_index_forward(true),
    )
    .await?;

    Ok(panels
        .into_iter()
        .filter(|panel| !matches!(panel.attributes, PanelAttribute::None))
        .map(Into::into)
        .collect())
}
