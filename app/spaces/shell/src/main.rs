use common::PopupService;
use common::{components::PopupService, use_query_store, PopupZone};
use dioxus::prelude::*;
use ratel_auth::AuthProvider;
use space_shell::*;
fn main() {
    let config = config::get();
    dioxus::logger::init(config.common.log_level.into()).expect("logger failed to init");

    #[cfg(not(feature = "server"))]
    space_shell::web::launch(App);

    #[cfg(feature = "server")]
    server::serve(App);
}

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    use_context_provider(|| PopupService::new());
    use_query_store();
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        AuthProvider {}
        Router::<Route> {}
        PopupZone {}

    }
}
