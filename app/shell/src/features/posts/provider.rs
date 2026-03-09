use crate::features::posts::*;

#[component]
pub fn Provider(children: Element) -> Element {
    rsx! {
        Fragment { {children} }
    }
}
