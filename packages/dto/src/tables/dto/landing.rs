use bdk::prelude::*;

use crate::{SpaceSummary, User};

//내 아이디 별 관심 스페이스 리스트 조회
//TODO: query landing data
#[api_model(base = "/v1/landings", database = skip, read_action = find_one)]
pub struct LandingData {
    pub my_spaces: Vec<SpaceSummary>,
    pub following_spaces: Vec<SpaceSummary>,
    pub follower_list: Vec<User>, // follow 할 수 있는 계정 조회
    pub profile_data: User,       // 나의 프로필 조회
}
