use std::{fmt::Display, str::FromStr};

use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::common::*;

/*
Previously, the code was written assuming a 1:N relationship between SpaceAction and SpaceReward, using the structure shown below.

However, after the change, the relationship is now 1:1, so this Enum is not needed.

If the relationship needs to be changed back to 1:N in the future, please utilize the field below.
 */
#[derive(
    Debug,
    Clone,
    DynamoEnum,
    SerializeDisplay,
    DeserializeFromStr,
    Eq,
    PartialEq,
    Default,
    Translate,
)]
pub enum RewardUserBehavior {
    #[default]
    #[translate(en = "Poll Response", ko = "투표 응답")]
    RespondPoll,
    #[translate(en = "Discussion Comment", ko = "토론 댓글")]
    DiscussionComment,
    #[translate(en = "Quiz Answer", ko = "퀴즈 답변")]
    QuizAnswer,
    #[translate(en = "Follow", ko = "팔로우")]
    Follow,
    #[translate(en = "Attend Meet", ko = "회의 참석")]
    AttendMeet,
    // Signup,
    // Subscribe,
}

impl RewardUserBehavior {
    pub fn action(&self) -> RewardAction {
        match self {
            Self::RespondPoll => RewardAction::SpacePoll,
            Self::DiscussionComment => RewardAction::SpaceDiscussion,
            Self::QuizAnswer => RewardAction::SpaceStudyAndQuiz,
            Self::Follow => RewardAction::SpaceFollow,
            Self::AttendMeet => RewardAction::SpaceMeet,
        }
    }

    pub fn list_behaviors(action: RewardAction) -> Vec<Self> {
        match action {
            RewardAction::SpacePoll => vec![Self::RespondPoll],
            RewardAction::SpaceDiscussion => vec![Self::DiscussionComment],
            RewardAction::SpaceStudyAndQuiz => vec![Self::QuizAnswer],
            RewardAction::SpaceFollow => vec![Self::Follow],
            RewardAction::SpaceMeet => vec![Self::AttendMeet],
        }
    }
}
