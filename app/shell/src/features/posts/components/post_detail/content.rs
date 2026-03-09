use crate::features::posts::controllers::dto::*;
use crate::features::posts::models::PostArtworkMetadata;
use crate::features::posts::types::*;
use crate::features::posts::*;
use crate::common::components::TiptapEditor;
use dioxus::prelude::*;

#[component]
pub fn PostContent(
    post_type: PostType,
    urls: Vec<String>,
    title: String,
    html_contents: String,
    artwork_metadata: Vec<PostArtworkMetadata>,
) -> Element {
    if post_type == PostType::Artwork {
        let image_url = urls.first().cloned();
        let bg_color = artwork_metadata
            .iter()
            .find(|m| m.trait_type == "background_color")
            .map(|m| m.value.clone())
            .unwrap_or_else(|| "#ffffff".to_string());

        rsx! {
            div { class: "flex flex-col py-5 px-4 w-full border rounded-[10px] bg-card-bg border-card-border",
                div { class: "flex flex-col w-full md:flex-row min-h-[600px]",
                    div { class: "flex flex-1 justify-center items-center",
                        div {
                            class: "flex flex-col justify-center p-5",
                            style: "background-color: {bg_color};",
                            if let Some(url) = &image_url {
                                img {
                                    src: url.clone(),
                                    alt: title.clone(),
                                    class: "object-contain max-w-full max-h-[800px]",
                                }
                            } else {
                                div { class: "text-text-secondary", "No image available" }
                            }
                        }
                    }
                    div { class: "flex flex-col flex-1 gap-6 p-8 bg-card",
                        div { class: "flex flex-col gap-1",
                            p { class: "text-sm text-text-secondary", "Artwork Name" }
                            h1 { class: "text-2xl font-bold text-text-primary", {title} }
                        }
                        if !artwork_metadata.is_empty() {
                            ArtworkMetadataSection { metadata: artwork_metadata.clone() }
                        }
                        if !html_contents.is_empty() {
                            div { class: "flex flex-col gap-2",
                                h2 { class: "text-lg font-semibold text-text-primary",
                                    "Description"
                                }
                                TiptapEditor {
                                    class: "w-full bg-transparent",
                                    content: html_contents.clone(),
                                    editable: false,
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        let image_url = urls.first().cloned().filter(|url| !url.is_empty());
        rsx! {
            div { class: "flex flex-col py-5 px-4 w-full border rounded-[10px] bg-card-bg border-card-border",
                div { class: "flex flex-col gap-5 w-full",
                    TiptapEditor {
                        class: "w-full bg-transparent",
                        content: html_contents.clone(),
                        editable: false,
                    }
                    if let Some(url) = image_url {
                        div { class: "relative px-2",
                            div { class: "relative aspect-video",
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
            h2 { class: "font-semibold text-md text-text-primary", "Artwork Metadata" }
            div { class: "flex flex-col gap-3",
                for item in filtered {
                    div {
                        key: "{item.trait_type}",
                        class: "flex justify-between items-start p-3 rounded-lg bg-background",
                        span { class: "text-sm font-medium capitalize text-text-secondary",
                            {item.trait_type.replace('_', " ")}
                        }
                        span { class: "text-xs font-semibold text-text-secondary",
                            {item.value.clone()}
                        }
                    }
                }
            }
        }
    }
}
