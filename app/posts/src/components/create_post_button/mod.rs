use crate::controllers::create_post::create_post_handler;
use crate::*;
use common::components::{Button, ButtonStyle};
use dioxus::prelude::*;

translate! {
    CreatePostButtonTranslate;

    write: {
        en: "Create Post",
        ko: "게시물 작성",
    },
}

#[component]
pub fn CreatePostButton(#[props(default)] team_pk: Option<String>) -> Element {
    let tr: CreatePostButtonTranslate = use_translate();
    let nav = use_navigator();

    rsx! {
        Button {
            style: ButtonStyle::Secondary,
            class: "flex flex-row items-center gap-2 justify-start w-full cursor-pointer",
            aria_label: "Create Post",
            onclick: move |_| {
                let team_pk = team_pk.clone();
                let nav = nav.clone();
                async move {
                    let team_id = team_pk.map(|pk| pk.parse().unwrap_or_default());
                    match create_post_handler(team_id).await {
                        Ok(resp) => {
                            let post_pk: FeedPartition = resp.post_pk.into();
                            nav.push(format!("/posts/{post_pk}/edit"));
                        }
                        Err(e) => {
                            dioxus::logger::tracing::error!("Failed to create post: {:?}", e);
                        }
                    }
                }
            },
            div { class: "flex flex-row w-full items-center justify-start gap-2.5",
                icons::edit::Edit1 { class: "w-4 h-4 [&>path]:stroke-btn-secondary-text" }
                span { class: "font-bold text-base/[22px]", "{tr.write}" }
            }
        }
    }
}
