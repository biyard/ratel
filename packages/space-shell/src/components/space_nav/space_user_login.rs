use crate::*;
use ratel_auth::LoginModal;

#[component]
pub fn SpaceUserLogin() -> Element {
    let mut popup = use_popup();
    let lang = use_language();
    let tr: SpaceUserLoginTranslate = use_translate();

    rsx! {
        button {
            class: "flex justify-end items-center p-4 w-full cursor-pointer hover:opacity-80",
            onclick: move |_| {
                popup.open(rsx! {
                    LoginModal {}
                }).with_title(tr.title);
            },
            "Sign In"
        }
    }
}

use crate::*;

translate! {
    SpaceUserLoginTranslate;

    title: {
        en: "Join the Movement",
        ko: "로그인 및 회원가입",
    },
}
