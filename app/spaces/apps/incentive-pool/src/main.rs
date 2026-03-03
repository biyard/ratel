use dioxus::prelude::*;
use space_app_incentive_pool::*;

fn main() {
    dioxus::launch(App);
}

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
pub const INCENTIVE_POOL_JS: Asset = asset!("/assets/space-incentive-pool.js", AssetOptions::js());

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Script { src: INCENTIVE_POOL_JS }
        Router::<Route> {}
    }
}
