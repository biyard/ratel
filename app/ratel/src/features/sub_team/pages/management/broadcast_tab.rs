//! "Broadcast / 전체 공지 관리" tab — consumes `UseSubTeamBroadcast`.
//!
//! Mirrors `assets/design/sub-team/subteam-management-page.html`
//! (the `data-tab="broadcast"` panel): a compose CTA, a Drafts card
//! with rich draft rows, and a Published card with timestamped
//! announcement items.

use crate::features::sub_team::models::SubTeamAnnouncementStatus;
use crate::features::sub_team::{
    use_sub_team_broadcast, SubTeamAnnouncementResponse, SubTeamTranslate, UseSubTeamBroadcast,
};
use crate::route::Route;
use crate::*;

#[component]
pub fn BroadcastTab(username: String, team_display_name: String) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let UseSubTeamBroadcast {
        mut announcements,
        mut handle_delete,
        mut handle_publish,
        ..
    } = use_sub_team_broadcast()?;

    let items = announcements.items();
    let drafts: Vec<SubTeamAnnouncementResponse> = items
        .iter()
        .filter(|a| a.status == SubTeamAnnouncementStatus::Draft)
        .cloned()
        .collect();
    let published: Vec<SubTeamAnnouncementResponse> = items
        .iter()
        .filter(|a| a.status == SubTeamAnnouncementStatus::Published)
        .cloned()
        .collect();

    let nav = use_navigator();
    let username_for_cta = username.clone();
    let drafts_count = drafts.len();
    let published_count = published.len();
    let drafts_meta_label = tr
        .broadcast_drafts_count
        .replace("{n}", &drafts_count.to_string());
    let published_meta_label = tr
        .broadcast_published_count
        .replace("{n}", &published_count.to_string());

    rsx! {
        // ── Compose CTA ─────────────────────────────────────────────
        a {
            class: "bc-cta",
            "data-testid": "sub-team-broadcast-compose-cta",
            onclick: move |_| {
                nav.push(Route::TeamSubTeamBroadcastComposePage {
                    username: username_for_cta.clone(),
                });
            },
            div { class: "bc-cta__icon",
                lucide_dioxus::Send { class: "w-5 h-5 [&>path]:stroke-current" }
            }
            div { class: "bc-cta__body",
                div { class: "bc-cta__label", "{tr.broadcast_compose}" }
                div { class: "bc-cta__sub", "{tr.broadcast_compose_sub}" }
            }
            lucide_dioxus::ChevronRight { class: "bc-cta__arrow [&>path]:stroke-current" }
        }

        // ── Drafts card ─────────────────────────────────────────────
        section { class: "card",
            div { class: "card__head",
                h2 { class: "card__title", "{tr.broadcast_drafts_card_title}" }
                span { class: "card__dash" }
                span { class: "card__meta", "{drafts_meta_label}" }
            }
            if drafts.is_empty() {
                div { class: "inline-note", "{tr.empty_list}" }
            } else {
                div { class: "u-col u-gap-10", id: "drafts-list",
                    for draft in drafts.iter() {
                        DraftRow {
                            key: "{draft.id}",
                            item: draft.clone(),
                            username: username.clone(),
                            team_display_name: team_display_name.clone(),
                            on_delete: move |id| handle_delete.call(id),
                            on_publish: move |id| handle_publish.call(id),
                        }
                    }
                }
            }
        }

        // ── Published card ──────────────────────────────────────────
        section { class: "card",
            div { class: "card__head",
                h2 { class: "card__title", "{tr.broadcast_published_card_title}" }
                span { class: "card__dash" }
                span { class: "card__meta", "{published_meta_label}" }
            }
            if published.is_empty() {
                div { class: "inline-note", "{tr.empty_list}" }
            } else {
                div { class: "u-col u-gap-10",
                    for (i, item) in published.iter().enumerate() {
                        PublishedItem {
                            key: "{item.id}",
                            item: item.clone(),
                            is_pinned: i == 0,
                        }
                    }
                }
            }
            {announcements.more_element()}
        }
    }
}

#[component]
fn DraftRow(
    item: SubTeamAnnouncementResponse,
    username: String,
    team_display_name: String,
    on_delete: EventHandler<String>,
    on_publish: EventHandler<String>,
) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let nav = use_navigator();
    let announcement_id = item.id.clone();
    let edit_username = username.clone();
    let edit_id = announcement_id.clone();
    let delete_id = announcement_id.clone();
    let publish_id = announcement_id.clone();

    let saved_at_label = format_autosaved(item.updated_at, &tr);
    let chars = if !item.html_contents.is_empty() {
        strip_html_chars(&item.html_contents)
    } else {
        item.body.chars().count()
    };
    let chars_label = tr.broadcast_draft_chars.replace("{n}", &chars.to_string());
    let title_display = if item.title.trim().is_empty() {
        tr.broadcast_untitled.to_string()
    } else {
        item.title.clone()
    };

    rsx! {
        div { class: "draft-row",
            div { class: "draft-row__dot" }
            div { class: "draft-row__body",
                div { class: "draft-row__title", "{title_display}" }
                div { class: "draft-row__meta",
                    // "clock · saved-at · N chars" — inline icon + text
                    span { class: "draft-row__meta-cell",
                        lucide_dioxus::Clock {
                            class: "w-3 h-3 [&>circle]:stroke-current [&>polyline]:stroke-current",
                        }
                        span { "{saved_at_label} · {chars_label}" }
                    }
                    span {
                        "{tr.broadcast_draft_posting_as} "
                        strong { "{team_display_name}" }
                    }
                    span { "{tr.broadcast_draft_target}" }
                }
            }
            a {
                class: "draft-row__edit",
                "data-testid": "sub-team-broadcast-draft-edit",
                onclick: move |_| {
                    nav.push(Route::TeamSubTeamBroadcastEditPage {
                        username: edit_username.clone(),
                        announcement_id: edit_id.clone(),
                    });
                },
                lucide_dioxus::Pencil { class: "w-3 h-3 [&>path]:stroke-current" }
                "{tr.edit}"
            }
            button {
                class: "draft-row__edit",
                "data-testid": "sub-team-broadcast-draft-publish",
                onclick: move |_| on_publish.call(publish_id.clone()),
                lucide_dioxus::Send { class: "w-3 h-3 [&>path]:stroke-current" }
                "{tr.broadcast_publish}"
            }
            button {
                class: "draft-row__del",
                "aria-label": "Delete",
                onclick: move |_| on_delete.call(delete_id.clone()),
                lucide_dioxus::Trash2 { class: "w-3 h-3 [&>path]:stroke-current" }
            }
        }
    }
}

#[component]
fn PublishedItem(item: SubTeamAnnouncementResponse, is_pinned: bool) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let ts = item.published_at.unwrap_or(item.updated_at);
    let relative = format_relative_short(ts, &tr);
    let absolute = format_absolute_date(ts);
    let fanout_label = tr
        .broadcast_fanout_meta
        .replace("{n}", &item.fan_out_count.to_string());
    let comments_label = tr
        .broadcast_comments_meta
        .replace("{n}", &item.comments_count.to_string());
    let kind_label = if is_pinned {
        tr.broadcast_pinned.to_string()
    } else {
        tr.broadcast_unpinned.to_string()
    };

    rsx! {
        div { class: "bc-item",
            div { class: "bc-item__top",
                span {
                    class: "bc-item__kind",
                    style: if is_pinned {
                        ""
                    } else {
                        "background:rgba(255,255,255,0.04);color:var(--sub-team-muted);border-color:var(--sub-team-border)"
                    },
                    if is_pinned {
                        lucide_dioxus::Pin { class: "w-3 h-3 [&>path]:stroke-current [&>line]:stroke-current" }
                    } else {
                        lucide_dioxus::Hash { class: "w-3 h-3 [&>line]:stroke-current" }
                    }
                    "{kind_label}"
                }
                span { class: "bc-item__time", "{relative} · {absolute}" }
            }
            div { class: "bc-item__title-text", "{item.title}" }
            div { class: "bc-item__meta",
                span { class: "bc-item__meta-cell",
                    lucide_dioxus::Users { class: "w-3 h-3 [&>path]:stroke-current [&>circle]:stroke-current" }
                    "{fanout_label}"
                }
                span { class: "bc-item__meta-cell",
                    lucide_dioxus::MessageSquare { class: "w-3 h-3 [&>path]:stroke-current" }
                    "{comments_label}"
                }
            }
        }
    }
}

/// Granular "saved N ago" formatter for the draft row meta. Returns
/// localised strings like "3분 전 자동 저장" / "3 min ago auto-saved".
fn format_autosaved(ts_ms: i64, tr: &SubTeamTranslate) -> String {
    if ts_ms <= 0 {
        return tr.broadcast_autosaved_idle.to_string();
    }
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let diff_ms = (now - ts_ms).max(0);
    let diff_secs = diff_ms / 1000;
    let diff_mins = diff_secs / 60;
    let diff_hours = diff_mins / 60;
    let diff_days = diff_hours / 24;
    if diff_secs < 60 {
        tr.broadcast_autosaved_just_now.to_string()
    } else if diff_mins < 60 {
        tr.broadcast_autosaved_minutes
            .replace("{n}", &diff_mins.to_string())
    } else if diff_hours < 24 {
        tr.broadcast_autosaved_hours
            .replace("{n}", &diff_hours.to_string())
    } else if diff_days == 1 {
        tr.broadcast_autosaved_yesterday.to_string()
    } else if diff_days < 7 {
        tr.broadcast_autosaved_days
            .replace("{n}", &diff_days.to_string())
    } else {
        format_absolute_date(ts_ms)
    }
}

/// Short relative timestamp ("Today" / "Yesterday" / "N days ago").
fn format_relative_short(ts_ms: i64, tr: &SubTeamTranslate) -> String {
    if ts_ms <= 0 {
        return "—".to_string();
    }
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let diff_days = ((now - ts_ms) / 86_400_000).max(0);
    match diff_days {
        0 => tr.broadcast_time_today.to_string(),
        1 => tr.broadcast_time_yesterday.to_string(),
        2..=6 => tr.broadcast_time_days_ago.replace("{n}", &diff_days.to_string()),
        _ => format_absolute_date(ts_ms),
    }
}

fn format_absolute_date(ts_ms: i64) -> String {
    if ts_ms <= 0 {
        return String::new();
    }
    use chrono::TimeZone;
    chrono::Utc
        .timestamp_millis_opt(ts_ms)
        .single()
        .map(|t| t.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}

fn strip_html_chars(html: &str) -> usize {
    let mut count = 0usize;
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if in_tag => {}
            _ => count += 1,
        }
    }
    count
}
