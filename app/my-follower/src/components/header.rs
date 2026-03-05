use crate::components::MyFollowerHeaderTranslate;
use crate::*;

#[component]
pub fn MyFollowerHeader() -> Element {
    let nav = use_navigator();
    let tr: MyFollowerHeaderTranslate = use_translate();

    rsx! {
        div { class: "flex flex-row w-full justify-start items-center gap-2.5",
            div {
                class: "cursor-pointer w-fit h-fit",
                onclick: move |_| {
                    nav.go_back();
                },
                icons::arrows::ArrowLeft { class: "[&>path]:stroke-text-primary" }
            }

            div { class: "font-semibold text-text-primary text-[20px]", {tr.title} }
        }
    }
}
