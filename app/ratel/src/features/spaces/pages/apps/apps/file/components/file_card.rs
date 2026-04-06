use crate::features::spaces::pages::apps::apps::file::*;

#[component]
pub fn FileCard(file: File, editable: bool, on_delete: Option<EventHandler<String>>) -> Element {
    let icon = file_extension_icon(&file.ext);
    let file_url: String = file.url.clone().unwrap_or_default();

    rsx! {
        SpaceCard {
            class: "flex flex-row justify-start items-center w-full gap-2 cursor-pointer !rounded-[8px] !bg-card !p-4 hover:!bg-card-hover transition-colors"
                .to_string(),
            onclick: {
                let _url = file_url.clone();
                move |_: Event<MouseData>| {
                    #[cfg(not(feature = "server"))]
                    if !_url.is_empty() {
                        let _ = crate::common::web_sys::window()
                            .and_then(|w| w.open_with_url_and_target(&_url, "_blank").ok());
                    }
                }
            },
            div { class: "[&>svg]:size-9 shrink-0", {icon} }
            div { class: "flex flex-col w-full justify-start items-start gap-1 min-w-0",
                p { class: "font-semibold text-xs text-web-font-primary truncate w-full",
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
                        Button {
                            class: "size-6 shrink-0 !p-0 !text-red-400 hover:!bg-transparent hover:!text-red-300"
                                .to_string(),
                            size: ButtonSize::Icon,
                            style: ButtonStyle::Text,
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
    rsx! { FileExtensionIcon { ext: ext.clone(), size: 36 } }
}
