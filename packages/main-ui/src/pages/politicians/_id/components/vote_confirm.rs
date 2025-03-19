#![allow(non_snake_case)]
use bdk::prelude::*;
use dioxus_popup::PopupService;
use dto::VoteOption;

use crate::components::{
    button::{outlined_button::OutlinedButton, primary_button::PrimaryButton},
    confirm_popup::{SigninPopupFooter, WelcomeHeader},
};

#[component]
pub fn VoteConfirm(vote: VoteOption, lang: Language) -> Element {
    let tr: VoteConfirmTranslate = translate(&lang);
    let mut popup: PopupService = use_context();

    rsx! {
        div { id: "vote_confirm_popup", class: "max-w-390 w-full",
            div { class: "w-full flex flex-col gap-35",
                WelcomeHeader { lang, title: tr.title, description: tr.description }

                div { class: "w-full flex flex-row gap-15",
                    OutlinedButton {
                        width: "100%",
                        onclick: move |_| {
                            popup.close();
                        },
                        {tr.btn_confirm}
                    }
                    PrimaryButton {
                        width: "100%",
                        onclick: move |_| {
                            popup.close();
                        },
                        {tr.btn_confirm}
                    }
                }
            }

            SigninPopupFooter { lang }
        }
    }
}

translate! {
    VoteConfirmTranslate;

    title: {
        ko: "투표 확인",
        en: "Confirm Your Vote",
    },

    description: {
        ko: "투표가 기록됩니다. 이후 수정이 불가능할 수 있습니다. 계속 진행하기 전에 선택을 확인해주세요.",
        en: "Your vote will be recorded, and modifications may not be possible afterward. Please confirm your selection before proceeding.",
    },

   btn_confirm:{
        ko: "확인",
        en: "Confirm",
    },

    btn_cancel: {
        ko: "취소",
        en: "Cancel",
    },
}
