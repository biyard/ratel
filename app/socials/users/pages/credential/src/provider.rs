use crate::*;

#[component]
pub fn Provider(children: Element) -> Element {
    const PORTONE_SDK: &str = "https://cdn.portone.io/v2/browser-sdk.js";
    const CREDENTIAL_JS: &str = include_str!("../assets/ratel-user-credential.js");

    rsx! {
        Fragment {
            document::Script { src: PORTONE_SDK }
            document::Script { dangerous_inner_html: CREDENTIAL_JS }
            {children}
        }
    }
}
