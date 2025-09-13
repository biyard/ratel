use crate::types::dynamo_entity_type::EntityType;
use dto::{Conversation, ConversationType};

use super::base_model::{BaseModel, DynamoModel};
use super::serde_helpers as sh;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DynamoConversation {
    #[serde(flatten)]
    pub base: BaseModel,
    pub id: i64,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(with = "sh::conversation_type_num")]
    pub conversation_type: ConversationType,
    pub created_at: i64,
    pub updated_at: i64,
}

impl DynamoConversation {
    pub fn from_postgres_conversation(conversation: &Conversation) -> Self {
        Self {
            base: BaseModel {
                pk: format!("CONVERSATION#{}", conversation.id),
                sk: format!("METADATA"),
                entity_type: EntityType::Discussion, // closest match for conversations
                created_at: conversation.created_at,
                updated_at: conversation.updated_at,
                gsi1_pk: Some(format!("CONVERSATION#{}", conversation.id)),
                gsi1_sk: Some(format!("TYPE#{:?}#{}", conversation.conversation_type, conversation.created_at)),
                gsi2_pk: None,
                gsi2_sk: None,
                ..Default::default()
            },
            id: conversation.id,
            title: conversation.title.clone(),
            description: conversation.description.clone(),
            conversation_type: conversation.conversation_type,
            created_at: conversation.created_at,
            updated_at: conversation.updated_at,
        }
    }
}

impl DynamoModel for DynamoConversation {
    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}
