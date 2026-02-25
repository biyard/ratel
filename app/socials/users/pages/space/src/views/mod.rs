use crate::controllers::list_my_spaces::{MySpaceResponse, list_my_spaces_handler};
use common::hooks::use_infinite_query;
use dioxus::prelude::*;

#[component]
pub fn Home(username: String) -> Element {
    let mut v =
        use_infinite_query(move |bookmark| async move { list_my_spaces_handler(bookmark).await })?;
    let items = v.items();

    if items.is_empty() {
        return rsx! {
            div { class: "flex flex-row justify-start items-center w-full text-base font-medium text-gray-500 border border-gray-500 h-fit px-[16px] py-[20px] rounded-[8px]",
                "No spaces available"
            }
        };
    }

    rsx! {
        div { class: "flex relative flex-1",
            div { class: "flex flex-1 max-mobile:px-2.5",
                div { class: "flex flex-col flex-1 gap-4",
                    for space in items {
                        SpaceCard { space: space.clone() }
                    }
                    if v.has_more() {
                        {v.more_element()}
                    } else {
                        div { class: "flex flex-row justify-center items-center w-full text-base font-medium text-gray-500 h-fit px-[16px] py-[20px]",
                            "You have reached the end of your spaces."
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SpaceCard(space: MySpaceResponse) -> Element {
    let space_id: common::types::SpacePartition = space.space_pk.clone().into();
    let nav = use_navigator();
    let href = format!("/spaces/{}", space_id);

    rsx! {
        div {
            class: "flex flex-col w-full justify-start items-start px-4 py-5 rounded-[10px] bg-card-bg-secondary border border-card-border transition-colors cursor-pointer hover:bg-card-bg",
            onclick: move |_| {
                nav.push(href.clone());
            },
            div { class: "flex flex-col gap-2 w-full",
                div { class: "flex gap-3 items-center",
                    div { class: "flex flex-col gap-2",
                        div { class: "text-base font-semibold text-text-primary", "{space.title}" }
                        div { class: "flex flex-row items-center gap-2",
                            if !space.author_profile_url.is_empty() {
                                img {
                                    class: "w-5 rounded-full",
                                    src: "{space.author_profile_url}",
                                }
                            }
                            p { class: "text-sm text-text-secondary", "{space.author_display_name}" }
                        }
                    }
                }
            }
        }
    }
}
