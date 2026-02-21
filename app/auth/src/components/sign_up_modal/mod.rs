use crate::controllers::send_code::{SendCodeRequest, send_code_handler};
use crate::controllers::signup::{SignupRequest, SignupType, signup_handler};
use crate::controllers::verify_code::{VerifyCodeRequest, verify_code_handler};
use crate::*;

#[component]
pub fn SignupModal() -> Element {
    let tr: SignupModalTranslate = use_translate();
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut display_name = use_signal(|| String::new());
    let mut username = use_signal(|| String::new());
    let mut auth_code = use_signal(|| String::new());
    let profile_url =
        use_signal(|| "https://metadata.ratel.foundation/ratel/default-profile.png".to_string());
    let mut agreed_tos = use_signal(|| false);
    let mut agreed_news = use_signal(|| false);
    let mut sent_code = use_signal(|| false);
    let mut is_valid_email = use_signal(|| false);
    let mut loading = use_signal(|| false);
    let mut email_warning = use_signal(|| String::new());
    let mut password_warning = use_signal(|| String::new());
    let mut username_warning = use_signal(|| String::new());
    let mut terms_error = use_signal(|| String::new());
    let mut error_message: Signal<Option<String>> = use_signal(|| None);
    let mut popup = use_popup();

    let is_valid_email_format = |email: &str| -> bool {
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 {
            return false;
        }
        let local = parts[0];
        let domain = parts[1];
        !local.is_empty()
            && !domain.is_empty()
            && domain.contains('.')
            && !local.contains(char::is_whitespace)
            && !domain.contains(char::is_whitespace)
    };

    let is_valid_password = |pw: &str| -> bool {
        pw.len() >= 8
            && pw.chars().any(|c| c.is_ascii_alphabetic())
            && pw.chars().any(|c| c.is_ascii_digit())
            && pw
                .chars()
                .any(|c| "!@#$%^&*()_+{}[]:;<>,.?~\\/-".contains(c))
    };

    let is_valid_username = |name: &str| -> bool {
        !name.is_empty()
            && name
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-')
    };

    let submit_disabled = !agreed_tos()
        || username().is_empty()
        || !is_valid_username(&username())
        || display_name().trim().is_empty()
        || !is_valid_email();

    rsx! {
        div {
            class: "overflow-y-scroll w-full max-h-screen scrollbar-hide",
            id: "signup_popup",
            div {
                class: "flex flex-col gap-4 w-full max-w-100 mx-auto",

                // Profile Image
                div { class: "flex relative justify-center items-center mx-auto group size-40 max-mobile:size-20",
                    img {
                        src: "{profile_url}",
                        alt: "Profile",
                        class: "object-cover relative w-40 h-40 rounded-full cursor-pointer group max-mobile:size-20",
                    }
                    div { class: "flex absolute inset-0 justify-center items-center w-40 h-40 font-semibold text-center text-white rounded-full opacity-0 transition-opacity duration-300 group-hover:opacity-100 bg-component-bg/50",
                        {tr.clicked_image}
                    }
                }

                if let Some(err) = error_message() {
                    div { class: "text-sm text-red-500", "{err}" }
                }

                // Email + Verification Code
                div { class: "flex flex-col w-full gap-1.25",
                    label { class: "font-bold text-c-cg-30 text-base/7", {tr.email} }
                    div { class: "flex flex-row gap-2.5 w-full items-center",
                        input {
                            autocomplete: "email",
                            class: "flex px-5 w-full min-w-0 h-11 text-base font-medium border outline-none bg-input-box-bg border-input-box-border rounded-lg placeholder-gray-500 text-text-primary disabled:opacity-50 disabled:cursor-not-allowed",
                            disabled: loading() || is_valid_email(),
                            name: "email",
                            placeholder: "{tr.email_placeholder}",
                            r#type: "email",
                            value: email(),
                            oninput: move |ev| {
                                let val = ev.data().value();
                                email.set(val.clone());
                                if !val.is_empty() && !is_valid_email_format(&val) {
                                    email_warning.set(tr.invalid_email_format.to_string());
                                } else {
                                    email_warning.set(String::new());
                                }
                            },
                        }
                        if !is_valid_email() {
                            button {
                                class: "inline-flex gap-2.5 justify-center items-center py-1.5 px-4 h-auto text-xs font-bold whitespace-nowrap rounded-full transition-all outline-none bg-btn-secondary-bg text-btn-secondary-text border-btn-secondary-outline hover:bg-btn-secondary-hover-bg hover:text-btn-secondary-hover-text disabled:opacity-50 disabled:pointer-events-none",
                                disabled: loading(),
                                onclick: move |_| async move {
                                    if !is_valid_email_format(&email()) {
                                        email_warning.set(tr.invalid_email_format.to_string());
                                        return;
                                    }
                                    email_warning.set(String::new());
                                    loading.set(true);
                                    let result = send_code_handler(SendCodeRequest::Email {
                                        email: email.read().clone(),
                                    })
                                    .await;
                                    loading.set(false);
                                    match result {
                                        Ok(_) => {
                                            sent_code.set(true);
                                        }
                                        Err(e) => {
                                            email_warning.set(format!("{}: {e}", tr.failed_send_code));
                                        }
                                    }
                                },
                                {tr.send}
                            }
                        }
                    }
                    if !email_warning().is_empty() {
                        p { class: "mt-1 text-sm text-red-500", {email_warning} }
                    }

                    // Verification code row
                    div {
                        aria_hidden: if !sent_code() || is_valid_email() { "true" } else { "false" },
                        class: "flex flex-row gap-2.5 w-full items-center aria-hidden:hidden",
                        input {
                            class: "flex px-5 w-full min-w-0 h-11 text-base font-medium border outline-none bg-input-box-bg border-input-box-border rounded-lg placeholder-gray-500 text-text-primary disabled:opacity-50 disabled:cursor-not-allowed",
                            disabled: loading(),
                            name: "otp",
                            placeholder: "{tr.code_placeholder}",
                            r#type: "text",
                            value: auth_code(),
                            oninput: move |ev| {
                                auth_code.set(ev.data().value());
                            },
                        }
                        button {
                            class: "inline-flex gap-2.5 justify-center items-center py-1.5 px-4 h-auto text-xs font-bold whitespace-nowrap rounded-full transition-all outline-none bg-btn-secondary-bg text-btn-secondary-text border-btn-secondary-outline hover:bg-btn-secondary-hover-bg hover:text-btn-secondary-hover-text disabled:opacity-50 disabled:pointer-events-none",
                            disabled: loading(),
                            onclick: move |_| async move {
                                loading.set(true);
                                let result = verify_code_handler(VerifyCodeRequest::Email {
                                    email: email.read().clone(),
                                    code: auth_code.read().clone(),
                                })
                                .await;
                                loading.set(false);
                                match result {
                                    Ok(resp) => {
                                        if resp.success {
                                            is_valid_email.set(true);
                                        } else {
                                            email_warning.set(tr.failed_verify_code.to_string());
                                        }
                                    }
                                    Err(e) => {
                                        email_warning.set(format!("{}: {e}", tr.failed_verify_code));
                                    }
                                }
                            },
                            {tr.verify}
                        }
                    }
                }

                // Password
                div { class: "flex flex-col w-full gap-1.25",
                    label { class: "font-bold text-c-cg-30 text-base/7", {tr.password} }
                    input {
                        class: "flex px-5 w-full min-w-0 h-11 text-base font-medium border outline-none bg-input-box-bg border-input-box-border rounded-lg placeholder-gray-500 text-text-primary disabled:opacity-50 disabled:cursor-not-allowed",
                        disabled: loading(),
                        placeholder: "{tr.password_placeholder}",
                        r#type: "password",
                        value: password(),
                        oninput: move |ev| {
                            let val = ev.data().value();
                            password.set(val.clone());
                            if val.len() > 7 && !is_valid_password(&val) {
                                password_warning.set(tr.invalid_password_format.to_string());
                            } else {
                                password_warning.set(String::new());
                            }
                        },
                    }
                    if !password_warning().is_empty() {
                        p { class: "mt-1 text-sm text-red-500", {password_warning} }
                    }
                }

                // Display Name
                div { class: "flex flex-col w-full gap-1.25",
                    label { class: "font-bold text-c-cg-30 text-base/7", {tr.display_name} }
                    input {
                        class: "flex px-5 w-full min-w-0 h-11 text-base font-medium border outline-none bg-input-box-bg border-input-box-border rounded-lg placeholder-gray-500 text-text-primary disabled:opacity-50 disabled:cursor-not-allowed",
                        disabled: loading(),
                        placeholder: "{tr.display_name_placeholder}",
                        r#type: "text",
                        value: display_name(),
                        oninput: move |ev| {
                            display_name.set(ev.data().value());
                        },
                    }
                }

                // Username
                div { class: "flex flex-col w-full gap-1.25",
                    label { class: "font-bold text-c-cg-30 text-base/7", {tr.user_name} }
                    input {
                        class: "flex px-5 w-full min-w-0 h-11 text-base font-medium border outline-none bg-input-box-bg border-input-box-border rounded-lg placeholder-gray-500 text-text-primary disabled:opacity-50 disabled:cursor-not-allowed",
                        disabled: loading(),
                        placeholder: "{tr.username_placeholder}",
                        r#type: "text",
                        value: username(),
                        oninput: move |ev| {
                            let val = ev.data().value();
                            username.set(val.clone());
                            if !val.is_empty() && !is_valid_username(&val) {
                                username_warning.set(tr.invalid_username_format.to_string());
                            } else {
                                username_warning.set(String::new());
                            }
                        },
                    }
                    if !username_warning().is_empty() {
                        p { class: "mt-1 text-sm text-c-p-50 light:text-red-600", {username_warning} }
                    }
                }

                // Terms Checkboxes
                div { class: "flex flex-col items-start mt-5 mb-5 gap-2.25",
                    label { class: "flex flex-row gap-2 items-center cursor-pointer",
                        input {
                            r#type: "checkbox",
                            checked: agreed_tos(),
                            onchange: move |ev| {
                                let checked = ev.data().value() == "true";
                                agreed_tos.set(checked);
                                if checked {
                                    terms_error.set(String::new());
                                }
                            },
                        }
                        span { class: "text-sm text-gray-400",
                            strong { {tr.agree_tos_required} }
                            {tr.agree_tos_text}
                        }
                    }
                    if !terms_error().is_empty() {
                        p { class: "-mt-1 text-sm text-red-500", {terms_error} }
                    }
                    label { class: "flex flex-row gap-2 items-center cursor-pointer",
                        input {
                            r#type: "checkbox",
                            checked: agreed_news(),
                            onchange: move |ev| {
                                let checked = ev.data().value() == "true";
                                agreed_news.set(checked);
                            },
                        }
                        span { class: "text-sm text-gray-400", {tr.agree_news} }
                    }
                }

                // Submit Button
                button {
                    class: "flex justify-center items-center py-3 px-5 w-full font-bold text-white rounded-lg transition-all bg-primary hover:bg-primary/90 disabled:opacity-50 disabled:pointer-events-none",
                    disabled: submit_disabled || loading(),
                    onclick: move |_| async move {
                        terms_error.set(String::new());
                        error_message.set(None);

                        if display_name().trim().is_empty() {
                            return;
                        }

                        if username().is_empty() || !is_valid_username(&username()) {
                            return;
                        }

                        if !agreed_tos() {
                            terms_error.set(tr.terms_required.to_string());
                            return;
                        }

                        loading.set(true);
                        let req = SignupRequest {
                            signup_type: SignupType::Email {
                                email: email.read().clone(),
                                password: password.read().clone(),
                                code: auth_code.read().clone(),
                            },
                            display_name: display_name.read().clone(),
                            username: username.read().clone(),
                            profile_url: profile_url.read().clone(),
                            description: String::new(),
                            term_agreed: agreed_tos(),
                            informed_agreed: agreed_news(),
                            evm_address: None,
                            phone_number: None,
                            device_id: None,
                        };

                        let result = signup_handler(req).await;
                        loading.set(false);

                        match result {
                            Ok(_) => {
                                popup.close();
                            }
                            Err(e) => {
                                error_message.set(Some(format!("{e}")));
                            }
                        }
                    },
                    if loading() { {tr.loading} } else { {tr.finish_signup} }
                }

                // Footer
                div { class: "flex flex-row gap-2.5 justify-center items-center w-full",
                    div { class: "font-medium cursor-pointer text-neutral-400 text-xs/3.5",
                        {tr.privacy_policy}
                    }
                    div { class: "font-medium cursor-pointer text-neutral-400 text-xs/3.5",
                        {tr.terms_of_service}
                    }
                }
            }
        }
    }
}

translate! {
    SignupModalTranslate;

    email: {
        en: "Email",
        ko: "이메일",
    },
    send: {
        en: "Send",
        ko: "전송하기",
    },
    verify: {
        en: "Verify",
        ko: "인증하기",
    },
    verification_code: {
        en: "Verification Code",
        ko: "인증 코드",
    },
    password: {
        en: "Password",
        ko: "비밀번호",
    },
    password_placeholder: {
        en: "Enter your password",
        ko: "비밀번호를 입력하세요",
    },
    display_name: {
        en: "Display Name",
        ko: "이름",
    },
    display_name_placeholder: {
        en: "Enter your display name",
        ko: "이름을 입력하세요",
    },
    user_name: {
        en: "User Name",
        ko: "사용자 이름",
    },
    username_placeholder: {
        en: "Enter your user name",
        ko: "사용자 이름을 입력하세요",
    },
    email_placeholder: {
        en: "Enter your email address",
        ko: "이메일 주소를 입력하세요",
    },
    code_placeholder: {
        en: "Enter the verification code",
        ko: "인증 코드를 입력하세요",
    },
    agree_tos_required: {
        en: "[Required] ",
        ko: "[필수] ",
    },
    agree_tos_text: {
        en: "I have read and accept the Terms of Service.",
        ko: "서비스 이용약관을 읽고 동의합니다.",
    },
    agree_news: {
        en: "I want to receive announcements and news from Ratel.",
        ko: "Ratel의 공지사항과 소식을 받아보고 싶습니다.",
    },
    finish_signup: {
        en: "Finished Sign-up",
        ko: "회원가입하기",
    },
    loading: {
        en: "Loading...",
        ko: "로딩 중...",
    },
    invalid_password_format: {
        en: "Password must contain letters, numbers, and special characters (min 8 chars).",
        ko: "비밀번호는 문자, 숫자, 특수문자를 포함하여 최소 8자 이상이어야 합니다.",
    },
    invalid_username_format: {
        en: "Only numbers, lowercase letters, -, _ and more than one character can be entered.",
        ko: "숫자, 소문자, -, _ 및 두 글자 이상만 입력할 수 있습니다.",
    },
    invalid_email_format: {
        en: "Please enter a valid email address.",
        ko: "유효한 이메일 주소를 입력해주세요.",
    },
    terms_required: {
        en: "You must agree to the Terms of Service to proceed.",
        ko: "서비스 이용약관에 동의해야 진행할 수 있습니다.",
    },
    clicked_image: {
        en: "Click to change profile image",
        ko: "프로필 이미지 변경 클릭",
    },
    privacy_policy: {
        en: "Privacy Policy",
        ko: "개인정보 처리방침",
    },
    terms_of_service: {
        en: "Terms of Service",
        ko: "서비스 이용약관",
    },
    failed_send_code: {
        en: "Failed to send verification code",
        ko: "인증 코드 전송에 실패했습니다",
    },
    failed_verify_code: {
        en: "Verification code is incorrect or has expired",
        ko: "인증 코드가 올바르지 않거나 만료되었습니다",
    },
}
