pub mod create_discussion;
pub mod delete_discussion;
pub mod end_recording;
pub mod exit_meeting;
pub mod get_discussion;
pub mod get_meeting;
pub mod list_discussions;
pub mod participant_meeting;
pub mod start_meeting;
pub mod start_recording;
pub mod update_discussion;

pub use create_discussion::*;
pub use delete_discussion::*;
pub use end_recording::*;
pub use exit_meeting::*;
pub use get_discussion::*;
pub use get_meeting::*;
pub use list_discussions::*;
pub use participant_meeting::*;
pub use start_meeting::*;
pub use start_recording::*;
pub use update_discussion::*;

#[cfg(test)]
pub mod tests;

use crate::AppState;
use bdk::prelude::*;
use by_axum::aide::axum::routing::*;
use by_axum::axum::*;

pub fn discussions_route() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            post(create_discussion_handler).get(list_discussions_handler),
        )
        .route(
            "/:discussion_pk",
            patch(update_discussion_handler)
                .get(get_discussion_handler)
                .delete(delete_discussion_handler),
        )
        .route("/:discussion-pk/meeting", get(get_meeting_handler))
        .route(
            "/:discussion_pk/start-meeting",
            patch(start_meeting_handler),
        )
        .route(
            "/:discussion_pk/participant-meeting",
            patch(participant_meeting_handler),
        )
        .route("/:discussion_pk/exit-meeting", patch(exit_meeting_handler))
        .route(
            "/:discussion_pk/start-recording",
            patch(start_recording_handler),
        )
        .route(
            "/:discussion_pk/end-recording",
            patch(end_recording_handler),
        )
}
