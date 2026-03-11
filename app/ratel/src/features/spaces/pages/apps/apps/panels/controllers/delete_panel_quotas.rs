use crate::features::spaces::pages::apps::apps::panels::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DeletePanelKey {
    pub panel_id: SpacePanelAttributeEntityType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DeletePanelQuotaRequest {
    pub keys: Vec<DeletePanelKey>,
}

#[delete("/api/spaces/{space_id}/panels", role: SpaceUserRole)]
pub async fn delete_panel_quotas(
    space_id: SpacePartition,
    req: DeletePanelQuotaRequest,
) -> crate::common::Result<bool> {
    SpacePanelQuota::can_edit(role)?;

    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.into();
    let panel_pk = CompositePartition(space_pk, Partition::PanelAttribute);
    let items = req
        .keys
        .into_iter()
        .map(|key| {
            let panel_sk: EntityType = key.panel_id.into();
            SpacePanelQuota::delete_transact_write_item(panel_pk.clone(), panel_sk)
        })
        .collect();

    crate::transact_write_items!(cli, items)
        .map_err(|e| Error::InternalServerError(format!("Failed to delete panel quotas: {e}")))?;

    Ok(true)
}
