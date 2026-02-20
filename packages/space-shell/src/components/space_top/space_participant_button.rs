use crate::*;

#[component]
pub fn SpaceParticipantButton() -> Element {
    rsx! {
        Button { style: ButtonStyle::Primary, onclick: move |_| {}, "Participate" }
    }
}
