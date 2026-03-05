use crate::controllers::dto::*;
use crate::models::PostArtworkMetadata;
use crate::types::*;
use crate::*;
use common::components::TiptapEditor;
use dioxus::prelude::*;

#[component]
pub fn PostContent(detail: PostDetailResponse) -> Element {
    let post = match &detail.post {
        Some(p) => p.clone(),
        None => return rsx! {},
    };

    if post.post_type == PostType::Artwork {
        let image_url = post.urls.first().cloned();
        let bg_color = detail
            .artwork_metadata
            .iter()
            .find(|m| m.trait_type == "background_color")
            .map(|m| m.value.clone())
            .unwrap_or_else(|| "#ffffff".to_string());

        rsx! {
            div { class: "flex flex-col w-full border rounded-[10px] bg-card-bg border-card-border px-4 py-5",
                div { class: "flex flex-col md:flex-row w-full min-h-[600px]",
                    div { class: "flex justify-center items-center flex-1",
                        div {
                            class: "flex flex-col justify-center p-5",
                            style: "background-color: {bg_color};",
                            if let Some(url) = &image_url {
                                img {
                                    src: url.clone(),
                                    alt: post.title.clone(),
                                    class: "object-contain max-w-full max-h-[800px]",
                                }
                            } else {
                                div { class: "text-text-secondary", "No image available" }
                            }
                        }
                    }
                    div { class: "flex flex-col gap-6 flex-1 p-8 bg-card",
                        div { class: "flex flex-col gap-1",
                            p { class: "text-sm text-text-secondary", "Artwork Name" }
                            h1 { class: "text-2xl font-bold text-text-primary", {post.title} }
                        }
                        if !detail.artwork_metadata.is_empty() {
                            ArtworkMetadataSection { metadata: detail.artwork_metadata.clone() }
                        }
                        if !post.html_contents.is_empty() {
                            div { class: "flex flex-col gap-2",
                                h2 { class: "text-lg font-semibold text-text-primary",
                                    "Description"
                                }
                                TiptapEditor {
                                    class: "w-full bg-transparent",
                                    content: post.html_contents.clone(),
                                    editable: false,
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        let image_url = post.urls.first().cloned().filter(|url| !url.is_empty());
        rsx! {
            div { class: "flex flex-col w-full border rounded-[10px] bg-card-bg border-card-border px-4 py-5",
                div { class: "flex flex-col gap-5 w-full",
                    TiptapEditor {
                        class: "w-full bg-transparent",
                        content: post.html_contents.clone(),
                        editable: false,
                    }
                    if let Some(url) = image_url {
                        div { class: "px-2 relative",
                            div { class: "aspect-video relative",
                                img {
                                    src: url,
                                    alt: "Uploaded image",
                                    class: "object-cover w-full rounded-[8px]",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ArtworkMetadataSection(metadata: Vec<PostArtworkMetadata>) -> Element {
    let filtered: Vec<_> = metadata
        .iter()
        .filter(|m| m.trait_type != "background_color")
        .collect();

    if filtered.is_empty() {
        return rsx! {};
    }

    rsx! {
        div { class: "flex flex-col gap-4",
            h2 { class: "text-md font-semibold text-text-primary", "Artwork Metadata" }
            div { class: "flex flex-col gap-3",
                for item in filtered {
                    div {
                        key: "{item.trait_type}",
                        class: "flex justify-between items-start p-3 rounded-lg bg-background",
                        span { class: "text-sm font-medium text-text-secondary capitalize",
                            {item.trait_type.replace('_', " ")}
                        }
                        span { class: "text-xs text-text-secondary font-semibold",
                            {item.value.clone()}
                        }
                    }
                }
            }
        }
    }
}
