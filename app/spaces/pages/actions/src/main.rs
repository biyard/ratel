use dioxus::prelude::*;
use space_page_actions::*;

fn main() {
    dioxus::launch(App);
}

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
fn App() -> Element {
    use_context_provider(|| LayoverService::new());

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> { Layover {} }

    }
}
