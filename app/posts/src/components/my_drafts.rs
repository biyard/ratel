use crate::components::FeedCard;
use crate::controllers::dto::*;
use crate::controllers::list_user_drafts::list_user_drafts_handler;
use crate::*;
use dioxus::prelude::*;

#[component]
pub fn MyDrafts() -> Element {
    let resource = use_server_future(move || async move { list_user_drafts_handler(None).await })?;

    let resolved = resource.suspend()?;
    let data = resolved.read();
    let (items, has_next): (Vec<PostResponse>, bool) = match data.as_ref() {
        Ok(data) => {
            let has_next = data.bookmark.is_some();
            (data.items.clone(), has_next)
        }
        Err(_) => (vec![], false),
    };

    if items.is_empty() {
        return rsx! {
            div { class: "flex flex-row justify-start items-center w-full text-base font-medium text-gray-500 border border-gray-500 h-fit px-[16px] py-[20px] rounded-[8px]",
                "No drafts available"
            }
        };
    }

    rsx! {
        div { class: "flex flex-col flex-1 max-mobile:px-[10px]",
            div { class: "flex flex-col flex-1 gap-4",
                for post in items {
                    FeedCard { key: "{post.pk}", post: post.clone() }
                }

                if !has_next {
                    div {
                        class: "my-6 text-center text-gray-400",
                        aria_label: "End of feed message",
                        "You have reached the end of your drafts."
                    }
                }
            }
        }
    }
}
