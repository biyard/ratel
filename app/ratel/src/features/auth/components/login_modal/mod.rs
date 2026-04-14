use crate::common::assets::RATEL_LOGO;
use crate::common::components::popup::PopupService;
use crate::features::auth::context::UserContext;
use crate::features::auth::controllers::login::{
    login_handler, wallet_check_handler, wallet_nonce_handler, LoginRequest, WalletCheckRequest,
};
use crate::features::auth::hooks::use_user_context;
use crate::features::auth::interop::sign_in;
#[cfg(feature = "web")]
use crate::features::auth::interop::{wallet_connect, wallet_open_app, wallet_sign_message};
use crate::features::auth::views::ForgotPassword;
use crate::features::auth::*;

#[derive(Debug, Clone, PartialEq)]
enum WalletStep {
    None,
    WaitingSignature,
    Done,
}

/// Shared email-login action. Extracted as a free `async fn` so it can be
/// awaited from multiple event handlers (button onclick, email/password
/// `onkeydown` Enter handlers) without moving a closure multiple times or
/// falling back to `spawn`. Signals are `Copy`, so each caller passes them
/// by value.
async fn submit_email_login(
    email: Signal<String>,
    password: Signal<String>,
    mut show_password: Signal<bool>,
    mut loading: Signal<bool>,
    mut error_message: Signal<Option<String>>,
    mut user_ctx: Store<UserContext>,
    mut popup: PopupService,
    on_success: Option<Callback<()>>,
) {
    error_message.set(None);

    if !show_password() {
        show_password.set(true);
        return;
    }

    loading.set(true);
    let result = login_handler(LoginRequest::Email {
        email: email.read().clone(),
        password: password.read().clone(),
        device_id: None,
    })
    .await;
    loading.set(false);

    match result {
        Ok(user) => {
            user_ctx.set(UserContext {
                user: Some(user.user),
                refresh_token: user.refresh_token,
                membership: None,
            });
            if let Some(handler) = &on_success {
                handler.call(());
            }
            popup.close();
        }
        Err(e) => {
            error_message.set(Some(format!("{e}")));
        }
    }
}

#[component]
pub fn LoginModal(#[props(optional)] on_success: Option<Callback<()>>) -> Element {
    let tr: LoginModalTranslate = use_translate();
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let show_password = use_signal(|| false);
    let mut loading = use_signal(|| false);
    let mut error_message: Signal<Option<String>> = use_signal(|| None);
    //NOTE: Web Feature issue
    #[allow(unused_mut)]
    let mut wallet_step = use_signal(|| WalletStep::None);
    let mut popup = use_popup();
    let navigator = use_navigator();
    let mut user_ctx = use_user_context();

    let handle_open_signup = move |_| {
        popup.close();
        popup.open(rsx! {
            SignupModal { on_success }
        });
    };

    let handle_forgot_password = move |_| {
        popup.close();
        popup.open(rsx! {
            ForgotPassword {}
        });
    };

    let handle_open_wallet_app = move |_| async move {
        #[cfg(feature = "web")]
        {
            let _ = wallet_open_app().await;
        }
    };

    let handle_google_login = move |_| async move {
        error_message.set(None);
        loading.set(true);

        match sign_in().await {
            Ok(user_info) => {
                let oauth_email = user_info
                    .email
                    .clone()
                    .filter(|email| !email.trim().is_empty());
                let access_token = user_info.access_token.clone();
                let result = login_handler(LoginRequest::OAuth {
                    provider: OauthProvider::Google,
                    access_token: user_info.access_token,
                })
                .await;
                loading.set(false);

                match result {
                    Ok(user) => {
                        user_ctx.set(UserContext {
                            user: Some(user.user),
                            refresh_token: user.refresh_token,
                            membership: None,
                        });
                        if let Some(handler) = &on_success {
                            handler.call(());
                        }
                        popup.close();
                    }
                    Err(Error::Auth(AuthError::UserNotFound)) => {
                        popup.close();
                        popup.open(rsx! {
                            SignupModal {
                                initial_email: oauth_email,
                                initial_oauth_access_token: access_token,
                                on_success,
                            }
                        });
                    }
                    Err(e) => {
                        error_message.set(Some(format!("{e}")));
                    }
                }
            }
            Err(e) => {
                loading.set(false);
                error_message.set(Some(format!("{e}")));
            }
        }
    };

    let handle_wallet_login = move |_| async move {
        error_message.set(None);
        loading.set(true);

        #[cfg(feature = "web")]
        {
            let connect_result = match wallet_connect().await {
                Ok(r) => r,
                Err(e) => {
                    loading.set(false);
                    let msg = format!("{e}");
                    if !msg.contains("User cancelled") {
                        error_message.set(Some(msg));
                    }
                    return;
                }
            };

            let check_resp = match wallet_check_handler(WalletCheckRequest {
                evm_address: connect_result.address.clone(),
            })
            .await
            {
                Ok(r) => r,
                Err(e) => {
                    loading.set(false);
                    error_message.set(Some(format!("{e}")));
                    return;
                }
            };

            if !check_resp.exists {
                loading.set(false);
                popup.close();
                popup.open(rsx! {
                    SignupModal {
                        initial_wallet_address: Some(connect_result.address),
                        on_success,
                    }
                });
                return;
            }

            let nonce_resp = match wallet_nonce_handler().await {
                Ok(r) => r,
                Err(e) => {
                    loading.set(false);
                    error_message.set(Some(format!("{e}")));
                    return;
                }
            };

            wallet_step.set(WalletStep::WaitingSignature);

            let signature = match wallet_sign_message(&nonce_resp.message).await {
                Ok(s) => s,
                Err(e) => {
                    loading.set(false);
                    wallet_step.set(WalletStep::None);
                    let msg = format!("{e}");
                    if !msg.contains("User cancelled") && !msg.contains("rejected") {
                        error_message.set(Some(msg));
                    }
                    return;
                }
            };

            wallet_step.set(WalletStep::Done);

            let login_result = login_handler(LoginRequest::Wallet {
                signature,
                evm_address: connect_result.address.clone(),
                message: nonce_resp.message,
            })
            .await;
            loading.set(false);
            wallet_step.set(WalletStep::None);

            match login_result {
                Ok(user) => {
                    user_ctx.set(UserContext {
                        user: Some(user.user),
                        refresh_token: user.refresh_token,
                        membership: None,
                    });
                    if let Some(handler) = &on_success {
                        handler.call(());
                    }
                    popup.close();
                }
                Err(e) => {
                    error_message.set(Some(format!("{e}")));
                }
            }
        }
        #[cfg(not(feature = "web"))]
        {
            loading.set(false);
        }
    };

    rsx! {
        div {
            class: "flex flex-col gap-5 w-100 max-w-100 mx-1.25 max-mobile:w-full! max-mobile:max-w-full!",
            id: "login_popup",

            // Loading overlay
            if wallet_step() == WalletStep::WaitingSignature {

                div { class: "flex absolute inset-0 flex-col gap-10 justify-center items-center p-5 w-full h-full bg-background z-1",
                    div { class: "flex flex-col justify-between w-full h-full max-h-200",
                        div { class: "flex flex-col gap-4 w-full",
                            p { class: "text-xl font-semibold text-center text-text-primary",
                                {tr.waiting_wallet_signature}
                            }
                            p { class: "text-base text-center text-muted-foreground",
                                {tr.waiting_wallet_description}
                            }
                        }

                        Button {
                            class: "w-full",
                            onclick: handle_open_wallet_app,
                            shape: ButtonShape::Square,
                            {tr.open_wallet}
                        }
                    }
                
                }
            } else if loading() {
                div { class: "flex absolute inset-0 justify-center items-center w-full h-full bg-background/95",
                    crate::common::components::LoadingIndicator { class: "size-8" }
                }
            }
            img { src: RATEL_LOGO, alt: "Ratel", class: "object-contain h-10" }

            div { class: "flex flex-col gap-4 w-full",
                div { class: "flex flex-row gap-1 justify-start items-center w-full text-sm",
                    label { class: "font-medium text-text-primary", {tr.new_user} }
                    button {
                        class: "text-primary/70 light:text-primary hover:text-primary",
                        onclick: handle_open_signup,
                        {tr.create_account}
                    }
                }

                if let Some(err) = error_message() {
                    div { class: "text-sm text-red-500", "{err}" }
                }

                div { class: "flex flex-col gap-2.5 w-full",
                    label { r#for: "email", class: "text-sm", {tr.email_address} }
                    div { class: "relative w-full",
                        Input {
                            id: "email",
                            name: "username",
                            autocomplete: "email",
                            "data-slot": "input",
                            "data-testid": "email-input",
                            disabled: loading(),
                            placeholder: "{tr.email_placeholder}",
                            r#type: InputType::Email,
                            value: email(),
                            oninput: move |ev: FormEvent| {
                                email.set(ev.data().value());
                            },
                            onconfirm: move |_| async move {
                                submit_email_login(
                                        email,
                                        password,
                                        show_password,
                                        loading,
                                        error_message,
                                        user_ctx,
                                        popup,
                                        on_success,
                                    )
                                    .await;
                            },
                        }
                    }
                }
                div {
                    aria_hidden: if show_password() { "false" } else { "true" },
                    class: "flex flex-col gap-2.5 w-full aria-hidden:hidden",
                    label { r#for: "password", class: "text-sm", {tr.password} }
                    div { class: "relative w-full",
                        Input {
                            id: "password",
                            name: "password",
                            autocomplete: "current-password",
                            "data-slot": "input",
                            "data-testid": "password-input",
                            disabled: loading(),
                            placeholder: "{tr.password_placeholder}",
                            r#type: InputType::Password,
                            value: password(),
                            oninput: move |ev: FormEvent| {
                                password.set(ev.data().value());
                            },
                            onconfirm: move |_| async move {
                                submit_email_login(
                                        email,
                                        password,
                                        show_password,
                                        loading,
                                        error_message,
                                        user_ctx,
                                        popup,
                                        on_success,
                                    )
                                    .await;
                            },
                        }
                    }
                }
                div { class: "flex flex-row gap-2.5 justify-between items-center w-full text-sm",
                    button {
                        class: "text-sm text-primary/70 hover:text-primary",
                        onclick: handle_forgot_password,
                        {tr.forgot_password}
                    }
                    button {
                        class: "inline-flex gap-2.5 justify-center items-center py-1.5 px-4 h-auto text-xs font-bold whitespace-nowrap rounded-full transition-all outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:size-[15px] shrink-0 [&amp;_svg]:shrink-0 font-[var(--font-raleway)] bg-btn-secondary-bg text-btn-secondary-text border-btn-secondary-outline web light:bg-neutral-600 hover:bg-btn-secondary-hover-bg hover:border-btn-secondary-hover-outline hover:text-btn-secondary-hover-text disabled:bg-btn-secondary--disable-bg disabled:border-btn-secondary-disable-outline disabled:text-btn-secondary-disable-text",
                        "data-slot": "button",
                        "data-testid": "continue-button",
                        disabled: loading(),
                        onclick: move |_| async move {
                            submit_email_login(
                                    email,
                                    password,
                                    show_password,
                                    loading,
                                    error_message,
                                    user_ctx,
                                    popup,
                                    on_success,
                                )
                                .await;
                        },
                        if loading() {
                            {tr.loading}
                        } else {
                            {tr.continue_btn}
                        }
                    }
                }
            }
            div { class: "font-light text-center rule-with-text align-center", {tr.or} }
            div { class: "flex flex-col gap-2.5",
                button {
                    class: "flex flex-row gap-5 items-center px-5 w-full cursor-pointer rounded-[10px] bg-[#000203] py-5.5",
                    disabled: loading(),
                    onclick: handle_google_login,
                    svg {
                        fill: "none",
                        height: "24",
                        view_box: "0 0 24 24",
                        width: "24",
                        xmlns: "http://www.w3.org/2000/svg",
                        g { clip_path: "url(#clip0_2052_51930)",
                            path {
                                d: "M21.7623 12.1871C21.7623 11.3677 21.6958 10.7697 21.552 10.1497H12.1953V13.848H17.6874C17.5768 14.7671 16.9788 16.1512 15.65 17.0813L15.6314 17.2051L18.5898 19.4969L18.7948 19.5174C20.6771 17.7789 21.7623 15.221 21.7623 12.1871Z",
                                fill: "#4285F4",
                            }
                            path {
                                d: "M12.1937 21.9313C14.8844 21.9313 17.1432 21.0454 18.7932 19.5174L15.6484 17.0813C14.8069 17.6682 13.6774 18.0779 12.1937 18.0779C9.55834 18.0779 7.32163 16.3395 6.5243 13.9366L6.40743 13.9466L3.33124 16.3273L3.29102 16.4391C4.92979 19.6945 8.29598 21.9313 12.1937 21.9313Z",
                                fill: "#34A853",
                            }
                            path {
                                d: "M6.52477 13.9366C6.31439 13.3165 6.19264 12.6521 6.19264 11.9656C6.19264 11.279 6.31439 10.6147 6.51371 9.9946L6.50813 9.86253L3.3934 7.4436L3.29149 7.49208C2.61607 8.84299 2.22852 10.36 2.22852 11.9656C2.22852 13.5712 2.61607 15.0881 3.29149 16.439L6.52477 13.9366Z",
                                fill: "#FBBC05",
                            }
                            path {
                                d: "M12.1937 5.85336C14.065 5.85336 15.3273 6.66168 16.047 7.33718L18.8596 4.59107C17.1322 2.9855 14.8844 2 12.1937 2C8.29598 2 4.92979 4.23672 3.29102 7.49214L6.51323 9.99466C7.32163 7.59183 9.55834 5.85336 12.1937 5.85336Z",
                                fill: "#EB4335",
                            }
                        }
                        defs {
                            clipPath { id: "clip0_2052_51930",
                                rect {
                                    fill: "white",
                                    height: "20",
                                    transform: "translate(2 2)",
                                    width: "20",
                                }
                            }
                        }
                    }
                    div { class: "text-base font-semibold text-white", {tr.continue_with_google} }
                }
                button {
                    class: "flex flex-row gap-5 items-center px-5 w-full cursor-pointer rounded-[10px] bg-[#3B99FC] py-5.5",
                    disabled: loading(),
                    onclick: handle_wallet_login,
                    icons::wallet::WalletConnect { class: "fill-white", width: "24", height: "24" }
                    div { class: "text-base font-semibold text-white", {tr.continue_with_wallet} }
                }
            }
            div { class: "flex flex-row gap-2.5 justify-center items-center w-full",
                button {
                    class: "p-0 font-medium bg-transparent border-none cursor-pointer text-neutral-400 text-xs/3.5",
                    onclick: move |_| {
                        popup.close();
                        navigator.push("/privacy");
                    },
                    {tr.privacy_policy}
                }
                button {
                    class: "p-0 font-medium bg-transparent border-none cursor-pointer text-neutral-400 text-xs/3.5",
                    onclick: move |_| {
                        popup.close();
                        navigator.push("/terms");
                    },
                    {tr.terms_of_service}
                }
            }
        }
    }
}

translate! {
    LoginModalTranslate;

    new_user: {
        en: "New user?",
        ko: "새 사용자?",
    },
    create_account: {
        en: "Create an account",
        ko: "계정 만들기",
    },
    email_address: {
        en: "Email address",
        ko: "이메일 주소",
    },
    email_placeholder: {
        en: "Enter your email address",
        ko: "이메일 주소를 입력하세요",
    },
    password: {
        en: "Password",
        ko: "비밀번호",
    },
    password_placeholder: {
        en: "Enter your password",
        ko: "비밀번호를 입력하세요",
    },
    forgot_password: {
        en: "Forgot password?",
        ko: "비밀번호를 잊으셨나요?",
    },
    continue_btn: {
        en: "Continue",
        ko: "계속",
    },
    loading: {
        en: "Loading...",
        ko: "로딩 중...",
    },
    or: {
        en: "Or",
        ko: "또는",
    },
    continue_with_google: {
        en: "Continue With Google",
        ko: "Google로 계속하기",
    },
    continue_with_wallet: {
        en: "Continue With Wallet",
        ko: "지갑으로 계속하기",
    },
    waiting_wallet_signature: {
        en: "Approve signature in your wallet",
        ko: "지갑에서 서명을 승인해주세요",
    },
    waiting_wallet_description: {
        en: "A signature request has been sent to your wallet app.",
        ko: "지갑 앱으로 서명 요청이 전송되었습니다.",
    },
    open_wallet: {
        en: "Signin",
        ko: "로그인",
    },
    privacy_policy: {
        en: "Privacy Policy",
        ko: "개인정보 처리방침",
    },
    terms_of_service: {
        en: "Terms of Service",
        ko: "서비스 이용약관",
    },
}
