use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/", table = election_pledges_quizzes_likes)]
pub struct ElectionPledgeQuizLike {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = election_pledges)]
    pub election_pledge_id: i64,
    #[api_model(many_to_one = quizzes)]
    pub quiz_id: i64,
}

#[derive(Validate)]
#[api_model(base = "/", table = election_pledges_quizzes_dislikes)]
pub struct ElectionPledgeQuizDislike {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = election_pledges)]
    pub election_pledge_id: i64,
    #[api_model(many_to_one = quizzes)]
    pub quiz_id: i64,
}
