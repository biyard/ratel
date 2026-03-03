use crate::components::MembershipPlan;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        MembershipPlan {}
    }
}
