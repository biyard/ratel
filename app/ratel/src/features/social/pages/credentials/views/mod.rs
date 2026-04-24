mod i18n;

use super::controllers::get_credentials::{get_credentials_handler, CredentialResponse};
use super::controllers::sign_attributes::{sign_attributes_handler, SignAttributesRequest};
use super::*;
use crate::features::auth::hooks::use_user_context;
use crate::features::did::controllers::{
    get_did_summary::get_did_summary_handler, DidAttributeItem, DidSummaryResponse,
};
use dioxus::prelude::*;

pub use i18n::CredentialsTranslate;

const YEAR_MS: i64 = 365 * 24 * 60 * 60 * 1000;

fn format_date(ts_millis: i64) -> String {
    use chrono::DateTime;
    DateTime::from_timestamp_millis(ts_millis)
        .map(|dt| dt.format("%b %d, %Y").to_string())
        .unwrap_or_else(|| "—".to_string())
}

fn format_datetime_utc(ts_millis: i64) -> String {
    use chrono::DateTime;
    DateTime::from_timestamp_millis(ts_millis)
        .map(|dt| dt.format("%Y-%m-%d %H:%M UTC").to_string())
        .unwrap_or_else(|| "—".to_string())
}

fn time_ago(ts_millis: i64) -> String {
    crate::common::utils::time::time_ago(ts_millis)
}

fn attr_label_for(key: &str, tr: &CredentialsTranslate) -> String {
    match key {
        "age" => tr.attr_age.to_string(),
        "gender" => tr.attr_gender.to_string(),
        "university" => tr.attr_university.to_string(),
        "employer" => tr.attr_employer.to_string(),
        "membership" => tr.attr_membership.to_string(),
        other => other.to_string(),
    }
}

fn gender_label(raw: &str, tr: &CredentialsTranslate) -> String {
    match raw.to_lowercase().as_str() {
        "male" => tr.attr_male.to_string(),
        "female" => tr.attr_female.to_string(),
        _ => raw.to_string(),
    }
}

#[component]
pub fn Home(username: String) -> Element {
    let _ = username;
    let tr: CredentialsTranslate = use_translate();
    let user_ctx = use_user_context();
    let mut toast = use_toast();
    let mut popup = use_popup();
    let nav = use_navigator();

    let mut summary = use_loader(get_did_summary_handler)?;
    let mut credential = use_loader(get_credentials_handler)?;

    let summary_val = summary();
    let _credential_val = credential();

    let refresh = use_callback(move |_: ()| {
        summary.restart();
        credential.restart();
    });

    let fallback_did = user_ctx().did();

    rsx! {
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "anonymous",
        }
        document::Stylesheet { href: "https://fonts.googleapis.com/css2?family=Orbitron:wght@400;500;600;700;800;900&family=Outfit:wght@300;400;500;600;700&display=swap" }
        document::Stylesheet { href: asset!("./style.css") }
        document::Script { defer: true, src: asset!("./script.js") }

        div { class: "credentials-arena",
            div { class: "arena-topbar",
                div { class: "arena-topbar__left",
                    button {
                        class: "back-btn",
                        "aria-label": tr.back,
                        onclick: move |_| {
                            nav.go_back();
                        },
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "15 18 9 12 15 6" }
                        }
                    }
                    div { class: "topbar-title",
                        span { class: "topbar-title__main", "{tr.page_title}" }
                        if summary_val.verified_count > 0 {
                            span { class: "topbar-title__status", "{tr.status_verified}" }
                        } else {
                            span { class: "topbar-title__status topbar-title__status--pending",
                                "{tr.status_pending}"
                            }
                        }
                    }
                }
                button {
                    class: "topbar-btn",
                    disabled: summary_val.verified_count == 0,
                    onclick: move |_| {
                        let summary = summary_val.clone();
                        spawn(async move {
                            export_vc(&summary).await;
                        });
                    },
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                        polyline { points: "7 10 12 15 17 10" }
                        line {
                            x1: "12",
                            y1: "15",
                            x2: "12",
                            y2: "3",
                        }
                    }
                    "{tr.export_vc}"
                }
            }

            div { class: "page",
                HeroCard {
                    tr: tr.clone(),
                    summary: summary_val.clone(),
                    fallback_did: fallback_did.clone(),
                }

                StatsStrip { tr: tr.clone(), summary: summary_val.clone() }

                div { class: "section-head",
                    span { class: "section-head__title", "{tr.methods_title}" }
                    span { class: "section-head__hint",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            rect {
                                x: "3",
                                y: "11",
                                width: "18",
                                height: "11",
                                rx: "2",
                                ry: "2",
                            }
                            path { d: "M7 11V7a5 5 0 0 1 10 0v4" }
                        }
                        "{tr.methods_hint}"
                    }
                }

                MethodsSection {
                    tr: tr.clone(),
                    summary: summary_val.clone(),
                    on_refresh: refresh,
                }

                div { class: "section-head",
                    span { class: "section-head__title", "{tr.attrs_title}" }
                    span { class: "section-head__hint",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            rect {
                                x: "3",
                                y: "11",
                                width: "18",
                                height: "11",
                                rx: "2",
                                ry: "2",
                            }
                            path { d: "M7 11V7a5 5 0 0 1 10 0v4" }
                        }
                        "{tr.attrs_hint}"
                    }
                }

                AttrsGrid {
                    tr: tr.clone(),
                    summary: summary_val.clone(),
                    on_refresh: refresh,
                }

                div { class: "section-head",
                    span { class: "section-head__title", "{tr.proof_title}" }
                    span { class: "section-head__hint",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "20 6 9 17 4 12" }
                        }
                        "{tr.proof_hint}"
                    }
                }

                ProofCard { tr: tr.clone(), summary: summary_val.clone() }

                PrivacyNotice { tr: tr.clone(), did: summary_val.did.clone() }
            }
        }
    }
}

#[component]
fn HeroCard(tr: CredentialsTranslate, summary: DidSummaryResponse, fallback_did: String) -> Element {
    let did_display = if summary.did.is_empty() {
        fallback_did
    } else {
        summary.did.clone()
    };
    let issued_at = if summary.issued_at > 0 {
        summary.issued_at
    } else {
        crate::common::utils::time::now()
    };
    let expires_at = issued_at + YEAR_MS;

    rsx! {
        div { class: "vc-hero",
            div { class: "vc-hero__main",
                div { class: "vc-hero__eyebrow",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" }
                    }
                    span { "{tr.hero_eyebrow}" }
                    span { style: "color:var(--text-dim)", "·" }
                    strong { "{tr.hero_methods}" }
                }
                div { class: "vc-hero__crest",
                    div { class: "vc-hero__shield",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" }
                            polyline { points: "9 12 11 14 15 10" }
                        }
                    }
                    div { class: "vc-hero__crest-body",
                        span { class: "vc-hero__crest-title", "{tr.hero_crest_title}" }
                        span { class: "vc-hero__crest-sub", "{tr.hero_crest_sub}" }
                    }
                }
                div { class: "vc-hero__did",
                    span { class: "vc-hero__did-label",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" }
                            path { d: "M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" }
                        }
                        "{tr.hero_did_label}"
                    }
                    div { class: "vc-hero__did-value",
                        span {
                            class: "vc-hero__did-value-text",
                            "data-did-value": true,
                            "{did_display}"
                        }
                        button {
                            class: "vc-hero__did-copy",
                            "aria-label": tr.hero_copy_did,
                            "data-did-copy": true,
                            svg {
                                class: "icon-copy",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                rect {
                                    x: "9",
                                    y: "9",
                                    width: "13",
                                    height: "13",
                                    rx: "2",
                                    ry: "2",
                                }
                                path { d: "M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" }
                            }
                            svg {
                                class: "icon-ok",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "3",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                polyline { points: "20 6 9 17 4 12" }
                            }
                        }
                    }
                }
                div { class: "vc-hero__meta",
                    span { class: "vc-hero__meta-item",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            rect {
                                x: "3",
                                y: "4",
                                width: "18",
                                height: "18",
                                rx: "2",
                                ry: "2",
                            }
                            line {
                                x1: "16",
                                y1: "2",
                                x2: "16",
                                y2: "6",
                            }
                            line {
                                x1: "8",
                                y1: "2",
                                x2: "8",
                                y2: "6",
                            }
                            line {
                                x1: "3",
                                y1: "10",
                                x2: "21",
                                y2: "10",
                            }
                        }
                        "{tr.hero_issued} "
                        strong { "{format_date(issued_at)}" }
                    }
                    span { class: "vc-hero__meta-item",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            circle { cx: "12", cy: "12", r: "10" }
                            polyline { points: "12 6 12 12 16 14" }
                        }
                        "{tr.hero_expires} "
                        strong { "{format_date(expires_at)}" }
                    }
                    span { class: "vc-hero__meta-item",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "20 6 9 17 4 12" }
                        }
                        strong { "{summary.verified_count}" }
                        "/{summary.total_slots} {tr.hero_verified_count}"
                    }
                }
            }
            div { class: "vc-hero__qr", "aria-label": tr.hero_qr_hint,
                div {
                    class: "vc-hero__qr-canvas",
                    dangerous_inner_html: render_qr_placeholder(),
                }
                span { class: "vc-hero__qr-hint", "{tr.hero_qr_hint}" }
            }
        }
    }
}

#[component]
fn StatsStrip(tr: CredentialsTranslate, summary: DidSummaryResponse) -> Element {
    let last_verified = summary
        .last_verified_at
        .map(time_ago)
        .unwrap_or_else(|| tr.stat_never.to_string());
    rsx! {
        div { class: "stats-strip",
            div { class: "stat-card stat-card--violet",
                div { class: "stat-card__icon",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" }
                    }
                }
                div { class: "stat-card__body",
                    span { class: "stat-card__label", "{tr.stat_attributes}" }
                    span { class: "stat-card__value",
                        "{summary.verified_count} "
                        small { "/ {summary.total_slots} {tr.stat_attributes_sub}" }
                    }
                }
            }
            div { class: "stat-card stat-card--teal",
                div { class: "stat-card__icon",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "16 18 22 12 16 6" }
                        polyline { points: "8 6 2 12 8 18" }
                    }
                }
                div { class: "stat-card__body",
                    span { class: "stat-card__label", "{tr.stat_methods}" }
                    span { class: "stat-card__value",
                        "2 "
                        small { "{tr.stat_methods_sub}" }
                    }
                }
            }
            div { class: "stat-card",
                div { class: "stat-card__icon",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        circle { cx: "12", cy: "12", r: "10" }
                        polyline { points: "12 6 12 12 16 14" }
                    }
                }
                div { class: "stat-card__body",
                    span { class: "stat-card__label", "{tr.stat_last_verified}" }
                    span { class: "stat-card__value", "{last_verified}" }
                }
            }
        }
    }
}

#[component]
fn MethodsSection(
    tr: CredentialsTranslate,
    summary: DidSummaryResponse,
    on_refresh: Callback<()>,
) -> Element {
    let kyc_age_verified = summary
        .attributes
        .iter()
        .any(|a| a.key == "age" && a.verified);
    let kyc_gender_verified = summary
        .attributes
        .iter()
        .any(|a| a.key == "gender" && a.verified);
    let uni_verified = summary
        .attributes
        .iter()
        .any(|a| a.key == "university" && a.verified);

    let kyc_has_any = kyc_age_verified && kyc_gender_verified;
    let user_ctx = use_user_context();
    let mut toast = use_toast();
    let mut popup = use_popup();
    let tr_kyc = tr.clone();

    let on_kyc = move |_| async move {
        let _ = &user_ctx;
        let _ = &toast;
        #[cfg(not(feature = "server"))]
        {
            let conf = super::config::get();
            let Some(prefix) = user_ctx().user_id() else {
                toast.warn(tr_kyc.verification_error.to_string());
                return;
            };
            match super::interop::verify_identity(
                conf.portone.store_id,
                conf.portone.inicis_channel_key,
                &prefix,
            )
            .await
            {
                Ok(_) => on_refresh.call(()),
                Err(err) => {
                    toast.error(err);
                }
            }
        }
    };

    let tr_code = tr.clone();
    let on_code = move |_| {
        let tr = tr_code.clone();
        popup.open(rsx! {
            CodeModal { tr, on_success: on_refresh }
        });
    };

    rsx! {
        div { class: "methods",
            div { class: "method-card method-card--kyc",
                div { class: "method-card__head",
                    div { class: "method-card__badge method-card__badge--kyc",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" }
                            polyline { points: "9 12 11 14 15 10" }
                        }
                    }
                    div { class: "method-card__head-body",
                        span { class: "method-card__title", "{tr.kyc_title}" }
                        span { class: "method-card__sub method-card__sub--kyc",
                            "{tr.kyc_sub_prefix} "
                            strong { "{tr.kyc_sub_highlight}" }
                        }
                    }
                }
                div { class: "method-card__desc", "{tr.kyc_desc}" }
                div { class: "method-card__attrs",
                    MethodChip { label: tr.attr_age, verified: kyc_age_verified }
                    MethodChip { label: tr.attr_gender, verified: kyc_gender_verified }
                }
                button {
                    class: if kyc_has_any { "method-card__cta method-card__cta--ghost" } else { "method-card__cta method-card__cta--kyc" },
                    onclick: on_kyc,
                    if kyc_has_any {
                        "{tr.kyc_rerun}"
                    } else {
                        "{tr.kyc_run}"
                    }
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2.5",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M1 4v6h6" }
                        path { d: "M3.51 15a9 9 0 1 0 2.13-9.36L1 10" }
                    }
                }
            }

            div { class: "method-card method-card--code",
                div { class: "method-card__head",
                    div { class: "method-card__badge method-card__badge--code",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            rect {
                                x: "3",
                                y: "3",
                                width: "18",
                                height: "18",
                                rx: "2",
                                ry: "2",
                            }
                            line {
                                x1: "9",
                                y1: "3",
                                x2: "9",
                                y2: "21",
                            }
                            line {
                                x1: "15",
                                y1: "3",
                                x2: "15",
                                y2: "21",
                            }
                            line {
                                x1: "3",
                                y1: "9",
                                x2: "21",
                                y2: "9",
                            }
                            line {
                                x1: "3",
                                y1: "15",
                                x2: "21",
                                y2: "15",
                            }
                        }
                    }
                    div { class: "method-card__head-body",
                        span { class: "method-card__title", "{tr.code_title}" }
                        span { class: "method-card__sub method-card__sub--code",
                            "{tr.code_sub_prefix} "
                            strong { "{tr.code_sub_highlight}" }
                        }
                    }
                }
                div { class: "method-card__desc", "{tr.code_desc}" }
                div { class: "method-card__attrs",
                    MethodChip { label: tr.attr_university, verified: uni_verified }
                    MethodChip { label: tr.attr_employer, verified: false }
                    MethodChip { label: tr.attr_membership, verified: false }
                }
                div { class: "code-hint",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M21 2l-2 2m-7.61 7.61a5.5 5.5 0 1 1-7.778 7.778 5.5 5.5 0 0 1 7.777-7.777zm0 0L15.5 7.5m0 0l3 3L22 7l-3-3m-3.5 3.5L19 4" }
                    }
                    span { "{tr.code_hint}" }
                }
                button {
                    class: "method-card__cta method-card__cta--code",
                    onclick: on_code,
                    "{tr.code_cta}"
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2.5",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "9 18 15 12 9 6" }
                    }
                }
            }
        }
    }
}

#[component]
fn MethodChip(label: String, verified: bool) -> Element {
    let cls = if verified { "method-chip" } else { "method-chip method-chip--missing" };
    rsx! {
        span { class: "{cls}",
            if verified {
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "3",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    polyline { points: "20 6 9 17 4 12" }
                }
            }
            "{label}"
        }
    }
}

#[component]
fn AttrsGrid(
    tr: CredentialsTranslate,
    summary: DidSummaryResponse,
    on_refresh: Callback<()>,
) -> Element {
    rsx! {
        div { class: "attrs",
            for item in summary.attributes.iter().cloned() {
                AttrCard {
                    key: "{item.key}",
                    tr: tr.clone(),
                    item,
                    on_refresh,
                }
            }
        }
    }
}

#[component]
fn AttrCard(
    tr: CredentialsTranslate,
    item: DidAttributeItem,
    on_refresh: Callback<()>,
) -> Element {
    let label = attr_label_for(&item.key, &tr);
    let verified = item.verified;
    let card_class = if verified {
        "attr-card attr-card--verified"
    } else {
        "attr-card attr-card--unverified"
    };
    let method_class = if item.method == "kyc" {
        "attr-card__method attr-card__method--kyc"
    } else {
        "attr-card__method attr-card__method--code"
    };
    let method_label = if item.method == "kyc" {
        tr.method_kyc.to_string()
    } else {
        tr.method_code.to_string()
    };

    let display_value = if verified {
        let raw = item.value.clone().unwrap_or_default();
        if item.key == "gender" {
            gender_label(&raw, &tr)
        } else {
            raw
        }
    } else {
        String::new()
    };

    let unverified_label = match item.key.as_str() {
        "membership" => tr.no_codes_redeemed.to_string(),
        _ => tr.not_verified.to_string(),
    };
    let add_label = match item.key.as_str() {
        "employer" => tr.add_code_employer.to_string(),
        "membership" => tr.add_code_membership.to_string(),
        _ => tr.add_code_generic.to_string(),
    };

    let tr_for_modal = tr.clone();
    let mut popup = use_popup();
    let on_add = move |_| {
        let tr = tr_for_modal.clone();
        popup.open(rsx! {
            CodeModal { tr, on_success: on_refresh }
        });
    };

    let user_ctx = use_user_context();
    let mut toast = use_toast();
    let tr_for_kyc = tr.clone();
    let on_kyc_redo = move |_| async move {
        let _ = &user_ctx;
        let _ = &toast;
        #[cfg(not(feature = "server"))]
        {
            let conf = super::config::get();
            let Some(prefix) = user_ctx().user_id() else {
                toast.warn(tr_for_kyc.verification_error.to_string());
                return;
            };
            match super::interop::verify_identity(
                conf.portone.store_id,
                conf.portone.inicis_channel_key,
                &prefix,
            )
            .await
            {
                Ok(_) => on_refresh.call(()),
                Err(err) => {
                    toast.error(err);
                }
            }
        }
    };

    rsx! {
        div { class: "{card_class}",
            div { class: "attr-card__head",
                div { class: "attr-card__title",
                    div { class: "attr-card__icon",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            {attr_icon_for(&item.key)}
                        }
                    }
                    span { class: "attr-card__label", "{label}" }
                }
                span { class: "{method_class}", "{method_label}" }
            }
            if verified {
                div { class: "attr-card__value",
                    if item.key == "age" {
                        span { class: "attr-card__value-main", "{display_value}" }
                        span { class: "attr-card__value-sub", "{tr.attr_age_sub}" }
                    } else if item.key == "university" || (item.key != "gender") {
                        span { class: "attr-card__value-main attr-card__value-main--text",
                            "{display_value}"
                        }
                    } else {
                        span { class: "attr-card__value-main", "{display_value}" }
                    }
                }
                div { class: "attr-card__meta",
                    if item.method == "kyc" {
                        span { class: "attr-card__meta-item",
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                path { d: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" }
                            }
                            strong { "{tr.meta_portone}" }
                            " · KCB"
                        }
                        span {
                            class: "attr-card__meta-item",
                            onclick: on_kyc_redo,
                            style: "cursor:pointer",
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M1 4v6h6" }
                                path { d: "M3.51 15a9 9 0 1 0 2.13-9.36L1 10" }
                            }
                            "{tr.kyc_rerun}"
                        }
                    } else {
                        span { class: "attr-card__meta-item",
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                rect {
                                    x: "3",
                                    y: "3",
                                    width: "18",
                                    height: "18",
                                    rx: "2",
                                    ry: "2",
                                }
                                line {
                                    x1: "9",
                                    y1: "3",
                                    x2: "9",
                                    y2: "21",
                                }
                                line {
                                    x1: "15",
                                    y1: "3",
                                    x2: "15",
                                    y2: "21",
                                }
                            }
                            "{tr.meta_code_label}"
                        }
                    }
                }
            } else {
                div { class: "attr-card__value",
                    span { class: "attr-card__value-missing", "{unverified_label}" }
                }
                button { class: "attr-card__add", onclick: on_add,
                    "{add_label}"
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2.5",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "9 18 15 12 9 6" }
                    }
                }
            }
        }
    }
}

#[component]
fn ProofCard(tr: CredentialsTranslate, summary: DidSummaryResponse) -> Element {
    let issued = if summary.issued_at > 0 {
        format_datetime_utc(summary.issued_at)
    } else {
        "—".to_string()
    };
    let subject = if summary.did.is_empty() { "—".to_string() } else { summary.did.clone() };
    let issuer_display = "did:web:ratel.foundation:issuer#key-1";

    rsx! {
        div { class: "proof",
            div { class: "proof__col",
                div { class: "proof__col-title",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M21 2l-2 2m-7.61 7.61a5.5 5.5 0 1 1-7.778 7.778 5.5 5.5 0 0 1 7.777-7.777zm0 0L15.5 7.5m0 0l3 3L22 7l-3-3m-3.5 3.5L19 4" }
                    }
                    "{tr.proof_meta}"
                }
                div { class: "proof__row",
                    span { "{tr.proof_format}" }
                    span { "{tr.proof_format_value}" }
                }
                div { class: "proof__row",
                    span { "{tr.proof_suite}" }
                    span { "{tr.proof_suite_value}" }
                }
                div { class: "proof__row",
                    span { "{tr.proof_subject}" }
                    span { "{subject}" }
                }
                div { class: "proof__row",
                    span { "{tr.proof_issued}" }
                    span { "{issued}" }
                }
            }
            div { class: "proof__col",
                div { class: "proof__col-title",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        rect {
                            x: "3",
                            y: "11",
                            width: "18",
                            height: "11",
                            rx: "2",
                            ry: "2",
                        }
                        path { d: "M7 11V7a5 5 0 0 1 10 0v4" }
                    }
                    "{tr.proof_sig_title}"
                }
                div { class: "proof__row proof__row--ok",
                    span { "{tr.proof_issuer_key}" }
                    span { "{tr.proof_issuer_key_value}" }
                }
                div { class: "proof__row proof__row--ok",
                    span { "{tr.proof_integrity}" }
                    span { "{tr.proof_integrity_value}" }
                }
                div { class: "proof__row proof__row--ok",
                    span { "{tr.proof_revocation}" }
                    span { "{tr.proof_revocation_value}" }
                }
                div { class: "proof__sig",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "20 6 9 17 4 12" }
                    }
                    span {
                        "{tr.proof_signed_by} "
                        strong { "{issuer_display}" }
                        " {tr.proof_signed_by_suffix}"
                    }
                }
            }
        }
    }
}

#[component]
fn PrivacyNotice(tr: CredentialsTranslate, did: String) -> Element {
    let did_display = if did.is_empty() { "did:web:ratel.foundation:{user}".to_string() } else { did };
    rsx! {
        div { class: "privacy",
            div { class: "privacy__icon",
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    rect {
                        x: "3",
                        y: "11",
                        width: "18",
                        height: "11",
                        rx: "2",
                        ry: "2",
                    }
                    path { d: "M7 11V7a5 5 0 0 1 10 0v4" }
                }
            }
            div { class: "privacy__body",
                span { class: "privacy__title", "{tr.privacy_title}" }
                span { class: "privacy__desc",
                    "{tr.privacy_desc_prefix} "
                    code { "{did_display}" }
                    " {tr.privacy_desc_suffix}"
                }
            }
        }
    }
}


#[component]
fn CodeModal(tr: CredentialsTranslate, on_success: Callback<()>) -> Element {
    let mut popup = use_popup();
    let mut toast = use_toast();
    let mut code_value = use_signal(String::new);
    let mut submitting = use_signal(|| false);

    let submit = move |_| {
        if submitting() {
            return;
        }
        let code = code_value().trim().to_string();
        if code.is_empty() {
            toast.error(crate::common::Error::InvalidCodeInput);
            return;
        }
        submitting.set(true);
        spawn(async move {
            match sign_attributes_handler(SignAttributesRequest::Code { code }).await {
                Ok(_) => {
                    popup.close();
                    on_success.call(());
                }
                Err(e) => {
                    toast.error(e);
                }
            }
            submitting.set(false);
        });
    };

    rsx! {
        div { class: "p-6 w-full max-w-md",
            h2 { class: "mb-4 text-xl font-bold text-modal-label-text", "{tr.modal_code_title}" }
            div { class: "mb-4",
                input {
                    r#type: "text",
                    value: "{code_value}",
                    oninput: move |e| code_value.set(e.value()),
                    placeholder: tr.modal_code_placeholder,
                    class: "py-2 px-3 w-full rounded border border-gray-300 dark:bg-gray-700 dark:border-gray-600 text-neutral-500",
                }
            }
            div { class: "flex gap-2 justify-end",
                button {
                    class: "hover:text-white text-neutral-500",
                    onclick: move |_| popup.close(),
                    "{tr.modal_cancel}"
                }
                button {
                    class: "py-2 px-4 rounded-md bg-enable-button-bg text-enable-button-white-text",
                    disabled: submitting(),
                    onclick: submit,
                    "{tr.modal_submit}"
                }
            }
        }
    }
}

async fn export_vc(summary: &DidSummaryResponse) {
    #[cfg(not(feature = "server"))]
    {
        let json = serde_json::to_string_pretty(&build_vc_document(summary)).unwrap_or_default();
        export_vc_download(&json);
    }
    #[cfg(feature = "server")]
    {
        let _ = summary;
    }
}

#[cfg(not(feature = "server"))]
#[crate::common::wasm_bindgen::prelude::wasm_bindgen(js_namespace = ["window", "ratel", "credentials"], js_name = "export_vc")]
unsafe extern "C" {
    fn export_vc_download(json: &str);
}

#[cfg(not(feature = "server"))]
fn build_vc_document(summary: &DidSummaryResponse) -> serde_json::Value {
    use serde_json::json;
    let mut subject = serde_json::Map::new();
    subject.insert("id".to_string(), json!(summary.did.clone()));
    for attr in &summary.attributes {
        if attr.verified {
            if let Some(v) = &attr.value {
                subject.insert(attr.key.clone(), json!(v));
            }
        }
    }
    json!({
        "@context": ["https://www.w3.org/ns/credentials/v2"],
        "type": ["VerifiableCredential", "RatelPersonalIdentity"],
        "issuer": "did:web:ratel.foundation:issuer",
        "credentialSubject": serde_json::Value::Object(subject),
    })
}

fn attr_icon_for(key: &str) -> Element {
    match key {
        "age" => rsx! {
            rect {
                x: "3",
                y: "4",
                width: "18",
                height: "18",
                rx: "2",
                ry: "2",
            }
            line {
                x1: "16",
                y1: "2",
                x2: "16",
                y2: "6",
            }
            line {
                x1: "8",
                y1: "2",
                x2: "8",
                y2: "6",
            }
            line {
                x1: "3",
                y1: "10",
                x2: "21",
                y2: "10",
            }
        },
        "gender" => rsx! {
            path { d: "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" }
            circle { cx: "12", cy: "7", r: "4" }
        },
        "university" => rsx! {
            path { d: "M22 10v6M2 10l10-5 10 5-10 5z" }
            path { d: "M6 12v5c3 3 9 3 12 0v-5" }
        },
        "employer" => rsx! {
            rect {
                x: "2",
                y: "7",
                width: "20",
                height: "14",
                rx: "2",
                ry: "2",
            }
            path { d: "M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16" }
        },
        _ => rsx! {
            path { d: "M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" }
            circle { cx: "9", cy: "7", r: "4" }
            path { d: "M23 21v-2a4 4 0 0 0-3-3.87" }
            path { d: "M16 3.13a4 4 0 0 1 0 7.75" }
        },
    }
}

fn render_qr_placeholder() -> String {
    // Decorative-only static QR silhouette; replace with a real QR render
    // once the VC resolver endpoint is live.
    "<svg width=\"100%\" height=\"100%\" viewBox=\"0 0 29 29\" xmlns=\"http://www.w3.org/2000/svg\"><rect width=\"29\" height=\"29\" fill=\"#ffffff\"/><g fill=\"#0a0a0a\"><rect x=\"0\" y=\"0\" width=\"7\" height=\"7\"/><rect x=\"1\" y=\"1\" width=\"5\" height=\"5\" fill=\"#ffffff\"/><rect x=\"2\" y=\"2\" width=\"3\" height=\"3\"/><rect x=\"22\" y=\"0\" width=\"7\" height=\"7\"/><rect x=\"23\" y=\"1\" width=\"5\" height=\"5\" fill=\"#ffffff\"/><rect x=\"24\" y=\"2\" width=\"3\" height=\"3\"/><rect x=\"0\" y=\"22\" width=\"7\" height=\"7\"/><rect x=\"1\" y=\"23\" width=\"5\" height=\"5\" fill=\"#ffffff\"/><rect x=\"2\" y=\"24\" width=\"3\" height=\"3\"/><rect x=\"9\" y=\"3\" width=\"2\" height=\"1\"/><rect x=\"12\" y=\"3\" width=\"1\" height=\"1\"/><rect x=\"14\" y=\"3\" width=\"3\" height=\"1\"/><rect x=\"19\" y=\"3\" width=\"2\" height=\"1\"/><rect x=\"8\" y=\"8\" width=\"1\" height=\"1\"/><rect x=\"10\" y=\"8\" width=\"2\" height=\"1\"/><rect x=\"13\" y=\"8\" width=\"1\" height=\"1\"/><rect x=\"15\" y=\"8\" width=\"1\" height=\"1\"/><rect x=\"17\" y=\"8\" width=\"3\" height=\"1\"/><rect x=\"21\" y=\"8\" width=\"1\" height=\"1\"/><rect x=\"9\" y=\"10\" width=\"2\" height=\"1\"/><rect x=\"12\" y=\"10\" width=\"1\" height=\"1\"/><rect x=\"14\" y=\"10\" width=\"2\" height=\"1\"/><rect x=\"17\" y=\"10\" width=\"1\" height=\"1\"/><rect x=\"19\" y=\"10\" width=\"2\" height=\"1\"/><rect x=\"22\" y=\"10\" width=\"2\" height=\"1\"/><rect x=\"8\" y=\"12\" width=\"1\" height=\"1\"/><rect x=\"10\" y=\"12\" width=\"1\" height=\"1\"/><rect x=\"12\" y=\"12\" width=\"3\" height=\"1\"/><rect x=\"16\" y=\"12\" width=\"2\" height=\"1\"/><rect x=\"19\" y=\"12\" width=\"1\" height=\"1\"/><rect x=\"21\" y=\"12\" width=\"3\" height=\"1\"/><rect x=\"9\" y=\"14\" width=\"3\" height=\"1\"/><rect x=\"13\" y=\"14\" width=\"1\" height=\"1\"/><rect x=\"15\" y=\"14\" width=\"3\" height=\"1\"/><rect x=\"19\" y=\"14\" width=\"2\" height=\"1\"/><rect x=\"22\" y=\"14\" width=\"1\" height=\"1\"/><rect x=\"8\" y=\"16\" width=\"2\" height=\"1\"/><rect x=\"11\" y=\"16\" width=\"1\" height=\"1\"/><rect x=\"13\" y=\"16\" width=\"2\" height=\"1\"/><rect x=\"16\" y=\"16\" width=\"1\" height=\"1\"/><rect x=\"18\" y=\"16\" width=\"2\" height=\"1\"/><rect x=\"21\" y=\"16\" width=\"2\" height=\"1\"/></g></svg>".to_string()
}
