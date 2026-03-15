use crate::features::posts::components::FeedCard;
use crate::features::posts::controllers::dto::PostResponse;
use crate::features::timeline::controllers::dto::TimelineCategoryRow;
use crate::features::timeline::controllers::list_timeline::list_timeline_handler;
use crate::features::timeline::*;

fn category_display_name(category: &str) -> &'static str {
    match category {
        "following" => "Following",
        "team_member" => "From Your Teams",
        "popular" => "Popular",
        _ => "Posts",
    }
}

/// A single horizontal row of posts for a timeline category (Netflix-style).
#[component]
pub fn TimelineRow(row: TimelineCategoryRow) -> Element {
    let category = row.category.clone();
    let display_name = category_display_name(&category);
    let mut posts = use_signal(move || row.items.clone());
    let mut bookmark = use_signal(move || row.bookmark.clone());
    let mut has_more = use_signal(move || row.has_more);
    let mut loading = use_signal(|| false);

    let items = posts.read().clone();

    if items.is_empty() {
        return rsx! {};
    }

    rsx! {
        section {
            class: "flex flex-col gap-3 w-full",
            aria_label: "{display_name} section",

            // Section header
            div { class: "flex items-center justify-between px-1",
                h2 { class: "text-lg font-semibold text-text-primary",
                    {display_name}
                }
                if *has_more.read() {
                    button {
                        class: "flex items-center gap-1 text-sm text-primary hover:underline cursor-pointer",
                        disabled: *loading.read(),
                        onclick: {
                            let category = category.clone();
                            move |_| {
                                let category = category.clone();
                                let bk = bookmark.read().clone();
                                loading.set(true);
                                spawn(async move {
                                    match list_timeline_handler(category, bk).await {
                                        Ok(more_row) => {
                                            posts.extend(more_row.items);
                                            bookmark.set(more_row.bookmark);
                                            has_more.set(more_row.has_more);
                                        }
                                        Err(e) => {
                                            tracing::error!("Failed to load more: {:?}", e);
                                        }
                                    }
                                    loading.set(false);
                                });
                            }
                        },
                        "See more"
                        crate::common::lucide_dioxus::ChevronRight { size: 16 }
                    }
                }
            }

            // Horizontal scroll of post cards
            div { class: "flex overflow-x-auto gap-4 pb-2 snap-x snap-mandatory scrollbar-thin",
                for post in items {
                    div {
                        class: "snap-start shrink-0 w-[340px] max-mobile:w-[280px]",
                        key: "tl-{post.pk}",
                        FeedCard { post: post.clone() }
                    }
                }
                if *loading.read() {
                    div { class: "snap-start shrink-0 w-[340px] flex items-center justify-center text-text-secondary",
                        "Loading..."
                    }
                }
            }
        }
    }
}
