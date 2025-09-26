use crate::types::*;

use bdk::prelude::*;
use dto::File;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct DeliberationSpaceSummary {
    pub pk: Partition,
    pub sk: EntityType,

    pub html_contents: String,
    // INFO: Serialize multiple file vectors and save them in String format
    pub file: String,
}

impl DeliberationSpaceSummary {
    pub fn new(pk: Partition, html_contents: String, files: Vec<File>) -> Self {
        let sk = EntityType::DeliberationSpaceSummary;

        let file = Self::serialize_files(&files);

        Self {
            pk,
            sk,
            html_contents,
            file,
        }
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

#[derive(Debug, Clone, Default, serde::Serialize, schemars::JsonSchema)]
pub struct DeliberationSummaryResponse {
    pub html_contents: String,
    pub files: Vec<File>,
}

impl From<DeliberationSpaceSummary> for DeliberationSummaryResponse {
    fn from(deliberation_summary: DeliberationSpaceSummary) -> Self {
        Self {
            html_contents: deliberation_summary.clone().html_contents,
            files: deliberation_summary.clone().files(),
        }
    }
}
