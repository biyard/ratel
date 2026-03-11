use crate::types::*;
use crate::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize, DynamoEntity, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct Category {
    pub pk: Partition,
    pub sk: EntityType,

    pub name: String,
    pub created_at: i64,
}

#[cfg(feature = "server")]
impl Category {
    pub fn new(name: String) -> Self {
        let created_at = chrono::Utc::now().timestamp_millis();
        Self {
            pk: Partition::Category,
            sk: EntityType::Category(name.clone()),
            name,
            created_at,
        }
    }

    pub async fn get_or_create_by_name(
        cli: &aws_sdk_dynamodb::Client,
        name: String,
    ) -> Result<Self> {
        let category = Self::new(name.clone());

        match category.create(cli).await {
            Ok(_) => Ok(category),
            Err(Error::Aws(crate::common::utils::aws::error::AwsError::DynamoDb(
                aws_sdk_dynamodb::Error::ConditionalCheckFailedException(_),
            ))) => Self::get(cli, Partition::Category, Some(EntityType::Category(name)))
                .await?
                .ok_or_else(|| Error::NotFound("category".to_string())),
            Err(e) => Err(e),
        }
    }


}

#[cfg(all(test, feature = "server"))]
mod tests {
    use super::*;
    use aws_sdk_dynamodb::{
        Client, Config,
        config::{Credentials, Region},
    };
    use crate::common::config::server::dynamodb::DynamoConfig;
    use crate::common::aws_config::AwsConfig;

    async fn setup_ddb() -> Client {
        let dynamo_config = DynamoConfig::default();
        let aws_config = AwsConfig::default();

        let mut builder = Config::builder()
            .region(Region::new(aws_config.region))
            .behavior_version_latest()
            .credentials_provider(Credentials::new(
                aws_config.access_key_id,
                aws_config.secret_access_key,
                None,
                None,
                "loaded-from-config",
            ));

        if let Some(ep) = dynamo_config.endpoint {
            builder = builder.endpoint_url(ep);
        }

        Client::from_conf(builder.build())
    }

    #[tokio::test]
    async fn test_upsert_category() {
        let cli = setup_ddb().await;
        let result = Category::get_or_create_by_name(&cli, "Policy".to_string()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "Policy");
    }

    #[tokio::test]
    async fn test_upsert_is_idempotent() {
        let cli = setup_ddb().await;
        let a = Category::get_or_create_by_name(&cli, "Idempotent".to_string()).await.unwrap();
        let b = Category::get_or_create_by_name(&cli, "Idempotent".to_string()).await.unwrap();
        assert_eq!(a.created_at, b.created_at);
    }

    #[tokio::test]
    async fn test_upsert_empty_name_is_handled_by_controller() {
        let cli = setup_ddb().await;
        // 빈 이름 검증은 controller에서 담당하므로, 모델 레벨에서는 저장 가능
        let result = Category::get_or_create_by_name(&cli, "".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_find_all_contains_created_category() {
        let cli = setup_ddb().await;
        let name = format!("TestCat-{}", chrono::Utc::now().timestamp_millis());
        Category::get_or_create_by_name(&cli, name.clone()).await.unwrap();

        let (all, _) = Category::query(&cli, Partition::Category, Category::opt().limit(100)).await.unwrap();
        assert!(all.iter().any(|c| c.name == name));
    }
}