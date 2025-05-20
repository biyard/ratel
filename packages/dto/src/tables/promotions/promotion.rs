use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/promotions", table = promotions, read_action = [hot_promotion], action = [promote_feed(feed_id = i64)], action_by_id = [delete, update, approve])]
pub struct Promotion {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = [promote_feed])]
    pub name: String,
    #[api_model(summary, action = [promote_feed])]
    pub description: String,
    #[api_model(summary, action = [promote_feed])]
    pub image_url: String,

    #[api_model(summary, type = INTEGER)]
    pub promotion_type: PromotionType,

    #[api_model(summary, type = INTEGER)]
    pub status: PromotionStatus,

    #[api_model(action = [promote_feed])]
    pub start_at: i64,
    #[api_model(action = [promote_feed])]
    pub end_at: i64,

    #[api_model(action_by_id = [approve])]
    pub priority: i64,

    #[api_model(many_to_one = users)]
    pub requester_id: i64,

    // Feed Type
    #[api_model(summary, nullable)]
    pub feed_id: Option<i64>,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum PromotionType {
    #[default]
    Feed = 1,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum PromotionStatus {
    #[default]
    Requested = 1,
    Approved = 2,
}
