use crate::*;

#[get("/api/action-types")]
pub async fn list_action_types() -> Result<Vec<ActionType>> {
    Ok(ActionType::VARIANTS.to_vec())
}

#[derive(Debug, Serialize, Deserialize, Translate, Clone)]
pub enum ActionType {
    #[translate(ko = "숙의 및 퀴즈")]
    StudyAndQuiz,
    #[translate(ko = "투표")]
    Poll,
    #[translate(ko = "주제 토론", en = "Topic Discussion")]
    TopicDiscussion,
}
