use crate::i18n::SpaceFileTranslate;
use crate::*;
use common::components::FileUploader;

fn guess_extension_from_url(url: &str) -> FileExtension {
    let path = url.split('?').next().unwrap_or(url);
    let ext = path.rsplit('.').next().unwrap_or("").to_lowercase();
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

fn extract_filename_from_url(url: &str) -> String {
    let path = url.split('?').next().unwrap_or(url);
    path.rsplit('/')
        .next()
        .unwrap_or("untitled")
        .to_string()
}

#[component]
pub fn FileUploadZone(on_upload: EventHandler<File>) -> Element {
    let tr: SpaceFileTranslate = use_translate();
    let mut is_loading = use_signal(|| false);

    rsx! {
        FileUploader {
            accept: "*/*",
            on_upload_success: move |url: String| {
                is_loading.set(false);
                let file = File {
                    id: String::new(),
                    name: extract_filename_from_url(&url),
                    size: String::new(),
                    ext: guess_extension_from_url(&url),
                    url: Some(url),
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
                        if is_loading() {
                            {tr.uploading}
                        } else {
                            {tr.upload}
                        }
                    }
                    p { class: "text-xs text-card-meta",
                        {tr.drag_or_click}
                    }
                }
            }
        }
    }
}
