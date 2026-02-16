use crate::*;

#[component]
pub fn SpaceUserLogin() -> Element {
    rsx! {
        button { class: "w-full flex justify-end  items-center", "Sign In" }
    }
}
