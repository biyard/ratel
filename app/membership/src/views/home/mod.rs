use crate::components::MembershipPlan;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Fragment {
            document::Script { src: "https://cdn.portone.io/v2/browser-sdk.js" }
            MembershipPlan {}
        }
    }
}
