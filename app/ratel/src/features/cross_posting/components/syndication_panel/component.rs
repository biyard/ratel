use crate::common::*;
use crate::features::cross_posting::hooks::{UseSyndicationPanel, use_syndication_panel};
use crate::features::cross_posting::i18n::SyndicationPanelTranslate;
use crate::features::cross_posting::models::{ErrorCategory, JobState};
use crate::features::cross_posting::types::{SocialPlatform, SyndicationJobView};

/// Author-only post-detail panel.
///
/// Mounts under the post body on the post-detail route. The hook's loader
/// returns `None` for non-authors (server enforces via session check), so
/// this component renders a blank fragment in that case — the parent
/// `post_detail` should still author-gate the mount for efficiency.
///
/// Status pills, engagement counts, and the Retry CTA all key off the
/// `SyndicationJobView::state` enum (Pending / Published / Failed / Skipped).
/// Retry is allowed only on Failed; the back-end re-validates this.
#[component]
pub fn SyndicationPanel(post_id: FeedPartition) -> Element {
    let UseSyndicationPanel {
        panel,
        mut handle_retry,
    } = use_syndication_panel(post_id)?;
    let t: SyndicationPanelTranslate = use_translate();

    let Some(data) = panel() else {
        return rsx! {};
    };

    let jobs = data.jobs;
    if jobs.is_empty() {
        return rsx! {};
    }

    let total = jobs.len();
    let total_published = jobs
        .iter()
        .filter(|j| j.state == JobState::Published)
        .count();
    let total_failed = jobs.iter().filter(|j| j.state == JobState::Failed).count();

    // Engagement totals (only Published jobs carry counts).
    let total_likes: i32 = jobs
        .iter()
        .filter_map(|j| j.engagement.as_ref().map(|e| e.likes))
        .sum();
    let total_comments: i32 = jobs
        .iter()
        .filter_map(|j| j.engagement.as_ref().map(|e| e.comments))
        .sum();
    let total_reposts: i32 = jobs
        .iter()
        .filter_map(|j| j.engagement.as_ref().map(|e| e.reposts))
        .sum();

    rsx! {
        document::Stylesheet { href: asset!("./style.css") }

        section { class: "syn",
            div { class: "syn-head",
                span { class: "syn-head__title", "{t.title}" }
                span { class: "syn-head__summary",
                    strong { "{total_published} of {total}" }
                    " {t.summary_succeeded}"
                    if total_failed > 0 {
                        " · {total_failed} {t.summary_failed_suffix}"
                    }
                }
            }

            div { class: "syn-stats",
                div { class: "syn-stat",
                    div { class: "syn-stat__value",
                        span { class: "syn-stat__value--accent", "{total_likes}" }
                    }
                    div { class: "syn-stat__label", "{t.stat_likes}" }
                }
                div { class: "syn-stat",
                    div { class: "syn-stat__value", "{total_comments}" }
                    div { class: "syn-stat__label", "{t.stat_comments}" }
                }
                div { class: "syn-stat",
                    div { class: "syn-stat__value", "{total_reposts}" }
                    div { class: "syn-stat__label", "{t.stat_reposts}" }
                }
            }

            div { class: "syn-list",
                for job in jobs {
                    SyndicationCard {
                        key: "{job.platform}",
                        job: job.clone(),
                        on_retry: move |p: SocialPlatform| handle_retry.call(p),
                    }
                }
            }
        }
    }
}

#[component]
fn SyndicationCard(job: SyndicationJobView, on_retry: EventHandler<SocialPlatform>) -> Element {
    let t: SyndicationPanelTranslate = use_translate();

    let status_data = match job.state {
        JobState::Published => "success",
        JobState::Pending => "pending",
        JobState::Failed => "failed",
        JobState::Skipped => "skipped",
    };
    let pill_class = match job.state {
        JobState::Published => "status-pill status-pill--success",
        JobState::Pending => "status-pill status-pill--pending",
        JobState::Failed => "status-pill status-pill--failed",
        JobState::Skipped => "status-pill status-pill--skipped",
    };
    let pill_label = match job.state {
        JobState::Published => t.status_published,
        JobState::Pending => t.status_pending,
        JobState::Failed => t.status_failed,
        JobState::Skipped => t.status_skipped,
    };
    let logo_class = match job.platform {
        SocialPlatform::Bluesky => "syn-logo syn-logo--bsky",
        SocialPlatform::LinkedIn => "syn-logo syn-logo--linkedin",
        SocialPlatform::Threads => "syn-logo syn-logo--threads",
    };

    let attempts = job.attempts;
    let platform = job.platform;

    rsx! {
        article { class: "syn-card", "data-status": "{status_data}",

            div { class: "syn-card__body",
                span { class: "{logo_class}",
                    PlatformLogo { platform }
                }
                div { class: "syn-card__main",
                    div { class: "syn-card__name-row",
                        span { class: "syn-card__name", "{platform.display_name()}" }
                        span { class: "{pill_class}", "{pill_label}" }
                    }
                    if job.state == JobState::Failed {
                        if let Some(msg) = job.last_error_message.as_ref() {
                            div { class: "syn-card__err",
                                svg {
                                    "viewBox": "0 0 24 24",
                                    "fill": "none",
                                    "stroke": "currentColor",
                                    "stroke-width": "2",
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    circle { "cx": "12", "cy": "12", "r": "10" }
                                    line {
                                        "x1": "12",
                                        "y1": "8",
                                        "x2": "12",
                                        "y2": "12",
                                    }
                                    line {
                                        "x1": "12",
                                        "y1": "16",
                                        "x2": "12.01",
                                        "y2": "16",
                                    }
                                }
                                span {
                                    "{msg} · {t.attempts_label} "
                                    strong { "{attempts}" }
                                }
                            }
                        }
                    }
                    if job.state == JobState::Pending {
                        div { class: "syn-card__sub",
                            span { class: "syn-card__sub-item",
                                svg {
                                    "viewBox": "0 0 24 24",
                                    "fill": "none",
                                    "stroke": "currentColor",
                                    "stroke-width": "2",
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    circle { "cx": "12", "cy": "12", "r": "10" }
                                    polyline { "points": "12 6 12 12 16 14" }
                                }
                                span { "{t.queued_hint}" }
                            }
                        }
                    }
                }
                div { class: "syn-card__actions",
                    if job.state == JobState::Published {
                        if let Some(url) = job.external_post_url.as_ref() {
                            a {
                                class: "mini-btn",
                                href: "{url}",
                                target: "_blank",
                                svg {
                                    "viewBox": "0 0 24 24",
                                    "fill": "none",
                                    "stroke": "currentColor",
                                    "stroke-width": "2",
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    polyline { "points": "15 3 21 3 21 9" }
                                    polyline { "points": "10 14 21 3" }
                                    path { "d": "M21 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h6" }
                                }
                                "{t.btn_view}"
                            }
                        }
                    } else if job.state == JobState::Failed
                        && !matches!(job.last_error_category, Some(ErrorCategory::AuthExpired))
                    {
                        // AuthExpired → user must reconnect (no retry CTA);
                        // other failure categories allow author-initiated retry.
                        button {
                            class: "mini-btn mini-btn--retry",
                            onclick: move |_| on_retry.call(platform),
                            svg {
                                "viewBox": "0 0 24 24",
                                "fill": "none",
                                "stroke": "currentColor",
                                "stroke-width": "2",
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                polyline { "points": "23 4 23 10 17 10" }
                                path { "d": "M20.49 15a9 9 0 1 1-2.12-9.36L23 10" }
                            }
                            "{t.btn_retry}"
                        }
                    }
                }
            }

            if let Some(eng) = job.engagement.as_ref() {
                div { class: "syn-card__engagement",
                    span { class: "engage",
                        svg {
                            "viewBox": "0 0 24 24",
                            "fill": "none",
                            "stroke": "currentColor",
                            "stroke-width": "2",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            path { "d": "M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" }
                        }
                        strong { "{eng.likes}" }
                        " {t.engage_likes}"
                    }
                    span { class: "engage",
                        svg {
                            "viewBox": "0 0 24 24",
                            "fill": "none",
                            "stroke": "currentColor",
                            "stroke-width": "2",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            path { "d": "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
                        }
                        strong { "{eng.comments}" }
                        " {t.engage_comments}"
                    }
                    span { class: "engage",
                        svg {
                            "viewBox": "0 0 24 24",
                            "fill": "none",
                            "stroke": "currentColor",
                            "stroke-width": "2",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            path { "d": "M17 1l4 4-4 4" }
                            path { "d": "M3 11V9a4 4 0 0 1 4-4h14" }
                            path { "d": "M7 23l-4-4 4-4" }
                            path { "d": "M21 13v2a4 4 0 0 1-4 4H3" }
                        }
                        strong { "{eng.reposts}" }
                        " {t.engage_reposts}"
                    }
                }
            }
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
