use crate::components::post_detail::*;
use crate::controllers::dto::*;
use crate::controllers::get_post::get_post_handler;
use crate::*;
use dioxus::prelude::*;

#[component]
pub fn PostDetail(post_pk: String) -> Element {
    let post_pk_clone = post_pk.clone();
    let mut resource = use_server_future(move || {
        let pk = post_pk_clone.clone();
        async move { get_post_handler(pk.parse().unwrap()).await }
    })?;

    let resolved = resource.suspend()?;
    let data = resolved.read();

    match data.as_ref() {
        Ok(detail) => {
            let on_refresh = move || {
                resource.restart();
            };
            rsx! {
                div { class: "flex flex-col gap-6 w-full max-w-[906px] mx-auto py-6 px-6 max-tablet:mr-[20px]",
                    PostDetailHeader { detail: detail.clone(), post_pk: post_pk.clone() }
                    PostContent { detail: detail.clone() }
                    CommentSection {
                        detail: detail.clone(),
                        post_pk: post_pk.clone(),
                        on_refresh,
                    }
                }
            }
        }
        Err(_) => {
            rsx! {
                div { class: "flex flex-col items-center justify-center text-text-primary py-6 px-6 mx-auto w-full max-w-[906px]",
                    h2 { class: "text-xl font-bold", "Post not found" }
                    p { class: "text-sm text-text-secondary mt-2",
                        "The post you're looking for doesn't exist or has been removed."
                    }
                }
            }
        }
    }
}
