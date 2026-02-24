use super::feed_card::{time_ago, FeedContents, UserBadge};
use crate::controllers::delete_post::delete_post_handler;
use crate::controllers::dto::*;
use crate::controllers::list_user_drafts::list_user_drafts_handler;
use crate::*;
use dioxus::prelude::*;
use icons::edit::Delete2;

fn feed_end_message(msg: &str) -> Element {
    rsx! {
        div {
            class: "my-6 text-center text-gray-400",
            aria_label: "End of feed message",
            "🎉 {msg}"
        }
    }
}

#[component]
pub fn MyDrafts() -> Element {
    let resource = use_server_future(move || async move { list_user_drafts_handler(None).await })?;
    let nav = use_navigator();

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
            div { class: "flex flex-row justify-start items-center w-full text-base font-medium border border-gray-500 h-fit px-[16px] py-[20px] rounded-[8px] text-text-primary",
                "No drafts available."
            }
        };
    }

    rsx! {
        div { class: "flex flex-col flex-1 gap-2.5",
            for post in items.iter() {
                {
                    let post_pk_for_nav = post.pk.clone();
                    let post_pk_for_delete = post.pk.clone();
                    let contents_preview: String = post.html_contents.chars().take(200).collect();
                    rsx! {
                        div {
                            key: "{post.pk}",
                            class: "flex flex-col pt-5 pb-2.5 gap-2.5 rounded-lg border cursor-pointer bg-card-bg border-card-enable-border",
                            onclick: move |_| {
                                let nav = nav.clone();
                                let post_pk: FeedPartition = post_pk_for_nav.clone().into();
                                nav.push(format!("/posts/{post_pk}/edit"));
                            },
                            div { class: "flex flex-row justify-end items-center px-5",
                                div {
                                    class: "cursor-pointer hover-bg-white w-[21px] h-[21px] z-100",
                                    onclick: move |e: MouseEvent| {
                                        e.stop_propagation();
                                        e.prevent_default();
                                        let mut resource = resource;
                                        let post_pk: FeedPartition = post_pk_for_delete.clone().into();
                                        async move {
                                            if delete_post_handler(post_pk, None).await.is_ok() {
                                                resource.restart();
                                            }
                                        }
                                    },
                                    Delete2 {
                                        width: "24",
                                        height: "24",
                                        class: "[&>path]:stroke-neutral-500 [&>path]:fill-transparent",
                                    }
                                }
                            }
                            div { class: "flex flex-row gap-1 items-center px-5 w-full font-bold align-middle line-clamp-2 text-xl/[25px] tracking-[0.5px] text-text-primary",
                                div { class: "text-sm font-normal", "(Draft)" }
                                div { class: "font-normal", "{post.title}" }
                            }
                            div { class: "flex flex-row justify-between items-center px-5",
                                UserBadge {
                                    profile_url: post.author_profile_url.clone(),
                                    name: post.author_display_name.clone(),
                                    author_type: post.author_type,
                                }
                                p { class: "text-sm font-light align-middle text-text-primary", "{time_ago(post.updated_at)}" }
                            }
                            div { class: "flex flex-row justify-between px-5" }
                            FeedContents { contents: contents_preview, urls: post.urls.clone() }
                        }
                    }
                }
            }
            if !has_next {
                {feed_end_message("You have reached the end of your drafts.")}
            }
        }
    }
}
