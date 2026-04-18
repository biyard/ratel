use crate::{
    common::RewardUserBehavior, features::spaces::pages::actions::*,
    spaces::pages::actions::actions::quiz::SpaceQuiz,
};
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
    pub fn to_behavior(&self) -> RewardUserBehavior {
        match self {
            SpaceActionType::Poll => RewardUserBehavior::RespondPoll,
            SpaceActionType::TopicDiscussion => RewardUserBehavior::DiscussionComment,
            SpaceActionType::Quiz => RewardUserBehavior::QuizAnswer,
            SpaceActionType::Follow => RewardUserBehavior::Follow,
        }
    }

    pub async fn create(&self, space_id: SpacePartition) -> Result<Route> {
        match self {
            SpaceActionType::Poll => {
                let response = crate::features::spaces::pages::actions::actions::poll::controllers::create_poll(space_id.clone()).await?;
                let poll_id = SpacePollEntityType::from(response.sk);
                Ok(Route::PollActionPage {
                    space_id: space_id.clone(),
                    poll_id: poll_id.clone(),
                })
            }
            SpaceActionType::TopicDiscussion => {
                let response = crate::features::spaces::pages::actions::actions::discussion::controllers::create_discussion(space_id.clone()).await?;
                let discussion_id: SpacePostEntityType = response.sk.try_into().unwrap_or_default();
                Ok(Route::DiscussionActionEditorPage {
                    space_id: space_id.clone(),
                    discussion_id: discussion_id.clone(),
                })
            }
            SpaceActionType::Follow => {
                let response = crate::features::spaces::pages::actions::actions::follow::controllers::create_follow(space_id.clone()).await?;
                let follow_id = SpaceActionFollowEntityType::from(response.sk);
                Ok(Route::FollowActionPage {
                    space_id: space_id.clone(),
                    follow_id: follow_id.clone(),
                })
            }
            SpaceActionType::Quiz => {
                let response = crate::features::spaces::pages::actions::actions::quiz::controllers::create_quiz(space_id.clone()).await?;
                Ok(Route::QuizActionPage {
                    space_id: space_id.clone(),
                    quiz_id: response.quiz_id.clone(),
                })
            }
        }
    }
}
