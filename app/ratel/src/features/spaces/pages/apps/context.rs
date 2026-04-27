use crate::spaces::space_common::providers::{use_space_context, SpaceContextProvider};

use super::*;

/// App-scoped shared context.
///
/// `role` intentionally does not live on this struct. Consumers that
/// need the user's space role should read it from `space_ctx`:
///
/// * `ctx.space_ctx.role()` — the real, persisted role (Creator,
///   Participant, …). Use this for admin-surface gating, e.g.
///   creator-only app pages (panels, analyze). Previously exposed via
///   `use_space_role()` which wraps `current_role` — that memo flips
///   Creator → Participant once the space is Ongoing so the creator
///   can preview the participant view, and it silently broke every
///   "creator-only" gate on this tree when the space was started.
/// * `ctx.space_ctx.current_role()` — the preview-aware memo. Use
///   this for UI that should track the creator's preview toggle.
#[derive(Clone, Copy, DioxusController)]
pub struct Context {
    pub apps: Loader<Vec<SpaceApp>>,
    pub space_ctx: SpaceContextProvider,
}

pub fn use_space_apps_context() -> Context {
    use_context()
}

impl Context {
    pub fn init(space_id: ReadSignal<SpacePartition>) -> Result<Self, Loading> {
        let apps = use_loader(move || async move { get_space_apps(space_id()).await })?;
        let space_ctx = use_space_context();

        let srv = Self { apps, space_ctx };
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
