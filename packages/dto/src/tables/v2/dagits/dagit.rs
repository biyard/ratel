use bdk::prelude::*;

use crate::*;

#[api_model(table = dagits)]
pub struct Dagit {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = spaces)]
    pub space_id: i64,

    #[api_model(many_to_many = dagit_artworks, foreign_table_name = artworks, foreign_primary_key = dagit_id, foreign_reference_key = artwork_id, nested)]
    pub artworks: Vec<Artwork>,

    #[api_model(many_to_many = dagit_oracles, foreign_table_name = oracles, foreign_primary_key = dagit_id, foreign_reference_key = oracle_id, nested)]
    pub oracles: Vec<Oracle>,
}
