use crate::features::posts::components::FeedCard;
use crate::features::posts::controllers::dto::*;
use crate::features::posts::controllers::list_user_posts::{list_team_posts_handler, list_user_posts_handler};
use crate::features::posts::*;
use crate::common::hooks::use_infinite_query;
use dioxus::prelude::*;

#[component]
pub fn MyPosts(username: String) -> Element {
    let username_signal = use_signal(|| username);
    let mut v = use_infinite_query(move |bookmark| {
        let username = username_signal();
        async move { list_user_posts_handler(username, bookmark).await }
    })?;

    let items = v.items();

    if items.is_empty() {
        return rsx! {
            div { class: "flex flex-row justify-start items-center w-full text-base font-medium text-gray-500 border border-gray-500 h-fit px-[16px] py-[20px] rounded-[8px]",
                "No posts available"
            }
        };
    }

    rsx! {
        div { class: "flex flex-col flex-1 max-mobile:px-[10px]",
            div { class: "flex flex-col flex-1 gap-4",
                for post in items {
                    FeedCard { key: "{post.pk}", post: post.clone() }
                }

                if v.has_more() {
                    {v.more_element()}
                } else {
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
    let mut teamname_signal = use_signal(|| teamname.clone());
    let mut v = use_infinite_query(move |bookmark| {
        let teamname = teamname_signal();
        async move { list_team_posts_handler(teamname, None, bookmark).await }
    })?;

    let mut v_clone = v.clone();
    use_effect(use_reactive((&teamname,), move |(name,)| {
        if *teamname_signal.peek() != name {
            teamname_signal.set(name);
            v_clone.restart();
        }
    }));

    let items = v.items();

    if items.is_empty() {
        return rsx! {
            div { class: "flex flex-row justify-start items-center w-full text-base font-medium text-gray-500 border border-gray-500 h-fit px-[16px] py-[20px] rounded-[8px]",
                "No posts available"
            }
        };
    }

    rsx! {
        div { class: "flex flex-col flex-1 max-mobile:px-[10px]",
            div { class: "flex flex-col flex-1 gap-4",
                for post in items {
                    FeedCard { key: "{post.pk}", post: post.clone() }
                }

                if v.has_more() {
                    {v.more_element()}
                } else {
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
