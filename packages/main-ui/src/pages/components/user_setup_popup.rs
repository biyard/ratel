#![allow(non_snake_case)]
use bdk::prelude::*;
use dioxus_popup::PopupService;
use dto::{ServiceError, Subscription};

use super::{
    signin_popup_footer::SigninPopupFooter, welcome_header::WelcomeHeader,
    welcome_popup::WelcomePopup,
};
use crate::{
    components::{button::primary_button::PrimaryButton, checkbox::Checkbox},
    config,
    pages::components::LabeledInput,
    services::user_service::UserService,
    theme::Theme,
};

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
    let value = email.clone();
    rsx! {
        div { id, class: "w-390 max-[450px]:w-350",
            WelcomeHeader { lang, title: tr.title, description: tr.message }
            div { class: "flex flex-col items-start justify-start w-full pt-10 gap-20 mt-35",
                // Email
                div { class: "w-full flex flex-col gap-5",
                    div { class: "flex flex-row items-start",
                        span { class: "text-[15px]/28 font-bold text-neutral-400", {tr.email} }
                    }
                    input {
                        class: "border border-c-wg-70 bg-bg text-secondary w-full max-[400px]:w-300 h-59 px-24 py-[17.5px] text-lg/24 font-medium placeholder-neutral-600 rounded-lg",
                        disabled: !value.clone().is_empty(),
                        value,
                    }
                }

                div { class: "grid gap-8 w-full grid-cols-2",
                    LabeledInput {
                        label_name: tr.first_name,
                        placeholder: tr.enter_nickname,
                        oninput: move |value: String| {
                            fname_valid.set(value.chars().all(|c| c.is_alphanumeric()));
                            firstname.set(value);
                        },
                        if !fname_valid() {
                            span { class: "text-sm/24 font-bold text-c-p-50", {tr.special_characters} }
                        }
                    }

                    LabeledInput {
                        label_name: tr.last_name,
                        placeholder: tr.enter_nickname,
                        oninput: move |value: String| {
                            lname_valid.set(value.chars().all(|c| c.is_alphanumeric()));
                            lastname.set(value);
                        },
                        if !lname_valid() {
                            span { class: "text-sm/24 font-bold text-c-p-50", {tr.special_characters} }
                        }
                    }
                }

                div { class: "flex flex-col gap-10 items-start",
                    Checkbox {
                        id: "agree_checkbox",
                        onchange: move |check| {
                            agreed.set(check);
                        },
                        span { class: "text-sm text-cb-text",
                            {tr.agree}
                            span { class: "font-bold", {tr.term_of_service} }
                        }
                    }
                    Checkbox {
                        id: "announcement_checkbox",
                        onchange: move |check| {
                            announcement_agree.set(check);
                        },
                        span { class: "text-sm text-cb-text", {tr.receive_announcement} }
                    }
                }

                PrimaryButton {
                    width: "100%",
                    disabled: !agreed(),
                    onclick: move |_| {
                        if agreed() {
                            let nickname = format!("{} {}", firstname(), lastname());
                            let principal = principal.clone();
                            let email = email.clone();
                            let profile_url = profile_url.clone();
                            spawn(async move {
                                if announcement_agree() {
                                    let endpoint = config::get().main_api_endpoint;
                                    match Subscription::get_client(&endpoint)
                                        .subscribe(email.clone())
                                        .await
                                    {
                                        Ok(_) => {}
                                        Err(e) => {
                                            tracing::error!("UserSetupPopup::subscribe: error={:?}", e);
                                        }
                                    }
                                }
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
                                            WelcomePopup { lang }
                                        })
                                        .with_id("welcome_popup")
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
        en: "Name",
    },

    special_characters: {
        ko: "특수문자는 입력할 수 없습니다.",
        en: "Special characters are not allowed.",
    },

    agree: {
        ko: "을 읽어보았으며 동의합니다",
        en: "[Required] I have read and accept the",
    },

    // TODO: need bold text
    term_of_service: {
        ko: "[필수] 서비스 이용약관",
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
