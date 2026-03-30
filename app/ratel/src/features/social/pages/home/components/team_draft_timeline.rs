use crate::common::utils::time::time_ago;
use crate::common::*;
use crate::features::posts::components::FeedContents;
use crate::features::posts::controllers::dto::*;
use crate::features::posts::controllers::list_user_drafts::list_team_drafts_handler;
use crate::features::posts::types::*;

translate! {
    TeamDraftTimelineTranslate;

    drafts: {
        en: "Drafts",
        ko: "초안",
    },
}

/// A horizontal row of the team's draft posts, displayed at the top of the team home page.
///
/// Fetches team drafts and renders the entire section only when drafts exist.
/// When there are no drafts, returns nothing.
#[component]
pub fn TeamDraftTimeline(username: String) -> Element {
    let tr: TeamDraftTimelineTranslate = use_translate();
    let teamname_signal = use_signal(|| username);

    let drafts = use_server_future(move || {
        let teamname = teamname_signal();
        async move {
            let result = list_team_drafts_handler(teamname, None).await;
            if let Err(ref e) = result {
                tracing::error!("Failed to load team drafts: {:?}", e);
            }
            result
        }
    })?;

    let val = drafts.read();
    let res = val.as_ref().unwrap();

    let items = match res {
        Ok(resp) => resp.items.clone(),
        Err(_) => vec![],
    };

    if items.is_empty() {
        return rsx! {};
    }

    let nav = use_navigator();
    let lang = use_language();
    let mut can_scroll_left = use_signal(|| false);
    let mut can_scroll_right = use_signal(|| false);

    rsx! {
        section {
            class: "flex flex-col gap-3 w-full",
            aria_label: "Team Drafts section",
            "data-testid": "team-draft-timeline",
            h2 { class: "px-1 text-lg font-semibold text-text-primary", "{tr.drafts}" }
            div { class: "relative",
                div {
                    class: "flex overflow-x-auto gap-4 pb-2 snap-x snap-mandatory scrollbar-none",
                    onmounted: move |_| {
                        spawn(async move {
                            let mut eval_result = document::eval(TEAM_DRAFT_CHECK_SCROLL_JS);
                            if let Ok(val) = eval_result.recv::<Vec<bool>>().await {
                                can_scroll_left.set(val[0]);
                                can_scroll_right.set(val[1]);
                            }
                        });
                    },
                    onscroll: move |_| {
                        spawn(async move {
                            let mut eval_result = document::eval(TEAM_DRAFT_CHECK_SCROLL_JS);
                            if let Ok(val) = eval_result.recv::<Vec<bool>>().await {
                                can_scroll_left.set(val[0]);
                                can_scroll_right.set(val[1]);
                            }
                        });
                    },
                    for post in items {
                        {
                            let post_pk = post.pk.clone();
                            rsx! {
                                div {
                                    key: "team-draft-{post.pk}",
                                    class: "flex flex-col pt-5 pb-5 border cursor-pointer snap-start shrink-0 w-[340px] max-mobile:w-[280px] rounded-[10px] bg-card-bg-secondary border-card-enable-border",
                                    onclick: move |_| {
                                        let nav = nav.clone();
                                        let post_pk: FeedPartition = post_pk.clone().into();
                                        nav.push(format!("/posts/{post_pk}/edit"));
                                    },
                                    Col { class: "gap-2 px-5 w-full",
                                        Badge {
                                            size: BadgeSize::Small,
                                            variant: BadgeVariant::Rounded,
                                            color: BadgeColor::Orange,
                                            {post.status.translate(&lang())}
                                        }
                                        p { class: "w-full text-base font-normal text-text-primary truncate line-clamp-1",
                                            "{post.title}"
                                        }
                                    }
                                    FeedContents {
                                        contents: post.html_contents.chars().take(200).collect::<String>(),
                                        urls: post.urls.clone(),
                                    }
                                    div { class: "flex flex-row items-center px-5 mt-4",
                                        p { class: "text-sm font-light align-middle text-foreground-muted",
                                            "{time_ago(post.updated_at)}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if can_scroll_left() {
                    button {
                        class: "absolute left-0 top-1/2 p-1 rounded-full transition-colors -translate-y-1/2 cursor-pointer z-[101] hover:bg-accent/20",
                        aria_label: "Scroll Team Drafts left",
                        onclick: move |_| {
                            let _ = document::eval(TEAM_DRAFT_SCROLL_LEFT_JS);
                        },
                        lucide_dioxus::ChevronLeft {
                            size: 20,
                            class: "transition-colors [&>path]:stroke-foreground-muted hover:[&>path]:stroke-text-primary",
                        }
                    }
                }
                if can_scroll_right() {
                    button {
                        class: "absolute right-0 top-1/2 p-1 rounded-full transition-colors -translate-y-1/2 cursor-pointer z-[101] hover:bg-accent/20",
                        aria_label: "Scroll Team Drafts right",
                        onclick: move |_| {
                            let _ = document::eval(TEAM_DRAFT_SCROLL_RIGHT_JS);
                        },
                        lucide_dioxus::ChevronRight {
                            size: 20,
                            class: "transition-colors [&>path]:stroke-foreground-muted hover:[&>path]:stroke-text-primary",
                        }
                    }
                }
            }
        }
    }
}

const TEAM_DRAFT_CHECK_SCROLL_JS: &str = r#"
    const el = document.querySelector('[aria-label="Team Drafts section"] .scrollbar-none');
    if (el) {
        dioxus.send([el.scrollLeft > 0, el.scrollLeft + el.clientWidth < el.scrollWidth - 1]);
    } else {
        dioxus.send([false, false]);
    }
"#;

const TEAM_DRAFT_SCROLL_LEFT_JS: &str = r#"
    const el = document.querySelector('[aria-label="Team Drafts section"] .scrollbar-none');
    if (el) el.scrollBy({ left: -340, behavior: 'smooth' });
"#;

const TEAM_DRAFT_SCROLL_RIGHT_JS: &str = r#"
    const el = document.querySelector('[aria-label="Team Drafts section"] .scrollbar-none');
    if (el) el.scrollBy({ left: 340, behavior: 'smooth' });
"#;
