use super::controllers::{
    change_password_handler, get_mcp_secret_handler, get_user_detail_handler,
    regenerate_mcp_secret_handler, update_profile_handler, ChangePasswordRequest,
    UpdateProfileRequest,
};
use super::Result as AppResult;
use super::*;
#[cfg(not(feature = "server"))]
use crate::common::{wasm_bindgen, wasm_bindgen_futures, web_sys};
use crate::features::auth::hooks::use_user_context;
use crate::features::membership::controllers::{
    get_billing_info_handler, get_membership_handler, update_billing_card_handler,
    UpdateBillingCardRequest,
};
use crate::features::membership::models::CardInfo;
use dioxus::prelude::*;
#[cfg(not(feature = "server"))]
use web_sys::js_sys::{Reflect, JSON};

translate! {
    UserSettingsTranslate;

    settings: { en: "Settings", ko: "설정" },
    profile: { en: "Profile", ko: "프로필" },
    username: { en: "Username", ko: "사용자명" },
    email: { en: "Email", ko: "이메일" },
    display_name: { en: "Display Name", ko: "닉네임" },
    description: { en: "Description", ko: "소개" },
    display_name_placeholder: { en: "Display name", ko: "닉네임을 입력하세요" },
    description_placeholder: { en: "Tell us about yourself", ko: "자기소개를 입력하세요" },
    save: { en: "Save", ko: "저장" },
    saving: { en: "Saving...", ko: "저장 중..." },
    upload: { en: "Upload", ko: "업로드" },
    profile_updated: { en: "Profile updated successfully.", ko: "프로필이 업데이트되었습니다." },
    invalid_words: { en: "Invalid words detected.", ko: "허용되지 않는 단어가 감지되었습니다." },

    password: { en: "Password", ko: "비밀번호" },
    current_password: { en: "Current Password", ko: "현재 비밀번호" },
    new_password: { en: "New Password", ko: "새 비밀번호" },
    confirm_password: { en: "Confirm Password", ko: "비밀번호 확인" },
    current_password_placeholder: { en: "Enter current password", ko: "현재 비밀번호를 입력하세요" },
    new_password_placeholder: { en: "Min 8 characters", ko: "최소 8자 이상" },
    confirm_password_placeholder: { en: "Confirm new password", ko: "새 비밀번호 확인" },
    change_password: { en: "Change Password", ko: "비밀번호 변경" },
    changing_password: { en: "Changing...", ko: "변경 중..." },
    password_changed: { en: "Password changed successfully.", ko: "비밀번호가 변경되었습니다." },
    all_fields_required: { en: "All fields are required.", ko: "모든 항목을 입력해주세요." },
    password_min_length: { en: "Password must be at least 8 characters.", ko: "비밀번호는 최소 8자 이상이어야 합니다." },
    passwords_not_match: { en: "Passwords do not match.", ko: "비밀번호가 일치하지 않습니다." },

    subscription_billing: { en: "Subscription & Billing", ko: "구독 및 결제" },
    current_plan: { en: "Current Plan", ko: "현재 플랜" },
    credits: { en: "Credits", ko: "크레딧" },
    expires: { en: "Expires", ko: "만료일" },
    unlimited: { en: "Unlimited", ko: "무제한" },
    change_plan: { en: "Change Plan", ko: "플랜 변경" },
    card: { en: "Card", ko: "카드" },
    card_holder: { en: "Card Holder", ko: "카드 소유자" },
    change_card: { en: "Change Card", ko: "카드 변경" },
    add_card: { en: "Add Card", ko: "카드 추가" },
    cancel: { en: "Cancel", ko: "취소" },
    card_number: { en: "Card Number", ko: "카드 번호" },
    expiry_month: { en: "Expiry Month", ko: "만료 월" },
    expiry_year: { en: "Expiry Year", ko: "만료 연도" },
    birth_date: { en: "Birth Date (YYMMDD)", ko: "생년월일 (YYMMDD)" },
    card_password_label: { en: "Card Password (first 2 digits)", ko: "카드 비밀번호 (앞 2자리)" },
    save_card: { en: "Save Card", ko: "카드 저장" },
    saving_card: { en: "Saving...", ko: "저장 중..." },
    card_updated: { en: "Card updated successfully.", ko: "카드가 업데이트되었습니다." },
    login_required: { en: "Please log in to access settings.", ko: "설정에 접근하려면 로그인해주세요." },

    mcp_server: { en: "MCP Server", ko: "MCP 서버" },
    mcp_description: { en: "Connect your Ratel account to AI assistants like Claude Desktop using the Model Context Protocol.", ko: "Model Context Protocol을 사용하여 Claude Desktop과 같은 AI 어시스턴트에 Ratel 계정을 연결하세요." },
    mcp_server_url: { en: "Server URL", ko: "서버 URL" },
    mcp_generate: { en: "Generate Secret", ko: "시크릿 생성" },
    mcp_regenerate: { en: "Regenerate", ko: "재생성" },
    mcp_generating: { en: "Generating...", ko: "생성 중..." },
    mcp_copied: { en: "Copied to clipboard!", ko: "클립보드에 복사되었습니다!" },
    mcp_copy: { en: "Copy", ko: "복사" },
    mcp_not_generated: { en: "No secret generated yet. Click Generate to create one.", ko: "아직 시크릿이 생성되지 않았습니다. 생성 버튼을 클릭하세요." },
}

#[component]
pub fn Home(username: String) -> Element {
    let tr: UserSettingsTranslate = use_translate();
    let mut user_ctx = use_user_context();
    let user = user_ctx.read().user.clone();

    let Some(user) = user else {
        return rsx! {
            div { class: "flex flex-col items-center justify-center w-full h-full py-10",
                p { class: "text-foreground-muted", {tr.login_required} }
            }
        };
    };

    let detail_resource =
        use_server_future(move || async move { get_user_detail_handler().await })?;
    let detail_state = detail_resource.value();

    let mut profile_url = use_signal(|| user.profile_url.clone());
    let mut nickname = use_signal(|| user.display_name.clone());
    let mut description = use_signal(|| user.description.clone());
    let mut user_email = use_signal(String::new);
    let mut evm_address = use_signal(String::new);
    let mut saving = use_signal(|| false);
    let mut message = use_signal(|| Option::<(String, bool)>::None);
    let mut detail_loaded = use_signal(|| false);

    {
        let detail_state = detail_state.clone();
        use_effect(move || {
            let detail_state = detail_state.read();
            let Some(state) = detail_state.as_ref() else {
                return;
            };
            if let Ok(detail) = state {
                evm_address.set(detail.evm_address.clone().unwrap_or_default());
                user_email.set(detail.email.clone());
                detail_loaded.set(true);
            }
        });
    }

    let on_save_profile = {
        let mut user_ctx = user_ctx.clone();
        move |_: MouseEvent| {
            let nick = nickname().trim().to_string();
            let profile = profile_url();
            let desc = description().trim().to_string();
            if is_blocked_text(&nick) || is_blocked_text(&desc) {
                message.set(Some((tr.invalid_words.to_string(), false)));
                return;
            }
            spawn(async move {
                saving.set(true);
                message.set(None);
                let result = update_profile_handler(UpdateProfileRequest {
                    nickname: nick,
                    profile_url: profile,
                    description: desc,
                })
                .await;
                saving.set(false);
                match result {
                    Ok(resp) => {
                        user_ctx.with_mut(|ctx| {
                            if let Some(user) = ctx.user.as_mut() {
                                user.display_name = resp.user.display_name.clone();
                                user.profile_url = resp.user.profile_url.clone();
                                user.description = resp.user.description.clone();
                            }
                        });
                        message.set(Some((tr.profile_updated.to_string(), true)));
                    }
                    Err(e) => {
                        message.set(Some((format!("Failed to update profile: {e}"), false)));
                    }
                }
            });
        }
    };

    let save_blocked = is_blocked_text(&nickname()) || is_blocked_text(&description());

    rsx! {
        div { class: "w-full max-w-[800px] mx-auto flex flex-col gap-6 px-4 mt-5",
            h1 { class: "text-xl font-bold text-text-primary", {tr.settings} }

            // Card 1: Profile
            Card { variant: CardVariant::Outlined, class: "p-6",
                div { class: "flex flex-col gap-5 w-full",
                    h2 { class: "text-lg font-bold text-text-primary", {tr.profile} }

                    div { class: "flex items-center gap-4",
                        FileUploader {
                            on_upload_success: move |url: String| profile_url.set(url),
                            accept: "image/*",
                            if profile_url().is_empty() {
                                div { class: "w-16 h-16 rounded-full bg-card-bg flex items-center justify-center text-xs text-foreground-muted cursor-pointer",
                                    {tr.upload}
                                }
                            } else {
                                img {
                                    src: "{profile_url()}",
                                    alt: "Profile",
                                    class: "w-16 h-16 rounded-full object-cover cursor-pointer",
                                }
                            }
                        }
                        div { class: "flex flex-col gap-0.5",
                            span { class: "text-sm font-semibold text-text-primary",
                                "{user.display_name}"
                            }
                            span { class: "text-xs text-foreground-muted", "@{user.username}" }
                        }
                    }

                    SettingsRow { label: tr.username.to_string(),
                        Input {
                            value: format!("@{}", user.username),
                            disabled: true,
                        }
                    }

                    SettingsRow { label: tr.email.to_string(),
                        Input { value: user_email(), disabled: true }
                    }

                    SettingsRow { label: tr.display_name.to_string(),
                        Input {
                            value: nickname(),
                            placeholder: tr.display_name_placeholder,
                            maxlength: 30,
                            oninput: move |e: Event<FormData>| nickname.set(e.value()),
                        }
                    }

                    div { class: "flex flex-col gap-1.5",
                        label { class: "text-sm font-semibold text-text-primary", {tr.description} }
                        TextArea {
                            value: description(),
                            placeholder: tr.description_placeholder,
                            class: "min-h-[100px] resize-y",
                            oninput: move |e: Event<FormData>| description.set(e.value()),
                        }
                    }

                    if let Some((msg, is_success)) = message() {
                        div { class: if is_success { "text-sm text-banner-success-text" } else { "text-sm text-destructive" },
                            "{msg}"
                        }
                    }

                    div { class: "flex justify-end",
                        Button {
                            style: ButtonStyle::Primary,
                            disabled: saving() || save_blocked,
                            onclick: on_save_profile,
                            if saving() {
                                {tr.saving}
                            } else {
                                {tr.save}
                            }
                        }
                    }
                }
            }

            // Card 2: Password
            Card { variant: CardVariant::Outlined, class: "p-6", PasswordCard {} }

            // Card 3: Subscription & Billing
            Card { variant: CardVariant::Outlined, class: "p-6", SubscriptionCard {} }

            // Card 4: MCP Server
            Card { variant: CardVariant::Outlined, class: "p-6", McpServerCard {} }
        }
    }
}

#[component]
fn SettingsRow(label: String, children: Element) -> Element {
    rsx! {
        div { class: "flex flex-col gap-1.5",
            label { class: "text-sm font-semibold text-text-primary", "{label}" }
            {children}
        }
    }
}

#[component]
fn PasswordCard() -> Element {
    let mut current_password = use_signal(String::new);
    let mut new_password = use_signal(String::new);
    let mut confirm_password = use_signal(String::new);
    let mut saving = use_signal(|| false);
    let mut message = use_signal(|| Option::<(String, bool)>::None);

    let on_change = move |_: MouseEvent| {
        let current = current_password().trim().to_string();
        let new_pw = new_password().trim().to_string();
        let confirm = confirm_password().trim().to_string();

        if current.is_empty() || new_pw.is_empty() || confirm.is_empty() {
            message.set(Some(("All fields are required.".to_string(), false)));
            return;
        }
        if new_pw.len() < 8 {
            message.set(Some((
                "Password must be at least 8 characters.".to_string(),
                false,
            )));
            return;
        }
        if new_pw != confirm {
            message.set(Some(("Passwords do not match.".to_string(), false)));
            return;
        }

        spawn(async move {
            saving.set(true);
            message.set(None);
            let result = change_password_handler(ChangePasswordRequest {
                current_password: current,
                new_password: new_pw,
            })
            .await;
            saving.set(false);
            match result {
                Ok(_) => {
                    current_password.set(String::new());
                    new_password.set(String::new());
                    confirm_password.set(String::new());
                    message.set(Some(("Password changed successfully.".to_string(), true)));
                }
                Err(e) => {
                    message.set(Some((format!("{e}"), false)));
                }
            }
        });
    };

    rsx! {
        div { class: "flex flex-col gap-5 w-full",
            h2 { class: "text-lg font-bold text-text-primary", "Password" }

            SettingsRow { label: "Current Password",
                Input {
                    r#type: InputType::Password,
                    placeholder: "Enter current password",
                    value: current_password(),
                    oninput: move |e: Event<FormData>| current_password.set(e.value()),
                }
            }

            SettingsRow { label: "New Password",
                Input {
                    r#type: InputType::Password,
                    placeholder: "Min 8 characters",
                    value: new_password(),
                    oninput: move |e: Event<FormData>| new_password.set(e.value()),
                }
            }

            SettingsRow { label: "Confirm Password",
                Input {
                    r#type: InputType::Password,
                    placeholder: "Confirm new password",
                    value: confirm_password(),
                    oninput: move |e: Event<FormData>| confirm_password.set(e.value()),
                }
            }

            if let Some((msg, is_success)) = message() {
                div { class: if is_success { "text-sm text-banner-success-text" } else { "text-sm text-destructive" },
                    "{msg}"
                }
            }

            div { class: "flex justify-end",
                Button {
                    style: ButtonStyle::Secondary,
                    disabled: saving(),
                    onclick: on_change,
                    if saving() {
                        "Changing..."
                    } else {
                        "Change Password"
                    }
                }
            }
        }
    }
}

#[component]
fn SubscriptionCard() -> Element {
    let membership = use_server_future(move || async move { get_membership_handler().await })?;
    let mut billing_info =
        use_server_future(move || async move { get_billing_info_handler().await })?;

    let membership_data = membership.read();
    let billing_data = billing_info.read();

    let (tier_label, remaining, total, expired_at, is_free) =
        if let Some(Ok(m)) = membership_data.as_ref() {
            let tier = m.tier.0.replace("MEMBERSHIP#", "");
            let free = tier.eq_ignore_ascii_case("free");
            (
                tier,
                m.remaining_credits,
                m.total_credits,
                m.expired_at,
                free,
            )
        } else {
            ("Free".to_string(), 0, 0, 0, true)
        };

    let billing = billing_data
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .cloned()
        .unwrap_or_default();

    let expiry_text = if expired_at == 0 {
        "Unlimited".to_string()
    } else {
        use chrono::{TimeZone, Utc};
        Utc.timestamp_millis_opt(expired_at)
            .single()
            .map(|dt| dt.format("%Y-%m-%d").to_string())
            .unwrap_or_default()
    };

    let mut show_card_form = use_signal(|| false);
    let mut card_number = use_signal(String::new);
    let mut expiry_month = use_signal(String::new);
    let mut expiry_year = use_signal(String::new);
    let mut birth_or_biz = use_signal(String::new);
    let mut card_password = use_signal(String::new);
    let mut saving = use_signal(|| false);
    let mut card_message = use_signal(|| Option::<(String, bool)>::None);

    let is_valid = use_memo(move || {
        !card_number.read().trim().is_empty()
            && !expiry_month.read().trim().is_empty()
            && !expiry_year.read().trim().is_empty()
            && !birth_or_biz.read().trim().is_empty()
            && !card_password.read().trim().is_empty()
    });

    let on_save_card = move |_: MouseEvent| {
        let info = CardInfo {
            card_number: card_number().trim().to_string(),
            expiry_year: expiry_year().trim().to_string(),
            expiry_month: expiry_month().trim().to_string(),
            birth_or_business_registration_number: birth_or_biz().trim().to_string(),
            password_two_digits: card_password().trim().to_string(),
        };
        spawn(async move {
            saving.set(true);
            card_message.set(None);
            match update_billing_card_handler(UpdateBillingCardRequest { card_info: info }).await {
                Ok(_) => {
                    card_number.set(String::new());
                    expiry_month.set(String::new());
                    expiry_year.set(String::new());
                    birth_or_biz.set(String::new());
                    card_password.set(String::new());
                    show_card_form.set(false);
                    card_message.set(Some(("Card updated successfully.".to_string(), true)));
                    billing_info.restart();
                }
                Err(e) => {
                    card_message.set(Some((format!("{e}"), false)));
                }
            }
            saving.set(false);
        });
    };

    rsx! {
        div { class: "flex flex-col gap-5 w-full",
            h2 { class: "text-lg font-bold text-text-primary", "Subscription & Billing" }

            div { class: "flex flex-col gap-3",
                Row {
                    main_axis_align: MainAxisAlign::Between,
                    cross_axis_align: CrossAxisAlign::Center,
                    span { class: "text-sm text-foreground-muted", "Current Plan" }
                    Row {
                        class: "gap-2",
                        cross_axis_align: CrossAxisAlign::Center,
                        Badge {
                            color: match tier_label.as_str() {
                                "Pro" => BadgeColor::Blue,
                                "Max" => BadgeColor::Purple,
                                "Vip" => BadgeColor::Orange,
                                _ => BadgeColor::Grey,
                            },
                            "{tier_label}"
                        }
                        Link {
                            to: "/membership",
                            class: "text-xs text-primary hover:underline no-underline",
                            "Change Plan"
                        }
                    }
                }
                Row { main_axis_align: MainAxisAlign::Between,
                    span { class: "text-sm text-foreground-muted", "Credits" }
                    span { class: "text-sm text-text-primary", "{remaining} / {total}" }
                }
                Row { main_axis_align: MainAxisAlign::Between,
                    span { class: "text-sm text-foreground-muted", "Expires" }
                    span { class: "text-sm text-text-primary", "{expiry_text}" }
                }
                if !is_free {
                    if let Some(ref masked) = billing.masked_card_number {
                        Row { main_axis_align: MainAxisAlign::Between,
                            span { class: "text-sm text-foreground-muted", "Card" }
                            span { class: "text-sm text-text-primary", "{masked}" }
                        }
                    }
                    if !billing.customer_name.is_empty() {
                        Row { main_axis_align: MainAxisAlign::Between,
                            span { class: "text-sm text-foreground-muted", "Card Holder" }
                            span { class: "text-sm text-text-primary", "{billing.customer_name}" }
                        }
                    }
                    div { class: "flex justify-end pt-1",
                        Button {
                            style: ButtonStyle::Secondary,
                            onclick: move |_| show_card_form.set(!show_card_form()),
                            if show_card_form() {
                                "Cancel"
                            } else if billing.has_card {
                                "Change Card"
                            } else {
                                "Add Card"
                            }
                        }
                    }
                }
            }

            if !is_free {
                if let Some((msg, is_success)) = card_message() {
                    div { class: if is_success { "text-sm text-banner-success-text" } else { "text-sm text-destructive" },
                        "{msg}"
                    }
                }

                if show_card_form() {
                    div { class: "flex flex-col gap-4 rounded-[10px] border border-border p-4",
                        SettingsRow { label: "Card Number",
                            Input {
                                placeholder: "0000000000000000",
                                maxlength: 16,
                                value: card_number(),
                                oninput: move |e: Event<FormData>| {
                                    let v = e.value().chars().filter(|c| c.is_ascii_digit()).collect::<String>();
                                    card_number.set(v);
                                },
                            }
                        }
                        div { class: "flex gap-3",
                            div { class: "flex-1 flex flex-col gap-1.5",
                                label { class: "text-sm font-semibold text-text-primary",
                                    "Expiry Month"
                                }
                                Input {
                                    placeholder: "MM",
                                    maxlength: 2,
                                    value: expiry_month(),
                                    oninput: move |e: Event<FormData>| {
                                        let v = e.value().chars().filter(|c| c.is_ascii_digit()).collect::<String>();
                                        expiry_month.set(v);
                                    },
                                }
                            }
                            div { class: "flex-1 flex flex-col gap-1.5",
                                label { class: "text-sm font-semibold text-text-primary",
                                    "Expiry Year"
                                }
                                Input {
                                    placeholder: "YY",
                                    maxlength: 2,
                                    value: expiry_year(),
                                    oninput: move |e: Event<FormData>| {
                                        let v = e.value().chars().filter(|c| c.is_ascii_digit()).collect::<String>();
                                        expiry_year.set(v);
                                    },
                                }
                            }
                        }
                        SettingsRow { label: "Birth Date (YYMMDD)",
                            Input {
                                placeholder: "YYMMDD",
                                maxlength: 10,
                                value: birth_or_biz(),
                                oninput: move |e: Event<FormData>| {
                                    let v = e.value().chars().filter(|c| c.is_ascii_digit()).collect::<String>();
                                    birth_or_biz.set(v);
                                },
                            }
                        }
                        SettingsRow { label: "Card Password (first 2 digits)",
                            Input {
                                r#type: InputType::Password,
                                class: "w-20",
                                placeholder: "••",
                                maxlength: 2,
                                value: card_password(),
                                oninput: move |e: Event<FormData>| {
                                    let v = e.value().chars().filter(|c| c.is_ascii_digit()).collect::<String>();
                                    card_password.set(v);
                                },
                            }
                        }
                        div { class: "flex justify-end",
                            Button {
                                style: ButtonStyle::Primary,
                                disabled: !is_valid() || saving(),
                                onclick: on_save_card,
                                if saving() {
                                    "Saving..."
                                } else {
                                    "Save Card"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(not(feature = "server"))]
async fn connect_wallet_address() -> AppResult<Option<String>> {
    let promise =
        super::interop::connect_wallet().map_err(|e| Error::Unknown(format_js_error(e)))?;
    let value = wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|e| Error::Unknown(format_js_error(e)))?;
    Ok(value.as_string())
}

#[cfg(feature = "server")]
async fn connect_wallet_address() -> AppResult<Option<String>> {
    Err(Error::NotSupported(
        "Wallet connection is only available on web".to_string(),
    ))
}

#[cfg(not(feature = "server"))]
fn format_js_error(err: wasm_bindgen::JsValue) -> String {
    if let Some(msg) = err.as_string() {
        msg
    } else {
        if err.is_object() {
            if let Ok(message) = Reflect::get(&err, &wasm_bindgen::JsValue::from_str("message")) {
                if let Some(msg) = message.as_string() {
                    return msg;
                }
            }
        }
        if let Ok(json) = JSON::stringify(&err) {
            if let Some(msg) = json.as_string() {
                return msg;
            }
        }
        "Unknown error".to_string()
    }
}

#[component]
fn McpServerCard() -> Element {
    let tr: UserSettingsTranslate = use_translate();
    let mut mcp_secret =
        use_server_future(move || async move { get_mcp_secret_handler().await })?;
    let mcp_data = mcp_secret.read();

    let secret_value = mcp_data
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .and_then(|resp| resp.secret.clone());

    let mut generating = use_signal(|| false);
    let mut copied = use_signal(|| false);
    let mut mcp_message = use_signal(|| Option::<(String, bool)>::None);

    let on_generate = move |_: MouseEvent| {
        spawn(async move {
            generating.set(true);
            mcp_message.set(None);
            match regenerate_mcp_secret_handler().await {
                Ok(_) => {
                    mcp_secret.restart();
                }
                Err(e) => {
                    mcp_message.set(Some((format!("{e}"), false)));
                }
            }
            generating.set(false);
        });
    };

    let mcp_url = secret_value.as_ref().map(|s| {
        let origin = web_sys::window()
            .and_then(|w| w.location().origin().ok())
            .unwrap_or_default();
        format!("{origin}/mcp/{s}")
    });

    let on_copy = {
        let mcp_url = mcp_url.clone();
        move |_: MouseEvent| {
            if let Some(ref url) = mcp_url {
                #[cfg(not(feature = "server"))]
                {
                    let url = url.clone();
                    spawn(async move {
                        if let Some(window) = web_sys::window() {
                            let clipboard = window.navigator().clipboard();
                            let _ = wasm_bindgen_futures::JsFuture::from(
                                clipboard.write_text(&url),
                            )
                            .await;
                            copied.set(true);
                            gloo_timers::future::TimeoutFuture::new(2000).await;
                            copied.set(false);
                        }
                    });
                }
            }
        }
    };

    rsx! {
        div { class: "flex flex-col gap-5 w-full",
            h2 { class: "text-lg font-bold text-text-primary", {tr.mcp_server} }
            p { class: "text-sm text-foreground-muted", {tr.mcp_description} }

            if let Some(ref url) = mcp_url {
                SettingsRow { label: tr.mcp_server_url.to_string(),
                    div { class: "flex gap-2 items-center",
                        Input { value: url.clone(), disabled: true }
                        Button {
                            style: ButtonStyle::Secondary,
                            size: ButtonSize::Small,
                            onclick: on_copy,
                            if copied() {
                                {tr.mcp_copied}
                            } else {
                                {tr.mcp_copy}
                            }
                        }
                    }
                }
            } else {
                p { class: "text-sm text-foreground-muted italic", {tr.mcp_not_generated} }
            }

            if let Some((msg, is_success)) = mcp_message() {
                div { class: if is_success { "text-sm text-banner-success-text" } else { "text-sm text-destructive" },
                    "{msg}"
                }
            }

            div { class: "flex justify-end",
                Button {
                    style: if secret_value.is_some() { ButtonStyle::Secondary } else { ButtonStyle::Primary },
                    disabled: generating(),
                    onclick: on_generate,
                    if generating() {
                        {tr.mcp_generating}
                    } else if secret_value.is_some() {
                        {tr.mcp_regenerate}
                    } else {
                        {tr.mcp_generate}
                    }
                }
            }
        }
    }
}

fn is_blocked_text(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("test") || value.contains("테스트")
}
