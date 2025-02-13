#![allow(non_snake_case)]

use crate::{pages::patrons::_id::controller::Controller, route::Route};

use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn PatronsLoginPage(id: String, lang: Language) -> Element {
    let mut _ctrl = Controller::new()?;

    rsx! {
        div { "LoginSupportUsInputBox" }
    }
}

#[component]
pub fn LoginSupportUsInputBox(cx: Route, lang: Language) -> Element {
    let tr: SupportUsTranslate = translate(&lang);
    let mut checked = use_signal(|| false);

    rsx! {
        div { class: "w-full min-h-[30px] flex flex-col justify-start items-start",
            div { class: "grow shrink basis-0 text-white text-xl font-semibold font-['Inter']",
                "{tr.title}"
            }
            div { class: "self-stretch border-b border-[#414462] justify-center items-center gap-2.5" }
            div { class: "min-h-[30px] self-stretch text-[#414462] text-xs font-normal font-['Inter']",
                "{tr.sub_text}"
            }
            div {
                label {
                    input {
                        r#type: "checkbox",
                        checked: "{checked}",
                        onchange: move |_| checked.set(!checked()),
                    }
                    " If you agree to the above terms and conditions, click \"Agree and Continue\" to finalize your support."
                }
            }
        }
    }
}
