use crate::*;

#[component]
pub fn Provider(children: Element) -> Element {
    rsx! {
        Fragment {
            document::Script { src: "/assets/ratel-post.js" }
            {children}
        }
    }
}
