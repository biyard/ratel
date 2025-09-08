use bdk::prelude::*;

#[api_model(table = conversation_participants)]
pub struct ConversationParticipant {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = conversations)]
    pub conversation_id: i64,
    #[api_model(many_to_one = users)]
    pub user_id: i64,

    #[api_model(type = INTEGER)]
    pub role: ParticipantRole,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum ParticipantRole {
    #[default]
    #[translate(en = "Member")]
    Member = 0,
    #[translate(en = "Admin")]
    Admin = 1,
}
