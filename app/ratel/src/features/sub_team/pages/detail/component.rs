//! Sub-team detail (activity dashboard) page. Consumes
//! `UseSubTeamActivity` which bundles overview/counts/per-member rows.

use crate::features::social::controllers::find_team::find_team_handler;
use crate::features::sub_team::{use_sub_team_activity, SubTeamTranslate, UseSubTeamActivity};
use crate::route::Route;
use crate::*;

/// Turns an epoch-ms timestamp into a short Korean-friendly relative
/// label with a `YYYY-MM-DD` fallback when the gap is older than a
/// week. Keeps the table compact instead of showing raw 13-digit
/// timestamps. Locale-aware so the English UI doesn't show Korean
/// "오늘" / "N일 전".
fn format_last_active(ts_ms: i64, lang: Language) -> String {
    if ts_ms <= 0 {
        return "—".to_string();
    }
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let diff_days = ((now - ts_ms) / 86_400_000).max(0);
    let is_ko = matches!(lang, Language::Ko);
    match diff_days {
        0 => {
            if is_ko {
                "오늘".to_string()
            } else {
                "Today".to_string()
            }
        }
        1 => {
            if is_ko {
                "어제".to_string()
            } else {
                "Yesterday".to_string()
            }
        }
        2..=6 => {
            if is_ko {
                format!("{diff_days}일 전")
            } else {
                format!("{diff_days} days ago")
            }
        }
        _ => {
            use chrono::TimeZone;
            chrono::Utc
                .timestamp_millis_opt(ts_ms)
                .single()
                .map(|t| t.format("%Y-%m-%d").to_string())
                .unwrap_or_else(|| "—".to_string())
        }
    }
}

#[component]
pub fn TeamSubTeamDetailPage(username: String, sub_team_id: String) -> Element {
    let tr: SubTeamTranslate = use_translate();

    let username_for_load = username.clone();
    let team_resource = use_loader(move || {
        let name = username_for_load.clone();
        async move { find_team_handler(name).await }
    })?;

    let team_pk = team_resource().pk;
    let team_id: TeamPartition = team_pk
        .parse::<TeamPartition>()
        .unwrap_or(TeamPartition(String::new()));
    use_context_provider(|| team_id);
    // sub_team_id seeded via context for the hook.
    let sub_team_id_for_ctx = sub_team_id.clone();
    use_context_provider(move || sub_team_id_for_ctx.clone());

    rsx! {
        SeoMeta { title: "Sub-team · {tr.tab_sub_teams}" }

        DetailView { username: username.clone(), sub_team_id: sub_team_id.clone() }
    }
}

#[component]
fn DetailView(username: String, sub_team_id: String) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let lang_signal = use_language();
    let nav = use_navigator();

    let UseSubTeamActivity {
        detail,
        counts,
        mut members,
        ..
    } = use_sub_team_activity()?;

    // Overview (detail)
    let overview = detail();
    let _counts_data = counts();

    let display_name = overview.display_name.clone();
    let handle = overview.username.clone();
    let current_lang = lang_signal();
    let privacy_notice_text = if matches!(current_lang, Language::Ko) {
        overview.privacy_notice.ko.clone()
    } else {
        overview.privacy_notice.en.clone()
    };

    let post_count = overview.post_count;
    let space_count = overview.space_count;
    let active_members = overview.active_member_count;

    let deregister_username = username.clone();
    let deregister_sub_team = sub_team_id.clone();
    let initials: String = display_name
        .chars()
        .take(2)
        .collect::<String>()
        .to_uppercase();

    let member_rows = members.items();

    let mut member_search: Signal<String> = use_signal(String::new);
    rsx! {
        // Detail page lives OUTSIDE `SocialLayout → TeamArenaLayout` —
        // it owns its own back/home topbar so navigating in from the
        // parent's management list doesn't end up with two stacked
        // arena topbars (parent's + ours). Page-scoped scroll
        // container per `feedback_arena_page_scroll.md`.
        div { class: "sub-team-detail",
            div { class: "arena-topbar",
                div { class: "arena-topbar__brand",
                    div {
                        class: "brand-home",
                        "aria-label": "Back",
                        onclick: move |_| {
                            nav.go_back();
                        },
                        lucide_dioxus::ChevronLeft { class: "w-4 h-4 [&>path]:stroke-current" }
                    }
                    span { class: "brand-home__divider" }
                    div { class: "arena-topbar__logo arena-topbar__logo--child", "{initials}" }
                    div { class: "u-col",
                        span { class: "arena-topbar__title arena-topbar__title--child",
                            "{display_name}"
                        }
                        span { class: "arena-topbar__handle", "@{handle}" }
                    }
                    span { class: "arena-topbar__status arena-topbar__status--active",
                        "Active"
                    }
                }
            }

            div { class: "page page--wide",

                // Team hero — avatar + name/handle + window toggle.
                div { class: "team-hero",
                    div { class: "avatar avatar--lg avatar--teal", "{initials}" }
                    div { class: "team-hero__main",
                        div { class: "team-hero__title", "{display_name}" }
                        div { class: "team-hero__handle", "@{handle}" }
                    }
                }

                // Metrics — Posts / Spaces / Active members.
                div { class: "metric-grid",
                    div { class: "metric metric--posts",
                        div { class: "metric__label",
                            lucide_dioxus::FileText { class: "w-3 h-3 [&>path]:stroke-current" }
                            "{tr.post_count}"
                        }
                        div {
                            class: "metric__value",
                            id: "m-posts",
                            "data-testid": "sub-team-detail-post-count",
                            "{post_count}"
                        }
                    }
                    div { class: "metric metric--spaces",
                        div { class: "metric__label",
                            lucide_dioxus::Hash { class: "w-3 h-3 [&>path]:stroke-current" }
                            "{tr.space_count}"
                        }
                        div {
                            class: "metric__value",
                            id: "m-spaces",
                            "data-testid": "sub-team-detail-space-count",
                            "{space_count}"
                        }
                    }
                    div { class: "metric metric--members",
                        div { class: "metric__label",
                            lucide_dioxus::Users { class: "w-3 h-3 [&>path]:stroke-current" }
                            "{tr.active_member_count}"
                        }
                        div {
                            class: "metric__value",
                            id: "m-members",
                            "data-testid": "sub-team-detail-active-members",
                            "{active_members}"
                        }
                    }
                }

                // Privacy notice
                div {
                    class: "notice notice--teal",
                    "data-testid": "sub-team-detail-privacy-notice",
                    div { class: "notice__icon",
                        lucide_dioxus::Lock { class: "w-4 h-4 [&>path]:stroke-current" }
                    }
                    div { class: "notice__body",
                        span { class: "notice__title", "{tr.privacy_notice_short}" }
                        span { class: "notice__text", "{privacy_notice_text}" }
                    }
                }

                // Member activity table + search.
                section { class: "card",
                    div { class: "card__head",
                        h2 { class: "card__title", "{tr.per_member_activity}" }
                        span { class: "card__dash" }
                        span { class: "card__meta", "{member_rows.len()}" }
                    }
                    div { class: "member-head",
                        div { class: "member-search",
                            span { class: "member-search__icon",
                                lucide_dioxus::Search { class: "w-3 h-3 [&>path]:stroke-current" }
                            }
                            input {
                                class: "member-search__input",
                                placeholder: "{tr.member_search_placeholder}",
                                value: "{member_search()}",
                                oninput: move |e| member_search.set(e.value()),
                            }
                        }
                        span { class: "member-sort", "{tr.member_sort_active}" }
                    }
                    if member_rows.is_empty() {
                        div { class: "inline-note", "{tr.empty_list}" }
                    } else {
                        table { class: "member-table",
                            thead {
                                tr {
                                    th { "{tr.member_handle_header}" }
                                    th { "{tr.member_posts_header}" }
                                    th { "{tr.member_spaces_header}" }
                                    th { "{tr.member_last_active_header}" }
                                }
                            }
                            tbody { id: "member-rows",
                                for m in member_rows
                                    .iter()
                                    .filter(|m| {
                                        let q = member_search();
                                        q.is_empty() || m.handle.to_lowercase().contains(&q.to_lowercase())
                                            || m.display_name.to_lowercase().contains(&q.to_lowercase())
                                    })
                                {
                                    tr { key: "{m.user_id}",
                                        td {
                                            span { class: "member-handle",
                                                span { class: "avatar avatar--sm avatar--teal",
                                                    {m.display_name.chars().take(2).collect::<String>().to_uppercase()}
                                                }
                                                span { class: "member-handle__info",
                                                    span { class: "member-handle__name",
                                                        "@{m.handle}"
                                                    }
                                                    span { class: "member-handle__role",
                                                        "{m.display_name}"
                                                    }
                                                }
                                            }
                                        }
                                        td {
                                            span { class: "member-metric", "{m.post_count}" }
                                        }
                                        td {
                                            span { class: "member-metric",
                                                "{m.space_count_participated}"
                                            }
                                        }
                                        td {
                                            // Plain `String` expression — dioxus turns it into
                                            // a text node directly. The earlier `{ ... rsx! { ... } }`
                                            // nesting allocated a fresh element id for the inner
                                            // `rsx!` on every render, which the reconciler had
                                            // trouble reclaiming on this page's remount cycles
                                            // (hundreds of `cannot reclaim ElementId(N)` errors
                                            // followed by the CharacterData HierarchyRequestError
                                            // panic).
                                            {
                                                m.last_active_at
                                                    .map(|ts| format_last_active(ts, current_lang))
                                                    .unwrap_or_else(|| "—".to_string())
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    {members.more_element()}
                }

                // Direct announcement (only-to-this-sub-team)
                DirectAnnouncementCard {}

                // Danger zone (deregister)
                section { class: "card",
                    div { class: "card__head",
                        h2 { class: "card__title card__title--danger", "{tr.danger_zone}" }
                        span { class: "card__dash" }
                    }
                    div { class: "danger-zone",
                        div { class: "danger-zone__icon",
                            lucide_dioxus::TriangleAlert { class: "w-4 h-4 [&>path]:stroke-current" }
                        }
                        div { class: "danger-zone__body",
                            div { class: "danger-zone__title", "{tr.deregister_title}" }
                        }
                        a {
                            class: "btn btn--danger",
                            "data-testid": "sub-team-detail-deregister-btn",
                            onclick: move |_| {
                                nav.push(Route::TeamSubTeamDeregisterPage {
                                    username: deregister_username.clone(),
                                    sub_team_id: deregister_sub_team.clone(),
                                });
                            },
                            lucide_dioxus::X { class: "w-3 h-3 [&>path]:stroke-current" }
                            "{tr.deregister_confirm}"
                        }
                    }
                }
            }
        }
    }
}

/// "이 하위팀에만 공지" card. Inline compose (no draft) + history list.
/// Posts to `POST /api/teams/:parent/sub-teams/:child/direct-message`
/// then re-fetches `GET /...direct-messages` so the new entry shows up
/// without a full page reload.
#[component]
fn DirectAnnouncementCard() -> Element {
    use crate::features::sub_team::types::SendDirectMessageRequest;

    let tr: SubTeamTranslate = use_translate();

    let UseSubTeamActivity {
        direct_messages,
        mut handle_send_direct,
        ..
    } = use_sub_team_activity()?;

    let mut title: Signal<String> = use_signal(String::new);
    let mut body: Signal<String> = use_signal(String::new);
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);

    let send_disabled = title().trim().is_empty() || handle_send_direct.pending();
    let items = direct_messages();

    rsx! {
        section { class: "card",
            div { class: "card__head",
                h2 { class: "card__title", "{tr.direct_announce_title}" }
                span { class: "card__dash" }
            }

            div { class: "direct-msg",
                input {
                    class: "direct-msg__title",
                    r#type: "text",
                    "data-testid": "sub-team-direct-msg-title",
                    placeholder: "{tr.direct_announce_title_input}",
                    value: "{title()}",
                    oninput: move |e| {
                        title.set(e.value());
                        error_msg.set(None);
                    },
                }
                textarea {
                    class: "direct-msg__body",
                    "data-testid": "sub-team-direct-msg-body",
                    placeholder: "{tr.direct_announce_placeholder}",
                    value: "{body()}",
                    oninput: move |e| body.set(e.value()),
                }
                div { class: "direct-msg__foot",
                    span { class: "direct-msg__note", "{tr.direct_announce_note}" }
                    button {
                        class: "btn btn--primary btn--small",
                        "data-testid": "sub-team-direct-msg-send",
                        r#type: "button",
                        disabled: send_disabled,
                        onclick: move |_| {
                            if !send_disabled {
                                let req = SendDirectMessageRequest {
                                    title: title(),
                                    body: body(),
                                };
                                handle_send_direct.call(req);
                                title.set(String::new());
                                body.set(String::new());
                                error_msg.set(None);
                            }
                        },
                        lucide_dioxus::Send { class: "w-3 h-3 [&>path]:stroke-current" }
                        "{tr.direct_announce_send}"
                    }
                }
                if let Some(msg) = error_msg() {
                    div { class: "direct-msg__error", "{msg}" }
                }
            }

            // History
            div { class: "direct-msg__history",
                div { class: "direct-msg__history-title", "{tr.direct_announce_history_title}" }
                if items.items.is_empty() {
                    div { class: "direct-msg__empty", "{tr.direct_announce_history_empty}" }
                } else {
                    for ann in items.items.iter() {
                        DirectAnnouncementRow {
                            key: "{ann.id}",
                            target_post_pk: ann.target_post_pk.clone(),
                            title: ann.title.clone(),
                            body: ann.html_contents.clone(),
                            created_at: ann.created_at,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DirectAnnouncementRow(
    target_post_pk: Option<String>,
    title: String,
    body: String,
    created_at: i64,
) -> Element {
    let nav = use_navigator();
    let lang_signal = use_language();
    let when = format_last_active(created_at, lang_signal());
    // Backend persists the fan-out Post pk on the announcement row
    // (`target_post_pk`); we strip the optional `FEED#` prefix before
    // parsing into a `FeedPartition` because the route segment is the
    // raw uuid only.
    let parsed_pk: Option<FeedPartition> = target_post_pk.and_then(|s| {
        s.strip_prefix("FEED#")
            .unwrap_or(s.as_str())
            .parse::<FeedPartition>()
            .ok()
    });
    let clickable = parsed_pk.is_some();
    rsx! {
        a {
            class: "direct-msg__row",
            "data-testid": "sub-team-direct-msg-row",
            r#type: "button",
            "data-clickable": "{clickable}",
            onclick: move |_| {
                if let Some(pk) = parsed_pk.clone() {
                    nav.push(Route::PostDetail { post_id: pk });
                }
            },
            div { class: "direct-msg__row-head",
                span { class: "direct-msg__row-title", "{title}" }
                span { class: "direct-msg__row-when", "{when}" }
            }
            if !body.is_empty() {
                div {
                    class: "direct-msg__row-body",
                    dangerous_inner_html: "{body}",
                }
            }
        }
    }
}
