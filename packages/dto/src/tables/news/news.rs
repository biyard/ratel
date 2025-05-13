use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/news", table = news, action = [], action_by_id = [delete, update])]
pub struct News {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = create)]
    pub title: String,
    #[api_model(summary, action = create)]
    pub html_content: String,
    #[api_model(many_to_one = users)]
    pub user_id: i64,
}
