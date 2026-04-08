use crate::common::components::SeoMeta;
use crate::features::posts::components::post_detail::*;
use crate::features::posts::controllers::dto::*;
use crate::features::posts::controllers::get_post::get_post_handler;
use crate::features::posts::*;
use dioxus::prelude::*;

#[component]
pub fn PostDetail(post_id: FeedPartition) -> Element {
    let p1 = post_id.clone();
    let mut resource = use_loader(move || {
        let post_id = p1.clone();
        async move { get_post_handler(post_id).await }
    })?;

    let detail = resource();

    let post_title = detail
        .post
        .as_ref()
        .map(|p| p.title.clone())
        .unwrap_or_default();
    let post_description = detail
        .post
        .as_ref()
        .map(|p| {
            let re = regex::Regex::new(r"<[^>]*>").unwrap();
            let text = re.replace_all(&p.html_contents, "").to_string();
            if text.len() > 200 {
                text[..200].to_string()
            } else {
                text
            }
        })
        .unwrap_or_default();
    let post_image = detail
        .post
        .as_ref()
        .and_then(|p| p.urls.first().cloned())
        .unwrap_or_default();

    rsx! {
        SeoMeta {
            title: post_title,
            description: post_description,
            image: post_image,
        }
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
