use crate::common::*;
use crate::features::social::pages::team_arena::{use_team_arena, TeamArenaTab};
use crate::route::Route;

#[allow(unused_imports)]
use dioxus::prelude::*;

mod admin_page;
mod management_page;
mod subscription_page;
mod viewer_page;

#[allow(unused_imports)]
use admin_page::*;
pub use management_page::ManagementPage;
pub use subscription_page::SubscriptionPage;
#[allow(unused_imports)]
pub use viewer_page::*;

use super::controllers::{
    delete_team_handler, get_team_settings_handler, update_team_handler, UpdateTeamRequest,
};
use super::i18n::TeamSettingsTranslate;

#[component]
pub fn Home(username: String) -> Element {
    let tr: TeamSettingsTranslate = use_translate();
    let nav = use_navigator();
    let mut toast = use_toast();
    let mut team_ctx = crate::common::contexts::use_team_context();

    // Sync the arena topbar's active tab.
    let mut arena = use_team_arena();
    use_effect(move || arena.active_tab.set(TeamArenaTab::Settings));
    let mut refresh_trigger = arena.refresh_trigger;

    let mut team_resource = use_server_future(use_reactive((&username,), |(name,)| async move {
        get_team_settings_handler(name).await
    }))?;

    let team = {
        let binding = team_resource.read();
        binding.as_ref().and_then(|r| r.as_ref().ok()).cloned()
    };

    // Permission gate — viewers see a placeholder.
    let can_edit = team.as_ref().map(|t| t.role.is_admin_or_owner()).unwrap_or(false);

    if !can_edit {
        return rsx! {
            div { class: "ts-page",
                div { class: "ts-card",
                    p { style: "color: var(--text-muted); font-size: 13px; text-align: center; padding: 40px 20px;",
                        "{tr.no_permission}"
                    }
                }
            }
        };
    }

    // Form state — initialized from the server snapshot, then re-synced
    // whenever the server data changes (e.g., after Save Changes triggers
    // `team_resource.restart()`).
    let initial_nickname = team.as_ref().map(|t| t.nickname.clone()).unwrap_or_default();
    let initial_description = team
        .as_ref()
        .map(|t| t.html_contents.clone())
        .unwrap_or_default();
    let initial_profile = team
        .as_ref()
        .and_then(|t| t.profile_url.clone())
        .unwrap_or_default();
    let updated_at = team.as_ref().map(|t| t.updated_at).unwrap_or(0);
    let display_name_for_delete = team
        .as_ref()
        .map(|t| {
            if t.nickname.is_empty() {
                t.username.clone()
            } else {
                t.nickname.clone()
            }
        })
        .unwrap_or_else(|| username.clone());

    let mut nickname = use_signal(|| initial_nickname.clone());
    let mut description = use_signal(|| initial_description.clone());
    let mut profile_url = use_signal(|| initial_profile.clone());
    let mut server_baseline = use_signal(|| {
        (
            initial_nickname.clone(),
            initial_description.clone(),
            initial_profile.clone(),
        )
    });
    let mut saving = use_signal(|| false);

    // Sync form state when the server snapshot updates. We read
    // `team_resource` inside the effect so Dioxus tracks it as a dependency
    // and re-runs after `team_resource.restart()` or a page refresh.
    use_effect(move || {
        let data = team_resource.read();
        let t = data.as_ref().and_then(|r| r.as_ref().ok());
        let snap = (
            t.map(|t| t.nickname.clone()).unwrap_or_default(),
            t.map(|t| t.html_contents.clone()).unwrap_or_default(),
            t.and_then(|t| t.profile_url.clone()).unwrap_or_default(),
        );
        let baseline = server_baseline.peek().clone();
        if baseline != snap {
            nickname.set(snap.0.clone());
            description.set(snap.1.clone());
            profile_url.set(snap.2.clone());
            server_baseline.set(snap);
        }
    });

    let dirty = use_memo(move || {
        let (n0, d0, p0) = server_baseline();
        nickname() != n0 || description() != d0 || profile_url() != p0
    });

    let on_save = {
        let username_for_save = username.clone();
        move |_| {
            if !dirty() || saving() {
                return;
            }
            let username = username_for_save.clone();
            saving.set(true);
            spawn(async move {
                let req = UpdateTeamRequest {
                    nickname: Some(nickname()),
                    description: Some(description()),
                    profile_url: Some(profile_url()),
                    dao_address: None,
                    thumbnail_url: None,
                };
                match update_team_handler(username, req).await {
                    Ok(_) => {
                        toast.info(tr.save_success);
                        team_resource.restart();
                        // Refresh team switcher dropdown so the new name/logo
                        // appear immediately across the topbar/dropdown.
                        if let Ok(resp) = crate::features::social::controllers::get_user_teams_handler(None).await {
                            team_ctx.set_teams(resp.items);
                        }
                        // Force the arena layout to refetch the team profile
                        // so the topbar title/logo update without a page reload.
                        refresh_trigger.with_mut(|n| *n = n.wrapping_add(1));
                    }
                    Err(e) => {
                        toast.error(e);
                    }
                }
                saving.set(false);
            });
        }
    };

    let mut delete_open = use_signal(|| false);
    let on_delete = {
        let username_for_delete = username.clone();
        move |_| {
            let username = username_for_delete.clone();
            saving.set(true);
            spawn(async move {
                match delete_team_handler(username).await {
                    Ok(_) => {
                        toast.info(tr.delete_success);
                        delete_open.set(false);
                        nav.push(Route::Index {});
                    }
                    Err(e) => {
                        toast.error(e);
                    }
                }
                saving.set(false);
            });
        }
    };

    rsx! {

        div { class: "ts-section-label",
            span { class: "ts-section-label__dash" }
            span { class: "ts-section-label__title",
                "Team "
                strong { "{tr.settings_label}" }
            }
            span { class: "ts-section-label__dash" }
        }

        div { class: "ts-page",

            // ── Profile card ────────────────────────────
            div { class: "ts-card",
                div { class: "ts-card__header",
                    span { class: "ts-card__title", "{tr.team_profile}" }
                    span { class: "ts-card__dash" }
                }

                div { class: "ts-field",
                    label { class: "ts-field__label", "{tr.team_logo}" }
                    div { class: "ts-logo-row",
                        if !profile_url().is_empty() {
                            img {
                                class: "ts-logo-thumb",
                                src: "{profile_url()}",
                                alt: "team logo",
                            }
                        } else {
                            div { class: "ts-logo-thumb",
                                svg {
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "1.8",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    path { d: "M20.91 11.12L12 2L3.09 11.12c-.6.6-.6 1.57 0 2.17L11 21a2 2 0 0 0 2 0l7.91-7.71c.6-.6.6-1.57 0-2.17z" }
                                    circle {
                                        cx: "12",
                                        cy: "12",
                                        r: "2.2",
                                        fill: "currentColor",
                                    }
                                }
                            }
                        }
                        div { style: "display:flex;flex-direction:column;gap:6px;flex:1;min-width:0;align-items:flex-start;",
                            FileUploader {
                                accept: Some("image/*".to_string()),
                                on_upload_success: move |url| profile_url.set(url),
                                class: Some("ts-logo-upload-wrap".to_string()),
                                span { class: "ts-logo-upload-btn",
                                    svg {
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                                        polyline { points: "17 8 12 3 7 8" }
                                        line {
                                            x1: "12",
                                            y1: "3",
                                            x2: "12",
                                            y2: "15",
                                        }
                                    }
                                    "{tr.upload_logo}"
                                }
                            }
                            span { class: "ts-field__hint", "{tr.logo_hint}" }
                        }
                    }
                }

                div { class: "ts-field",
                    label { class: "ts-field__label", r#for: "ts-team-name", "{tr.team_name}" }
                    input {
                        id: "ts-team-name",
                        class: "ts-field__input",
                        r#type: "text",
                        value: "{nickname}",
                        placeholder: "{tr.team_name_placeholder}",
                        oninput: move |e| nickname.set(e.value()),
                    }
                }

                div { class: "ts-field",
                    label { class: "ts-field__label", r#for: "ts-team-desc", "{tr.description}" }
                    textarea {
                        id: "ts-team-desc",
                        class: "ts-field__textarea",
                        placeholder: "{tr.description_placeholder}",
                        value: "{description}",
                        oninput: move |e| description.set(e.value()),
                    }
                }

                div { class: "ts-save-footer",
                    span { class: "ts-save-timestamp",
                        "{tr.last_saved} "
                        strong { "{format_timestamp(updated_at)}" }
                    }
                    button {
                        class: "ts-btn-primary",
                        r#type: "button",
                        disabled: !dirty() || saving(),
                        onclick: on_save,
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2.5",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z" }
                            polyline { points: "17 21 17 13 7 13 7 21" }
                            polyline { points: "7 3 7 8 15 8" }
                        }
                        "{tr.save_changes}"
                    }
                }
            }

            // ── Subscription & Billing ──────────────────
            TsSubscriptionCard { username: username.clone() }

            // ── Danger zone ─────────────────────────────
            div { class: "ts-card ts-danger-card",
                div { class: "ts-danger-card__header",
                    div { class: "ts-danger-card__left",
                        span { class: "ts-danger-card__title", "{tr.danger_zone}" }
                        span { class: "ts-danger-card__desc", "{tr.danger_zone_desc}" }
                    }
                    button {
                        class: "ts-btn-danger",
                        r#type: "button",
                        onclick: move |_| delete_open.set(true),
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "3 6 5 6 21 6" }
                            path { d: "M19 6l-2 14a2 2 0 0 1-2 2H9a2 2 0 0 1-2-2L5 6" }
                            path { d: "M10 11v6M14 11v6" }
                            path { d: "M9 6V4a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v2" }
                        }
                        "{tr.delete_team}"
                    }
                }
            }
        }

        // ── Delete confirm modal ────────────────────────
        if delete_open() {
            div {
                class: "ts-modal-overlay",
                onclick: move |_| delete_open.set(false),
                div {
                    class: "ts-modal",
                    onclick: move |e: Event<MouseData>| e.stop_propagation(),
                    div { class: "ts-modal__header",
                        span { class: "ts-modal__title", "{tr.delete_team_confirm_title}" }
                        button {
                            class: "ts-modal__close",
                            r#type: "button",
                            aria_label: "Close",
                            onclick: move |_| delete_open.set(false),
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2.5",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                line {
                                    x1: "18",
                                    y1: "6",
                                    x2: "6",
                                    y2: "18",
                                }
                                line {
                                    x1: "6",
                                    y1: "6",
                                    x2: "18",
                                    y2: "18",
                                }
                            }
                        }
                    }
                    div { class: "ts-modal__body",
                        p { class: "ts-modal__desc",
                            "{tr.delete_confirm_pre}"
                            strong { "{display_name_for_delete}" }
                            "{tr.delete_confirm_post}"
                        }
                        div { class: "ts-modal__actions",
                            button {
                                class: "ts-modal__cancel",
                                r#type: "button",
                                onclick: move |_| delete_open.set(false),
                                "{tr.cancel}"
                            }
                            button {
                                class: "ts-modal__confirm-danger",
                                r#type: "button",
                                disabled: saving(),
                                onclick: on_delete,
                                "{tr.delete}"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn format_timestamp(ms: i64) -> String {
    use chrono::{Local, TimeZone};
    if ms == 0 {
        return "—".to_string();
    }
    Local
        .timestamp_millis_opt(ms)
        .single()
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_else(|| "—".to_string())
}

fn format_expiry(ms: i64) -> String {
    use chrono::{Local, TimeZone};
    if ms == 0 {
        return "—".to_string();
    }
    Local
        .timestamp_millis_opt(ms)
        .single()
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "—".to_string())
}

#[component]
fn TsSubscriptionCard(username: String) -> Element {
    use crate::features::membership::controllers::{
        get_team_billing_info_handler, get_team_membership_handler,
        update_team_billing_card_handler, UpdateBillingCardRequest,
    };
    use crate::features::membership::models::CardInfo;
    use crate::features::social::pages::user_membership::components::format_membership_tier_label;

    let tr: TeamSettingsTranslate = use_translate();
    let mut toast = use_toast();

    let membership = use_server_future({
        let username = username.clone();
        move || {
            let username = username.clone();
            async move { get_team_membership_handler(username).await }
        }
    })?;
    let mut billing_resource = use_server_future({
        let username = username.clone();
        move || {
            let username = username.clone();
            async move { get_team_billing_info_handler(username).await }
        }
    })?;

    let membership_data = membership.read();
    let billing_data = billing_resource.read();

    let (tier_label, remaining, total, expired_at, is_free) = match membership_data.as_ref() {
        Some(Ok(m)) => {
            let tier = format_membership_tier_label(&m.tier.0, tr.enterprise);
            let free = tier.eq_ignore_ascii_case("free");
            (tier, m.remaining_credits, m.total_credits, m.expired_at, free)
        }
        _ => ("Free".to_string(), 0_i64, 10_i64, 0_i64, true),
    };

    let billing = billing_data
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .cloned()
        .unwrap_or_default();

    let tier_class = match tier_label.as_str() {
        "Pro" => "ts-plan-badge ts-plan-badge--pro",
        "Max" => "ts-plan-badge ts-plan-badge--max",
        "Vip" => "ts-plan-badge ts-plan-badge--vip",
        _ => "ts-plan-badge",
    };

    let mut show_card_form = use_signal(|| false);
    let mut card_number = use_signal(String::new);
    let mut expiry_month = use_signal(String::new);
    let mut expiry_year = use_signal(String::new);
    let mut birth_or_biz = use_signal(String::new);
    let mut card_password = use_signal(String::new);
    let mut card_saving = use_signal(|| false);

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
            if !is_valid() || card_saving() {
                return;
            }
            let info = CardInfo {
                card_number: card_number().trim().to_string(),
                expiry_year: expiry_year().trim().to_string(),
                expiry_month: expiry_month().trim().to_string(),
                birth_or_business_registration_number: birth_or_biz().trim().to_string(),
                password_two_digits: card_password().trim().to_string(),
            };
            let username = username.clone();
            spawn(async move {
                card_saving.set(true);
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
                        toast.info(tr.card_updated);
                        billing_resource.restart();
                    }
                    Err(e) => {
                        toast.error(e);
                    }
                }
                card_saving.set(false);
            });
        }
    };

    rsx! {
        div { class: "ts-card",
            div { class: "ts-card__header",
                span { class: "ts-card__title", "{tr.subscription_billing}" }
                span { class: "ts-card__dash" }
            }

            div { class: "ts-billing-row",
                span { class: "ts-billing-row__label", "{tr.current_plan}" }
                div { class: "ts-billing-row__right",
                    span { class: "{tier_class}", "{tier_label}" }
                    Link {
                        class: "ts-plan-change-link",
                        "data-testid": "team-settings-view-membership",
                        to: Route::TeamMemberships {
                            username: username.clone(),
                        },
                        "{tr.view_membership}"
                    }
                    Link {
                        class: "ts-plan-change-link",
                        "data-testid": "team-settings-change-plan",
                        to: Route::TeamSettingSubscription {
                            username: username.clone(),
                        },
                        "{tr.change_plan}"
                    }
                }
            }

            div { class: "ts-billing-row",
                span { class: "ts-billing-row__label", "{tr.credits}" }
                div { class: "ts-billing-row__right",
                    span { class: "ts-credit-value",
                        "{remaining}"
                        span { " / {total}" }
                    }
                }
            }

            if !is_free {
                div { class: "ts-billing-row",
                    span { class: "ts-billing-row__label", "{tr.expires}" }
                    div { class: "ts-billing-row__right",
                        span { class: "ts-credit-value", "{format_expiry(expired_at)}" }
                    }
                }
                if let Some(ref masked) = billing.masked_card_number {
                    div { class: "ts-billing-row",
                        span { class: "ts-billing-row__label", "{tr.card}" }
                        div { class: "ts-billing-row__right",
                            span { class: "ts-credit-value", "{masked}" }
                        }
                    }
                }
                if !billing.customer_name.is_empty() {
                    div { class: "ts-billing-row",
                        span { class: "ts-billing-row__label", "{tr.card_holder}" }
                        div { class: "ts-billing-row__right",
                            span { class: "ts-credit-value", "{billing.customer_name}" }
                        }
                    }
                }
                div { class: "ts-billing-row",
                    span { class: "ts-billing-row__label" }
                    div { class: "ts-billing-row__right",
                        button {
                            class: "ts-logo-upload-btn",
                            r#type: "button",
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

                if show_card_form() {
                    div { class: "ts-card-form",
                        div { class: "ts-field",
                            label { class: "ts-field__label", "{tr.card_number}" }
                            input {
                                class: "ts-field__input",
                                r#type: "text",
                                placeholder: "0000000000000000",
                                maxlength: "16",
                                value: "{card_number}",
                                oninput: move |e| {
                                    let v = e.value().chars().filter(|c| c.is_ascii_digit()).collect::<String>();
                                    card_number.set(v);
                                },
                            }
                        }
                        div { class: "ts-field-row",
                            div { class: "ts-field",
                                label { class: "ts-field__label", "{tr.expiry_month}" }
                                input {
                                    class: "ts-field__input",
                                    r#type: "text",
                                    placeholder: "MM",
                                    maxlength: "2",
                                    value: "{expiry_month}",
                                    oninput: move |e| {
                                        let v = e.value().chars().filter(|c| c.is_ascii_digit()).collect::<String>();
                                        expiry_month.set(v);
                                    },
                                }
                            }
                            div { class: "ts-field",
                                label { class: "ts-field__label", "{tr.expiry_year}" }
                                input {
                                    class: "ts-field__input",
                                    r#type: "text",
                                    placeholder: "YY",
                                    maxlength: "2",
                                    value: "{expiry_year}",
                                    oninput: move |e| {
                                        let v = e.value().chars().filter(|c| c.is_ascii_digit()).collect::<String>();
                                        expiry_year.set(v);
                                    },
                                }
                            }
                        }
                        div { class: "ts-field",
                            label { class: "ts-field__label", "{tr.birth_date}" }
                            input {
                                class: "ts-field__input",
                                r#type: "text",
                                placeholder: "YYMMDD",
                                maxlength: "10",
                                value: "{birth_or_biz}",
                                oninput: move |e| birth_or_biz.set(e.value()),
                            }
                        }
                        div { class: "ts-field",
                            label { class: "ts-field__label", "{tr.card_password_label}" }
                            input {
                                class: "ts-field__input",
                                r#type: "password",
                                placeholder: "••",
                                maxlength: "2",
                                value: "{card_password}",
                                oninput: move |e| {
                                    let v = e.value().chars().filter(|c| c.is_ascii_digit()).collect::<String>();
                                    card_password.set(v);
                                },
                            }
                        }
                        div { class: "ts-save-footer",
                            span { class: "ts-save-timestamp" }
                            button {
                                class: "ts-btn-primary",
                                r#type: "button",
                                disabled: !is_valid() || card_saving(),
                                onclick: on_save_card,
                                if card_saving() {
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
