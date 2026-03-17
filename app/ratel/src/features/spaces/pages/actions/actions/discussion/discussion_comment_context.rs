use super::*;
use crate::common::hooks::{InfiniteQuery, use_infinite_query};
use crate::common::types::ListResponse;

#[derive(Clone, Copy, DioxusController)]
pub struct DiscussionCommentContext {
    pub comments:
        InfiniteQuery<String, DiscussionCommentResponse, ListResponse<DiscussionCommentResponse>>,
}

pub fn use_discussion_comment_context() -> DiscussionCommentContext {
    use_context()
}

impl DiscussionCommentContext {
    pub fn init(
        space_id: ReadSignal<SpacePartition>,
        discussion_id: ReadSignal<SpacePostEntityType>,
    ) -> std::result::Result<Self, RenderError> {
        let comments = use_infinite_query(move |bookmark| {
            list_comments(space_id(), discussion_id(), bookmark)
        })?;

        let srv = Self { comments };
        use_context_provider(move || srv);
        Ok(srv)
    }
}
