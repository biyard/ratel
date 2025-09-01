use bdk::prelude::*;
use validator::Validate;


#[derive(Validate)]
#[api_model(table = messages)]
pub struct Message {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model()]
    pub seq_id: i64,

    #[api_model()]
    pub html_content: String,

    #[api_model(type = INTEGER)]
    pub status: MessageStatus,
    
    #[api_model(many_to_one = users)]
    pub sender_id: i64,
    #[api_model(many_to_one = conversations)]
    pub conversation_id: i64,

}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum MessageStatus {
    #[default]
    #[translate(en = "Sent")]
    Sent = 0,
    #[translate(en = "Delivered")]
    Delivered = 1,
    #[translate(en = "Read")]
    Read = 2,
}