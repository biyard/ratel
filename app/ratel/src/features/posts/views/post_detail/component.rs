use crate::common::components::SeoMeta;
use crate::features::auth::hooks::use_user_context;
use crate::features::cross_posting::components::SyndicationPanel;
use crate::features::posts::hooks::{use_post_detail, UsePostDetail};
use crate::features::posts::*;

use super::comments::PostCommentsPanel;
use super::i18n::*;

#[component]
pub fn PostDetail(post_id: FeedPartition) -> Element {
    let tr: PostDetailSyndicatedTranslate = use_translate();
    let nav = use_navigator();

    // Inject the route's post id as context before the hook runs — the
    // hook itself is argument-free (per rule 2) and reads the id back out
    // via `use_context`. This keeps the controller's public shape free
    // of route-derived parameters.
    use_context_provider(|| post_id.clone());

    let UsePostDetail {
        detail,
        liked,
        like_count,
        mut comments_open,
        mut toggle_like,
        mut share,
        ..
    } = use_post_detail()?;

    let snapshot = detail();
    let post = snapshot.post.clone();

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

    // Author check — post-detail syndication panel is author-only (FR-7).
    // Compare the loaded post's `author_username` against the session user.
    let user_ctx = use_user_context();
    let current_username = user_ctx
        .read()
        .user
        .as_ref()
        .map(|u| u.username.clone())
        .unwrap_or_default();
    let is_author = post
        .as_ref()
        .map(|p| !current_username.is_empty() && p.author_username == current_username)
        .unwrap_or(false);

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

    let on_share = move |_| share.call();
    let on_toggle_like = move |_| toggle_like.call();
    let open_comments = move |_| comments_open.set(true);
    let close_comments = move |_| comments_open.set(false);

    rsx! {
        SeoMeta {
            title: post_title.clone(),
            description: seo_description,
            image: post_image.clone(),
        }
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
                            onclick: on_toggle_like,
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

                if is_author {
                    SyndicationPanel { post_id: post_id.clone() }
                }
            }

            if comments_open() {
                div { class: "pd-drawer-backdrop", onclick: close_comments }
                div { class: "pd-drawer-wrap", PostCommentsPanel {} }
            }
        }
    }
}

// Note: the previous `SyndicationSection` / `PendingSynCard` placeholders
// have been replaced by the real `SyndicationPanel` (Phase 1A, PR E3),
// which renders the live per-platform job state from
// `get_syndication_panel_handler`.

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
