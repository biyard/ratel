use crate::controllers::create_post::create_post_handler;
use crate::*;
use dioxus::prelude::*;

translate! {
    CreatePostButtonTranslate;

    write: {
        en: "Write",
        ko: "글쓰기",
    },
}

#[component]
pub fn CreatePostButton(#[props(default)] team_pk: Option<String>) -> Element {
    let tr: CreatePostButtonTranslate = use_translate();
    let nav = use_navigator();

    rsx! {
        button {
            class: "flex items-center gap-2 justify-start w-full px-4 py-3 rounded-full bg-btn-secondary text-btn-secondary-text font-bold text-base cursor-pointer hover:opacity-80 transition-opacity",
            aria_label: "Create Post",
            onclick: move |_| {
                let team_pk = team_pk.clone();
                let nav = nav.clone();
                async move {
                    let team_id = team_pk.map(|pk| pk.parse().unwrap_or_default());
                    match create_post_handler(team_id).await {
                        Ok(resp) => {
                            let post_pk = resp.post_pk.to_string();
                            nav.push(format!("/posts/{post_pk}/edit"));
                        }
                        Err(e) => {
                            dioxus::logger::tracing::error!("Failed to create post: {:?}", e);
                        }
                    }
                }
            },
            icons::edit::Edit1 { class: "w-4 h-4 [&>path]:stroke-btn-secondary-text" }
            span { "{tr.write}" }
        }
    }
}
