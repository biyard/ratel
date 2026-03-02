use crate::*;

#[component]
pub fn FileCard(file: File, editable: bool, on_delete: Option<EventHandler<String>>) -> Element {
    let icon = file_extension_icon(&file.ext);
    let file_url: String = file.url.clone().unwrap_or_default();

    rsx! {
        div {
            class: "flex flex-row justify-start items-center w-full gap-2 p-4 bg-card border border-separator rounded-[8px] cursor-pointer hover:bg-card-hover transition-colors",
            onclick: {
                let _url = file_url.clone();
                move |_: Event<MouseData>| {
                    #[cfg(not(feature = "server"))]
                    if !_url.is_empty() {
                        let _ = common::web_sys::window()
                            .and_then(|w| w.open_with_url_and_target(&_url, "_blank").ok());
                    }
                }
            },
            div { class: "[&>svg]:size-9 shrink-0", {icon} }
            div { class: "flex flex-col w-full justify-start items-start gap-1 min-w-0",
                p { class: "font-semibold text-xs text-font-primary truncate w-full",
                    "{file.name}"
                }
                if !file.size.is_empty() {
                    p { class: "font-normal text-xs text-card-meta", "{file.size}" }
                }
            }
            if editable {
                {
                    let file_id = file.id.clone();
                    rsx! {
                        button {
                            class: "flex items-center justify-center size-6 shrink-0 text-red-400 hover:text-red-300",
                            onclick: move |evt: Event<MouseData>| {
                                evt.stop_propagation();
                                if let Some(ref handler) = on_delete {
                                    handler.call(file_id.clone());
                                }
                            },
                            icons::validations::Clear { width: "16", height: "16", class: "[&>path]:stroke-current" }
                        }
                    }
                }
            }
        }
    }
}

fn file_extension_icon(ext: &FileExtension) -> Element {
    match ext {
        FileExtension::JPG => rsx! {
            icons::files::Jpg { width: "36", height: "36" }
        },
        FileExtension::PNG => rsx! {
            icons::files::Png { width: "36", height: "36" }
        },
        FileExtension::PDF => rsx! {
            icons::files::Pdf { width: "36", height: "36" }
        },
        FileExtension::ZIP => rsx! {
            icons::files::Zip { width: "36", height: "36" }
        },
        FileExtension::WORD => rsx! {
            icons::files::Docx { width: "36", height: "36" }
        },
        FileExtension::PPTX => rsx! {
            icons::files::Pptx { width: "36", height: "36" }
        },
        FileExtension::EXCEL => rsx! {
            icons::files::Xlsx { width: "36", height: "36" }
        },
        FileExtension::MP4 => rsx! {
            icons::files::Mp4 { width: "36", height: "36" }
        },
        FileExtension::MOV => rsx! {
            icons::files::Mov { width: "36", height: "36" }
        },
        FileExtension::MKV => rsx! {
            icons::file::File {
                width: "36",
                height: "36",
                class: "[&>path]:stroke-current text-card-meta",
            }
        },
    }
}
