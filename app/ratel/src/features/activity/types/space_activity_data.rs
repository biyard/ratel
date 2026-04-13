use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SpaceActivityData {
    #[default]
    Unknown,
    Poll {
        poll_id: String,
        answered_optional_count: u32,
    },
    Quiz {
        quiz_id: SpaceQuizAttemptEntityType,
        passed: bool,
        correct_count: u32,
        pass_threshold: u32,
    },
    Follow {
        follow_id: String,
    },
    Discussion {
        discussion_id: SpacePostPartition,
        #[serde(default)]
        comment_id: SpacePostCommentEntityType,
        is_first_contribution: bool,
    },
}
