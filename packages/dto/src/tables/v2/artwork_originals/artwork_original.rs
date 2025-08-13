use bdk::prelude::*;

#[api_model(table = artwork_originals)]
pub struct ArtworkOriginal {
    #[api_model(primary_key)]
    pub id: i64,

    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = artworks)]
    pub artwork_id: i64,

    #[api_model(many_to_one = users)]
    pub owner_id: i64,

    #[api_model(action = create)]
    pub image: String,
}
