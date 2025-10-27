use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, DynamoEntity)]
pub struct SpaceFile {
    pub pk: Partition,
    pub sk: EntityType,

    pub files: Vec<File>,
}

impl SpaceFile {
    pub fn new(pk: Partition, files: Vec<File>) -> Self {
        Self {
            pk,
            sk: EntityType::SpaceFile,

            files,
        }
    }

    pub fn keys(space_pk: &Partition) -> (Partition, EntityType) {
        (space_pk.clone(), EntityType::SpaceFile)
    }

    pub async fn delete_one(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
    ) -> crate::Result<()> {
        let file = SpaceFile::get(&cli, space_pk.clone(), Some(EntityType::SpaceFile)).await?;

        if file.is_none() {
            return Ok(());
        }

        SpaceFile::delete(&cli, &space_pk.clone(), Some(EntityType::SpaceFile)).await?;

        Ok(())
    }
}
