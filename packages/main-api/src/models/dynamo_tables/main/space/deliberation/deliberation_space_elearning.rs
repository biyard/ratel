use crate::types::*;

use bdk::prelude::*;
use dto::File;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct DeliberationSpaceElearning {
    pub pk: Partition,
    pub sk: EntityType,

    // INFO: Serialize multiple file vectors and save them in String format
    pub file: String,
}

impl DeliberationSpaceElearning {
    pub fn new(pk: Partition, files: Vec<File>) -> Self {
        let sk = EntityType::DeliberationSpaceElearning;

        let file = Self::serialize_files(&files);

        Self { pk, sk, file }
    }

    pub fn files(&self) -> Vec<File> {
        serde_json::from_str(&self.file).unwrap_or_default()
    }

    pub fn set_files(&mut self, files: Vec<File>) {
        self.file = serde_json::to_string(&files).unwrap_or_else(|_| "[]".to_string());
    }

    #[inline]
    fn serialize_files(files: &[File]) -> String {
        serde_json::to_string(files).unwrap_or_else(|_| "[]".to_string())
    }
}
