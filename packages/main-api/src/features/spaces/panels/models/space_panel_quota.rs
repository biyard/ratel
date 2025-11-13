use crate::*;
use crate::{
    features::spaces::panels::{PanelAttribute, PanelAttributeWithQuota},
    types::*,
};

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    DynamoEntity,
    JsonSchema,
    Default,
    OperationIo,
)]
pub struct SpacePanelQuota {
    pub pk: CompositePartition,
    pub sk: EntityType,

    #[schemars(description = "space total quotas")]
    pub quotas: i64,
    #[schemars(description = "space panel participants")]
    pub remains: i64,
    pub attributes: PanelAttribute,
}

impl SpacePanelQuota {
    pub fn new(
        space_pk: Partition,
        attribute_label: String, // ex) VerifiableAttribute::Gender
        attribute_value: String, // ex) Age
        quotas: i64,
        attributes: PanelAttribute,
    ) -> Self {
        Self {
            pk: CompositePartition(space_pk, Partition::PanelAttribute),
            sk: EntityType::SpacePanelAttribute(
                attribute_label.to_string(),
                attribute_value.to_string(),
            ),
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
