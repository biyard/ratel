use crate::common::components::{FileUploader, UploadedFileMeta};
use crate::features::spaces::pages::apps::apps::file::i18n::SpaceFileTranslate;
use crate::features::spaces::pages::apps::apps::file::*;

fn guess_extension_from_name(name: &str) -> FileExtension {
    let ext = name
        .rsplit('.')
        .next()
        .unwrap_or("")
        .to_lowercase();
    match ext.as_str() {
        "jpg" | "jpeg" => FileExtension::JPG,
        "png" => FileExtension::PNG,
        "pdf" => FileExtension::PDF,
        "zip" => FileExtension::ZIP,
        "doc" | "docx" => FileExtension::WORD,
        "ppt" | "pptx" => FileExtension::PPTX,
        "xls" | "xlsx" => FileExtension::EXCEL,
        "mp4" => FileExtension::MP4,
        "mov" => FileExtension::MOV,
        "mkv" => FileExtension::MKV,
        _ => FileExtension::default(),
    }
}

#[component]
pub fn FileUploadZone(on_upload: EventHandler<File>) -> Element {
    let tr: SpaceFileTranslate = use_translate();
    let mut is_loading = use_signal(|| false);

    rsx! {
        FileUploader {
            accept: "*/*",
            on_upload_success: move |_: String| {},
            on_upload_meta: move |meta: UploadedFileMeta| {
                is_loading.set(false);
                let file = File {
                    id: meta.url.clone(),
                    name: meta.name.clone(),
                    size: meta.size.clone(),
                    ext: guess_extension_from_name(&meta.name),
                    url: Some(meta.url),
                    uploader_name: None,
                    uploader_profile_url: None,
                    uploaded_at: None,
                };
                on_upload.call(file);
            },
            div { class: "relative w-full min-h-[140px] rounded-xl border-2 border-dashed border-separator hover:border-btn-primary-bg transition-colors duration-150 ease-in-out flex items-center justify-center cursor-pointer",
                div { class: "flex flex-col items-center gap-2",
                    div { class: "w-10 h-10 rounded-full border border-separator flex items-center justify-center text-card-meta text-2xl leading-none",
                        "+"
                    }
                    p { class: "text-sm text-card-meta font-medium",
                        if is_loading() {
                            {tr.uploading}
                        } else {
                            {tr.upload}
                        }
                    }
                    p { class: "text-xs text-card-meta", {tr.drag_or_click} }
                }
            }
        }
    }
}
