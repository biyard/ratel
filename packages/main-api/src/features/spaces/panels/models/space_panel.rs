use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, JsonSchema, Default)]
pub struct SpacePanel {
    pub pk: Partition,
    pub sk: EntityType,

    pub name: String,
    #[schemars(description = "space total quotas")]
    pub quotas: i64,
    #[schemars(description = "space panel participants")]
    pub participants: i64,
    pub attributes: Vec<Attribute>,
}

impl SpacePanel {
    pub fn new(pk: Partition, name: String, quotas: i64, attributes: Vec<Attribute>) -> Self {
        let uid = uuid::Uuid::new_v4().to_string();

        Self {
            pk,
            sk: EntityType::SpacePanel(uid),
            name,
            quotas,
            participants: 0,
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
