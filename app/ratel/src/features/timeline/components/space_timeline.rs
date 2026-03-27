use crate::common::hooks::use_infinite_query;
use crate::features::auth::UserType;
use crate::features::posts::components::FeedCard;
use crate::features::posts::controllers::dto::PostResponse;
use crate::features::posts::types::{BoosterType, PostStatus};
use crate::features::social::pages::space::controllers::list_my_spaces::{
    list_my_spaces_handler, MySpaceResponse,
};
use crate::features::timeline::*;

/// A horizontal row of spaces the user is participating in, displayed on the home timeline.
#[component]
pub fn SpaceTimeline() -> Element {
    let mut v = use_infinite_query(move |bookmark| async move {
        list_my_spaces_handler(bookmark, Some(true)).await
    })?;

    let items = v.items();

    if items.is_empty() {
        return rsx! {};
    }

    let mut can_scroll_left = use_signal(|| false);
    let mut can_scroll_right = use_signal(|| false);
    let mut scroll_check_pending = use_signal(|| false);
    let mut scroll_dirty = use_signal(|| false);

    use_effect(move || {
        let _len = v.items().len();
        spawn(async move {
            let mut result = document::eval(CHECK_SCROLL_JS);
            if let Ok(val) = result.recv::<Vec<bool>>().await {
                can_scroll_left.set(val[0]);
                can_scroll_right.set(val[1]);
            }
        });
    });

    rsx! {
        section {
            class: "flex flex-col gap-3 w-full",
            aria_label: "My Spaces section",

            div { class: "flex justify-between items-center px-1 w-full",
                h2 { class: "flex-1 text-lg font-semibold text-text-primary", "My Spaces" }
            }

            div { class: "relative",
                div {
                    class: "flex overflow-x-auto gap-4 pb-2 snap-x snap-mandatory scrollbar-none",
                    onmounted: move |_| {
                        spawn(async move {
                            let mut result = document::eval(CHECK_SCROLL_JS);
                            if let Ok(val) = result.recv::<Vec<bool>>().await {
                                can_scroll_left.set(val[0]);
                                can_scroll_right.set(val[1]);
                            }
                        });
                    },
                    onscroll: move |_| {
                        if scroll_check_pending() {
                            scroll_dirty.set(true);
                            return;
                        }
                        scroll_check_pending.set(true);
                        spawn(async move {
                            loop {
                                scroll_dirty.set(false);
                                let mut result = document::eval(CHECK_SCROLL_JS);
                                if let Ok(val) = result.recv::<Vec<bool>>().await {
                                    can_scroll_left.set(val[0]);
                                    can_scroll_right.set(val[1]);
                                }
                                if !scroll_dirty() {
                                    break;
                                }
                            }
                            scroll_check_pending.set(false);
                        });
                    },
                    for space in items {
                        {
                            let post = space_to_post(space.clone());
                            let space_id: SpacePartition = space.space_pk.clone().into();
                            let href = format!("/spaces/{}", space_id);
                            rsx! {
                                div {
                                    class: "snap-start shrink-0 w-[340px] max-mobile:w-[280px]",
                                    key: "space-{space.space_pk}",
                                    FeedCard { post, href }
                                }
                            }
                        }
                    }
                    {v.more_element()}
                }
                if can_scroll_left() {
                    button {
                        class: "absolute left-0 top-1/2 p-1 rounded-full transition-colors -translate-y-1/2 cursor-pointer z-[101] hover:bg-accent/20",
                        aria_label: "Scroll My Spaces left",
                        onclick: move |_| {
                            let _ = document::eval(SCROLL_LEFT_JS);
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
                        aria_label: "Scroll My Spaces right",
                        onclick: move |_| {
                            let _ = document::eval(SCROLL_RIGHT_JS);
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

fn space_to_post(space: MySpaceResponse) -> PostResponse {
    PostResponse {
        pk: space.post_pk.clone().into(),
        created_at: space.created_at,
        updated_at: space.created_at,
        title: space.title,
        html_contents: space.description,
        shares: 0,
        likes: 0,
        comments: 0,
        author_display_name: space.author_display_name,
        author_profile_url: space.author_profile_url,
        author_username: String::new(),
        auth_pk: space.space_pk.clone(),
        author_type: UserType::Individual,
        space_pk: Some(space.space_pk),
        space_type: None,
        booster: BoosterType::NoBoost,
        rewards: None,
        urls: vec![],
        liked: false,
        categories: vec![],
        status: PostStatus::Published,
    }
}

const CHECK_SCROLL_JS: &str = r#"
    const el = document.querySelector('[aria-label="My Spaces section"] .scrollbar-none');
    if (el) {
        dioxus.send([el.scrollLeft > 0, el.scrollLeft + el.clientWidth < el.scrollWidth - 1]);
    } else {
        dioxus.send([false, false]);
    }
"#;

const SCROLL_LEFT_JS: &str = r#"
    const el = document.querySelector('[aria-label="My Spaces section"] .scrollbar-none');
    if (el) el.scrollBy({ left: -340, behavior: 'smooth' });
"#;

const SCROLL_RIGHT_JS: &str = r#"
    const el = document.querySelector('[aria-label="My Spaces section"] .scrollbar-none');
    if (el) el.scrollBy({ left: 340, behavior: 'smooth' });
"#;
