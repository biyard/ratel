use crate::config;
use crate::*;
use common::*;
use ratel_auth::AuthProvider;

use dioxus::prelude::*;

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
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
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Script { src: MAIN_JS }
        ratel_user_setting::Provider {}
        ratel_user_credential::Provider {}
        ratel_membership::Provider {}
        common::Provider {}
        AuthProvider {}
        ratel_post::Provider {}
        ratel_team_dao::Provider {}

        Router::<Route> {}
        ToastProvider {}
        if env == Environment::Dev || env == Environment::Local {
            DevTools {}
        }
    }
}
