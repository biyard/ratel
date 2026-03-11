use super::*;

#[derive(Clone, Copy, DioxusController)]
pub struct Context {
    pub poll: Loader<PollResponse>,
    pub space_id: ReadSignal<SpacePartition>,
    pub poll_id: ReadSignal<SpacePollEntityType>,
}

pub fn use_space_poll_context() -> Context {
    use_context()
}

impl Context {
    pub fn init(
        space_id: ReadSignal<SpacePartition>,
        poll_id: ReadSignal<SpacePollEntityType>,
    ) -> Result<Self, Loading> {
        let poll = use_loader(move || async move { get_poll(space_id(), poll_id()).await })?;

        let srv = Self {
            poll,
            space_id,
            poll_id,
        };

        use_context_provider(move || srv);

        Ok(srv)
    }
}
