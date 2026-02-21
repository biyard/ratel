use space_action_poll::{Route, config};

use dioxus::prelude::*;

fn main() {
    let config = config::get();
    dioxus::logger::init(config.common.log_level.into()).expect("logger failed to init");

    #[cfg(not(feature = "server"))]
    space_action_poll::web::launch(App);

    #[cfg(feature = "server")]
    space_action_poll::server::serve(App);
}

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
