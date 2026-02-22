use crate::controllers::list_my_spaces::{list_my_spaces_handler, MySpaceResponse};
use dioxus::prelude::*;

#[component]
pub fn Home(username: String) -> Element {
    let resource = use_server_future(move || async move {
        list_my_spaces_handler(None).await
    })?;

    let resolved = resource.suspend()?;
    let data = resolved.read();
    let (items, has_next): (Vec<MySpaceResponse>, bool) = match data.as_ref() {
        Ok(data) => {
            let has_next = data.bookmark.is_some();
            (data.items.clone(), has_next)
        }
        Err(_) => (vec![], false),
    };

    if items.is_empty() {
        return rsx! {
            div { class: "flex flex-row justify-start items-center w-full text-base font-medium text-gray-500 border border-gray-500 h-fit px-[16px] py-[20px] rounded-[8px]",
                "No spaces found"
            }
        };
    }

    rsx! {
        div { class: "flex flex-col gap-4 w-full",
            for space in items {
                SpaceCard { space: space.clone() }
            }

            if !has_next {
                div { class: "my-6 text-center text-gray-400",
                    "You have reached the end of your spaces."
                }
            }
        }
    }
}

#[component]
fn SpaceCard(space: MySpaceResponse) -> Element {
    let space_id: common::types::SpacePartition = space.space_pk.clone().into();
    let href = format!("/spaces/{}", space_id);

    rsx! {
        a {
            class: "flex flex-row gap-4 items-center p-4 rounded-lg border border-[var(--border-primary)] hover:bg-[var(--bg-secondary)] transition-colors",
            href: "{href}",
            if !space.author_profile_url.is_empty() {
                img {
                    class: "w-10 h-10 rounded-full object-cover",
                    src: "{space.author_profile_url}",
                    alt: "{space.author_display_name}",
                }
            } else {
                div { class: "w-10 h-10 rounded-full bg-[var(--bg-tertiary)]" }
            }
            div { class: "flex flex-col flex-1 min-w-0",
                p { class: "text-base font-medium text-[var(--text-primary)] truncate",
                    "{space.title}"
                }
                p { class: "text-sm text-[var(--text-secondary)]",
                    "by {space.author_display_name}"
                }
            }
        }
    }
}
