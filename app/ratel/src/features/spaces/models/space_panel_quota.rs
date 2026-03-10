use crate::features::spaces::models::PanelAttribute;
use crate::features::spaces::models::PanelAttributeWithQuota;
use crate::features::spaces::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(DynamoEntity))]
pub struct SpacePanelQuota {
    pub pk: CompositePartition,
    pub sk: EntityType,
    pub quotas: i64,
    pub remains: i64,
    #[serde(default)]
    pub attributes: PanelAttribute,
}

impl SpacePanelQuota {
    pub fn can_view(_role: SpaceUserRole) -> crate::common::Result<()> {
        Ok(())
    }

    pub fn can_edit(role: SpaceUserRole) -> crate::common::Result<()> {
        match role {
            SpaceUserRole::Creator => Ok(()),
            _ => Err(Error::NoPermission),
        }
    }

    pub fn new(
        space_pk: Partition,
        attribute_label: String,
        attribute_value: String,
        quotas: i64,
        attributes: PanelAttribute,
    ) -> Self {
        Self {
            pk: CompositePartition(space_pk, Partition::PanelAttribute),
            sk: EntityType::SpacePanelAttribute(attribute_label, attribute_value),
            quotas,
            remains: quotas,
            attributes,
        }
    }

    pub fn keys(space_pk: &Partition, panel_pk: &Partition) -> (Partition, EntityType) {
        let panel_id = match panel_pk {
            Partition::Panel(v) => v.to_string(),
            _ => "".to_string(),
        };

        (space_pk.clone(), EntityType::SpacePanel(panel_id))
    }
}

impl From<(Partition, PanelAttributeWithQuota)> for SpacePanelQuota {
    fn from((space_pk, attr_with_quota): (Partition, PanelAttributeWithQuota)) -> Self {
        let quota = attr_with_quota.quota();
        let attr: PanelAttribute = attr_with_quota.into();
        let value = attr.to_value().unwrap_or_default();

        Self::new(space_pk, attr.to_key(), value, quota, attr)
    }
}
