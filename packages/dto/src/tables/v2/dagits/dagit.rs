use bdk::prelude::*;

use crate::*;

#[api_model(table = spaces)]
pub struct Dagit {
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

    #[api_model(many_to_many = dagit_artworks, foreign_table_name = artworks, foreign_primary_key = artwork_id, foreign_reference_key = space_id, nested)]
    pub artworks: Vec<Artwork>,

    #[api_model(many_to_many = dagit_oracles, foreign_table_name = oracles, foreign_primary_key = oracle_id, foreign_reference_key = space_id)]
    pub oracles: Vec<Oracle>,

    #[api_model(many_to_many = dagit_oracles, foreign_table_name = oracles, foreign_primary_key = oracle_id, foreign_reference_key = space_id, aggregator = exist)]
    pub is_oracle: bool,
}
