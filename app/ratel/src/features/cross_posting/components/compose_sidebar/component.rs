use crate::common::*;
use crate::features::cross_posting::hooks::{UseCrossPosting, use_cross_posting};
use crate::features::cross_posting::i18n::ComposeSidebarTranslate;
use crate::features::cross_posting::models::ConnectionStatus;
use crate::features::cross_posting::types::{ConnectionResponse, SocialPlatform};

/// Compose-time cross-post sidebar.
///
/// Drives `UseCrossPosting::per_post_enabled` (the `HashMap<SocialPlatform, bool>`
/// the publish action serializes into `enabled_platforms`) and exposes a live
/// "Reaching N networks" summary plus per-platform char-count warnings.
///
/// The parent (`post_edit`) controls visibility — Private / team-shared posts
/// don't render this component at all (FR-9 #50). Disconnected platforms emit
/// `on_connect_request` so the parent can navigate to Settings → Connections.
///
/// Char-count uses raw `content.chars().count()` (HTML tags included). This
/// **over-warns** slightly compared to Stage 2 dispatch which strips HTML on
/// the server — a conservative bias is preferred to under-warning the author.
#[component]
pub fn CrossPostSidebar(
    content: Signal<String>,
    on_connect_request: EventHandler<SocialPlatform>,
) -> Element {
    let UseCrossPosting {
        connections,
        mut per_post_enabled,
        reach_count,
        ..
    } = use_cross_posting();
    let t: ComposeSidebarTranslate = use_translate();

    let conn_list: Vec<ConnectionResponse> = connections();
    let bsky = conn_list
        .iter()
        .find(|c| c.platform == SocialPlatform::Bluesky)
        .cloned();
    let linkedin = conn_list
        .iter()
        .find(|c| c.platform == SocialPlatform::LinkedIn)
        .cloned();
    let threads = conn_list
        .iter()
        .find(|c| c.platform == SocialPlatform::Threads)
        .cloned();

    // Match `post_edit`'s `re-char-count` value (which strips HTML tags)
    // so the editor's footer count and the sidebar's per-platform footer
    // never disagree. Without strip_html the sidebar would over-count by
    // every `<p>`, `<br>`, etc. the rich editor inserts.
    let char_count = strip_html(&content()).chars().count();

    rsx! {
        // Root is a `div` (not `aside`) — the parent post-edit shell already
        // wraps the right-rail in an `aside.side-panel`, and HTML5 prefers
        // not to nest `<aside>` for non-tangential content. Mockup class
        // names (`crosspost`, `crosspost-head`, `pp-card`, …) are preserved.
        // CSS lives globally in `app/ratel/assets/main.css` (FOUC convention).
        div { class: "crosspost",
            div { class: "crosspost-head",
                span { class: "crosspost-head__eyebrow", "{t.eyebrow}" }
                h2 { class: "crosspost-head__title",
                    span { class: "crosspost-head__title-accent", "{t.title}" }
                    svg {
                        width: "18",
                        height: "18",
                        "viewBox": "0 0 24 24",
                        "fill": "none",
                        "stroke": "#6eedd8",
                        "stroke-width": "2",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        circle { "cx": "18", "cy": "5", "r": "3" }
                        circle { "cx": "6", "cy": "12", "r": "3" }
                        circle { "cx": "18", "cy": "19", "r": "3" }
                        line {
                            "x1": "8.59",
                            "y1": "13.51",
                            "x2": "15.42",
                            "y2": "17.49",
                        }
                        line {
                            "x1": "15.41",
                            "y1": "6.51",
                            "x2": "8.59",
                            "y2": "10.49",
                        }
                    }
                }
                p { class: "crosspost-head__sub", "{t.sub}" }
            }

            // ── Reach summary ──────────────────────────────────────
            div { class: "reach-summary",
                div { class: "reach-summary__left",
                    span { class: "reach-summary__label", "{t.reaching}" }
                    span { class: "reach-summary__value",
                        strong { "{reach_count()}" }
                        " {t.networks_suffix}"
                    }
                }
                div { class: "reach-summary__icons",
                    ReachChip {
                        platform: SocialPlatform::Bluesky,
                        active: is_active(&bsky, &per_post_enabled(), SocialPlatform::Bluesky),
                    }
                    ReachChip {
                        platform: SocialPlatform::LinkedIn,
                        active: is_active(&linkedin, &per_post_enabled(), SocialPlatform::LinkedIn),
                    }
                    ReachChip {
                        platform: SocialPlatform::Threads,
                        active: is_active(&threads, &per_post_enabled(), SocialPlatform::Threads),
                    }
                }
            }

            // ── Platform cards ─────────────────────────────────────
            PlatformCard {
                platform: SocialPlatform::Bluesky,
                connection: bsky.clone(),
                enabled: is_active(&bsky, &per_post_enabled(), SocialPlatform::Bluesky),
                char_count,
                on_toggle: move |new_val: bool| {
                    let mut map = per_post_enabled();
                    map.insert(SocialPlatform::Bluesky, new_val);
                    per_post_enabled.set(map);
                },
                on_connect: move |_| on_connect_request.call(SocialPlatform::Bluesky),
            }
            PlatformCard {
                platform: SocialPlatform::LinkedIn,
                connection: linkedin.clone(),
                enabled: is_active(&linkedin, &per_post_enabled(), SocialPlatform::LinkedIn),
                char_count,
                on_toggle: move |new_val: bool| {
                    let mut map = per_post_enabled();
                    map.insert(SocialPlatform::LinkedIn, new_val);
                    per_post_enabled.set(map);
                },
                on_connect: move |_| on_connect_request.call(SocialPlatform::LinkedIn),
            }
            PlatformCard {
                platform: SocialPlatform::Threads,
                connection: threads.clone(),
                enabled: is_active(&threads, &per_post_enabled(), SocialPlatform::Threads),
                char_count,
                on_toggle: move |new_val: bool| {
                    let mut map = per_post_enabled();
                    map.insert(SocialPlatform::Threads, new_val);
                    per_post_enabled.set(map);
                },
                on_connect: move |_| on_connect_request.call(SocialPlatform::Threads),
            }
        }
    }
}

/// True when the platform is connected AND the per-post override resolves to
/// "include in this publish". The override falls back to the persistent
/// `auto_post_enabled` flag when the user hasn't explicitly toggled this
/// platform for this draft.
fn is_active(
    conn: &Option<ConnectionResponse>,
    overrides: &std::collections::HashMap<SocialPlatform, bool>,
    platform: SocialPlatform,
) -> bool {
    let Some(c) = conn else { return false };
    if c.status != ConnectionStatus::Connected {
        return false;
    }
    overrides.get(&platform).copied().unwrap_or(c.auto_post_enabled)
}

#[component]
fn ReachChip(platform: SocialPlatform, active: bool) -> Element {
    let chip_class = match platform {
        SocialPlatform::Bluesky => "reach-chip reach-chip--bsky",
        SocialPlatform::LinkedIn => "reach-chip reach-chip--linkedin",
        SocialPlatform::Threads => "reach-chip reach-chip--threads",
    };
    rsx! {
        span { class: "{chip_class}", "data-off": "{!active}",
            PlatformLogo { platform }
        }
    }
}

#[component]
fn PlatformLogo(platform: SocialPlatform) -> Element {
    match platform {
        SocialPlatform::Bluesky => rsx! {
            svg { "viewBox": "0 0 24 24", "fill": "currentColor",
                path { "d": "M12 10.5c-1.3-2.5-4.9-7.2-8.2-9.5C.7-1.2 0 .5 0 1.5c0 1.1.6 9.1 1 10.3.8 4 5.5 5.1 9.6 4.4-6.5 1.1-12.3 3.5-4.7 11.4 2.5 2.6 3.5-2.6 4-5.2.5 2.6 1.6 7.8 4 5.2 7.7-7.9 1.9-10.4-4.6-11.5 4 .7 8.8-.4 9.6-4.4.4-1.2 1-9.2 1-10.3 0-1-.7-2.7-3.8-.5-3.3 2.3-6.9 7-8.2 9.6z" }
            }
        },
        SocialPlatform::LinkedIn => rsx! {
            svg { "viewBox": "0 0 24 24", "fill": "currentColor",
                path { "d": "M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433a2.062 2.062 0 01-2.063-2.065 2.063 2.063 0 112.063 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z" }
            }
        },
        SocialPlatform::Threads => rsx! {
            svg { "viewBox": "0 0 24 24", "fill": "currentColor",
                path { "d": "M12.186 24h-.007c-3.581-.024-6.334-1.205-8.184-3.509C2.35 18.44 1.5 15.586 1.472 12.01v-.017c.03-3.579.879-6.43 2.525-8.482C5.845 1.205 8.6.024 12.18 0h.014c2.746.02 5.043.725 6.826 2.098 1.677 1.29 2.858 3.13 3.509 5.467l-2.04.569c-1.104-3.96-3.898-5.984-8.304-6.015-2.91.022-5.11.936-6.54 2.717C4.307 6.504 3.616 8.914 3.589 12c.027 3.086.718 5.496 2.057 7.164 1.43 1.783 3.631 2.698 6.54 2.717 2.623-.02 4.358-.631 5.8-2.045 1.647-1.613 1.618-3.593 1.09-4.798z" }
            }
        },
    }
}

#[component]
fn PlatformCard(
    platform: SocialPlatform,
    connection: Option<ConnectionResponse>,
    enabled: bool,
    char_count: usize,
    on_toggle: EventHandler<bool>,
    on_connect: EventHandler<()>,
) -> Element {
    let t: ComposeSidebarTranslate = use_translate();

    let connected = connection
        .as_ref()
        .map(|c| c.status == ConnectionStatus::Connected)
        .unwrap_or(false);

    let platform_data = match platform {
        SocialPlatform::Bluesky => "bluesky",
        SocialPlatform::LinkedIn => "linkedin",
        SocialPlatform::Threads => "threads",
    };
    let logo_class = match platform {
        SocialPlatform::Bluesky => "pp-logo pp-logo--bsky",
        SocialPlatform::LinkedIn => "pp-logo pp-logo--linkedin",
        SocialPlatform::Threads => "pp-logo pp-logo--threads",
    };

    let limit = platform.char_limit();
    let warn_threshold = limit * 90 / 100; // 90% → warning state
    let count_state = if char_count > limit {
        "over"
    } else if char_count > warn_threshold {
        "warn"
    } else {
        "ok"
    };
    let bar_pct = ((char_count.min(limit) * 100) / limit).min(100);

    let connect_hint = match platform {
        SocialPlatform::Bluesky => t.connect_hint_bluesky,
        SocialPlatform::LinkedIn => t.connect_hint_linkedin,
        SocialPlatform::Threads => t.connect_hint_threads,
    };
    let connect_label = match platform {
        SocialPlatform::Bluesky => t.connect_btn_bluesky,
        SocialPlatform::LinkedIn => t.connect_btn_linkedin,
        SocialPlatform::Threads => t.connect_btn_threads,
    };

    rsx! {
        article {
            class: "pp-card",
            "data-platform": "{platform_data}",
            "data-enabled": "{enabled}",
            "data-connected": "{connected}",

            header { class: "pp-head",
                div { class: "pp-head__left",
                    span { class: "{logo_class}",
                        PlatformLogo { platform }
                    }
                    div {
                        span { class: "pp-name",
                            "{platform.display_name()}"
                            if let Some(c) = connection.as_ref() {
                                if connected {
                                    span { class: "pp-name__handle", "@{c.external_handle}" }
                                } else {
                                    span { class: "pp-name__status", "{t.not_connected}" }
                                }
                            } else {
                                span { class: "pp-name__status", "{t.not_connected}" }
                            }
                        }
                    }
                }
                if connected {
                    button {
                        class: "switch pp-head__switch",
                        "aria-checked": "{enabled}",
                        "aria-label": "{platform.display_name()}",
                        onclick: move |_| on_toggle.call(!enabled),
                    }
                }
            }

            if !connected {
                div { class: "connect-cta",
                    p { class: "connect-cta__hint", "{connect_hint}" }
                    button {
                        class: "connect-cta__btn",
                        onclick: move |_| on_connect.call(()),
                        svg {
                            "viewBox": "0 0 24 24",
                            "fill": "none",
                            "stroke": "currentColor",
                            "stroke-width": "2.5",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            polyline { "points": "5 12 19 12" }
                            polyline { "points": "12 5 19 12 12 19" }
                        }
                        "{connect_label}"
                    }
                }
            } else if enabled {
                footer { class: "pp-foot",
                    div {
                        class: "pp-foot__count",
                        "data-state": "{count_state}",
                        strong { "{char_count}" }
                        " / {limit}"
                        div { class: "pp-foot__count-bar",
                            div {
                                class: "pp-foot__count-bar-fill",
                                style: "width: {bar_pct}%;",
                            }
                        }
                    }
                    if count_state == "over" {
                        div { class: "pp-foot__badges",
                            span { class: "pp-badge pp-badge--truncated", "{t.truncated_badge}" }
                        }
                    }
                }
            }
        }
    }
}

/// Strip HTML tags so the sidebar's character count matches the
/// `.re-char-count` footer in `post_edit` (which does the same).
/// Mirrors the private helper in `post_edit/component.rs:1078`.
fn strip_html(html: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }
    result
}
