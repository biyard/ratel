use crate::features::spaces::space_common::*;

#[component]
pub fn SpaceUserLogin(onclick: EventHandler<()>) -> Element {
    let mut popup = use_popup();
    let lang = use_language();
    let tr: SpaceUserLoginTranslate = use_translate();

    rsx! {
        button {
            class: "flex justify-end items-center p-4 w-full cursor-pointer hover:opacity-80",
            onclick: move |_| {
                onclick.call(());
            },
            "Sign In"
        }
    }
}

translate! {
    SpaceUserLoginTranslate;

    title: {
        en: "Join the Movement",
        ko: "로그인 및 회원가입",
    },
}
