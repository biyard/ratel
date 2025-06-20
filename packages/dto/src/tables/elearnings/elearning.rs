use bdk::prelude::*;
use validator::Validate;

use crate::*;

#[derive(Validate)]
#[api_model(base = "/v1/spaces/:space-id/elearnings", table = elearnings)]
pub struct Elearning {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = spaces)]
    pub space_id: i64,
    #[api_model(version = v0.1, action = create, summary, type = JSONB)]
    #[serde(default)]
    pub files: Vec<File>,
}
