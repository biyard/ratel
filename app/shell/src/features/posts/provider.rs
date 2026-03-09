use crate::features::posts::*;

pub const RATEL_POST: Asset = asset!("/assets/ratel-post.js", AssetOptions::js());

#[component]
pub fn Provider(children: Element) -> Element {
    rsx! {
        Fragment {
            document::Script { src: RATEL_POST }
            {children}
        }
    }
}
