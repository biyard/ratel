use crate::File;
use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/spaces/:space-id/drafts", table = space_drafts)]
pub struct SpaceDraft {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = spaces)]
    pub space_id: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    #[serde(default)]
    pub title: String,
    #[api_model(summary, action = create, action_by_id = update)]
    #[serde(default)]
    pub html_contents: String,
    #[api_model(version = v0.1, action = create, action_by_id = update, summary, type = JSONB, action_by_id = [update_space])]
    #[serde(default)]
    pub files: Vec<File>,
}
