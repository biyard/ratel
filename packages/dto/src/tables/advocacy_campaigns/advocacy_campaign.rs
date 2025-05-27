use bdk::prelude::*;
use validator::Validate;

use crate::Author;

#[derive(Validate)]
#[api_model(base = "/v1/advocacy-campaigns", table = advocacy_campaign, action = [], action_by_id = [delete, update, agree])]
pub struct AdvocacyCampaign {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    pub title: String,

    pub contents: String,

    #[api_model(many_to_many = advocacy_campaign_authors, foreign_reference_key = advocacy_campaign_id, foreign_primary_key = user_id, foreign_table_name =users)]
    pub author: Vec<Author>,

    #[api_model(many_to_many = advocacy_campaign_voters, foreign_reference_key = advocacy_campaign_id, foreign_primary_key = user_id, foreign_table_name = users, unique)]
    pub voters: Vec<Author>,

    #[api_model(one_to_many = advocacy_campaign_voters, foreign_key = advocacy_campaign_id, aggregator=count)]
    pub votes: i64,

    #[api_model(many_to_many = advocacy_campaign_voters, foreign_reference_key = advocacy_campaign_id, foreign_primary_key = user_id, foreign_table_name = users, aggregator=exist)]
    pub voted: bool,
}
