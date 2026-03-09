use crate::common::{DevTools, LayoverService, ThemeService, ToastProvider, ToastService};
use crate::common::{Environment, PopupZone, components::PopupService, query_provider};
use dioxus::prelude::*;

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
///

pub const FAVICON: Asset = asset!("/assets/favicon.ico");

#[component]
pub fn App(children: Element, tailwind: Asset) -> Element {
    use_context_provider(|| PopupService::new());
    use_context_provider(|| LayoverService::new());
    ToastService::init();
    ThemeService::init();
    let _ = crate::features::auth::Context::init()?;
    crate::common::contexts::TeamContext::init();
    query_provider();
    let cfg = crate::common::CommonConfig::default();
    let env = cfg.env;
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: tailwind }
        crate::common::Provider {}
        crate::features::auth::AuthProvider {}
        ToastProvider {}

        PopupZone {}
        if env == Environment::Dev || env == Environment::Local {
            DevTools {}
        }
        {children}
    }
}
