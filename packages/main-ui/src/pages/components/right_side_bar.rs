use bdk::prelude::*;
use dto::{NewsSummary, Promotion, User};
use html2text::from_read;

use crate::{components::follow::Follow, utils::text::insert_word_breaks};

#[component]
pub fn RightSidebar(
    lang: Language,
    promotion: Promotion,
    news: Vec<NewsSummary>,
    followers: Vec<User>,

    follow: EventHandler<i64>,
) -> Element {
    let tr: RightSidebarTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full max-w-280 h-full justify-start items-start gap-10 max-tablet:!hidden",
            RoundedSection {
                lang,
                header: tr.hot_promotion,
                onclick: move |_| {
                    tracing::debug!("hot promotion view all clicked");
                },
                HotPromotion {
                    image: promotion.image_url.clone(),
                    title: promotion.name.clone(),
                    description: promotion.description.clone(),
                }
            }
            RoundedSection {
                lang,
                header: tr.news,
                onclick: move |_| {
                    tracing::debug!("news view all clicked");
                },
                div { class: "flex flex-col w-full justify-start items-start gap-15",
                    for (idx , ns) in news.iter().enumerate() {
                        NewsTopic {
                            title: ns.title.clone(),
                            description: ns.html_content.clone(),
                        }
                        if idx < news.len() - 1 {
                            div { class: "flex flex-row w-full h-1 justify-start items-start bg-neutral-800" }
                        }
                    }
                }
            }
                // ViewAllSection {
        //     lang,
        //     header: tr.add_feed,
        //     onclick: move |_| {
        //         tracing::debug!("feed view all clicked");
        //     },
        //     div { class: "flex flex-col w-full justify-start items-start gap-35",
        //         for follower in followers.iter().take(3) {
        //             FeedUser {
        //                 lang,
        //                 id: follower.id.clone(),
        //                 profile: follower.profile_url.clone(),
        //                 name: follower.nickname.clone(),
        //                 description: "".to_string(),
        //                 follow: move |id: i64| {
        //                     follow.call(id);
        //                 },
        //             }
        //         }
        //     }
        // }
        }
    }
}

#[component]
pub fn FeedUser(
    lang: Language,
    id: i64,
    profile: String,
    name: String,
    description: String,
    follow: EventHandler<i64>,
) -> Element {
    rsx! {
        div { class: "flex flex-row w-full justify-start items-start gap-10",
            img { class: "w-50 h-50 rounded-full object-cover", src: profile }
            div { class: "flex flex-col w-full justify-start items-start gap-10",
                div { class: "flex flex-col w-full justify-start items-start",
                    div { class: "font-medium text-white text-base/25 line-clamp-1 text-start",
                        {name}
                    }
                    div { class: "font-light text-white text-sm/16 line-clamp-1 text-start",
                        {description}
                    }
                }

                Follow {
                    lang,
                    onclick: move |_| {
                        tracing::debug!("follow button clicked");
                        follow.call(id);
                    },
                }
            }
        }
    }
}

#[component]
pub fn NewsTopic(title: String, description: String) -> Element {
    let plain_text = from_read(description.as_bytes(), 140).replace('\n', " ");
    let broken_text = insert_word_breaks(&plain_text);
    rsx! {
        div { class: "cursor-pointer flex flex-col w-full justify-start items-start gap-4",
            div { class: "font-medium text-white text-base/25 text-start line-clamp-1",
                {title}
            }
            div { class: "font-light text-white text-sm/20 w-full text-start line-clamp-2 text-ellipsis",
                {broken_text}
            }
        }
    }
}

#[component]
pub fn HotPromotion(image: String, title: String, description: String) -> Element {
    rsx! {
        div { class: "cursor-pointer flex flex-row w-full justify-start items-center gap-10",
            img { class: "w-60 h-60 rounded-[4px] object-cover", src: image }
            div { class: "flex flex-col w-full justify-start items-start gap-4",
                div { class: "flex flex-col w-full justify-start items-start gap-4",

                    div { class: "font-medium text-base/25 text-white line-clamp-1 text-start",
                        {title}
                    }

                    div { class: "font-light text-sm/20 text-white line-clamp-2 text-start",
                        {description}
                    }
                }
            }
        }
    }
}

#[component]
pub fn RoundedSection(
    lang: Language,
    header: String,
    onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    // let tr: ViewAllSectionTranslate = translate(&lang);

    rsx! {
        div { class: "w-full flex flex-col px-16 py-20 rounded-[10px] bg-footer justify-start items-start gap-10",
            div { class: "font-bold text-white text-[15px]/20", {header} }
            div { class: "flex flex-col w-full justify-start items-start gap-20", {children} }
        }
    }
}

translate! {
    RightSidebarTranslate;

    hot_promotion: {
        ko: "Hot Promotion",
        en: "Hot Promotion"
    },
    news: {
        ko: "News",
        en: "News"
    },
    add_feed: {
        ko: "Add to your feed",
        en: "Add to your feed"
    }
}

translate! {
    ViewAllSectionTranslate;

    view_all: {
        ko: "View all",
        en: "View all"
    }
}
