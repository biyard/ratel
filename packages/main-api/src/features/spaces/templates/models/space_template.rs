// use crate::types::*;
// use bdk::prelude::*;

// #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, JsonSchema, Default)]
// pub struct SpaceTemplate {
//     pub pk: Partition,
//     pub sk: EntityType,

//     pub template_name: String,
//     pub is_registered: bool,
// }

// impl SpaceTemplate {
//     pub fn new(template_name: String) -> Self {
//         Self {
//             pk: Partition::SpaceTemplate,
//             sk: EntityType::SpaceTemplate(template_name.clone()),
//             template_name,
//             is_registered: true,
//         }
//     }
// }
