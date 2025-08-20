use bdk::prelude::*;

#[api_model(table = spaces)]
pub struct DagitWithoutJoin {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(nullable)]
    pub started_at: Option<i64>,
    #[api_model(nullable)]
    pub ended_at: Option<i64>,

    #[api_model(nullable)]
    pub title: Option<String>,

    pub html_contents: String,

    #[api_model(many_to_one = users)]
    pub owner_id: i64,
}
