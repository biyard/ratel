use crate::components::post_detail::*;
use crate::controllers::dto::*;
use crate::controllers::get_post::get_post_handler;
use crate::*;
use dioxus::prelude::*;

#[component]
pub fn PostDetail(post_id: FeedPartition) -> Element {
    let p1 = post_id.clone();
    let mut resource = use_loader(move || {
        let post_id = p1.clone();
        async move { get_post_handler(post_id).await }
    })?;

    let detail = resource();

    rsx! {
        div { class: "flex flex-col gap-6 py-6 px-6 mx-auto w-full max-w-desktop max-tablet:px-2.5",
            PostDetailHeader { detail: detail.clone(), post_pk: post_id.clone() }
            PostContent {
                post_type: detail.post.as_ref().map(|p| p.post_type.clone()).unwrap_or_default(),
                urls: detail.post.as_ref().map(|p| p.urls.clone()).unwrap_or_default(),
                title: detail.post.as_ref().map(|p| p.title.clone()).unwrap_or_default(),
                html_contents: detail.post.as_ref().map(|p| p.html_contents.clone()).unwrap_or_default(),
                artwork_metadata: detail.artwork_metadata.clone(),
            }
            CommentSection {
                detail: detail.clone(),
                post_pk: post_id.clone(),
                on_refresh: move || {
                    resource.restart();
                },
            }
        }
    }
}
