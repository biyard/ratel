use bdk::prelude::*;

use crate::{Follower, SpaceSummary};

//TODO: query landing data
#[api_model(base = "/v1/landings", database = skip, read_action = find_one)]
pub struct LandingData {
    pub my_spaces: Vec<SpaceSummary>,
    pub following_spaces: Vec<SpaceSummary>,
    pub follower_list: Vec<Follower>,
}
