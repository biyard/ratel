#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::*;
use dto::ServiceError;

use super::{
    signin_popup_footer::SigninPopupFooter, welcome_header::WelcomeHeader,
    welcome_popup::WelcomePopup,
};
use crate::{components::checkbox::Checkbox, services::user_service::UserService, theme::Theme};

#[component]
pub fn UserSetupPopup(
    #[props(default ="user_setup_popup".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
    nickname: String,
    profile_url: String,
    email: String,
    principal: String,
    lang: Language,
) -> Element {
    let mut popup: PopupService = use_context();
    let mut firstname = use_signal(|| "".to_string());
    let mut fname_valid = use_signal(|| true);
    let mut lastname = use_signal(|| "".to_string());
    let mut lname_valid = use_signal(|| true);
    let mut agreed = use_signal(|| false);
    let mut announcement_agree = use_signal(|| false);
    let mut user_service: UserService = use_context();
    let theme: Theme = use_context();
    let theme = theme.get_data();
    let btn_color = use_memo(move || {
        if agreed() {
            "#FCB300".to_string()
        } else {
            "#A1A1A1".to_string()
        }
    });
    let tr = translate::<UserSetupPopupTranslate>(&lang);
    let agree_msg = match lang {
        Language::Ko => format!("{}{}", tr.term_of_service, tr.agree),
        Language::En => format!("{} {}", tr.agree, tr.term_of_service),
    };
    rsx! {
        div { id, class,
            WelcomeHeader { lang, title: tr.title, description: tr.message }
            div { class: "flex flex-col items-start justify-start w-full pt-10 gap-20 mt-35",
                // Email
                div { class: "flex flex-col gap-5",
                    div { class: "flex flex-row items-start",
                        span { class: "text-[15px]/28 font-bold text-neutral-400", {tr.email} }
                    }
                    div { class: "flex items-start w-full mt-10 gap-8",
                        input {
                            class: "w-full max-[400px]:w-300 h-59 px-24 py-[17.5px] bg-background text-lg/24 font-medium placeholder-neutral-600 rounded-lg text-white",
                            value: "{email}",
                            disabled: !email.is_empty(),
                        }
                    }
                }

                div { class: "grid gap-8 w-full grid-cols-2",
                    div { class: "flex flex-col items-start gap-5",
                        span { class: "text-neutral-400 text-[15px]/28 font-bold", {tr.first_name} }
                        input {
                            class: "w-full max-[400px]:w-190 h-59 px-24 py-[17.5px] bg-background text-lg/24 font-medium placeholder-neutral-600 rounded-lg text-white",
                            placeholder: "{tr.enter_nickname}",
                            value: firstname(),
                            oninput: move |e| {
                                let value = e.value();
                                fname_valid.set(value.chars().all(|c| c.is_alphanumeric()));
                                firstname.set(value);
                            },
                            if !fname_valid() {
                                span { class: "text-sm/24 font-bold text-c-p-50",
                                    {tr.special_characters}
                                }
                            }
                        }
                    }
                    div { class: "flex flex-col items-start gap-5",
                        span { class: "text-neutral-400 text-[15px]/28 font-bold", {tr.last_name} }
                        input {
                            class: "w-full max-[390px]:w-190 h-59 px-24 py-[17.5px] bg-background text-lg/24 font-medium placeholder-neutral-600 rounded-lg text-white",
                            placeholder: "{tr.enter_nickname}",
                            value: lastname(),
                            oninput: move |e| {
                                let value = e.value();
                                lname_valid.set(value.chars().all(|c| c.is_alphanumeric()));
                                lastname.set(value);
                            },
                            if !lname_valid() {
                                span { class: "text-sm/24 font-bold text-c-p-50",
                                    {tr.special_characters}
                                }
                            }
                        }
                    }
                }

                div { class: "flex flex-col gap-10 items-start",
                    Checkbox {
                        title: "{agree_msg}",
                        class: "text-white text-sm/16 font-normal",
                        onchange: move |check| {
                            agreed.set(check);
                        },
                    }
                    Checkbox {
                        title: "{tr.receive_announcement}",
                        class: "text-white text-sm/16 font-normal",
                        onchange: move |check| {
                            announcement_agree.set(check);
                        },
                    }
                }

                button {
                    class: "w-full rounded-[10px] bg-[{btn_color}] text-base/19 font-bold text-black h-59 flex items-center justify-center",
                    onclick: move |_| {
                        if agreed() {
                            let nickname = format!("{} {}", firstname(), lastname());
                            let principal = principal.clone();
                            let email = email.clone();
                            let profile_url = profile_url.clone();
                            spawn(async move {
                                if let Err(e) = user_service
                                    .login_or_signup(&principal, &email, &nickname, &profile_url)
                                    .await
                                {
                                    match e {
                                        ServiceError::UserAlreadyExists => {
                                            popup.close();
                                            return;
                                        }
                                        e => {
                                            tracing::error!("UserSetupPopup::signup: error={:?}", e);
                                        }
                                    }
                                } else {
                                    tracing::debug!("UserSetupPopup::signup: success");
                                    popup
                                        .open(rsx! {
                                            WelcomePopup { lang: lang.clone() }
                                        })
                                        .with_id("congratulation_popup")
                                        .with_title(tr.title)
                                        .without_close();
                                }
                            });
                        }
                    },
                    {tr.button}
                }
            }
            SigninPopupFooter { lang }
        }
    }
}

translate! {
    UserSetupPopupTranslate;

    title: {
        ko: "프로필 설정",
        en: "Finish your Profile!",
    },

    message: {
        ko: "프로필을 완성하면 활동할 수 있습니다.",
        en: "Completing your profile makes it easier for you to take action."
    }

    email: {
        ko: "이메일",
        en: "Email",
    }

    first_name: {
        ko: "이름",
        en: "First Name",
    },

    last_name: {
        ko: "성",
        en: "Last Name",
    },

    enter_nickname: {
        ko: "닉네임을 입력해주세요",
        en: "Please enter your nickname",
    },

    special_characters: {
        ko: "특수문자는 입력할 수 없습니다.",
        en: "Special characters are not allowed.",
    },

    agree: {
        ko: "을 읽어보았으며 동의합니다",
        en: "I have read and accept the",
    },

    // TODO: need bold text
    term_of_service: {
        ko: "서비스 이용약관",
        en: "Terms of Service",
    },

    receive_announcement: {
        ko: "Ratel의 공지사항 및 소식을 이메일로 받고 싶습니다.",
        en: "I want to receive announcements and news from Ratel.",
    },

    button: {
        ko: "가입 완료",
        en: "Finished Sign-up",
    },
}
