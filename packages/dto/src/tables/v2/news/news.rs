use bdk::prelude::*;


/// News article model for v2 API
#[api_model(table = news, action = [])]
pub struct News {
    #[api_model(primary_key)]
    pub id: i64,
    
    #[api_model(auto = [insert])]
    pub created_at: i64,
    
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(action = create, action_by_id = [update])]
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    
    #[api_model(action = create, action_by_id = [update])]
    #[validate(length(min = 1))]
    pub html_content: String,
    
    #[api_model(many_to_one = users)]
    pub user_id: i64,
}
