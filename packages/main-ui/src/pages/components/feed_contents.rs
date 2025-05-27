use bdk::prelude::{by_components::icons::edit::Edit1, *};
use dto::{FeedSummary, SpaceStatus};
use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, window};

use crate::pages::components::FeedContent;

#[derive(Clone, PartialEq, Translate)]
pub enum Tab {
    #[translate(ko = "For you", en = "For you")]
    Me,
    #[translate(ko = "Following", en = "Following")]
    Following,
}

#[component]
pub fn FeedContents(
    lang: Language,
    profile: String,
    my_user_id: i64,
    feeds: Vec<FeedSummary>,
    following_feeds: Vec<FeedSummary>,

    is_write: bool,
    onwrite: EventHandler<MouseEvent>,
    onclick: EventHandler<i64>,
) -> Element {
    let mut selected_tab = use_signal(|| Tab::Me);

    rsx! {
        div { class: "flex flex-col w-full h-full justify-start items-start text-white",
            FeedTab {
                lang,
                selected_tab: selected_tab(),
                onchange: move |tab| {
                    selected_tab.set(tab);
                },
            }

            div { class: "flex flex-row w-full justify-start items-start max-tablet:hidden",
                CreateFeed { lang, profile, onwrite }
            }

            div { class: "flex flex-col w-full h-[calc(100vh-250px)] max-tablet:!h-full overflow-y-scroll",

                if selected_tab() == Tab::Me {
                    MyFeedList {
                        lang,
                        my_user_id,
                        feeds,
                        onclick,
                        create_space: move |_| {},
                        add_size: move |_| {},
                    }
                } else {
                    FollowingFeedList {
                        lang,
                        my_user_id,
                        following_feeds,
                        onclick,
                    }
                }
            }

            a {
                class: "cursor-pointer fixed bottom-160 right-16 w-fit h-fit hidden max-tablet:flex rounded-full bg-primary p-15 aria-hidden:!hidden",
                href: "#create_feed",
                onclick: move |e| {
                    onwrite.call(e);
                },
                aria_hidden: is_write,
                Edit1 {
                    class: "[&>path]:stroke-neutral-900",
                    width: "30",
                    height: "30",
                }
            }
        }
    }
}

#[component]
pub fn CreateFeed(lang: Language, profile: String, onwrite: EventHandler<MouseEvent>) -> Element {
    let tr: CreateFeedTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-row w-full justify-start items-center bg-bg p-20 rounded-lg gap-10 mb-10",
            img { class: "w-36 h-36 rounded-full object-cover", src: profile }
            a {
                class: "flex flex-row w-full h-fit justify-start items-center bg-neutral-800 border border-neutral-700 rounded-[100px] font-normal text-text-secondary text-sm/16 px-15 py-10",
                onclick: move |e| {
                    onwrite.call(e);
                },
                href: "#create_feed",
                {tr.desc}
            }
        }
    }
}

#[component]
pub fn MyFeedList(
    lang: Language,
    my_user_id: i64,
    feeds: Vec<FeedSummary>,
    create_space: EventHandler<i64>,
    add_size: EventHandler<usize>,
    onclick: EventHandler<i64>,
) -> Element {
    let mut visible_count = use_signal(|| 10);
    let mut listener = use_signal(|| None as Option<EventListener>);

    let feed_container_id = "feed-scroll-container";

    use_effect({
        move || {
            if let Some(container) = window()
                .and_then(|w| w.document())
                .and_then(|d| d.get_element_by_id(feed_container_id))
                .and_then(|el| el.dyn_into::<HtmlElement>().ok())
            {
                let new_listener = EventListener::new(&container, "scroll", {
                    let container = container.clone();
                    move |_event| {
                        let scroll_top = container.scroll_top();
                        let scroll_height = container.scroll_height();
                        let client_height = container.client_height();

                        if scroll_top + client_height as i32 >= scroll_height as i32 - 5 {
                            add_size.call(visible_count() + 5);
                            visible_count.set(visible_count() + 5);
                            tracing::debug!("visible count: {}", visible_count());
                        }
                    }
                });

                listener.set(Some(new_listener));
            }
        }
    });

    let visible_feeds = feeds
        .iter()
        .take(visible_count())
        .cloned()
        .collect::<Vec<_>>();

    rsx! {
        div {
            id: feed_container_id,
            class: "flex flex-col w-full h-[calc(100vh-300px)] max-tablet:!h-[calc(100vh-300px)]  overflow-y-scroll gap-10",
            for feed in visible_feeds {
                if feed.spaces.is_empty() || feed.spaces[0].status != SpaceStatus::Draft
                    || (feed.spaces[0].user_id == my_user_id
                        && feed.spaces[0].status == SpaceStatus::Draft)
                {
                    FeedContent {
                        lang,
                        feed: feed.clone(),
                        is_creator: my_user_id == feed.user_id,
                        exist_spaces: !feed.spaces.is_empty(),
                        onclick,
                        create_space: move |_| {
                            create_space.call(feed.id);
                        },
                    }
                }
            }
        }
    }
}

#[component]
pub fn FollowingFeedList(
    lang: Language,
    my_user_id: i64,
    following_feeds: Vec<FeedSummary>,
    onclick: EventHandler<i64>,
) -> Element {
    let mut visible_count = use_signal(|| 10);
    let mut listener = use_signal(|| None as Option<EventListener>);
    let container_id = "following-scroll-container";

    use_effect({
        move || {
            if let Some(container) = window()
                .and_then(|w| w.document())
                .and_then(|d| d.get_element_by_id(container_id))
                .and_then(|el| el.dyn_into::<HtmlElement>().ok())
            {
                let event_listener = EventListener::new(&container, "scroll", {
                    let container = container.clone();
                    move |_event| {
                        let scroll_top = container.scroll_top();
                        let scroll_height = container.scroll_height();
                        let client_height = container.client_height();

                        if scroll_top + client_height as i32 >= scroll_height as i32 - 5 {
                            visible_count.set(visible_count() + 5);
                            tracing::debug!("Following visible count: {}", visible_count());
                        }
                    }
                });

                listener.set(Some(event_listener));
            }
        }
    });

    let visible_items = following_feeds
        .iter()
        .take(visible_count())
        .cloned()
        .collect::<Vec<_>>();

    rsx! {
        div {
            id: container_id,
            class: "flex flex-col w-full h-[calc(100vh-300px)] overflow-y-scroll gap-10",
            for feed in visible_items {
                FeedContent {
                    lang,
                    feed: feed.clone(),
                    is_creator: my_user_id == feed.user_id,
                    exist_spaces: !feed.spaces.is_empty(),
                    onclick,
                    create_space: move |_| {},
                }
            }
        }
    }
}

#[component]
pub fn FeedTab(lang: Language, selected_tab: Tab, onchange: EventHandler<Tab>) -> Element {
    let tabs = [Tab::Me, Tab::Following];

    rsx! {
        div { class: "flex flex-row w-full",
            for tab in tabs {
                div {
                    class: "flex flex-col flex-1 items-center cursor-pointer py-4",
                    onclick: {
                        let tab = tab.clone();
                        move |_| {
                            onchange.call(tab.clone());
                        }
                    },

                    div {
                        class: "font-bold text-sm/20 aria-selected:text-white text-neutral-400 h-25",
                        "aria-selected": selected_tab == tab,
                        {tab.translate(&lang)}
                    }
                    if selected_tab == tab {
                        div { class: "w-29 h-2 mt-1 rounded-full bg-yellow-400" }
                    } else {
                        div { class: "h-2 mt-1" }
                    }
                }
            }
        }
    }
}

translate! {
    CreateFeedTranslate;

    desc: {
        ko: "Discuss legislation. Drive change.",
        en: "Discuss legislation. Drive change."
    }
}
