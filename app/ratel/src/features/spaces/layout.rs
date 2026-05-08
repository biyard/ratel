use super::*;
use crate::features::auth::hooks::use_user_context;
use crate::features::auth::{LoginModal, UserContextStoreExt};
use crate::features::spaces::space_common::controllers::get_user_role;
use crate::features::spaces::space_common::providers::SpaceContextProvider;
use crate::features::spaces::space_common::{
    components::{SpaceNav, SpaceNavItem, SpaceTop, SpaceTopLabel},
    hooks::use_space_role,
};
use crate::features::spaces::*;

#[derive(Clone, Copy)]
pub struct SpaceLayoutUiContext {
    pub sidebar_visible: Signal<bool>,
}

pub fn use_space_layout_ui() -> SpaceLayoutUiContext {
    use_context()
}

#[component]
pub fn SpaceLayout(space_id: ReadSignal<SpacePartition>) -> Element {
    let ctx = SpaceContextProvider::init(space_id)?;

    use_context_provider(|| LayoverService::new());
    let sidebar_visible = use_signal(|| true);
    use_context_provider(move || SpaceLayoutUiContext { sidebar_visible });
    let space = ctx.space();

    let seo_image = if space.logo.is_empty() {
        "https://metadata.ratel.foundation/logos/logo-symbol.png".to_string()
    } else {
        space.logo.clone()
    };

    rsx! {
        SeoMeta {
            title: space.title.clone(),
            description: space.description(),
            image: seo_image.clone(),
        }
        Outlet::<Route> {}
        Layover {}
    }
}
