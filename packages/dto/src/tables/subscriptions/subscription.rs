use bdk::prelude::*;
use validator::Validate;

// TODO(api): implement POST /v1/subscriptions (subscribe)
#[derive(Validate)]
#[api_model(base = "/v1/subscriptions", table = subscriptions)]
pub struct Subscription {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[validate(email)]
    #[api_model(summary, action = [subscribe, sponsor], unique)] // TODO: version = v1.1
    pub email: String,
}
