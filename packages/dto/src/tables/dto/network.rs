use bdk::prelude::*;

use crate::{Follower, Industry};

//TODO: query landing data
#[api_model(base = "/v1/network", database = skip, read_action = find_one)]
pub struct NetworkData {
    pub industries: Vec<Industry>,
    pub suggested_teams: Vec<Follower>,
    pub suggested_users: Vec<Follower>,
}
