use crate::types::*;
use bdk::prelude::*;

use crate::features::spaces::files::FileLinkTarget;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, DynamoEntity)]
pub struct FileLink {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "FILE_URL", index = "gsi1", pk, name = "find_by_file_url")]
    pub file_url: String,

    pub link_targets: Vec<FileLinkTarget>,

    #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,
    pub updated_at: i64,
}

impl FileLink {
    pub fn new(space_pk: Partition, file_url: String, link_targets: Vec<FileLinkTarget>) -> Self {
        let now = chrono::Utc::now().timestamp_micros();
        let file_id = uuid::Uuid::new_v4().to_string();

        Self {
            pk: space_pk,
            sk: EntityType::FileLink(file_id),
            file_url,
            link_targets,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn keys(space_pk: &Partition, file_id: &str) -> (Partition, EntityType) {
        (space_pk.clone(), EntityType::FileLink(file_id.to_string()))
    }

    pub async fn add_link_target(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: Partition,
        file_url: String,
        target: FileLinkTarget,
    ) -> Result<Self, crate::Error> {
        let existing = Self::find_by_url(cli, &space_pk, &file_url).await?;

        if let Some(mut file_link) = existing {
            if !file_link.link_targets.contains(&target) {
                file_link.link_targets.push(target.clone());
                let now = chrono::Utc::now().timestamp_micros();

                // Use DynamoDB updater
                Self::updater(&file_link.pk, &file_link.sk)
                    .with_link_targets(file_link.link_targets.clone())
                    .with_updated_at(now)
                    .execute(cli)
                    .await?;

                file_link.updated_at = now;
            }
            Ok(file_link)
        } else {
            // Create new file link
            let file_link = Self::new(space_pk, file_url, vec![target]);
            file_link.create(cli).await?;
            Ok(file_link)
        }
    }

    /// Remove a link target from a file
    pub async fn remove_link_target(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        file_url: &str,
        target: &FileLinkTarget,
    ) -> Result<Option<Self>, crate::Error> {
        let existing = Self::find_by_url(cli, space_pk, file_url).await?;

        if let Some(mut file_link) = existing {
            file_link.link_targets.retain(|t| t != target);
            let now = chrono::Utc::now().timestamp_micros();

            // If no more targets, delete the file link
            if file_link.link_targets.is_empty() {
                Self::delete(cli, &file_link.pk, Some(file_link.sk.clone())).await?;
                return Ok(None);
            }

            // Use DynamoDB updater
            Self::updater(&file_link.pk, &file_link.sk)
                .with_link_targets(file_link.link_targets.clone())
                .with_updated_at(now)
                .execute(cli)
                .await?;

            file_link.updated_at = now;
            Ok(Some(file_link))
        } else {
            Ok(None)
        }
    }

    /// Find a file link by URL within a space using GSI
    pub async fn find_by_url(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        file_url: &str,
    ) -> Result<Option<Self>, crate::Error> {
        let prefixed_url = format!("FILE_URL#{}", file_url);

        let (items, _bookmark) =
            Self::find_by_file_url(cli, prefixed_url, FileLinkQueryOption::default()).await?;

        // Find the one matching our space
        for item in items {
            if item.pk == *space_pk {
                return Ok(Some(item));
            }
        }

        Ok(None)
    }

    /// Get all file links for a space
    pub async fn list_by_space(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
    ) -> Result<Vec<Self>, crate::Error> {
        let table_name = Self::table_name();

        let pk_value = serde_dynamo::to_attribute_value(space_pk).map_err(|e| {
            crate::Error::InternalServerError(format!("Serialization failed: {}", e))
        })?;

        let sk_prefix_value = serde_dynamo::to_attribute_value(
            &EntityType::FileLink(String::new()),
        )
        .map_err(|e| crate::Error::InternalServerError(format!("Serialization failed: {}", e)))?;

        let result = cli
            .query()
            .table_name(table_name)
            .key_condition_expression("pk = :pk AND begins_with(sk, :sk_prefix)")
            .expression_attribute_values(":pk", pk_value)
            .expression_attribute_values(":sk_prefix", sk_prefix_value)
            .send()
            .await
            .map_err(|e| crate::Error::InternalServerError(format!("Query failed: {}", e)))?;

        if let Some(items) = result.items {
            let file_links: Vec<FileLink> = items
                .into_iter()
                .filter_map(|item| serde_dynamo::from_item(item).ok())
                .collect();
            Ok(file_links)
        } else {
            Ok(vec![])
        }
    }

    /// Get all files linked to a specific target
    pub async fn get_files_by_target(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        target: &FileLinkTarget,
    ) -> Result<Vec<String>, crate::Error> {
        let all_links = Self::list_by_space(cli, space_pk).await?;

        let file_urls: Vec<String> = all_links
            .into_iter()
            .filter(|link| link.link_targets.contains(target))
            .map(|link| link.file_url)
            .collect();

        Ok(file_urls)
    }
}
