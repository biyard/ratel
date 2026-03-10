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
    pub attributes_vec: Vec<PanelAttribute>,
    /**
     * @deprecated
     * attributes_vec field instead.
     */
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
            attributes_vec: vec![attributes],
            attributes,
        }
    }

    pub fn new_with_attributes_vec(
        space_pk: Partition,
        quotas: i64,
        attributes_vec: Vec<PanelAttribute>,
    ) -> Self {
        let (attribute_label, attribute_value) = Self::attributes_vec_key(&attributes_vec);
        let attributes = attributes_vec.first().copied().unwrap_or_default();

        Self {
            pk: CompositePartition(space_pk, Partition::PanelAttribute),
            sk: EntityType::SpacePanelAttribute(attribute_label, attribute_value),
            quotas,
            remains: quotas,
            attributes_vec,
            attributes,
        }
    }

    fn attributes_vec_key(attributes_vec: &[PanelAttribute]) -> (String, String) {
        let mut encoded = attributes_vec
            .iter()
            .map(Self::encode_attribute)
            .collect::<Vec<_>>();

        if encoded.is_empty() {
            return ("none".to_string(), String::new());
        }

        let first = encoded.remove(0);
        let mut parts = first.splitn(2, '#');
        let label = parts.next().unwrap_or_default().to_string();
        let mut values = parts
            .next()
            .filter(|value| !value.is_empty())
            .map(|value| vec![value.to_string()])
            .unwrap_or_default();
        values.extend(encoded);

        (label, values.join("_"))
    }

    fn encode_attribute(attribute: &PanelAttribute) -> String {
        let key = attribute.to_key();
        match attribute.to_value() {
            Some(value) if !value.is_empty() => format!("{key}#{value}"),
            _ => key,
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
