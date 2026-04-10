use crate::common::*;

#[component]
pub fn CommentImageGrid(images: Vec<String>) -> Element {
    if images.is_empty() {
        return rsx! {};
    }

    let mut selected_image: Signal<Option<String>> = use_signal(|| None);

    let grid_class = match images.len() {
        1 => "grid grid-cols-1 gap-2 max-w-[320px]",
        _ => "grid grid-cols-2 gap-2 max-w-[480px]",
    };

    rsx! {
        div { class: "{grid_class}",
            for (i , url) in images.iter().enumerate() {
                img {
                    key: "{i}",
                    class: "aspect-video w-full cursor-pointer rounded-lg object-cover transition-opacity hover:opacity-90",
                    src: "{url}",
                    alt: "Comment image {i}",
                    onclick: {
                        let url = url.clone();
                        move |_| selected_image.set(Some(url.clone()))
                    },
                }
            }
        }

        if let Some(url) = selected_image() {
            DialogRoot {
                open: true,
                on_open_change: move |open: bool| {
                    if !open {
                        selected_image.set(None);
                    }
                },
                DialogContent {
                    class: "flex max-h-[90vh] max-w-[90vw] items-center justify-center border-none bg-transparent p-2 shadow-none",
                    img {
                        class: "max-h-[85vh] max-w-full rounded-lg object-contain",
                        src: "{url}",
                        alt: "Comment image full size",
                    }
                }
            }
        }
    }
}
