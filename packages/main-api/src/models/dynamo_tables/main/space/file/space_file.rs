use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct SpaceFile {
    pub pk: Partition,
    pub sk: EntityType,

    pub name: String,
    pub size: String,
    pub ext: FileExtension,
    pub url: Option<String>,
}

#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, JsonSchema, PartialEq, Eq, Default,
)]
pub enum FileExtension {
    #[default]
    JPG = 1,
    PNG = 2,
    PDF = 3,
    ZIP = 4,
    WORD = 5,
    PPTX = 6,
    EXCEL = 7,
    MP4 = 8,
    MOV = 9,
}

impl SpaceFile {
    pub fn new(
        pk: Partition,
        name: String,
        size: String,
        ext: FileExtension,
        url: Option<String>,
    ) -> crate::Result<Self> {
        if !matches!(pk, Partition::Space(_)) {
            return Err(crate::Error::InvalidPartitionKey(
                "Any Space must be under Space partition".to_string(),
            ));
        }

        Ok(Self {
            pk,
            sk: EntityType::SpaceFile,
            name,
            size,
            ext,
            url,
        })
    }
}
