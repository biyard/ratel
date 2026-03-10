use crate::spaces::space_common::providers::{use_space_context, SpaceContextProvider};

use super::*;

#[derive(Clone, Copy, DioxusController)]
pub struct Context {
    pub apps: Loader<Vec<SpaceApp>>,
    pub space_ctx: SpaceContextProvider,
    pub role: ReadSignal<SpaceUserRole>,
}

pub fn use_space_apps_context() -> Context {
    use_context()
}

impl Context {
    pub fn init(space_id: ReadSignal<SpacePartition>) -> Result<Self, Loading> {
        let apps = use_loader(move || async move { get_space_apps(space_id()).await })?;
        let space_ctx = use_space_context();
        let role = use_space_role();

        let srv = Self {
            apps,
            space_ctx,
            role,
        };
        use_context_provider(move || srv);

        Ok(srv)
    }
}

#[component]
pub fn ContextProvider(space_id: ReadSignal<SpacePartition>, children: Element) -> Element {
    let ctx = Context::init(space_id)?;

    rsx! {
        {children}
    }
}
