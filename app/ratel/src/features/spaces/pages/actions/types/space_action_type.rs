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

impl SpaceActionType {
    pub async fn create(&self, space_id: SpacePartition) -> Result<String> {
        use crate::features::spaces::space_common::types::route::{
            space_action_discussion, space_action_follow, space_action_poll, space_action_quiz,
        };
        match self {
            SpaceActionType::Poll => {
                let response = crate::features::spaces::pages::actions::actions::poll::controllers::create_poll(space_id.clone()).await?;
                let poll_id = SpacePollEntityType::from(response.sk);
                Ok(space_action_poll(&space_id, &poll_id))
            }
            SpaceActionType::TopicDiscussion => {
                let response = crate::features::spaces::pages::actions::actions::discussion::controllers::create_discussion(space_id.clone()).await?;
                let discussion_id: SpacePostEntityType = response.sk.try_into().unwrap_or_default();
                Ok(space_action_discussion(&space_id, &discussion_id))
            }
            SpaceActionType::Follow => {
                let response = crate::features::spaces::pages::actions::actions::follow::controllers::create_follow(space_id.clone()).await?;
                let follow_id = SpaceActionFollowEntityType::from(response.sk);
                Ok(space_action_follow(&space_id, &follow_id))
            }
            SpaceActionType::Quiz => {
                let response = crate::features::spaces::pages::actions::actions::quiz::controllers::create_quiz(space_id.clone()).await?;
                Ok(space_action_quiz(&space_id, &response.quiz_id))
            }
        }
    }
}
