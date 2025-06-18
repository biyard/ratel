use bdk::prelude::*;

#[api_model(base = "/v2/spaces/:space-id/discussions", table = discussion_participants)]
pub struct DiscussionParticipant {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, many_to_one = discussions)]
    pub discussion_id: i64,
    #[api_model(summary, create)]
    pub user_id: i64,
    #[api_model(summary, create)]
    pub participant_id: String,
}
