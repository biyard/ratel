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

    /// Add files to SpaceFile, avoiding duplicates by URL
    pub async fn add_files(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: Partition,
        new_files: Vec<File>,
    ) -> crate::Result<Self> {
        let (pk, sk) = Self::keys(&space_pk);

        // Get existing SpaceFile or create new one
        let mut space_file = Self::get(cli, &pk, Some(sk.clone()))
            .await?
            .unwrap_or_else(|| Self::new(space_pk.clone(), vec![]));

        let existing_urls: std::collections::HashSet<String> = space_file
            .files
            .iter()
            .filter_map(|f| f.url.as_ref().cloned())
            .collect();

        // Add new files that don't already exist
        for file in new_files {
            if let Some(url) = &file.url {
                if !existing_urls.contains(url) {
                    space_file.files.push(file);
                }
            }
        }

        space_file.upsert(cli).await?;

        Ok(space_file)
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
