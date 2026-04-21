use crate::common::components::SeoMeta;
use crate::features::posts::controllers::get_post::get_post_handler;
use crate::features::posts::controllers::like_post::like_post_handler;
use crate::features::posts::*;

use super::comments::PostCommentsPanel;
use super::i18n::*;

#[component]
pub fn PostDetail(post_id: FeedPartition) -> Element {
    let tr: PostDetailSyndicatedTranslate = use_translate();
    let nav = use_navigator();

    let p1 = post_id.clone();
    let mut resource = use_loader(move || {
        let post_id = p1.clone();
        async move { get_post_handler(post_id).await }
    })?;

    let detail = resource();
    let post = detail.post.clone();

    // Like state — signals seeded from the loader so optimistic updates on
    // click don't wait for the next resource(). `detail.is_liked` is the
    // viewer-specific liked flag resolved server-side in `get_post_handler`
    // (batch-read of PostLike by user.pk); `detail.post.likes` is the
    // aggregate counter on the Post row.
    let initial_likes = detail.post.as_ref().map(|p| p.likes).unwrap_or(0);
    let mut liked = use_signal(|| detail.is_liked);
    let mut like_count = use_signal(|| initial_likes);

    let mut comments_open = use_signal(|| false);

    let post_title = post
        .as_ref()
        .map(|p| p.title.clone())
        .unwrap_or_default();
    let post_html = post
        .as_ref()
        .map(|p| p.html_contents.clone())
        .unwrap_or_default();
    let post_image = post
        .as_ref()
        .and_then(|p| p.urls.first().cloned())
        .unwrap_or_default();
    let author_name = post
        .as_ref()
        .map(|p| {
            if p.author_display_name.is_empty() {
                p.author_username.clone()
            } else {
                p.author_display_name.clone()
            }
        })
        .unwrap_or_default();
    let author_initial = author_name
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "·".to_string());
    let author_avatar = post
        .as_ref()
        .map(|p| p.author_profile_url.clone())
        .unwrap_or_default();
    let created_at = post.as_ref().map(|p| p.created_at).unwrap_or(0);
    let comments = post.as_ref().map(|p| p.comments).unwrap_or(0);
    let read_minutes = read_minutes_from_html(&post_html);

    let seo_description = strip_html_excerpt(&post_html, 200);

    let edit_nav = nav;
    let p_for_edit = post_id.clone();
    let go_edit = move |_| {
        edit_nav.push(Route::PostEdit {
            post_id: p_for_edit.clone(),
        });
    };
    let go_back = move |_| {
        nav.go_back();
    };

    let mut toast = use_toast();
    let share_post_id = post_id.clone();
    let on_share = move |_| {
        let id = share_post_id.to_string();
        spawn(async move {
            // Runs in the browser — builds the absolute URL from
            // `window.location.origin` so staging/prod/localhost all
            // produce the right link, then asks the Clipboard API to copy
            // it. The eval returns a bool so Rust can show the right toast.
            let js = format!(
                r#"(async function() {{
                    var url = window.location.origin + '/posts/{id}';
                    try {{
                        await navigator.clipboard.writeText(url);
                        dioxus.send(true);
                    }} catch (e) {{
                        dioxus.send(false);
                    }}
                }})();"#
            );
            let mut eval = document::eval(&js);
            let ok = eval.recv::<bool>().await.unwrap_or(false);
            if ok {
                toast.info(tr.share_link_copied.to_string());
            } else {
                toast.warn(tr.share_link_copy_failed.to_string());
            }
        });
    };

    let like_post_id = post_id.clone();
    let toggle_like = move |_| {
        let next = !liked();
        // Optimistic UI — flip the signals now, revert on error.
        liked.set(next);
        like_count.set((like_count() + if next { 1 } else { -1 }).max(0));
        let post_id = like_post_id.clone();
        spawn(async move {
            if let Err(e) = like_post_handler(post_id, next).await {
                tracing::error!("like post failed: {e}");
                liked.set(!next);
                like_count.set((like_count() + if next { -1 } else { 1 }).max(0));
            }
        });
    };

    let open_comments = move |_| comments_open.set(true);
    let close_comments = move |_| comments_open.set(false);

    rsx! {
        SeoMeta {
            title: post_title.clone(),
            description: seo_description,
            image: post_image.clone(),
        }
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        document::Script { defer: true, src: asset!("./script.js") }

        div { class: "post-arena",
            div { class: "arena-topbar",
                div { class: "arena-topbar__left",
                    button {
                        class: "back-btn",
                        aria_label: "{tr.btn_back_aria}",
                        onclick: go_back,
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
                        span { class: "topbar-title__eyebrow", "{tr.topbar_eyebrow}" }
                        span { class: "topbar-title__main", "{tr.topbar_main}" }
                    }
                }
                div { class: "arena-topbar__right",
                    button { class: "topbar-btn", onclick: go_edit,
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" }
                            path { d: "M18.5 2.5a2.12 2.12 0 0 1 3 3L12 15l-4 1 1-4z" }
                        }
                        "{tr.btn_edit}"
                    }
                    button { class: "topbar-btn", onclick: on_share,
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            circle { cx: "18", cy: "5", r: "3" }
                            circle { cx: "6", cy: "12", r: "3" }
                            circle { cx: "18", cy: "19", r: "3" }
                            line {
                                x1: "8.59",
                                y1: "13.51",
                                x2: "15.42",
                                y2: "17.49",
                            }
                            line {
                                x1: "15.41",
                                y1: "6.51",
                                x2: "8.59",
                                y2: "10.49",
                            }
                        }
                        "{tr.btn_share}"
                    }
                }
            }

            main { class: "page",
                article { class: "post-hero",
                    div { class: "post-hero__meta",
                        span { class: "post-hero__avatar",
                            if !author_avatar.is_empty() {
                                img { src: "{author_avatar}", alt: "" }
                            } else {
                                "{author_initial}"
                            }
                        }
                        div { class: "post-hero__author",
                            span { class: "post-hero__author-name", "{author_name}" }
                            span { class: "post-hero__author-time",
                                "{format_published(created_at, read_minutes)}"
                            }
                        }
                    }

                    h1 { class: "post-hero__title", "{post_title}" }

                    if !post_image.is_empty() {
                        figure { class: "post-hero__image",
                            img { src: "{post_image}", alt: "" }
                        }
                    }

                    div {
                        class: "post-hero__body",
                        dangerous_inner_html: "{post_html}",
                    }

                    div { class: "post-hero__actions",
                        button {
                            class: "action-btn",
                            "data-active": liked(),
                            onclick: toggle_like,
                            svg {
                                view_box: "0 0 24 24",
                                fill: if liked() { "currentColor" } else { "none" },
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" }
                            }
                            strong { "{like_count()}" }
                            " {tr.action_likes_suffix}"
                        }
                        button { class: "action-btn", onclick: open_comments,
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
                            }
                            strong { "{comments}" }
                            " {tr.action_comments_suffix}"
                        }
                    }
                }

                SyndicationSection {}
            }

            if comments_open() {
                div { class: "pd-drawer-backdrop", onclick: close_comments }
                div { class: "pd-drawer-wrap",
                    PostCommentsPanel {
                        detail: detail.clone(),
                        post_pk: post_id.clone(),
                        on_refresh: move |_| {
                            resource.restart();
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn SyndicationSection() -> Element {
    let tr: PostDetailSyndicatedTranslate = use_translate();

    rsx! {
        section { class: "syn",
            div { class: "syn-head",
                span { class: "syn-head__title", "{tr.syn_title}" }
                span { class: "syn-head__summary",
                    strong { "0 of 3" }
                    " {tr.syn_summary_prefix}"
                }
            }

            div { class: "syn-stats",
                div { class: "syn-stat",
                    div { class: "syn-stat__value",
                        span { class: "syn-stat__value--accent", "0" }
                    }
                    div { class: "syn-stat__label", "{tr.stat_external_reads}" }
                }
                div { class: "syn-stat",
                    div { class: "syn-stat__value", "0" }
                    div { class: "syn-stat__label", "{tr.stat_reactions}" }
                }
                div { class: "syn-stat",
                    div { class: "syn-stat__value", "0" }
                    div { class: "syn-stat__label", "{tr.stat_backlink_clicks}" }
                }
            }

            div { class: "syn-list",
                PendingSynCard {
                    name: "Bluesky",
                    logo_class: "syn-logo--bsky",
                    icon: rsx! {
                        svg { view_box: "0 0 24 24", fill: "currentColor",
                            path { d: "M12 10.5c-1.3-2.5-4.9-7.2-8.2-9.5C.7-1.2 0 .5 0 1.5c0 1.1.6 9.1 1 10.3.8 4 5.5 5.1 9.6 4.4-6.5 1.1-12.3 3.5-4.7 11.4 2.5 2.6 3.5-2.6 4-5.2.5 2.6 1.6 7.8 4 5.2 7.7-7.9 1.9-10.4-4.6-11.5 4 .7 8.8-.4 9.6-4.4.4-1.2 1-9.2 1-10.3 0-1-.7-2.7-3.8-.5-3.3 2.3-6.9 7-8.2 9.6z" }
                        }
                    },
                }
                PendingSynCard {
                    name: "LinkedIn",
                    logo_class: "syn-logo--linkedin",
                    icon: rsx! {
                        svg { view_box: "0 0 24 24", fill: "currentColor",
                            path { d: "M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433a2.062 2.062 0 01-2.063-2.065 2.063 2.063 0 112.063 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z" }
                        }
                    },
                }
                PendingSynCard {
                    name: "Threads",
                    logo_class: "syn-logo--threads",
                    icon: rsx! {
                        svg { view_box: "0 0 24 24", fill: "currentColor",
                            path { d: "M12.186 24h-.007c-3.581-.024-6.334-1.205-8.184-3.509C2.35 18.44 1.5 15.586 1.472 12.01v-.017c.03-3.579.879-6.43 2.525-8.482C5.845 1.205 8.6.024 12.18 0h.014c2.746.02 5.043.725 6.826 2.098 1.677 1.29 2.858 3.13 3.509 5.467l-2.04.569c-1.104-3.96-3.898-5.984-8.304-6.015-2.91.022-5.11.936-6.54 2.717C4.307 6.504 3.616 8.914 3.589 12c.027 3.086.718 5.496 2.057 7.164 1.43 1.783 3.631 2.698 6.54 2.717 2.623-.02 4.358-.631 5.8-2.045 1.647-1.613 1.618-3.593 1.09-4.798z" }
                        }
                    },
                }
            }
        }
    }
}

/// Placeholder card shown while no real syndication integration is live.
/// All providers render identically with a "Coming soon" pill and muted
/// sub-line — no engagement numbers, no action buttons, no hardcoded
/// fixtures. When a provider ships, swap this for a real card component
/// that takes a `SyndicationStatus` and renders success/pending/failed
/// states against live data.
#[component]
fn PendingSynCard(name: String, logo_class: String, icon: Element) -> Element {
    let tr: PostDetailSyndicatedTranslate = use_translate();
    rsx! {
        article { class: "syn-card", "data-status": "pending",
            div { class: "syn-card__body",
                span { class: "syn-logo {logo_class}", {icon} }
                div { class: "syn-card__main",
                    div { class: "syn-card__name-row",
                        span { class: "syn-card__name", "{name}" }
                        span { class: "status-pill status-pill--pending", "{tr.status_coming_soon}" }
                    }
                    div { class: "syn-card__sub",
                        span { class: "syn-card__sub-item", "{tr.card_coming_soon_hint}" }
                    }
                }
            }
        }
    }
}
/// Render "Published {N} hours ago · {M} min read" from a unix-ms timestamp.
/// Degrades gracefully when `created_at` is 0 (fresh draft) by showing the
/// read estimate only.
fn format_published(created_at_ms: i64, read_minutes: u32) -> String {
    let now_ms = chrono::Utc::now().timestamp_millis();
    let delta_sec = (now_ms - created_at_ms).max(0) / 1000;
    let when = if created_at_ms <= 0 {
        String::new()
    } else if delta_sec < 60 {
        "Published just now".to_string()
    } else if delta_sec < 60 * 60 {
        format!("Published {}m ago", delta_sec / 60)
    } else if delta_sec < 24 * 60 * 60 {
        format!("Published {}h ago", delta_sec / 3_600)
    } else {
        format!("Published {}d ago", delta_sec / 86_400)
    };
    if when.is_empty() {
        format!("{read_minutes} min read")
    } else {
        format!("{when} · {read_minutes} min read")
    }
}

/// Rough reading time estimate: 220 words per minute on the HTML body,
/// clamped to a minimum of 1 minute so freshly-drafted posts don't show
/// "0 min read".
fn read_minutes_from_html(html: &str) -> u32 {
    let re = regex::Regex::new(r"<[^>]*>").ok();
    let text = match re {
        Some(ref r) => r.replace_all(html, " ").into_owned(),
        None => html.to_string(),
    };
    let words = text.split_whitespace().count() as u32;
    (words / 220).max(1)
}

/// First `n` chars of HTML content with tags stripped — used for SEO meta
/// description. Chars (not bytes) so CJK content isn't cut mid-codepoint.
fn strip_html_excerpt(html: &str, n: usize) -> String {
    let re = match regex::Regex::new(r"<[^>]*>") {
        Ok(r) => r,
        Err(_) => return html.chars().take(n).collect(),
    };
    let text = re.replace_all(html, "").to_string();
    if text.chars().count() > n {
        text.chars().take(n).collect()
    } else {
        text
    }
}
