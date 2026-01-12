use aws_sdk_dynamodb::types::{
    AttributeDefinition, BillingMode, GlobalSecondaryIndex, KeySchemaElement, KeyType, Projection,
    ProjectionType, ScalarAttributeType,
};
use dto::{Error, Result};

use crate::utils::aws::DynamoClient;

pub struct DynamoTable {
    pub name: String,
    pub pk_name: String,
    pub sk_name: Option<String>,
    pub gsi_configs: Vec<GlobalSecondaryIndexConfig>,
}

pub struct GlobalSecondaryIndexConfig {
    pub index_name: String,
    pub pk_name: String,
    pub sk_name: Option<String>,
}

impl DynamoTable {
    pub fn new(name: &str, pk_name: &str, sk_name: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            pk_name: pk_name.to_string(),
            sk_name: sk_name.map(|s| s.to_string()),
            gsi_configs: Vec::new(),
        }
    }

    pub fn with_gsi(mut self, index_name: &str, pk_name: &str, sk_name: Option<&str>) -> Self {
        self.gsi_configs.push(GlobalSecondaryIndexConfig {
            index_name: index_name.to_string(),
            pk_name: pk_name.to_string(),
            sk_name: sk_name.map(|s| s.to_string()),
        });
        self
    }
}

pub async fn create_dynamo_tables(tables: Vec<DynamoTable>) -> Result<()> {
    let client = DynamoClient::new(None, true); // We'll use the raw client

    for table in tables {
        create_table(&client, &table).await?;
    }

    Ok(())
}

async fn create_table(dynamo_client: &DynamoClient, table_config: &DynamoTable) -> Result<()> {
    // Check if table already exists
    match dynamo_client
        .client
        .describe_table()
        .table_name(&table_config.name)
        .send()
        .await
    {
        Ok(_) => {
            tracing::info!(
                "Table '{}' already exists, skipping creation",
                table_config.name
            );
            return Ok(());
        }
        Err(_) => {
            // Table doesn't exist, proceed with creation
        }
    }

    tracing::info!("Creating DynamoDB table: {}", table_config.name);

    // Build attribute definitions
    let mut attribute_definitions = vec![
        AttributeDefinition::builder()
            .attribute_name(&table_config.pk_name)
            .attribute_type(ScalarAttributeType::S)
            .build()
            .map_err(|e| Error::DynamoDbError(format!("Failed to build PK attribute: {:?}", e)))?,
    ];

    // Add sort key if present
    if let Some(sk_name) = &table_config.sk_name {
        attribute_definitions.push(
            AttributeDefinition::builder()
                .attribute_name(sk_name)
                .attribute_type(ScalarAttributeType::S)
                .build()
                .map_err(|e| {
                    Error::DynamoDbError(format!("Failed to build SK attribute: {:?}", e))
                })?,
        );
    }

    // Add GSI attributes
    for gsi in &table_config.gsi_configs {
        attribute_definitions.push(
            AttributeDefinition::builder()
                .attribute_name(&gsi.pk_name)
                .attribute_type(ScalarAttributeType::S)
                .build()
                .map_err(|e| {
                    Error::DynamoDbError(format!("Failed to build GSI PK attribute: {:?}", e))
                })?,
        );

        if let Some(sk_name) = &gsi.sk_name {
            attribute_definitions.push(
                AttributeDefinition::builder()
                    .attribute_name(sk_name)
                    .attribute_type(ScalarAttributeType::S)
                    .build()
                    .map_err(|e| {
                        Error::DynamoDbError(format!("Failed to build GSI SK attribute: {:?}", e))
                    })?,
            );
        }
    }

    // Build key schema
    let mut key_schema = vec![
        KeySchemaElement::builder()
            .attribute_name(&table_config.pk_name)
            .key_type(KeyType::Hash)
            .build()
            .map_err(|e| Error::DynamoDbError(format!("Failed to build PK key schema: {:?}", e)))?,
    ];

    if let Some(sk_name) = &table_config.sk_name {
        key_schema.push(
            KeySchemaElement::builder()
                .attribute_name(sk_name)
                .key_type(KeyType::Range)
                .build()
                .map_err(|e| {
                    Error::DynamoDbError(format!("Failed to build SK key schema: {:?}", e))
                })?,
        );
    }

    // Build Global Secondary Indexes
    let mut global_secondary_indexes = Vec::new();
    for gsi in &table_config.gsi_configs {
        let mut gsi_key_schema = vec![
            KeySchemaElement::builder()
                .attribute_name(&gsi.pk_name)
                .key_type(KeyType::Hash)
                .build()
                .map_err(|e| {
                    Error::DynamoDbError(format!("Failed to build GSI PK key schema: {:?}", e))
                })?,
        ];

        if let Some(sk_name) = &gsi.sk_name {
            gsi_key_schema.push(
                KeySchemaElement::builder()
                    .attribute_name(sk_name)
                    .key_type(KeyType::Range)
                    .build()
                    .map_err(|e| {
                        Error::DynamoDbError(format!("Failed to build GSI SK key schema: {:?}", e))
                    })?,
            );
        }

        let gsi_builder = GlobalSecondaryIndex::builder()
            .index_name(&gsi.index_name)
            .set_key_schema(Some(gsi_key_schema))
            .projection(
                Projection::builder()
                    .projection_type(ProjectionType::All)
                    .build(),
            );

        global_secondary_indexes.push(
            gsi_builder
                .build()
                .map_err(|e| Error::DynamoDbError(format!("Failed to build GSI: {:?}", e)))?,
        );
    }

    // Create table
    let mut create_table_input = dynamo_client
        .client
        .create_table()
        .table_name(&table_config.name)
        .set_attribute_definitions(Some(attribute_definitions))
        .set_key_schema(Some(key_schema))
        .billing_mode(BillingMode::PayPerRequest); // Use pay-per-request for simplicity

    if !global_secondary_indexes.is_empty() {
        create_table_input =
            create_table_input.set_global_secondary_indexes(Some(global_secondary_indexes));
    }

    create_table_input.send().await.map_err(|e| {
        Error::DynamoDbError(format!(
            "Failed to create table '{}': {:?}",
            table_config.name, e
        ))
    })?;

    tracing::info!("Successfully created table: {}", table_config.name);
    Ok(())
}

pub fn get_user_tables() -> Vec<DynamoTable> {
    vec![DynamoTable::new("users", "PK", Some("SK")).with_gsi("GSI1", "GSI1_PK", Some("GSI1_SK"))]
}
