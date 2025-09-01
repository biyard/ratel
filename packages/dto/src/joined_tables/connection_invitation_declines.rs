use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(table = connection_invitation_declines)]
pub struct ConnectionInvitationDecline {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, many_to_one = users)]
    pub user_id: i64,
    #[api_model(summary, many_to_one = users)]
    pub decline_user_id: i64,
}
