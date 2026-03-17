use super::*;

#[component]
pub fn Provider(children: Element) -> Element {
    const DAO_JS: Asset = asset!("/assets/ratel-team-dao.js", AssetOptions::js());
    rsx! {
        Fragment {
            document::Script { src: DAO_JS }
            {children}
        }
    }
}
