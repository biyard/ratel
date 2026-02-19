use app_shell::config;
use app_shell::*;
use common::*;
// use ratel_auth::AuthProvider;

use dioxus::prelude::*;

fn main() {
    let config = config::get();
    dioxus::logger::init(config.common.log_level.into()).expect("logger failed to init");

    #[cfg(not(feature = "server"))]
    web::launch(App);

    #[cfg(feature = "server")]
    server::serve(App);
}

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const MAIN_CSS: Asset = asset!("/assets/main.css");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
pub const MAIN_JS: Asset = asset!("/assets/ratel-app-shell.js");

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Script { src: MAIN_JS }
        // AuthProvider {}
        Router::<Route> {}
    }
}
