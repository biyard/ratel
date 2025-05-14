use bdk::prelude::*;

use crate::route::Route;
use dioxus_popup::PopupZone;

#[component]
pub fn SocialLayout(#[props(default = Language::En)] lang: Language) -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-start w-full min-h-[100vh] text-white max-[1440px]:px-[10px]",
            "Social layout"
            div { class: "w-full max-w-[1440px]", Outlet::<Route> {} }
        }
        PopupZone {}
    }
}
