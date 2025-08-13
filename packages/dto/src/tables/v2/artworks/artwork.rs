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

    #[api_model(action = [create, update])]
    pub title: String,

    #[api_model(action = [create, update])]
    pub description: String,

    #[api_model(action = [create, update])]
    pub dimensions: String,

    #[api_model(action = [create, update], type = JSONB)]
    #[serde(default)]
    pub file: Vec<File>,

    #[api_model(one_to_many = artwork_certifications, foreign_key = artwork_id, aggregator = exist)]
    pub is_certified: bool,
}
