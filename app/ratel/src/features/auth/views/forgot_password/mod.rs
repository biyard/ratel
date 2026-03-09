use crate::features::auth::controllers::reset_password::{reset_password_handler, ResetPasswordRequest};
use crate::features::auth::controllers::send_code::{
    send_password_reset_code_handler, SendPasswordResetCodeRequest,
};
use crate::features::auth::*;

#[component]
pub fn ForgotPassword() -> Element {
    let tr: ForgotPasswordTranslate = use_translate();
    let mut step = use_signal(|| 1u8);
    let mut email = use_signal(|| String::new());
    let mut code = use_signal(|| String::new());
    let mut new_password = use_signal(|| String::new());
    let mut confirm_password = use_signal(|| String::new());
    let mut loading = use_signal(|| false);
    let mut error_message: Signal<Option<String>> = use_signal(|| None);
    let mut success_message: Signal<Option<String>> = use_signal(|| None);

    rsx! {
        div {
            class: "flex flex-col gap-5 w-100 max-w-100 mx-1.25 max-mobile:w-full! max-mobile:max-w-full!",
            div { class: "flex flex-col gap-4 w-full",
                h2 { class: "text-lg font-semibold text-text-primary", {tr.forgot_password_title} }
                p { class: "text-sm text-muted-foreground", {tr.forgot_password_description} }

                if let Some(err) = error_message() {
                    div { class: "text-sm text-red-500", "{err}" }
                }

                if let Some(msg) = success_message() {
                    div { class: "text-sm text-green-500", "{msg}" }
                }

                if step() == 1 {
                    // Step 1: Send Code
                    div { class: "flex flex-col gap-2.5 w-full",
                        label { class: "text-sm", {tr.email_address} }
                        div { class: "relative w-full",
                            input {
                                autocomplete: "email",
                                class: "flex px-5 w-full min-w-0 h-9 text-base font-light border outline-none md:text-sm disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none shadow-xs transition-[color,box-shadow] file:text-text-primary file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium selection:bg-primary selection:text-primary-foreground placeholder:text-muted-foreground aria-invalid:ring-destructive/20 aria-invalid:outline aria-invalid:border-c-p-50 bg-input-box-bg border-input-box-border rounded-[10px] py-5.5 text-text-primary dark:bg-input/30 dark:aria-invalid:ring-destructive/40 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[1px]",
                                "data-slot": "input",
                                disabled: loading(),
                                placeholder: "{tr.email_placeholder}",
                                r#type: "email",
                                value: email(),
                                oninput: move |ev| {
                                    email.set(ev.data().value());
                                },
                            }
                        }
                    }
                    div { class: "flex flex-row gap-2.5 justify-between items-center w-full text-sm",
                        a {
                            class: "text-sm text-primary/70 hover:text-primary",
                            href: "/",
                            {tr.back_to_login}
                        }
                        button {
                            class: "inline-flex gap-2.5 justify-center items-center py-1.5 px-4 h-auto text-xs font-bold whitespace-nowrap rounded-full transition-all outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none [&_svg]:pointer-events-none [&_svg]:size-[15px] shrink-0 [&_svg]:shrink-0 bg-btn-secondary-bg text-btn-secondary-text border-btn-secondary-outline hover:bg-btn-secondary-hover-bg hover:border-btn-secondary-hover-outline hover:text-btn-secondary-hover-text disabled:bg-btn-secondary--disable-bg disabled:border-btn-secondary-disable-outline disabled:text-btn-secondary-disable-text",
                            "data-slot": "button",
                            disabled: loading(),
                            onclick: move |_| async move {
                                error_message.set(None);
                                success_message.set(None);

                                let email_val = email.read().trim().to_string();
                                if email_val.is_empty() {
                                    error_message.set(Some(tr.all_fields_required.to_string()));
                                    return;
                                }

                                loading.set(true);
                                let result = send_password_reset_code_handler(
                                    SendPasswordResetCodeRequest { email: email_val },
                                )
                                .await;
                                loading.set(false);

                                match result {
                                    Ok(_) => {
                                        success_message.set(Some(tr.verification_code_sent.to_string()));
                                        step.set(2);
                                    }
                                    Err(e) => {
                                        error_message.set(Some(format!("{e}")));
                                    }
                                }
                            },
                            if loading() { {tr.sending} } else { {tr.send_code} }
                        }
                    }
                } else if success_message().map(|m| m == tr.password_reset_success.to_string()).unwrap_or(false) {
                    // Success state after password reset
                    div { class: "flex flex-col gap-4 items-center w-full",
                        a {
                            class: "inline-flex gap-2.5 justify-center items-center py-1.5 px-4 h-auto text-xs font-bold whitespace-nowrap rounded-full transition-all outline-none focus-visible:ring-2 focus-visible:ring-offset-2 bg-btn-secondary-bg text-btn-secondary-text border-btn-secondary-outline hover:bg-btn-secondary-hover-bg hover:border-btn-secondary-hover-outline hover:text-btn-secondary-hover-text",
                            href: "/",
                            {tr.back_to_login}
                        }
                    }
                } else {
                    // Step 2: Reset Password
                    div { class: "flex flex-col gap-2.5 w-full",
                        label { class: "text-sm", {tr.verification_code} }
                        div { class: "relative w-full",
                            input {
                                class: "flex px-5 w-full min-w-0 h-9 text-base font-light border outline-none md:text-sm disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none shadow-xs transition-[color,box-shadow] file:text-text-primary file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium selection:bg-primary selection:text-primary-foreground placeholder:text-muted-foreground aria-invalid:ring-destructive/20 aria-invalid:outline aria-invalid:border-c-p-50 bg-input-box-bg border-input-box-border rounded-[10px] py-5.5 text-text-primary dark:bg-input/30 dark:aria-invalid:ring-destructive/40 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[1px]",
                                "data-slot": "input",
                                disabled: loading(),
                                placeholder: "{tr.code_placeholder}",
                                r#type: "text",
                                value: code(),
                                oninput: move |ev| {
                                    code.set(ev.data().value());
                                },
                            }
                        }
                    }
                    div { class: "flex flex-col gap-2.5 w-full",
                        label { class: "text-sm", {tr.new_password} }
                        div { class: "relative w-full",
                            input {
                                class: "flex px-5 w-full min-w-0 h-9 text-base font-light border outline-none md:text-sm disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none shadow-xs transition-[color,box-shadow] file:text-text-primary file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium selection:bg-primary selection:text-primary-foreground placeholder:text-muted-foreground aria-invalid:ring-destructive/20 aria-invalid:outline aria-invalid:border-c-p-50 bg-input-box-bg border-input-box-border rounded-[10px] py-5.5 text-text-primary dark:bg-input/30 dark:aria-invalid:ring-destructive/40 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[1px]",
                                "data-slot": "input",
                                disabled: loading(),
                                placeholder: "{tr.password_placeholder}",
                                r#type: "password",
                                value: new_password(),
                                oninput: move |ev| {
                                    new_password.set(ev.data().value());
                                },
                            }
                        }
                    }
                    div { class: "flex flex-col gap-2.5 w-full",
                        label { class: "text-sm", {tr.confirm_new_password} }
                        div { class: "relative w-full",
                            input {
                                class: "flex px-5 w-full min-w-0 h-9 text-base font-light border outline-none md:text-sm disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none shadow-xs transition-[color,box-shadow] file:text-text-primary file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium selection:bg-primary selection:text-primary-foreground placeholder:text-muted-foreground aria-invalid:ring-destructive/20 aria-invalid:outline aria-invalid:border-c-p-50 bg-input-box-bg border-input-box-border rounded-[10px] py-5.5 text-text-primary dark:bg-input/30 dark:aria-invalid:ring-destructive/40 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[1px]",
                                "data-slot": "input",
                                disabled: loading(),
                                placeholder: "{tr.confirm_password_placeholder}",
                                r#type: "password",
                                value: confirm_password(),
                                oninput: move |ev| {
                                    confirm_password.set(ev.data().value());
                                },
                            }
                        }
                    }
                    div { class: "flex flex-row gap-2.5 justify-between items-center w-full text-sm",
                        a {
                            class: "text-sm text-primary/70 hover:text-primary",
                            href: "/",
                            {tr.back_to_login}
                        }
                        button {
                            class: "inline-flex gap-2.5 justify-center items-center py-1.5 px-4 h-auto text-xs font-bold whitespace-nowrap rounded-full transition-all outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none [&_svg]:pointer-events-none [&_svg]:size-[15px] shrink-0 [&_svg]:shrink-0 bg-btn-secondary-bg text-btn-secondary-text border-btn-secondary-outline hover:bg-btn-secondary-hover-bg hover:border-btn-secondary-hover-outline hover:text-btn-secondary-hover-text disabled:bg-btn-secondary--disable-bg disabled:border-btn-secondary-disable-outline disabled:text-btn-secondary-disable-text",
                            "data-slot": "button",
                            disabled: loading(),
                            onclick: move |_| async move {
                                error_message.set(None);
                                success_message.set(None);

                                let code_val = code.read().trim().to_string();
                                let password_val = new_password.read().clone();
                                let confirm_val = confirm_password.read().clone();

                                if code_val.is_empty() || password_val.is_empty() || confirm_val.is_empty() {
                                    error_message.set(Some(tr.all_fields_required.to_string()));
                                    return;
                                }

                                if password_val.len() < 8 {
                                    error_message.set(Some(tr.password_min_length.to_string()));
                                    return;
                                }

                                if password_val != confirm_val {
                                    error_message.set(Some(tr.passwords_do_not_match.to_string()));
                                    return;
                                }

                                loading.set(true);
                                let result = reset_password_handler(ResetPasswordRequest {
                                    email: email.read().trim().to_string(),
                                    password: password_val,
                                    code: code_val,
                                })
                                .await;
                                loading.set(false);

                                match result {
                                    Ok(_) => {
                                        success_message.set(Some(tr.password_reset_success.to_string()));
                                    }
                                    Err(e) => {
                                        error_message.set(Some(format!("{e}")));
                                    }
                                }
                            },
                            if loading() { {tr.resetting} } else { {tr.reset_password} }
                        }
                    }
                }
            }
        }
    }
}

translate! {
    ForgotPasswordTranslate;

    forgot_password_title: {
        en: "Forgot Password?",
        ko: "비밀번호를 잊으셨나요?",
    },
    forgot_password_description: {
        en: "Enter your email address and we'll send you a verification code to reset your password.",
        ko: "이메일 주소를 입력하시면 비밀번호 재설정을 위한 인증 코드를 보내드립니다.",
    },
    email_address: {
        en: "Email address",
        ko: "이메일 주소",
    },
    email_placeholder: {
        en: "Enter your email address",
        ko: "이메일 주소를 입력하세요",
    },
    send_code: {
        en: "Send Code",
        ko: "코드 보내기",
    },
    sending: {
        en: "Sending...",
        ko: "전송 중...",
    },
    verification_code: {
        en: "Verification Code",
        ko: "인증 코드",
    },
    code_placeholder: {
        en: "Enter the 6-digit code",
        ko: "6자리 코드를 입력하세요",
    },
    new_password: {
        en: "New Password",
        ko: "새 비밀번호",
    },
    password_placeholder: {
        en: "Enter new password (min 8 characters)",
        ko: "새 비밀번호 입력 (최소 8자)",
    },
    confirm_new_password: {
        en: "Confirm New Password",
        ko: "새 비밀번호 확인",
    },
    confirm_password_placeholder: {
        en: "Confirm new password",
        ko: "새 비밀번호 확인",
    },
    reset_password: {
        en: "Reset Password",
        ko: "비밀번호 재설정",
    },
    resetting: {
        en: "Resetting...",
        ko: "재설정 중...",
    },
    back_to_login: {
        en: "Back to Login",
        ko: "로그인으로 돌아가기",
    },
    verification_code_sent: {
        en: "Verification code sent! Check your email.",
        ko: "인증 코드가 전송되었습니다! 이메일을 확인하세요.",
    },
    passwords_do_not_match: {
        en: "Passwords do not match",
        ko: "비밀번호가 일치하지 않습니다",
    },
    password_min_length: {
        en: "Password must be at least 8 characters",
        ko: "비밀번호는 8자 이상이어야 합니다",
    },
    all_fields_required: {
        en: "All fields are required",
        ko: "모든 항목을 입력해주세요",
    },
    password_reset_success: {
        en: "Password reset successful!",
        ko: "비밀번호가 성공적으로 재설정되었습니다!",
    },
}
