use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "", table = advocacy_campaign_voters, action = [agree])]
pub struct AdvocacyCampaignVoter {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    pub user_id: i64,
    pub advocacy_campaign_id: i64,
}
