use common::{DevTools, ThemeService, ToastProvider, ToastService};
use common::{Environment, PopupZone, components::PopupService, query_provider};
use dioxus::prelude::*;
use space_action_shell::Route;
use space_common::ratel_auth;

pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    let cfg = common::CommonConfig::default();
    let log_level = cfg.log_level;
    dioxus::logger::init(log_level.into()).expect("logger failed to init");

    #[cfg(not(feature = "server"))]
    space_common::components::launch(App);

    #[cfg(feature = "server")]
    space_common::components::serve(App);
}

#[component]
fn App() -> Element {
    rsx! {
        space_common::components::App { tailwind: TAILWIND_CSS, Router::<Route> {} }
    }
}
