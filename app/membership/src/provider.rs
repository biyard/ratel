use crate::*;

#[component]
pub fn Provider(children: Element) -> Element {
    const MEMBERSHIP_JS: Asset = asset!("/assets/ratel-membership.js");
    rsx! {
        Fragment {
            document::Script { src: MEMBERSHIP_JS }
            {children}
        }
    }
}
