#![allow(non_snake_case)]
use bdk::prelude::*;
use dioxus_popup::PopupService;
use dto::VoteOption;

use crate::{
    components::{
        button::{outlined_button::OutlinedButton, primary_button::PrimaryButton},
        confirm_popup::SigninPopupFooter,
        icons,
    },
    services::{user_service::UserService, vote_service::VoteService},
};

#[component]
pub fn VoteConfirm(
    vote: VoteOption,
    lang: Language,
    bill_id: i64,
    oncomplete: Option<EventHandler<()>>,
) -> Element {
    let tr: VoteConfirmTranslate = translate(&lang);
    let mut popup: PopupService = use_context();
    let vote_service: VoteService = use_context();
    let user_service: UserService = use_context();
    let description = tr.description.replace("{}", &vote.to_string());

    rsx! {
        div { id: "vote_confirm_popup", class: "max-w-390 w-full",
            div { class: "w-full flex flex-col gap-35",
                div { class: "w-full flex flex-col gap-24 items-center justify-center mt-35",
                    p { class: "text-white font-bold text-2xl", {tr.title} }
                    div { class: "flex w-full items-center justify-center",
                        icons::RatelCircle {
                            color: "#FCB300".to_string(),
                            size: "100".to_string(),
                        }
                        div { class: "absolute",
                            if vote == VoteOption::Supportive {
                                icons::ThumbsUp { color: "#FCB300".to_string() }
                            } else {
                                icons::ThumbsDown { color: "#FCB300".to_string() }
                            }
                        }
                    }
                    p { class: "text-neutral-400 text-center text-base font-medium",
                        {description}
                    }
                }

                div { class: "w-full flex flex-row gap-15",
                    OutlinedButton {
                        width: "100%",
                        onclick: move |_| {
                            popup.close();
                        },
                        {tr.btn_cancel}
                    }
                    PrimaryButton {
                        width: "100%",
                        onclick: move |_| {
                            tracing::debug!("voting button confirmed");
                            spawn(async move {
                                match vote_service.vote(bill_id, vote).await {
                                    Ok(ret) => {
                                        tracing::debug!("Voted successfully: {:?}", ret);
                                        if let Some(oncomplete) = oncomplete {
                                            oncomplete(());
                                        }
                                        popup.close();
                                    }
                                    Err(e) => {
                                        tracing::error!("Failed to vote: {:?}", e);
                                    }
                                }
                            });
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
        ko: "선택하신 항목은 {} 입니다. 제출 후에는 투표 변경이 어려울 수 있습니다. 계속 진행하기 전에 선택하신 항목을 다시 확인해주세요.",
        en: "You've selected {}. Once submitted, changing your vote may not be possible. Please review your choice before proceeding.",
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
