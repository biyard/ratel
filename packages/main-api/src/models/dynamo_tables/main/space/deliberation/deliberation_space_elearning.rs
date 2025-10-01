use crate::types::*;

use crate::types::File;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct DeliberationSpaceElearning {
    pub pk: Partition,
    pub sk: EntityType,
    pub files: Vec<File>,
}

impl DeliberationSpaceElearning {
    pub fn new(pk: Partition, files: Vec<File>) -> Self {
        let sk = EntityType::DeliberationSpaceElearning;

        Self { pk, sk, files }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct ElearningResponse {
    pub files: Vec<File>,
}

impl From<DeliberationSpaceElearning> for ElearningResponse {
    fn from(elearning: DeliberationSpaceElearning) -> Self {
        Self {
            files: elearning.clone().files,
        }
    }
}
