use crate::features::posts::components::FeedCard;
use crate::features::posts::controllers::dto::*;
use crate::features::posts::controllers::list_posts::list_posts_handler;
use crate::features::posts::*;
use common::hooks::use_infinite_query;
use dioxus::prelude::*;

#[component]
pub fn FeedList() -> Element {
    let mut v = use_infinite_query(list_posts_handler)?;

    rsx! {
        div { class: "flex flex-col flex-1 max-mobile:px-[10px]",
            div { class: "flex flex-col flex-1 gap-2.5",
                for post in v.items() {
                    FeedCard { key: "post-{post.pk}", post: post.clone() }
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
