//! `/launchpad/return` — OAuth-style landing after a Launchpad conversion.
//!
//! Launchpad redirects the user here (signed) once the conversion completes.
//! We verify the signature server-side via `launchpad_verify_return_handler`
//! and, on success, show the receipt plus a link to the community page that
//! Launchpad handed back. An invalid/expired signature renders an error
//! instead of a trusted link.

use crate::common::*;
use crate::features::launchpad_partner::LaunchpadPartnerTranslate;
use crate::features::launchpad_partner::handback::{
    LaunchpadReturnRequest, launchpad_verify_return_handler,
};
use crate::route::Route;

fn commas(value: i64) -> String {
    let sign = if value < 0 { "-" } else { "" };
    let digits = value.abs().to_string();
    let mut out = String::new();
    for (idx, ch) in digits.chars().rev().enumerate() {
        if idx > 0 && idx % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    format!("{sign}{}", out.chars().rev().collect::<String>())
}

fn short_id(value: &str) -> String {
    let v = value.trim();
    if v.len() <= 14 {
        return v.to_string();
    }
    format!("{}…{}", &v[..8], &v[v.len() - 6..])
}

#[component]
pub fn LaunchpadReturnPage(
    project_id: Option<String>,
    conversion_id: Option<String>,
    brand_tx_id: Option<String>,
    deducted_points: Option<String>,
    remaining_points: Option<String>,
    round_id: Option<String>,
    community_url: Option<String>,
    ts: Option<String>,
    sig: Option<String>,
) -> Element {
    let tr: LaunchpadPartnerTranslate = use_translate();
    let nav = use_navigator();
    // The user is signed into ratel (they originated here), so route "back to
    // rewards" to their own rewards page; fall back to home if unresolved.
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let username = use_memo(move || {
        user_ctx()
            .user
            .as_ref()
            .map(|u| u.username.clone())
            .unwrap_or_default()
    });

    let req = LaunchpadReturnRequest {
        project_id: project_id.unwrap_or_default(),
        conversion_id: conversion_id.unwrap_or_default(),
        brand_tx_id: brand_tx_id.unwrap_or_default(),
        deducted_points: deducted_points.unwrap_or_default(),
        remaining_points: remaining_points.unwrap_or_default(),
        round_id: round_id.unwrap_or_default(),
        community_url: community_url.unwrap_or_default(),
        ts: ts.unwrap_or_default(),
        sig: sig.unwrap_or_default(),
    };

    let res = use_loader({
        let req = req.clone();
        move || {
            let req = req.clone();
            async move { launchpad_verify_return_handler(req).await }
        }
    })?;
    let view = res();

    rsx! {
        document::Title { "{tr.ret_title}" }
        div {
            class: "min-h-screen flex items-center justify-center px-4 py-16",
            div {
                class: "w-full max-w-[460px] flex flex-col gap-6",
                style: "background: var(--bg-glass); backdrop-filter: blur(16px); -webkit-backdrop-filter: blur(16px); border:1px solid var(--border-subtle); border-radius:24px; padding:32px 28px;",

                if view.verified {
                    div { class: "flex flex-col gap-2",
                        span {
                            class: "text-xs font-bold tracking-widest uppercase",
                            style: "color:#fcb300;",
                            "Launchpad"
                        }
                        h1 { class: "text-2xl font-extrabold text-foreground", "{tr.ret_title}" }
                        p { class: "text-sm text-foreground-muted", "{tr.ret_subtitle}" }
                    }

                    div { class: "flex flex-col gap-3",
                        ReceiptRow { label: tr.ret_deducted.to_string(), value: commas(view.deducted_points) }
                        if let Some(remaining) = view.remaining_points {
                            ReceiptRow { label: tr.ret_remaining.to_string(), value: commas(remaining) }
                        }
                        ReceiptRow { label: tr.ret_brand_tx.to_string(), value: short_id(&view.brand_tx_id) }
                        ReceiptRow { label: tr.ret_conversion.to_string(), value: short_id(&view.conversion_id) }
                    }

                    div { class: "flex flex-col gap-3",
                        if !view.community_url.trim().is_empty() {
                            a {
                                class: "inline-flex items-center justify-center gap-2 px-5 py-3 rounded-full text-sm font-extrabold cursor-pointer transition-transform hover:-translate-y-px",
                                style: "background: linear-gradient(135deg,#ffd24a 0%,#fcb300 100%); color:#0a0a0a; box-shadow:0 10px 24px -10px rgba(252,179,0,0.5);",
                                href: "{view.community_url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "{tr.ret_open_community}"
                            }
                        }
                        button {
                            r#type: "button",
                            class: "inline-flex items-center justify-center px-5 py-2.5 rounded-full text-sm font-semibold cursor-pointer text-foreground-muted border border-[var(--border-subtle)] hover:text-foreground",
                            onclick: {
                                let username = username();
                                move |_| {
                                    // replace (not push) so the browser Back button
                                    // doesn't bring this return card back into view.
                                    if username.trim().is_empty() {
                                        nav.replace(Route::Index {});
                                    } else {
                                        nav.replace(Route::SocialReward { username: username.clone() });
                                    }
                                }
                            },
                            "{tr.ret_home}"
                        }
                    }
                } else {
                    div { class: "flex flex-col gap-2",
                        h1 { class: "text-xl font-extrabold text-foreground", "{tr.ret_invalid_title}" }
                        p { class: "text-sm text-foreground-muted", "{tr.ret_invalid_body}" }
                    }
                    button {
                        r#type: "button",
                        class: "inline-flex items-center justify-center px-5 py-2.5 rounded-full text-sm font-semibold cursor-pointer text-foreground-muted border border-[var(--border-subtle)] hover:text-foreground",
                        onclick: {
                            let username = username();
                            move |_| {
                                if username.trim().is_empty() {
                                    nav.push(Route::Index {});
                                } else {
                                    nav.push(Route::SocialReward { username: username.clone() });
                                }
                            }
                        },
                        "{tr.ret_home}"
                    }
                }
            }
        }
    }
}

#[component]
fn ReceiptRow(label: String, value: String) -> Element {
    rsx! {
        div {
            class: "flex justify-between items-center py-2 border-b border-[var(--border-subtle)]",
            span { class: "text-sm text-foreground-muted", "{label}" }
            strong { class: "text-sm font-bold text-foreground", "{value}" }
        }
    }
}
