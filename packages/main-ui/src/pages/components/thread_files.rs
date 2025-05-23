use bdk::prelude::{
    by_components::icons::{alignments::AlignToBottom, links_share::Link2},
    *,
};

use crate::{
    components::icons::{Docs, Jpg, Pdf, Png, Pptx, Xlsx, Zip},
    dto::file::{File, FileExtension},
    pages::components::BlackRoundedBox,
};

#[component]
pub fn ThreadFiles(
    lang: Language,
    files: Vec<File>,
    ondownload: EventHandler<(String, Option<String>)>,
) -> Element {
    let tr: ThreadFilesTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full",
            BlackRoundedBox {
                div { class: "flex flex-col w-full justify-start items-start gap-20",
                    div { class: "font-semibold text-white text-base/20", {tr.label} }
                    FileList { files, ondownload }
                }
            }
        }
    }
}

#[component]
pub fn FileList(files: Vec<File>, ondownload: EventHandler<(String, Option<String>)>) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-20",
            div { class: "flex flex-wrap w-full justify-start items-center gap-10",
                for file in files.clone() {
                    FileComponent {
                        name: file.name,
                        size: file.size,
                        ext: file.ext,
                        url: file.url,
                        ondownload,
                    }
                }
            }

            div { class: "flex flex-col w-full justify-start items-start",
                for (i , file) in files.iter().enumerate() {
                    FileLink {
                        name: file.name.clone(),
                        url: file.url.clone(),
                        ondownload,
                    }

                    if i != files.len() - 1 {
                        LineContainer {}
                    }
                }
            }
        }
    }
}

#[component]
pub fn LineContainer() -> Element {
    rsx! {
        div { class: "flex flex-row w-full h-1 bg-neutral-800" }
    }
}

#[component]
pub fn FileLink(
    name: String,
    url: Option<String>,
    ondownload: EventHandler<(String, Option<String>)>,
) -> Element {
    rsx! {
        div {
            class: "cursor-pointer flex flex-row w-full justify-start items-center px-10 py-20 gap-4",
            onclick: move |_| {
                ondownload.call((name.clone(), url.clone()));
            },
            Link2 {
                class: "[&>path]:stroke-neutral-500",
                width: "18",
                height: "18",
            }
            div { class: "font-medium text-neutral-400 text-[15px]/20 truncate overflow-hidden break-all whitespace-normal w-full",
                {name.clone()}
            }
        }
    }
}

#[component]
pub fn FileComponent(
    name: String,
    size: String,
    ext: FileExtension,
    url: Option<String>,
    ondownload: EventHandler<(String, Option<String>)>,
) -> Element {
    rsx! {
        div {
            class: "cursor-pointer flex flex-row max-w-[215px] max-tablet:max-w-full w-full h-fit justify-between items-center bg-neutral-800 rounded-lg gap-8 p-16",
            onclick: move |_| {
                ondownload.call((name.clone(), url.clone()));
            },
            div { class: "w-36 h-36",
                if ext == FileExtension::JPG {
                    Jpg {
                        class: "[&>path]:stroke-neutral-500",
                        width: "36",
                        height: "36",
                    }
                } else if ext == FileExtension::PNG {
                    Png {
                        class: "[&>path]:stroke-neutral-500",
                        width: "36",
                        height: "36",
                    }
                } else if ext == FileExtension::PDF {
                    Pdf {
                        class: "[&>path]:stroke-neutral-500",
                        width: "36",
                        height: "36",
                    }
                } else if ext == FileExtension::ZIP {
                    Zip {
                        class: "[&>path]:stroke-neutral-500",
                        width: "36",
                        height: "36",
                    }
                } else if ext == FileExtension::WORD {
                    Docs {
                        class: "[&>path]:stroke-neutral-500",
                        width: "36",
                        height: "36",
                    }
                } else if ext == FileExtension::PPTX {
                    Pptx {
                        class: "[&>path]:stroke-neutral-500",
                        width: "36",
                        height: "36",
                    }
                } else {
                    Xlsx {
                        class: "[&>path]:stroke-neutral-500",
                        width: "36",
                        height: "36",
                    }
                }
            }

            div { class: "flex flex-col w-full max-tablet:max-w-[calc(100%-100px)] justify-start items-start gap-2",
                div { class: "max-w-[120px] max-tablet:max-w-full font-semibold text-neutral-400 text-xs/18 truncate overflow-hidden whitespace-nowrap",
                    {name.clone()}
                }
                div { class: "font-normal text-[#6d6d6d] text-[10px]/16", {size} }
            }

            div { class: "cursor-pointer w-16 h-16",
                AlignToBottom {
                    class: "[&>path]:stroke-neutral-500",
                    width: "16",
                    height: "16",
                }
            }
        }
    }
}

translate! {
    ThreadFilesTranslate;

    label: {
        ko: "Attached Files",
        en: "Attached Files"
    }
}
