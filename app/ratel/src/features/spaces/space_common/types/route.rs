use crate::common::{
    SpaceActionFollowEntityType, SpacePartition, SpacePollEntityType, SpacePostEntityType,
    SpaceQuizEntityType,
};
use crate::Route;

// DEPRECATED: Use the `Route` enum directly instead of these helper functions.
// NOTE: it causes burrow error
pub fn space_action_poll(space_id: &SpacePartition, poll_id: &SpacePollEntityType) -> String {
    Route::PollActionPage {
        space_id: space_id.clone(),
        poll_id: poll_id.clone(),
    }
    .to_string()
}

pub fn space_action_discussion(
    space_id: &SpacePartition,
    discussion_id: &SpacePostEntityType,
) -> String {
    Route::DiscussionActionPage {
        space_id: space_id.clone(),
        discussion_id: discussion_id.clone(),
    }
    .to_string()
}

pub fn space_action_follow(
    space_id: &SpacePartition,
    follow_id: &SpaceActionFollowEntityType,
) -> String {
    Route::FollowActionPage {
        space_id: space_id.clone(),
        follow_id: follow_id.clone(),
    }
    .to_string()
}

pub fn space_action_quiz(space_id: &SpacePartition, quiz_id: &SpaceQuizEntityType) -> String {
    Route::QuizActionPage {
        space_id: space_id.clone(),
        quiz_id: quiz_id.clone(),
    }
    .to_string()
}

pub fn space_action_discussion_edit(
    space_id: &SpacePartition,
    discussion_id: &SpacePostEntityType,
) -> String {
    Route::DiscussionActionEditorPage {
        space_id: space_id.clone(),
        discussion_id: discussion_id.clone(),
    }
    .to_string()
}
