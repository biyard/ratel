use common::{DevTools, ThemeService, ToastProvider, ToastService};
use common::{Environment, PopupZone, components::PopupService, query_provider};
use dioxus::prelude::*;

#[cfg(not(feature = "server"))]
pub fn launch(app: fn() -> Element) {
    dioxus::launch(app);
}

#[cfg(feature = "server")]
pub fn serve(app: fn() -> Element) {
    let cfg = common::CommonConfig::default();
    dioxus::serve(move || {
        let cli = cfg.dynamodb();

        let session_layer =
            common::middlewares::session_layer::get_session_layer(cli, cfg.env.to_string());
        async move {
            use common::axum::middleware;

            let dioxus_router = dioxus::server::router(app);

            Ok(dioxus_router.layer(session_layer))
        }
    });
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
///

pub const FAVICON: Asset = asset!("/assets/favicon.ico");

#[component]
pub fn App(children: Element, tailwind: Asset) -> Element {
    use_context_provider(|| PopupService::new());

    ToastService::init();
    ThemeService::init();
    let _ = ratel_auth::Context::init()?;
    common::contexts::TeamContext::init();
    query_provider();
    let cfg = common::CommonConfig::default();
    let env = cfg.env;
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: tailwind }
        common::Provider {}
        ratel_auth::AuthProvider {}
        ToastProvider {}
        PopupZone {}
        if env == Environment::Dev || env == Environment::Local {
            DevTools {}
        }
        {children}
    }
}
