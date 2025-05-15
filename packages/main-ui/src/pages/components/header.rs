use bdk::prelude::*;

use super::*;
use crate::components::icons::RatelSymbolWithText;

#[component]
pub fn SocialHeader(onsearch: EventHandler<String>) -> Element {
    rsx! {
        nav { class: "w-full max-w-desktop m-10 flex flex-row justify-between items-center",
            div { class: "flex flex-row gap-20 items-center",
                RatelSymbolWithText {}
                SearchBox { onsearch }
            }

            div { class: "flex flex-row gap-10 items-center" }
        }
    }
}
