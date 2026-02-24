use crate::*;

#[component]
pub fn Provider(children: Element) -> Element {
    const SETTING_JS: Asset = asset!("/assets/ratel-user-setting.js");
    rsx! {
        Fragment {
            document::Script { src: SETTING_JS }
            {children}
        }
    }
}
