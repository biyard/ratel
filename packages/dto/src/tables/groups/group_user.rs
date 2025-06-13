use bdk::prelude::*;

#[api_model(table = groups)]
pub struct GroupUser {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = create, action_by_id = [update])]
    pub name: String,
    #[api_model(summary, version = v0.1, action = create, action_by_id = [update])]
    pub description: String,

    #[api_model(summary, version = v0.1, action = create, action_by_id = [update])]
    pub image_url: String,

    #[api_model(many_to_one = users)]
    pub creator_id: i64,

    #[api_model(many_to_many = group_members, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = group_id, aggregator = count)]
    #[serde(default)]
    pub member_count: i64,
    #[api_model(version = v0.1)]
    pub permissions: i64,
}
