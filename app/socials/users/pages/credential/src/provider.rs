use crate::*;

#[component]
pub fn Provider(children: Element) -> Element {
    const PORTONE_SDK: &str = "https://cdn.portone.io/v2/browser-sdk.js";

    rsx! {
        Fragment {
            document::Script { src: PORTONE_SDK }
            document::Script { src: asset!("/assets/ratel-user-credential.js") }
            {children}
        }
    }
}
