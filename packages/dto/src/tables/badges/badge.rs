use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/", table = badges, action = [], action_by_id = [delete, update])]
pub struct Badge {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = users)]
    pub creator_id: i64,

    #[api_model(action = create)]
    pub name: String,
    #[api_model(type = INTEGER)]
    pub scope: Scope,
    #[api_model(action = create)]
    pub image_url: String,

    #[api_model(action = create)]
    pub contract: Option<String>,
    #[api_model(action = create)]
    pub token_id: Option<i64>,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Scope {
    #[default]
    Global = 1,
    Space = 2,
    Team = 3,
}
