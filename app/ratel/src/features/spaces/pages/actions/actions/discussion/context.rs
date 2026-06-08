use super::*;
#[derive(Clone, Copy, DioxusController)]
pub struct Context {
    pub discussion: Loader<DiscussionResponse>,
    pub space_id: ReadSignal<SpacePartition>,
    pub discussion_id: ReadSignal<SpacePostEntityType>,
    /// Bumped by the topbar "Import from overview" button. `ContentCard`
    /// watches this counter and, on each increment, copies the space
    /// overview's title/content into the editor fields (always overwrite).
    pub import_request: Signal<u64>,
}

pub fn use_discussion_context() -> Context {
    use_context()
}

impl Context {
    pub fn init(
        space_id: ReadSignal<SpacePartition>,
        discussion_id: ReadSignal<SpacePostEntityType>,
    ) -> Result<Self, Loading> {
        let discussion = use_loader(move || async move {
            get_discussion_detail(space_id(), discussion_id()).await
        })?;

        let import_request = use_signal(|| 0u64);

        let srv = Self {
            discussion,
            space_id,
            discussion_id,
            import_request,
        };

        use_context_provider(move || srv);

        Ok(srv)
    }
}
