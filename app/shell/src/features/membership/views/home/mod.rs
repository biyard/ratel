use crate::features::membership::components::MembershipPlan;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "flex flex-col w-full min-h-screen bg-space-bg text-font-primary",
            div { class: "flex flex-col p-5 grow",
                document::Script { src: "https://cdn.portone.io/v2/browser-sdk.js" }
                MembershipPlan {}
            }
        }
    }
}
