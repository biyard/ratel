use common::{components::PopupService, query_provider, Environment, PopupZone};
use common::{DevTools, ThemeService, ToastProvider, ToastService};
use dioxus::prelude::*;
use space_common::ratel_auth;
use space_shell::Route;

pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    common::run(App);
}

#[component]
fn App() -> Element {
    rsx! {
        space_common::components::App { tailwind: TAILWIND_CSS, Router::<Route> {} }
    }
}
