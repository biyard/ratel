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

    pub async fn upsert_by_name(
        cli: &aws_sdk_dynamodb::Client,
        name: String,
    ) -> Result<Self> {
        if let Some(existing) =
            Self::get(cli, Partition::Category, Some(EntityType::Category(name.clone()))).await?
        {
            return Ok(existing);
        }

        let category = Self::new(name);
        category.create(cli).await?;
        Ok(category)
    }

    pub async fn find_all(cli: &aws_sdk_dynamodb::Client) -> Result<Vec<Self>> {
        use aws_sdk_dynamodb::types::AttributeValue;

        let resp = cli
            .query()
            .table_name(Self::table_name())
            .key_condition_expression("#pk = :pk")
            .expression_attribute_names("#pk", "pk")
            .expression_attribute_values(":pk", AttributeValue::S(Partition::Category.to_string()))
            .send()
            .await
            .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

        let items: Vec<Self> = resp
            .items
            .unwrap_or_default()
            .into_iter()
            .map(serde_dynamo::from_item)
            .collect::<std::result::Result<_, _>>()?;

        Ok(items)
    }
}

#[cfg(all(test, feature = "server"))]
mod tests {
    use super::*;
    use aws_sdk_dynamodb::{
        Client, Config,
        config::{Credentials, Region},
    };

    async fn setup_ddb() -> Client {
        let config = Config::builder()
            .region(Region::new("us-east-1"))
            .behavior_version_latest()
            .credentials_provider(Credentials::new("test", "test", None, None, "test"))
            .endpoint_url("http://localhost:4566")
            .build();
        Client::from_conf(config)
    }

    #[tokio::test]
    async fn test_upsert_category() {
        let cli = setup_ddb().await;
        let result = Category::upsert_by_name(&cli, "Policy".to_string()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "Policy");
    }

    #[tokio::test]
    async fn test_upsert_is_idempotent() {
        let cli = setup_ddb().await;
        let a = Category::upsert_by_name(&cli, "Idempotent".to_string()).await.unwrap();
        let b = Category::upsert_by_name(&cli, "Idempotent".to_string()).await.unwrap();
        assert_eq!(a.created_at, b.created_at);
    }

    #[tokio::test]
    async fn test_upsert_empty_name_is_handled_by_controller() {
        let cli = setup_ddb().await;
        // 빈 이름 검증은 controller에서 담당하므로, 모델 레벨에서는 저장 가능
        let result = Category::upsert_by_name(&cli, "".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_find_all_contains_created_category() {
        let cli = setup_ddb().await;
        let name = format!("TestCat-{}", chrono::Utc::now().timestamp_millis());
        Category::upsert_by_name(&cli, name.clone()).await.unwrap();

        let all = Category::find_all(&cli).await.unwrap();
        assert!(all.iter().any(|c| c.name == name));
    }
}