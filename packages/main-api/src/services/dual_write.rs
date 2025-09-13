use dto::{Result, User, Conversation, AuthClient};
use crate::{
    models::dynamo::{
        user_model::DynamoUser,
        conversation_model::DynamoConversation,
        oauth_client_model::DynamoAuthClient,
        base_model::DynamoModel
    },
    utils::aws::dynamo::DynamoClient,
    config
};

#[derive(Clone)]
pub struct DualWriteService {
    pub dynamo_client: DynamoClient,
    pub enabled: bool,
}

impl DualWriteService {
    pub fn new() -> Self {
        let conf = config::get();
        let dynamo_client = DynamoClient::new(conf.dual_write.table_name);
        
        Self {
            dynamo_client,
            enabled: conf.dual_write.enabled,
        }
    }

    pub async fn write_user(&self, user: &User) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let dynamo_user = DynamoUser::from_postgres_user(user);
        let item = dynamo_user.to_item()?;
        
        match self.dynamo_client.put_item(item).await {
            Ok(_) => {
                tracing::info!("Successfully wrote user {} to DynamoDB", user.id);
                Ok(())
            }
            Err(e) => {
                tracing::error!("Failed to write user {} to DynamoDB: {:?}", user.id, e);
                // Don't fail the entire operation if DynamoDB write fails during migration
                // Log the error and continue
                Ok(())
            }
        }
    }

    pub async fn write_conversation(&self, conversation: &Conversation) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let dynamo_conversation = DynamoConversation::from_postgres_conversation(conversation);
        let item = dynamo_conversation.to_item()?;
        
        match self.dynamo_client.put_item(item).await {
            Ok(_) => {
                tracing::info!("Successfully wrote conversation {} to DynamoDB", conversation.id);
                Ok(())
            }
            Err(e) => {
                tracing::error!("Failed to write conversation {} to DynamoDB: {:?}", conversation.id, e);
                // Don't fail the entire operation if DynamoDB write fails during migration
                Ok(())
            }
        }
    }

    pub async fn write_auth_client(&self, client: &AuthClient) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let dynamo_auth_client = DynamoAuthClient::from_postgres_client(client);
        let item = dynamo_auth_client.to_item()?;
        
        match self.dynamo_client.put_item(item).await {
            Ok(_) => {
                tracing::info!("Successfully wrote auth client {} to DynamoDB", client.client_id);
                Ok(())
            }
            Err(e) => {
                tracing::error!("Failed to write auth client {} to DynamoDB: {:?}", client.client_id, e);
                // Don't fail the entire operation if DynamoDB write fails during migration
                Ok(())
            }
        }
    }
}