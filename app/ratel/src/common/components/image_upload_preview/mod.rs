use crate::common::*;

#[derive(Clone, PartialEq, Debug)]
pub struct PendingImage {
    pub local_url: String,
    pub remote_url: Option<String>,
    pub uploading: bool,
}

impl PendingImage {
    pub fn is_ready(&self) -> bool {
        self.remote_url.is_some() && !self.uploading
    }
}

#[component]
pub fn ImageUploadPreview(images: Signal<Vec<PendingImage>>) -> Element {
    let items = images.read();
    if items.is_empty() {
        return rsx! {};
    }

    rsx! {
        div { class: "mt-2 grid grid-cols-4 gap-2",
            for (i , img) in items.iter().enumerate() {
                div { class: "group relative",
                    img {
                        class: "aspect-video w-full rounded-lg object-cover aria-busy:animate-pulse aria-busy:opacity-50",
                        "aria-busy": img.uploading,
                        src: "{img.local_url}",
                        alt: "Pending image {i}",
                    }
                    // Remove button
                    if !img.uploading {
                        button {
                            class: "absolute right-1 top-1 flex h-5 w-5 items-center justify-center rounded-full text-xs opacity-0 transition-opacity bg-destructive text-btn-primary-text group-hover:opacity-100",
                            "aria-label": "Remove image",
                            onclick: {
                                let mut images = images;
                                move |_| {
                                    images.write().remove(i);
                                }
                            },
                            "\u{00d7}"
                        }
                    }
                    // Loading indicator
                    if img.uploading {
                        div { class: "absolute inset-0 flex items-center justify-center",
                            div { class: "h-5 w-5 animate-spin rounded-full border-2 border-primary border-t-transparent" }
                        }
                    }
                }
            }
        }
    }
}
