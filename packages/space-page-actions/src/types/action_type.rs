use crate::*;

#[derive(Debug, Serialize, Deserialize, Translate, Clone, PartialEq)]
pub enum ActionType {
    #[translate(ko = "숙의 및 퀴즈")]
    StudyAndQuiz,
    #[translate(ko = "투표")]
    Poll,
    #[translate(ko = "주제 토론", en = "Topic Discussion")]
    TopicDiscussion,
}
