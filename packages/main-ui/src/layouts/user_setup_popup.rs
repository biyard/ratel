#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_popup::PopupService;

use crate::{
    components::checkbox::Checkbox, layouts::congraturation_popup::CongraturationPopup,
    theme::Theme,
};

#[component]
pub fn UserSetupPopup(
    #[props(default ="user_setup_popup".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
    #[props(default ="".to_string())] nickname: String,
    #[props(default ="".to_string())] profile_url: String,
    #[props(default ="".to_string())] email: String,
) -> Element {
    let mut popup: PopupService = use_context();
    let mut valid = use_signal(|| true);
    let mut nickname = use_signal(|| nickname.to_string());
    let theme: Theme = use_context();
    let theme = theme.get_data();

    rsx! {
        div { id, class,
            div { class: "flex flex-col items-start justify-start w-full pt-[10px] gap-[25px]",

                // Email
                if !email.is_empty() {
                    div { class: "flex flex-col gap-[8px]",
                        div { class: "flex flex-row items-start",
                            span { class: "text-[14px] font-bold leading-[24px]", "이메일" }
                        }
                        div { class: "flex flex-col items-start w-full mt-[10px] gap-[8px]",
                            input {
                                class: "w-[400px] max-[400px]:w-[300px] h-[59px] px-[24px] py-[17.5px] bg-[#2C2E42] text-[18px] font-bold leading-[24px] rounded-[4px] placeholder-[{theme.primary07}] rounded-[8px]",
                                value: email,
                                disabled: true,
                            }
                        }
                    }
                }

                // Nickname
                div { class: "flex flex-col gap-[8px]",
                    div { class: "flex flex-row items-start",
                        span { class: "text-[14px] font-bold leading-[24px]", "닉네임" }
                        span { class: "text-[14px] text-[#ff0000]", "*" }
                    }
                    div { class: "flex flex-col items-start w-full mt-[10px] gap-[8px]",
                        input {
                            class: "w-[400px] max-[400px]:w-[300px] h-[59px] px-[24px] py-[17.5px] bg-[#2C2E42] text-[18px] font-bold leading-[24px] rounded-[4px] placeholder-[{theme.primary07}] rounded-[8px]",
                            placeholder: "닉네임을 입력해주세요.",
                            value: nickname(),
                            oninput: move |e| {
                                let value = e.value();
                                valid.set(value.chars().all(|c| c.is_alphanumeric()));
                                nickname.set(value);
                            },

                        }
                        if !valid() {
                            span { class: "text-[14px] font-bold leading-[24px] text-[{theme.primary04}]",
                                "특수문자는 입력할 수 없습니다."
                            }
                        }
                    }
                }

                div { class: "flex flex-row gap-[10px] items-center",
                    Checkbox { title: "[필수] 이메일 및 계정주소 수집에 동의합니다." }
                                // button { class: "px-[10px] py-[2px] rounded-[4px] bg-[{theme.primary11}] hover:bg-[{theme.primary05}]",
                //     div { class: "text-[14px] font-bold h-[24px] text-center text-white align-middle flex items-center justify-center",
                //         "자세히보기"
                //     }
                // }
                }

                button {
                    class: "w-full mt-[10px] rounded-[12px] bg-[{theme.primary03}] opacity-50 hover:opacity-100 text-[18px] font-extrabold leading-[24px] text-[{theme.primary05}] h-[59px] flex items-center justify-center",
                    onclick: move |_| {
                        popup
                            .open(rsx! {
                                CongraturationPopup {}
                            })
                            .with_id("congraturation_popup")
                            .with_title("환영합니다!")
                            .without_close();
                    },
                    "다음"
                }
            }
        }
    }
}
