use bdk::prelude::*;

use crate::{pages::components::SocialHeader, route::Route};
use dioxus_popup::PopupZone;

#[component]
pub fn SocialLayout(#[props(default = Language::En)] lang: Language) -> Element {
    rsx! {
        div { class: "flex flex-col justify-start items-center",
            SocialHeader { lang, onsearch: |_| {} }

            div { class: "w-full max-w-[1440px]", Outlet::<Route> {} }
        }
        PopupZone {}
    }
}
