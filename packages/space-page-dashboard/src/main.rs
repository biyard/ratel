use dioxus::prelude::*;
use space_page_dashboard::*;

fn main() {
    dioxus::launch(App);
}

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const MAIN_CSS: Asset = asset!("/assets/main.css");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
