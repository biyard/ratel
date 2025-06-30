use bdk::prelude::*;

use crate::Badge;

#[api_model(table = spaces)]
pub struct BadgesOfSpace {
    #[api_model(summary, primary_key)]
    pub id: i64,

    #[api_model(many_to_many = space_badges, foreign_table_name = badges, foreign_primary_key = badge_id, foreign_reference_key = space_id)]
    pub badges: Vec<Badge>,
}
