use crate::common::hooks::use_infinite_query;
use crate::common::utils::time::time_ago;
use crate::features::social::pages::space::controllers::list_my_spaces::list_my_spaces_handler;
use crate::features::timeline::*;
use dioxus_translate::use_language;

/// A horizontal row of spaces the user is participating in, displayed on the home timeline.
#[component]
pub fn SpaceTimeline() -> Element {
    let mut v = use_infinite_query(move |bookmark| async move {
        list_my_spaces_handler(bookmark).await
    })?;

    let items = v.items();

    if items.is_empty() {
        return rsx! {};
    }

    let lang = use_language();
    let mut can_scroll_right = use_signal(|| false);
    let mut scroll_check_pending = use_signal(|| false);
    let mut scroll_dirty = use_signal(|| false);

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
                            if let Ok(val) = result.recv::<bool>().await {
                                can_scroll_right.set(val);
                            }
                        });
                    },
                    onscroll: move |_| {
                        // Trailing-edge throttle: if a scroll check is already in-flight,
                        // set a dirty flag so one final check runs after it completes.
                        if scroll_check_pending() {
                            scroll_dirty.set(true);
                            return;
                        }
                        scroll_check_pending.set(true);
                        spawn(async move {
                            loop {
                                scroll_dirty.set(false);
                                let mut result = document::eval(CHECK_SCROLL_JS);
                                if let Ok(val) = result.recv::<bool>().await {
                                    can_scroll_right.set(val);
                                }
                                // If more scroll events arrived while we were checking,
                                // run one more check to capture the final scroll position.
                                if !scroll_dirty() {
                                    break;
                                }
                            }
                            scroll_check_pending.set(false);
                        });
                    },
                    for space in items {
                        {
                            let space_id: SpacePartition = space.space_pk.clone().into();
                            let href = format!("/spaces/{}", space_id);
                            let status = space.status.clone();
                            rsx! {
                                Link {
                                    key: "space-{space.space_pk}",
                                    class: "flex flex-col gap-2.5 pt-5 pb-2.5 border cursor-pointer snap-start shrink-0 w-[340px] max-mobile:w-[280px] rounded-[10px] bg-card-bg-secondary border-card-enable-border hover:bg-card-bg transition-colors no-underline",
                                    to: "{href}",
                                    Col { class: "gap-1 px-5 w-full",
                                        if let Some(ref st) = status {
                                            Badge {
                                                size: BadgeSize::Small,
                                                variant: BadgeVariant::Rounded,
                                                color: match st {
                                                    SpaceStatus::InProgress => BadgeColor::Blue,
                                                    SpaceStatus::Started => BadgeColor::Green,
                                                    _ => BadgeColor::Grey,
                                                },
                                                {st.translate(&lang())}
                                            }
                                        }
                                        p { class: "w-full text-base font-semibold text-text-primary truncate line-clamp-1",
                                            "{space.title}"
                                        }
                                    }
                                    div { class: "flex flex-row justify-between items-center px-5",
                                        div { class: "flex flex-row items-center gap-2",
                                            if !space.author_profile_url.is_empty() {
                                                img {
                                                    class: "w-5 h-5 rounded-full object-cover",
                                                    src: "{space.author_profile_url}",
                                                    alt: "{space.author_display_name}",
                                                }
                                            }
                                            p { class: "text-sm text-foreground-muted",
                                                "{space.author_display_name}"
                                            }
                                        }
                                        p { class: "text-sm font-light text-foreground-muted",
                                            "{time_ago(space.created_at)}"
                                        }
                                    }
                                    div { class: "flex flex-row items-center gap-2 px-5",
                                        lucide_dioxus::Users { size: 14, class: "[&>path]:stroke-foreground-muted" }
                                        p { class: "text-xs text-foreground-muted",
                                            "{space.participants}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    {v.more_element()}
                }
                if can_scroll_right() {
                    button {
                        class: "absolute right-0 top-1/2 p-1 rounded-full transition-colors -translate-y-1/2 cursor-pointer z-101 hover:bg-accent/20",
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

const CHECK_SCROLL_JS: &str = r#"
    const el = document.querySelector('[aria-label="My Spaces section"] .scrollbar-none');
    if (el) {
        dioxus.send(el.scrollLeft + el.clientWidth < el.scrollWidth - 1);
    } else {
        dioxus.send(false);
    }
"#;

const SCROLL_RIGHT_JS: &str = r#"
    const el = document.querySelector('[aria-label="My Spaces section"] .scrollbar-none');
    if (el) el.scrollBy({ left: 340, behavior: 'smooth' });
"#;
