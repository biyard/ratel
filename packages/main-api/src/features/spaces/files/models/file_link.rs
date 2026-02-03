use crate::types::*;
use crate::utils::time::get_now_timestamp_micros;
use bdk::prelude::*;

use crate::features::spaces::files::FileLinkTarget;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, DynamoEntity)]
pub struct FileLink {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "FILE_URL", index = "gsi1", pk, name = "find_by_file_url")]
    pub file_url: String,

    pub link_target: FileLinkTarget,

    #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,
    pub updated_at: i64,
}

impl FileLink {
    pub fn new(space_pk: Partition, file_url: String, link_target: FileLinkTarget) -> Self {
        let now = get_now_timestamp_micros();
        let file_id = uuid::Uuid::new_v4().to_string();

        Self {
            pk: space_pk,
            sk: EntityType::FileLink(file_id),
            file_url,
            link_target,
            created_at: now,
            updated_at: now,
        }
    }

    pub async fn add_link_target(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: Partition,
        file_url: String,
        target: FileLinkTarget,
    ) -> Result<Self, crate::Error> {
        let existing = Self::find_by_url(cli, &space_pk, &file_url).await?;

        if let Some(file_link) = existing {
            if file_link.link_target != target {
                return Err(crate::Error::BadRequest(
                    "File already linked to a different target".to_string(),
                ));
            }
            Ok(file_link)
        } else {
            let file_link = Self::new(space_pk, file_url, target);
            file_link.create(cli).await?;
            Ok(file_link)
        }
    }

    pub async fn add_link_targets_batch(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: Partition,
        file_urls: Vec<String>,
        target: FileLinkTarget,
    ) -> Result<Vec<Self>, crate::Error> {
        if file_urls.is_empty() {
            return Ok(vec![]);
        }

        let mut existing_links = std::collections::HashMap::new();
        for file_url in &file_urls {
            if let Some(link) = Self::find_by_url(cli, &space_pk, file_url).await? {
                existing_links.insert(file_url.clone(), link);
            }
        }

        let mut results = Vec::new();
        let mut new_links = Vec::new();

        for file_url in file_urls {
            if let Some(existing) = existing_links.get(&file_url) {
                if existing.link_target != target {
                    return Err(crate::Error::BadRequest(
                        format!("File {} already linked to a different target", file_url),
                    ));
                }
                results.push(existing.clone());
            } else {
                let new_link = Self::new(space_pk.clone(), file_url, target.clone());
                new_links.push(new_link);
            }
        }

        // Batch create new links using transactions (DynamoDB allows up to 100 items per transaction)
        for chunk in new_links.chunks(100) {
            let transactions: Vec<_> = chunk
                .iter()
                .map(|link| link.create_transact_write_item())
                .collect();

            cli.transact_write_items()
                .set_transact_items(Some(transactions))
                .send()
                .await
                .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

            results.extend_from_slice(chunk);
        }

        Ok(results)
    }

    pub async fn remove_link_target(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        file_url: &str,
        target: &FileLinkTarget,
    ) -> Result<Option<Self>, crate::Error> {
        let existing = Self::find_by_url(cli, space_pk, file_url).await?;

        if let Some(file_link) = existing {
            if &file_link.link_target == target {
                Self::delete(cli, &file_link.pk, Some(file_link.sk.clone())).await?;
                return Ok(None);
            }
            Ok(Some(file_link))
        } else {
            Ok(None)
        }
    }

    pub async fn remove_link_targets_batch(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        file_urls: Vec<String>,
        target: &FileLinkTarget,
    ) -> Result<Vec<Option<Self>>, crate::Error> {
        if file_urls.is_empty() {
            return Ok(vec![]);
        }

        let mut existing_links = std::collections::HashMap::new();
        for file_url in &file_urls {
            if let Some(link) = Self::find_by_url(cli, space_pk, file_url).await? {
                existing_links.insert(file_url.clone(), link);
            }
        }

        let mut results = Vec::new();
        let mut to_delete = Vec::new();

        for file_url in file_urls {
            if let Some(existing) = existing_links.get(&file_url) {
                if &existing.link_target == target {
                    to_delete.push(existing.clone());
                    results.push(None);
                } else {
                    results.push(Some(existing.clone()));
                }
            } else {
                results.push(None);
            }
        }

        // Batch delete using transactions (DynamoDB allows up to 100 items per transaction)
        for chunk in to_delete.chunks(100) {
            let transactions: Vec<_> = chunk
                .iter()
                .map(|link| Self::delete_transact_write_item(link.pk.clone(), link.sk.clone()))
                .collect();

            cli.transact_write_items()
                .set_transact_items(Some(transactions))
                .send()
                .await
                .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;
        }

        Ok(results)
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
        let mut prefix = EntityType::FileLink(String::default()).to_string();
        prefix.retain(|c| c != '#');

        let (file_links, _bookmark) = Self::query_begins_with_sk(cli, space_pk, prefix).await?;
        Ok(file_links)
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
            .filter(|link| &link.link_target == target)
            .map(|link| link.file_url)
            .collect();

        Ok(file_urls)
    }
}
