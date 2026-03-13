use crate::common::hooks::{use_infinite_query, InfiniteQuery};
use crate::common::types::ListResponse;

use super::*;

#[derive(Clone, Copy, DioxusController)]
pub struct Context {
    pub comments:
        InfiniteQuery<String, DiscussionCommentResponse, ListResponse<DiscussionCommentResponse>>,
}

pub fn use_discussion_comment_context() -> Context {
    use_context()
}

impl Context {
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
