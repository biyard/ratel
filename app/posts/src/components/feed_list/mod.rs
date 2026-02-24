use crate::components::FeedCard;
use crate::controllers::dto::*;
use crate::controllers::list_posts::list_posts_handler;
use crate::*;
use common::hooks::use_infinite_query;
use dioxus::prelude::*;

#[component]
pub fn FeedList() -> Element {
    let mut v = use_infinite_query(move |bookmark| async move {
        debug!("FeedList: fetching posts with bookmark = {:?}", bookmark);
        let res = list_posts_handler(bookmark).await;
        debug!("FeedList: received response: {:?}", res);
        res
    })?;

    let items: Vec<PostResponse> = v.items();
    let has_more = v.has_more();

    rsx! {
        div { class: "flex flex-col flex-1 max-mobile:px-[10px]",
            div { class: "flex flex-col flex-1 gap-2.5",
                for (i , post) in items.into_iter().enumerate() {
                    FeedCard { key: "post-{i}-{post.pk}", post: post.clone() }
                }

                if has_more {
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
