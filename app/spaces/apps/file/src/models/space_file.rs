use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default, PartialEq)]
pub struct SpaceFile {
    pub pk: Partition,
    pub sk: EntityType,

    pub files: Vec<File>,
}

impl SpaceFile {
    pub fn new(space_pk: Partition, files: Vec<File>) -> Self {
        Self {
            pk: space_pk,
            sk: EntityType::SpaceFile,
            files,
        }
    }

    pub fn keys(space_pk: &Partition) -> (Partition, EntityType) {
        (space_pk.clone(), EntityType::SpaceFile)
    }
}
