use bdk::prelude::*;

use crate::*;

#[api_model(table = artworks)]
pub struct Artwork {
    #[api_model(primary_key)]
    pub id: i64,

    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = users)]
    pub owner_id: i64,

    pub title: String,

    pub description: Option<String>,

    #[api_model(type = JSONB)]
    #[serde(default)]
    pub file: Vec<File>,
}
