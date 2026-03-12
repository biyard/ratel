use crate::features::spaces::pages::actions::*;
#[derive(Debug, Clone, Default, Translate, Serialize, Deserialize, PartialEq)]
pub enum SpaceActionType {
    #[default]
    // #[translate(ko = "숙의 및 퀴즈", en = "Quiz")]
    // StudyAndQuiz,
    #[translate(ko = "투표", en = "Poll")]
    Poll,
    #[translate(ko = "토론", en = "Discussion")]
    TopicDiscussion,
    #[translate(ko = "팔로우", en = "Follow")]
    Follow,
    #[translate(ko = "퀴즈", en = "Quiz")]
    Quiz,
}
