use crate::components::FeedCard;
use crate::controllers::dto::*;
use crate::controllers::like_post::like_post_handler;
use crate::controllers::list_posts::list_posts_handler;
use crate::*;
use common::hooks::use_infinite_query;
use dioxus::prelude::*;

#[component]
pub fn FeedList() -> Element {
    let mut v = use_infinite_query(list_posts_handler)?;

    rsx! {
        div { class: "flex flex-col flex-1 max-mobile:px-[10px]",
            div { class: "flex flex-col flex-1 gap-2.5",
                for post in v.items() {
                    FeedCard {
                        key: "post-{post.pk}",
                        post: post.clone(),
                        on_like: move |value| {
                            let post_pk = post.pk.clone();
                            spawn(async move {
                                let _ = like_post_handler(post_pk, value).await;
                            });
                        },
                    }
                }

                if v.has_more() {
                    {v.more_element()}
                } else {
                    FeedEndMessage {}
                }
            }
        }
    }
}

#[component]
fn FeedEndMessage() -> Element {
    rsx! {
        div {
            class: "my-6 text-center text-gray-400",
            aria_label: "End of feed message",
            "You have reached the end of your feed."
        }
    }
}
