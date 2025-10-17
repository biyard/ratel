use crate::types::{space_file_feature_type::SpaceFileFeatureType, *};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, DynamoEntity)]
pub struct SpaceFile {
    pub pk: Partition,
    pub sk: EntityType,

    pub feature_type: SpaceFileFeatureType,
    pub files: Vec<File>,
}

impl SpaceFile {
    pub fn new(pk: Partition, feature_type: SpaceFileFeatureType, files: Vec<File>) -> Self {
        Self {
            pk,
            sk: EntityType::SpaceFile(feature_type.to_string()),

            feature_type,
            files,
        }
    }
}
