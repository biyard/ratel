use crate::config;
use crate::*;
use common::*;
use ratel_auth::AuthProvider;

use dioxus::prelude::*;

pub const MAIN_CSS: Asset = asset!("/assets/main.css");
pub const MAIN_JS: Asset = asset!("/assets/ratel-app-shell.js");

#[component]
pub fn App() -> Element {
    use_context_provider(|| PopupService::new());
    ToastService::init();
    ThemeService::init();
    let _ = ratel_auth::Context::init()?;
    common::contexts::TeamContext::init();
    let conf = config::get();
    let env = conf.common.env;
    common::query::query_provider();
    let keywords = vec![
        "ratel".to_string(),
        "knowledge platform".to_string(),
        "ai knowledge base".to_string(),
        "human knowledge dataset".to_string(),
        "ai training data".to_string(),
        "community intelligence".to_string(),
        "participatory platform".to_string(),
        "survey rewards".to_string(),
        "poll rewards".to_string(),
        "web3 knowledge economy".to_string(),
        "ai memory platform".to_string(),
        "vector knowledge database".to_string(),
        "collective intelligence".to_string(),
    ];

    rsx! {
        document::Link { rel: "icon", href: common::assets::FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        common::SeoMeta {
            title: "Ratel – AI Knowledge Platform Powered by Human Essences",
            description: "Ratel is a participatory knowledge platform where users share expertise, opinions, and insights as structured “Essences”. AI agents learn from the knowledge base while users earn rewards through surveys, polls, and discussions.",
            image: "https://metadata.ratel.foundation/logos/logo-symbol.png",
            url: "https://ratel.foundation",
            robots: Robots::IndexNofollow,
            keywords,
        }
        document::Script { src: MAIN_JS }
        document::Script { src: "https://cdn.portone.io/v2/browser-sdk.js" }

        common::Provider {}
        AuthProvider {}
        crate::features::posts::Provider {}

        Router::<Route> {}
        ToastProvider {}

        PopupZone {}
        if env == Environment::Dev || env == Environment::Local {
            DevTools {}
        }
    }
}
