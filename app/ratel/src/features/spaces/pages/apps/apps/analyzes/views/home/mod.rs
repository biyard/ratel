use super::*;
use crate::features::spaces::space_common::hooks::{use_space, use_space_role};

const DEFAULT_SPACE_LOGO: &str = "https://metadata.ratel.foundation/logos/logo-symbol.png";

/// Public entrypoint for the Analyzes list page. Creators get the full
/// arena list; other roles see a minimal "no access" state (matches the
/// existing pre-arena implementation which early-returned for non-creators).
#[component]
pub fn SpaceAnalyzesAppPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let role = use_space_role()();

    if role != SpaceUserRole::Creator {
        return rsx! {
            ViewerEmpty {}
        };
    }

    rsx! {
        CreatorArenaPage { space_id }
    }
}

#[component]
fn CreatorArenaPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let space = use_space();
    let nav = use_navigator();

    let UseSpaceAnalyzes { mut polls, .. } = use_space_analyzes(space_id)?;

    let poll_items = polls.items();
    let poll_count = poll_items.len();
    let polls_has_more = polls.has_more();
    let polls_is_loading = polls.is_loading();

    // Discussions section — gated to `local-dev` builds, matching the
    // pre-arena implementation. The cfg attribute can't sit directly on
    // a component call inside `rsx!`, so we stash each variant into a
    // value and reference it below.
    #[cfg(feature = "local-dev")]
    let discussions_section = rsx! {
        DiscussionsSection { space_id }
    };
    #[cfg(not(feature = "local-dev"))]
    let discussions_section = rsx! {};

    let space_data = space();
    let space_logo = if space_data.logo.is_empty() {
        DEFAULT_SPACE_LOGO.to_string()
    } else {
        space_data.logo.clone()
    };
    let space_title = space_data.title.clone();

    rsx! {
        document::Link { rel: "preload", href: asset!("./style.css"), r#as: "style" }
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div { class: "space-analyzes-arena",
            // ── Topbar ───────────────────────────────────────────
            header { class: "saz-topbar", role: "banner",
                div { class: "saz-topbar__left",
                    button {
                        r#type: "button",
                        class: "saz-back-btn",
                        "aria-label": "Back",
                        "data-testid": "topbar-back",
                        onclick: move |_| {
                            nav.go_back();
                        },
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            "stroke-width": "2",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            path { d: "M19 12H5" }
                            path { d: "M12 19l-7-7 7-7" }
                        }
                    }
                    img {
                        class: "saz-topbar__logo",
                        alt: "Space logo",
                        src: "{space_logo}",
                    }
                    nav { class: "saz-breadcrumb",
                        span { class: "saz-breadcrumb__item", "{space_title}" }
                        span { class: "saz-breadcrumb__sep", "›" }
                        span { class: "saz-breadcrumb__item", "Apps" }
                        span { class: "saz-breadcrumb__sep", "›" }
                        span { class: "saz-breadcrumb__item saz-breadcrumb__current",
                            "{tr.page_title}"
                        }
                    }
                    span { class: "saz-type-badge", "data-testid": "type-badge",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            "stroke-width": "2",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            line {
                                x1: "18",
                                y1: "20",
                                x2: "18",
                                y2: "10",
                            }
                            line {
                                x1: "12",
                                y1: "20",
                                x2: "12",
                                y2: "4",
                            }
                            line {
                                x1: "6",
                                y1: "20",
                                x2: "6",
                                y2: "14",
                            }
                        }
                        "{tr.page_title}"
                    }
                    span { class: "saz-topbar-title", "{tr.poll_section_title}" }
                }
            }

            // ── Body ─────────────────────────────────────────────
            main { class: "saz-body",
                // Polls section — always visible.
                section { class: "saz-section", "data-testid": "section-polls",
                    div { class: "saz-section__head",
                        div { class: "saz-section__title",
                            span { class: "saz-section__label", "{tr.poll_section_title}" }
                            span { class: "saz-section__count", "data-testid": "polls-count",
                                strong { "{poll_count}" }
                                " {tr.questions}"
                            }
                        }
                    }

                    if poll_items.is_empty() {
                        div { class: "saz-empty", "data-testid": "polls-empty",
                            span { class: "saz-empty__icon",
                                svg {
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    "stroke-width": "1.8",
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    line {
                                        x1: "18",
                                        y1: "20",
                                        x2: "18",
                                        y2: "10",
                                    }
                                    line {
                                        x1: "12",
                                        y1: "20",
                                        x2: "12",
                                        y2: "4",
                                    }
                                    line {
                                        x1: "6",
                                        y1: "20",
                                        x2: "6",
                                        y2: "14",
                                    }
                                }
                            }
                            span { class: "saz-empty__title", "{tr.no_polls}" }
                        }
                    } else {
                        div { class: "saz-list", "data-testid": "polls-list",
                            for poll in poll_items.iter() {
                                if poll.questions_count > 0 {
                                    {
                                        let poll_id = poll.poll_id.clone();
                                        let title = poll.title.clone();
                                        let questions_count = poll.questions_count;
                                        let questions = tr.questions.to_string();
                                        rsx! {
                                            div {
                                                key: "{poll.poll_id}",
                                                class: "saz-card",
                                                "data-testid": "poll-card",
                                                div { class: "saz-card__icon saz-card__icon--poll",
                                                    svg {
                                                        view_box: "0 0 24 24",
                                                        fill: "none",
                                                        stroke: "currentColor",
                                                        "stroke-width": "1.8",
                                                        "stroke-linecap": "round",
                                                        "stroke-linejoin": "round",
                                                        line {
                                                            x1: "18",
                                                            y1: "20",
                                                            x2: "18",
                                                            y2: "10",
                                                        }
                                                        line {
                                                            x1: "12",
                                                            y1: "20",
                                                            x2: "12",
                                                            y2: "4",
                                                        }
                                                        line {
                                                            x1: "6",
                                                            y1: "20",
                                                            x2: "6",
                                                            y2: "14",
                                                        }
                                                    }
                                                }
                                                div { class: "saz-card__body",
                                                    div { class: "saz-card__meta",
                                                        span { class: "saz-card__type", "Poll" }
                                                        span { class: "saz-card__count saz-card__count--poll",
                                                            "{questions_count} {questions}"
                                                        }
                                                    }
                                                    span { class: "saz-card__title", "{title}" }
                                                }
                                                div { class: "saz-card__action",
                                                    button {
                                                        r#type: "button",
                                                        class: "saz-btn-accent",
                                                        "data-testid": "poll-view-btn",
                                                        onclick: move |_| {
                                                            nav.push(Route::SpaceAnalyzeDetailPage {
                                                                space_id: space_id(),
                                                                poll_id: poll_id.clone(),
                                                            });
                                                        },
                                                        "{tr.view_analyze}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if polls_has_more {
                            button {
                                r#type: "button",
                                class: "saz-load-more",
                                "data-testid": "polls-load-more",
                                disabled: polls_is_loading,
                                onclick: move |_| {
                                    polls.next();
                                },
                                "{tr.more}"
                            }
                        }
                    }
                }

                // Discussions section — only rendered in local-dev
                // builds (see the cfg-gated value above).
                {discussions_section}
            }
        }
    }
}

/// Local-dev-only section. Isolated into its own component so the hook
/// call that reads `discussions` is never compiled in production.
#[cfg(feature = "local-dev")]
#[component]
fn DiscussionsSection(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let nav = use_navigator();
    let UseSpaceAnalyzes {
        mut discussions, ..
    } = use_space_analyzes(space_id)?;

    let items = discussions.items();
    let count = items.len();
    let has_more = discussions.has_more();
    let is_loading = discussions.is_loading();

    rsx! {
        section { class: "saz-section", "data-testid": "section-discussions",
            div { class: "saz-section__head",
                div { class: "saz-section__title",
                    span { class: "saz-section__label", "{tr.discussion_section_title}" }
                    span { class: "saz-section__count", "data-testid": "discussions-count",
                        strong { "{count}" }
                        " {tr.discussion_section_title}"
                    }
                }
            }

            if items.is_empty() {
                div { class: "saz-empty", "data-testid": "discussions-empty",
                    span { class: "saz-empty__icon",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            "stroke-width": "1.8",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
                        }
                    }
                    span { class: "saz-empty__title", "{tr.no_discussions}" }
                }
            } else {
                div { class: "saz-list", "data-testid": "discussions-list",
                    for discussion in items.iter() {
                        {
                            let discussion_id = discussion.discussion_id.clone();
                            let title = if discussion.title.trim().is_empty() {
                                tr.untitled_discussion.to_string()
                            } else {
                                discussion.title.clone()
                            };
                            rsx! {
                                div {
                                    key: "{discussion.discussion_id}",
                                    class: "saz-card",
                                    "data-testid": "discussion-card",
                                    div { class: "saz-card__icon saz-card__icon--discussion",
                                        svg {
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "currentColor",
                                            "stroke-width": "1.8",
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
                                            path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
                                        }
                                    }
                                    div { class: "saz-card__body",
                                        div { class: "saz-card__meta",
                                            span { class: "saz-card__type", "Discussion" }
                                        }
                                        span { class: "saz-card__title", "{title}" }
                                    }
                                    div { class: "saz-card__action",
                                        button {
                                            r#type: "button",
                                            class: "saz-btn-accent",
                                            "data-testid": "discussion-view-btn",
                                            onclick: move |_| {
                                                nav.push(Route::SpaceAnalyzeDiscussionPage {
                                                    space_id: space_id(),
                                                    discussion_id: discussion_id.clone(),
                                                });
                                            },
                                            "{tr.view_analyze}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if has_more {
                    button {
                        r#type: "button",
                        class: "saz-load-more",
                        "data-testid": "discussions-load-more",
                        disabled: is_loading,
                        onclick: move |_| {
                            discussions.next();
                        },
                        "{tr.more}"
                    }
                }
            }
        }
    }
}

#[component]
fn ViewerEmpty() -> Element {
    rsx! {
        document::Link { rel: "preload", href: asset!("./style.css"), r#as: "style" }
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div { class: "space-analyzes-arena",
            div { class: "saz-viewer",
                span { class: "saz-viewer__title", "No access" }
            }
        }
    }
}
