use crate::*;
#[derive(Debug, Clone, Default, Translate, Serialize, Deserialize)]
// #[serde_with(crate = "::common::serde_with")]
pub enum SpaceActionType {
    #[default]
    #[translate(ko = "숙의 및 퀴즈", en = "Quiz")]
    StudyAndQuiz,
    #[translate(ko = "투표", en = "Poll")]
    Poll,
    #[translate(ko = "토론", en = "Discussion")]
    TopicDiscussion,
}
