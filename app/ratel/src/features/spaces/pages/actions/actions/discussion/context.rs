use super::*;
use crate::features::spaces::space_common::types::space_page_actions_discussion_key;

#[derive(Clone, Copy, DioxusController)]
pub struct Context {
    pub discussion: Loader<DiscussionResponse>,
    pub space_id: ReadSignal<SpacePartition>,
    pub discussion_id: ReadSignal<SpacePostEntityType>,
}

pub fn use_discussion_context() -> Context {
    use_context()
}

impl Context {
    pub fn init(
        space_id: ReadSignal<SpacePartition>,
        discussion_id: ReadSignal<SpacePostEntityType>,
    ) -> Result<Self, Loading> {
        let key = space_page_actions_discussion_key(&space_id(), &discussion_id());
        let discussion = use_query(&key, {
            move || get_discussion_detail(space_id(), discussion_id())
        })?;

        let srv = Self {
            discussion,
            space_id,
            discussion_id,
        };

        use_context_provider(move || srv);

        Ok(srv)
    }
}
