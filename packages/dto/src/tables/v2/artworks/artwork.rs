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

    #[api_model(nullable)]
    pub description: Option<String>,

    #[api_model(type = JSONB)]
    pub file: File,

    #[api_model(one_to_many = artwork_certifications, foreign_key = artwork_id, aggregator = exist)]
    pub is_certified: bool,

    #[api_model(many_to_many = consensus_votes, foreign_table_name = oracles, foreign_primary_key = oracle_id, foreign_reference_key = consensus_id, aggregator = exist)]
    pub is_voted: bool,

    #[api_model(one_to_many = consensus, foreign_key = artwork_id, aggregator = exist)]
    pub has_consensus: bool,
}
