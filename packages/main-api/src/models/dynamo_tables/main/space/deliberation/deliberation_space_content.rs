use crate::types::*;

use crate::types::File;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct DeliberationSpaceContent {
    pub pk: Partition,
    pub sk: EntityType,

    pub html_contents: String,
    pub files: Vec<File>,
}

impl DeliberationSpaceContent {
    pub fn new(
        pk: Partition,
        entity_type: EntityType,
        html_contents: String,
        files: Vec<File>,
    ) -> Self {
        let sk = entity_type;

        Self {
            pk,
            sk,
            html_contents,
            files,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct DeliberationContentResponse {
    pub html_contents: String,
    pub files: Vec<File>,
}

impl From<DeliberationSpaceContent> for DeliberationContentResponse {
    fn from(deliberation_content: DeliberationSpaceContent) -> Self {
        Self {
            html_contents: deliberation_content.clone().html_contents,
            files: deliberation_content.clone().files,
        }
    }
}
