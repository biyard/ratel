use crate::types::FileLinkTarget;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default, PartialEq)]
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
    pub async fn list_by_space(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
    ) -> Result<Vec<Self>> {
        let mut prefix = EntityType::FileLink(String::default()).to_string();
        prefix.retain(|c| c != '#');

        let (file_links, _bookmark) = Self::query_begins_with_sk(cli, space_pk, prefix).await?;
        Ok(file_links)
    }
}
