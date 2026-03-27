use crate::common::utils::time::time_ago;
use crate::features::posts::components::{CreatePostButton, FeedContents, UserBadge};
use crate::features::posts::controllers::dto::*;
use crate::features::posts::controllers::list_user_drafts::list_user_drafts_handler;
use crate::features::posts::types::*;
use crate::features::timeline::*;
use dioxus_translate::use_language;

/// A horizontal row of the user's draft posts, displayed at the top of the timeline.
/// The header row (with "Drafts" heading and Create Post button) always renders so the
/// Create Post button is always accessible even while drafts are loading. The scrollable
/// draft cards only render when there are existing drafts.
///
/// NOTE: We intentionally avoid `use_server_future(...)?` here because the `?` operator
/// suspends the entire component while loading, hiding the Create Post button until the
/// server future resolves. Instead, we handle the `Option` state manually so the header
/// is always visible.
#[component]
pub fn DraftTimeline() -> Element {
    let drafts = use_server_future(move || async move {
        let result = list_user_drafts_handler(None).await;
        if let Err(ref e) = result {
            tracing::error!("Failed to load drafts: {:?}", e);
        }
        result
    });

    let items = match &*drafts.read() {
        Some(Ok(resp)) => resp.items.clone(),
        Some(Err(_)) => vec![],
        None => vec![], // Still loading
    };

    let has_drafts = !items.is_empty();
    let nav = use_navigator();
    let lang = use_language();
    let mut can_scroll_right = use_signal(|| false);

    rsx! {
        section { class: "flex flex-col gap-3 w-full", aria_label: "Drafts section",
            div { class: "flex justify-between items-center px-1 w-full",
                h2 { class: "flex-1 text-lg font-semibold text-text-primary", "Drafts" }
                CreatePostButton { class: "w-fit" }
            }

            if has_drafts {
                div { class: "relative",
                    div {
                        class: "flex overflow-x-auto gap-4 pb-2 snap-x snap-mandatory scrollbar-none",
                        onmounted: move |_| {
                            spawn(async move {
                                let mut eval = document::eval(r#"
                                    const el = document.querySelector('[aria-label="Drafts section"] .scrollbar-none');
                                    if (el) {
                                        dioxus.send(el.scrollLeft + el.clientWidth < el.scrollWidth - 1);
                                    } else {
                                        dioxus.send(false);
                                    }
                                "#);
                                if let Ok(val) = eval.recv::<bool>().await {
                                    can_scroll_right.set(val);
                                }
                            });
                        },
                        onscroll: move |_| {
                            spawn(async move {
                                let mut eval = document::eval(r#"
                                    const el = document.querySelector('[aria-label="Drafts section"] .scrollbar-none');
                                    if (el) {
                                        dioxus.send(el.scrollLeft + el.clientWidth < el.scrollWidth - 1);
                                    } else {
                                        dioxus.send(false);
                                    }
                                "#);
                                if let Ok(val) = eval.recv::<bool>().await {
                                    can_scroll_right.set(val);
                                }
                            });
                        },
                        for post in items {
                            {
                                let post_pk = post.pk.clone();
                                rsx! {
                                    div {
                                        key: "draft-{post.pk}",
                                        class: "flex flex-col gap-2.5 pt-5 pb-2.5 border cursor-pointer snap-start shrink-0 w-[340px] max-mobile:w-[280px] rounded-[10px] bg-card-bg-secondary border-card-enable-border",
                                        onclick: move |_| {
                                            let nav = nav.clone();
                                            let post_pk: FeedPartition = post_pk.clone().into();
                                            nav.push(format!("/posts/{post_pk}/edit"));
                                        },
                                        Col { class: "gap-1 px-5 w-full",
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
                                        div { class: "flex flex-row justify-between items-center px-5",
                                            UserBadge {
                                                profile_url: post.author_profile_url.clone(),
                                                name: post.author_display_name.clone(),
                                                author_type: post.author_type,
                                            }
                                            p { class: "text-sm font-light align-middle text-foreground-muted",
                                                "{time_ago(post.updated_at)}"
                                            }
                                        }
                                        div { class: "px-5 line-clamp-3",
                                            FeedContents {
                                                contents: post.html_contents.chars().take(200).collect::<String>(),
                                                urls: post.urls.clone(),
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if can_scroll_right() {
                        button {
                            class: "absolute right-0 top-1/2 p-1 rounded-full transition-colors -translate-y-1/2 cursor-pointer z-[101] hover:bg-accent/20",
                            onclick: move |_| {
                                let _ = document::eval(
                                    r#"
                                        const el = document.querySelector('[aria-label="Drafts section"] .scrollbar-none');
                                        if (el) el.scrollBy({ left: 340, behavior: 'smooth' });
                                    "#,
                                );
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
}
