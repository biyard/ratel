use crate::common::components::{FileUploader, UploadedFileMeta};
use crate::common::types::extract_filename_from_url;
use crate::features::spaces::pages::apps::apps::file::i18n::SpaceFileTranslate;
use crate::features::spaces::pages::apps::apps::file::*;

#[component]
pub fn FileUploadZone(on_upload: EventHandler<File>) -> Element {
    let tr: SpaceFileTranslate = use_translate();

    rsx! {
        FileUploader {
            accept: "*/*",
            on_upload_success: move |_: String| {},
            on_upload_meta: move |meta: UploadedFileMeta| {
                let UploadedFileMeta { url, name, size } = meta;
                let name = if name.trim().is_empty() {
                    extract_filename_from_url(&url)
                } else {
                    name
                };
                let ext = FileExtension::from_name_or_url(&name, &url);
                let file = File {
                    id: url.clone(),
                    name,
                    size,
                    ext,
                    url: Some(url),
                    uploader_name: None,
                    uploader_profile_url: None,
                    uploaded_at: None,
                };
                on_upload.call(file);
            },
            div {
                class: "relative w-full min-h-[140px] rounded-xl border-2 border-dashed border-separator hover:border-btn-primary-bg transition-colors duration-150 ease-in-out flex items-center justify-center cursor-pointer",
                div { class: "flex flex-col items-center gap-2",
                    div { class: "w-10 h-10 rounded-full border border-separator flex items-center justify-center text-card-meta text-2xl leading-none",
                        "+"
                    }
                    p { class: "text-sm text-card-meta font-medium",
                        {tr.upload}
                    }
                    p { class: "text-xs text-card-meta", {tr.drag_or_click} }
                }
            }
        }
    }
}
