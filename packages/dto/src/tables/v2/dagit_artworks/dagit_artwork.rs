use bdk::prelude::*;

#[api_model(table = dagit_artworks)]
pub struct DagitArtwork {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,

    #[api_model(many_to_one = dagits)]
    pub dagit_id: i64,

    #[api_model(many_to_one = artworks)]
    pub artwork_id: i64,
}
