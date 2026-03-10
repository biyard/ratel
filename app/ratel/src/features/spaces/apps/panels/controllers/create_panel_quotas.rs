use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreatePanelQuotaRequest {
    pub attributes: Vec<PanelAttributeWithQuota>,
}

#[post("/api/spaces/{space_id}/panels", role: SpaceUserRole)]
pub async fn create_panel_quotas(
    space_id: SpacePartition,
    req: CreatePanelQuotaRequest,
) -> crate::common::Result<Vec<SpacePanelQuotaResponse>> {
    SpacePanelQuota::can_edit(role)?;

    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.into();

    let panels: Vec<SpacePanelQuota> = req
        .attributes
        .into_iter()
        .map(|attribute| (space_pk.clone(), attribute).into())
        .collect();

    let items = panels
        .iter()
        .cloned()
        .map(|panel| panel.create_transact_write_item())
        .collect();

    crate::transact_write_items!(cli, items)
        .map_err(|e| Error::InternalServerError(format!("Failed to create panel quotas: {e}")))?;

    Ok(panels.into_iter().map(Into::into).collect())
}
