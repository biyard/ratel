mod use_user_role;
pub use use_user_role::*;

mod use_space;
pub use use_space::*;

use dioxus::fullstack::Loader;

use crate::common::types::ListResponse;
use crate::features::activity::controllers::{MyScoreResponse, RankingEntryResponse};
use crate::features::spaces::pages::actions::types::SpaceActionSummary;

pub fn use_actions() -> Loader<Vec<SpaceActionSummary>> {
    crate::features::spaces::space_common::providers::use_space_context().actions
}

pub fn use_ranking() -> Loader<ListResponse<RankingEntryResponse>> {
    crate::features::spaces::space_common::providers::use_space_context().ranking
}

pub fn use_my_score() -> Loader<MyScoreResponse> {
    crate::features::spaces::space_common::providers::use_space_context().my_score
}
