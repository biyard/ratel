use crate::features::spaces::pages::apps::apps::panels::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdatePanelQuotaRequest {
    pub panel_id: SpacePanelAttributeEntityType,
    pub quota: i64,
}

#[patch("/api/spaces/{space_id}/panels", role: SpaceUserRole)]
pub async fn update_panel_quota(
    space_id: SpacePartition,
    req: UpdatePanelQuotaRequest,
) -> crate::common::Result<SpacePanelQuotaResponse> {
    SpacePanelQuota::can_edit(role)?;

    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.into();
    let panel_sk: EntityType = req.panel_id.into();

    let panel = SpacePanelQuota::updater(
        CompositePartition(space_pk, Partition::PanelAttribute),
        panel_sk,
    )
    .with_quotas(req.quota)
    .with_remains(req.quota)
    .execute(cli)
    .await?;

    Ok(panel.into())
}
