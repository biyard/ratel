use crate::*;

#[component]
pub fn Setup(children: Element) -> Element {
    rsx! {
        document::Script { src: asset!("/assets/ratel-common.js") }

        {children}
    }
}
