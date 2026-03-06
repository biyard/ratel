use crate::views::*;
use crate::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/actions/quizzes/:quiz_id")]
        #[route("/")]
        MainPage { space_id: SpacePartition, quiz_id: SpaceQuizEntityType },
}
