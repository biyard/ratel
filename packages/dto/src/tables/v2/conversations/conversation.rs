use bdk::prelude::*;
use validator::Validate;

use crate::{ConversationParticipant, Message};

#[derive(Validate)]
#[api_model(table = conversations)]
pub struct Conversation {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model()]
    pub title: Option<String>,
    #[api_model()]
    pub description: Option<String>,

    #[api_model(type = INTEGER)]
    pub conversation_type: ConversationType,

    #[api_model(one_to_many = conversation_participants, foreign_key = conversation_id)]
    #[serde(default)]
    pub participants: Vec<ConversationParticipant>,

    #[api_model(one_to_many = messages, foreign_key = conversation_id)]
    #[serde(default)]
    pub messages: Vec<Message>,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum ConversationType {
    #[default]
    #[translate(en = "Direct")]
    Direct = 0,
    #[translate(en = "Group")]
    Group = 1,
    #[translate(en = "Channel")]
    Channel = 2,
}
