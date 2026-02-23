use crate::components::FeedCard;
use crate::controllers::list_user_posts::{list_team_posts_handler, list_user_posts_handler};
use crate::controllers::{dto::*, ListTeamPostsQueryParams};
use crate::*;
use dioxus::prelude::*;

// FIXME: Use GET when dioxus server functions support query params without body.
#[component]
pub fn MyPosts(username: String) -> Element {
    let resource = use_server_future(move || {
        let username = username.clone();
        async move { list_user_posts_handler(username, None).await }
    })?;

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
                "No posts available"
            }
        };
    }

    rsx! {
        div { class: "flex flex-col flex-1 max-mobile:px-[10px]",
            div { class: "flex flex-col flex-1",
                for post in items {
                    FeedCard { key: "{post.pk}", post: post.clone() }
                }

                if !has_next {
                    div {
                        class: "my-6 text-center text-gray-400",
                        aria_label: "End of feed message",
                        "You have reached the end of your feed."
                    }
                }
            }
        }
    }
}

#[component]
pub fn TeamPosts(teamname: String) -> Element {
    let resource = use_server_future(move || {
        let teamname = teamname.clone();
        async move {
            list_team_posts_handler(ListTeamPostsQueryParams {
                teamname,
                bookmark: None,
            })
            .await
        }
    })?;

    let resolved = resource.suspend()?;
    let data = resolved.read();
    let (items, has_next, error): (Vec<PostResponse>, bool, Option<String>) = match data.as_ref() {
        Ok(data) => {
            let has_next = data.bookmark.is_some();
            (data.items.clone(), has_next, None)
        }
        Err(e) => (vec![], false, Some(e.to_string())),
    };

    if let Some(error) = error {
        return rsx! {
            div { class: "flex flex-row justify-start items-center w-full text-base font-medium text-red-400 border border-red-400 h-fit px-[16px] py-[20px] rounded-[8px]",
                "Failed to load posts: {error}"
            }
        };
    }

    if items.is_empty() {
        return rsx! {
            div { class: "flex flex-row justify-start items-center w-full text-base font-medium text-gray-500 border border-gray-500 h-fit px-[16px] py-[20px] rounded-[8px]",
                "No posts available"
            }
        };
    }

    rsx! {
        div { class: "flex flex-col flex-1 max-mobile:px-[10px]",
            div { class: "flex flex-col flex-1",
                for post in items {
                    FeedCard { key: "{post.pk}", post: post.clone() }
                }

                if !has_next {
                    div {
                        class: "my-6 text-center text-gray-400",
                        aria_label: "End of feed message",
                        "You have reached the end of your feed."
                    }
                }
            }
        }
    }
}
