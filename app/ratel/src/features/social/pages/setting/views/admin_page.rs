use super::super::components::DeleteTeamPopup;
use super::super::controllers::TeamResponse;
use super::super::controllers::{UpdateTeamRequest, delete_team_handler, update_team_handler};
use super::super::*;
use crate::features::membership::controllers::{
    UpdateBillingCardRequest, get_team_billing_info_handler, get_team_membership_handler,
    update_team_billing_card_handler,
};
use crate::features::membership::models::CardInfo;
use crate::features::social::pages::user_membership::components::format_membership_tier_label;
use dioxus::prelude::*;

fn format_last_saved(ts_millis: i64) -> String {
    if ts_millis == 0 {
        return String::new();
    }
    use chrono::{TimeZone, Utc};
    let dt = match Utc.timestamp_millis_opt(ts_millis).single() {
        Some(dt) => dt,
        None => return String::new(),
    };

    #[cfg(not(feature = "server"))]
    {
        // Get browser's local timezone offset (minutes) and apply it
        let offset_minutes = js_sys::Date::new_0().get_timezone_offset() as i64;
        let local_dt = dt - chrono::Duration::minutes(offset_minutes);
        local_dt.format("%Y-%m-%d %H:%M").to_string()
    }

    #[cfg(feature = "server")]
    {
        dt.format("%Y-%m-%d %H:%M UTC").to_string()
    }
}

#[component]
pub fn AdminPage(username: String, team: TeamResponse) -> Element {
    let tr: TeamSettingsTranslate = use_translate();
    let mut popup = use_popup();
    let navigator = use_navigator();

    let mut toast = use_toast();
    let mut is_saving = use_signal(|| false);

    let mut team_state = use_signal(|| team);
    let mut message = use_signal(|| Option::<String>::None);

    let mut thumbnail_url = use_signal(|| team_state().thumbnail_url.clone().unwrap_or_default());
    let mut profile_url = use_signal(|| team_state().profile_url.clone().unwrap_or_default());
    let mut nickname = use_signal(|| team_state().nickname.clone());
    let mut html_contents = use_signal(|| team_state().html_contents.clone());

    let validation_nickname_required = tr.validation_nickname_required;
    let success_update_team = tr.success_update_team;
    let failed_update_team = tr.failed_update_team;

    let on_save = {
        let username = username.clone();
        move |_: MouseEvent| {
            let display_name = nickname().trim().to_string();
            let description = html_contents().trim().to_string();
            let username = username.clone();

            if display_name.is_empty() {
                message.set(Some(validation_nickname_required.to_string()));
                return;
            }

            spawn(async move {
                is_saving.set(true);
                message.set(None);

                let result = update_team_handler(
                    username,
                    UpdateTeamRequest {
                        nickname: Some(display_name),
                        description: Some(description),
                        profile_url: Some(profile_url()),
                        dao_address: None,
                        thumbnail_url: Some(thumbnail_url()),
                    },
                )
                .await;

                is_saving.set(false);
                match result {
                    Ok(updated) => {
                        team_state.set(updated);
                        toast.info(success_update_team.to_string());
                    }
                    Err(err) => {
                        message.set(Some(format!("{}: {}", failed_update_team, err)));
                    }
                }
            });
        }
    };

    let mut on_open_delete = {
        let mut popup = popup;
        let username = username.clone();
        let navigator = navigator.clone();
        move |_evt: MouseEvent| {
            let on_cancel = {
                let mut popup = popup;
                move |_evt: MouseEvent| {
                    popup.close();
                }
            };
            let on_confirm = {
                let mut popup = popup;
                let username = username.clone();
                let navigator = navigator.clone();
                move |_evt: MouseEvent| {
                    let mut popup = popup;
                    let username = username.clone();
                    let navigator = navigator.clone();
                    spawn(async move {
                        let result = delete_team_handler(username).await;
                        popup.close();
                        if result.is_ok() {
                            navigator.push("/");
                        } else if let Err(err) = result {
                            error!("Delete team failed: {}", err);
                        }
                    });
                }
            };
            popup.open(rsx! {
                DeleteTeamPopup { on_confirm, on_cancel }
            });
        }
    };

    let delete_team_permission = team_state().role.is_owner();
    let last_saved = format_last_saved(team_state().updated_at);

    rsx! {
        div { class: "flex flex-col gap-8 w-full",
            Card { variant: CardVariant::Outlined, class: "p-6 w-full",
                div { class: "flex flex-col gap-8 w-full",
                    // Thumbnail (banner)
                    div { class: "flex flex-col gap-2",
                        span { class: "text-sm font-semibold text-text-primary", "{tr.thumbnail}" }
                        FileUploader {
                            on_upload_success: move |url: String| thumbnail_url.set(url),
                            accept: "image/*",
                            class: "w-full",
                            if !thumbnail_url().is_empty() {
                                img {
                                    src: "{thumbnail_url()}",
                                    alt: "Thumbnail",
                                    class: "w-full h-40 object-cover rounded-[10px] cursor-pointer",
                                }
                            } else {
                                div { class: "w-full h-40 rounded-[10px] border-2 border-dashed border-border bg-card-bg flex flex-col items-center justify-center gap-2 cursor-pointer hover:bg-white/5 transition-colors",
                                    lucide_dioxus::ImagePlus { class: "w-6 h-6 [&>path]:stroke-foreground-muted [&>line]:stroke-foreground-muted [&>polyline]:stroke-foreground-muted [&>circle]:stroke-foreground-muted" }
                                    span { class: "text-sm text-foreground-muted", "{tr.upload_banner}" }
                                }
                            }
                        }
                        span { class: "text-xs text-foreground-muted", "{tr.thumbnail_hint}" }
                    }

                    // Team Logo
                    div { class: "flex flex-col gap-2",
                        span { class: "text-sm font-semibold text-text-primary", "{tr.team_logo}" }
                        FileUploader {
                            on_upload_success: move |url: String| profile_url.set(url),
                            accept: "image/*",
                            if !profile_url().is_empty() {
                                img {
                                    src: "{profile_url()}",
                                    alt: "Team Logo",
                                    class: "w-20 h-20 rounded-[10px] object-cover cursor-pointer",
                                }
                            } else {
                                div { class: "w-20 h-20 rounded-[10px] bg-card-bg flex flex-col items-center justify-center gap-1 cursor-pointer hover:bg-white/5 transition-colors",
                                    lucide_dioxus::ImagePlus { class: "w-5 h-5 [&>path]:stroke-foreground-muted [&>line]:stroke-foreground-muted [&>polyline]:stroke-foreground-muted [&>circle]:stroke-foreground-muted" }
                                }
                            }
                        }
                        span { class: "text-xs text-foreground-muted", "{tr.team_logo_hint}" }
                    }

                    // Team name
                    div { class: "flex flex-col gap-3 w-full",
                        label { class: "text-sm font-semibold text-text-primary", "{tr.team_name}" }
                        Input {
                            class: "w-full",
                            variant: InputVariant::Default,
                            r#type: InputType::Text,
                            placeholder: tr.display_name_hint.to_string(),
                            value: nickname(),
                            oninput: move |e: FormEvent| nickname.set(e.value()),
                        }
                    }

                    // Description + Last saved
                    div { class: "flex flex-col gap-2 w-full",
                        label { class: "text-sm font-semibold text-text-primary", "Description" }
                        TextArea {
                            placeholder: tr.team_description_hint.to_string(),
                            value: html_contents(),
                            oninput: move |e: FormEvent| html_contents.set(e.value()),
                            class: "w-full min-h-[120px] resize-y rounded-lg border border-input-box-border bg-input-box-bg px-3 py-2 text-sm text-text-primary placeholder:text-foreground-muted focus:outline-none focus:border-ring",
                        }
                        if !last_saved.is_empty() {
                            div { class: "flex justify-end",
                                span { class: "text-xs text-foreground-muted",
                                    "{tr.last_saved_at} {last_saved}"
                                }
                            }
                        }

                        div { class: "flex justify-end pt-2",
                            Button {
                                size: ButtonSize::Medium,
                                style: ButtonStyle::Primary,
                                loading: is_saving(),
                                onclick: on_save,
                                "{tr.save}"
                            }
                        }
                    }

                    if let Some(msg) = message() {
                        div { class: "text-sm text-destructive", "{msg}" }
                    }
                }
            }

            if delete_team_permission {
                Card { variant: CardVariant::Outlined, class: "p-6",
                    TeamSubscriptionCard { username: username.clone() }
                }
            }

            // Delete team — bottom right
            if delete_team_permission {
                Card { variant: CardVariant::Outlined, class: "p-6",
                    div { class: "flex justify-end",
                        Button {
                            size: ButtonSize::Medium,
                            style: ButtonStyle::Outline,
                            shape: ButtonShape::Square,
                            class: "text-destructive border-destructive hover:bg-destructive/10",
                            onclick: move |e| on_open_delete(e),
                            "{tr.delete_team}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TeamSettingsRow(label: String, children: Element) -> Element {
    rsx! {
        div { class: "flex flex-col gap-1.5",
            label { class: "text-sm font-semibold text-text-primary", "{label}" }
            {children}
        }
    }
}

#[component]
fn TeamSubscriptionCard(username: String) -> Element {
    let tr: TeamSettingsTranslate = use_translate();
    let membership = use_server_future({
        let username = username.clone();
        move || {
            let username = username.clone();
            async move { get_team_membership_handler(username).await }
        }
    })?;
    let mut billing_info = use_server_future({
        let username = username.clone();
        move || {
            let username = username.clone();
            async move { get_team_billing_info_handler(username).await }
        }
    })?;

    let membership_data = membership.read();
    let billing_data = billing_info.read();

    let (tier_label, remaining, total, expired_at, is_free) = match membership_data.as_ref() {
        Some(Ok(m)) => {
            let tier = format_membership_tier_label(&m.tier.0, tr.enterprise);
            let free = tier.eq_ignore_ascii_case("free");
            (
                tier,
                m.remaining_credits,
                m.total_credits,
                m.expired_at,
                free,
            )
        }
        Some(Err(_)) => ("Free".to_string(), 0, 0, 0, true),
        None => {
            return rsx! {
                div { class: "flex justify-center items-center w-full py-6",
                    crate::common::components::LoadingIndicator {}
                }
            };
        }
    };

    let billing = billing_data
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .cloned()
        .unwrap_or_default();

    let expiry_text = if expired_at == 0 {
        tr.unlimited.to_string()
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

    let on_save_card = {
        let username = username.clone();
        move |_: MouseEvent| {
            let info = CardInfo {
                card_number: card_number().trim().to_string(),
                expiry_year: expiry_year().trim().to_string(),
                expiry_month: expiry_month().trim().to_string(),
                birth_or_business_registration_number: birth_or_biz().trim().to_string(),
                password_two_digits: card_password().trim().to_string(),
            };
            let username = username.clone();
            spawn(async move {
                saving.set(true);
                card_message.set(None);
                match update_team_billing_card_handler(
                    username,
                    UpdateBillingCardRequest { card_info: info },
                )
                .await
                {
                    Ok(_) => {
                        card_number.set(String::new());
                        expiry_month.set(String::new());
                        expiry_year.set(String::new());
                        birth_or_biz.set(String::new());
                        card_password.set(String::new());
                        show_card_form.set(false);
                        card_message.set(Some((tr.card_updated.to_string(), true)));
                        billing_info.restart();
                    }
                    Err(e) => {
                        card_message.set(Some((format!("{e}"), false)));
                    }
                }
                saving.set(false);
            });
        }
    };

    rsx! {
        div { class: "flex flex-col gap-5 w-full",
            h2 { class: "text-lg font-bold text-text-primary", "{tr.subscription_billing}" }

            div { class: "flex flex-col gap-3",
                Row {
                    main_axis_align: MainAxisAlign::Between,
                    cross_axis_align: CrossAxisAlign::Center,
                    span { class: "text-sm text-foreground-muted", "{tr.current_plan}" }
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
                            to: format!("/{username}/team-settings/subscription"),
                            class: "text-xs text-primary hover:underline no-underline",
                            "{tr.change_plan}"
                        }
                    }
                }
                if !is_free {
                    Row { main_axis_align: MainAxisAlign::Between,
                        span { class: "text-sm text-foreground-muted", "{tr.credits}" }
                        span { class: "text-sm text-text-primary", "{remaining} / {total}" }
                    }
                    Row { main_axis_align: MainAxisAlign::Between,
                        span { class: "text-sm text-foreground-muted", "{tr.expires}" }
                        span { class: "text-sm text-text-primary", "{expiry_text}" }
                    }
                } else {
                    Row { main_axis_align: MainAxisAlign::Between,
                        span { class: "text-sm text-foreground-muted", "{tr.credits}" }
                        span { class: "text-sm text-text-primary", "0 / {total}" }
                    }
                }
                if !is_free {
                    if let Some(ref masked) = billing.masked_card_number {
                        Row { main_axis_align: MainAxisAlign::Between,
                            span { class: "text-sm text-foreground-muted", "{tr.card}" }
                            span { class: "text-sm text-text-primary", "{masked}" }
                        }
                    }
                    if !billing.customer_name.is_empty() {
                        Row { main_axis_align: MainAxisAlign::Between,
                            span { class: "text-sm text-foreground-muted", "{tr.card_holder}" }
                            span { class: "text-sm text-text-primary", "{billing.customer_name}" }
                        }
                    }
                    div { class: "flex justify-end pt-1",
                        Button {
                            style: ButtonStyle::Secondary,
                            onclick: move |_| show_card_form.set(!show_card_form()),
                            if show_card_form() {
                                "{tr.cancel}"
                            } else if billing.has_card {
                                "{tr.change_card}"
                            } else {
                                "{tr.add_card}"
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
                        TeamSettingsRow { label: tr.card_number.to_string(),
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
                                    "{tr.expiry_month}"
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
                                    "{tr.expiry_year}"
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
                        TeamSettingsRow { label: tr.birth_date.to_string(),
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
                        TeamSettingsRow { label: tr.card_password_label.to_string(),
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
                                    "{tr.saving_card}"
                                } else {
                                    "{tr.save_card}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
