use super::*;
use crate::common::components::FileUploader;
use crate::features::spaces::space_common::types::space_page_actions_quiz_key;

fn extension_from_url(url: &str) -> FileExtension {
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
    path.rsplit('/').next().unwrap_or("untitled").to_string()
}

fn file_icon(ext: &FileExtension) -> Element {
    match ext {
        FileExtension::JPG => rsx! { icons::files::Jpg { width: "36", height: "36" } },
        FileExtension::PNG => rsx! { icons::files::Png { width: "36", height: "36" } },
        FileExtension::PDF => rsx! { icons::files::Pdf { width: "36", height: "36" } },
        FileExtension::ZIP => rsx! { icons::files::Zip { width: "36", height: "36" } },
        FileExtension::WORD => rsx! { icons::files::Docx { width: "36", height: "36" } },
        FileExtension::PPTX => rsx! { icons::files::Pptx { width: "36", height: "36" } },
        FileExtension::EXCEL => rsx! { icons::files::Xlsx { width: "36", height: "36" } },
        FileExtension::MP4 => rsx! { icons::files::Mp4 { width: "36", height: "36" } },
        FileExtension::MOV => rsx! { icons::files::Mov { width: "36", height: "36" } },
        FileExtension::MKV => rsx! {
            icons::file::File {
                width: "36",
                height: "36",
                class: "[&>path]:stroke-current text-card-meta",
            }
        },
    }
}

#[component]
pub fn UploadTab(can_edit: bool) -> Element {
    let ctx = use_space_quiz_context();
    let quiz = ctx.quiz.read().clone();
    let current_section = use_signal(|| QuizCreatorSection::Upload);

    rsx! {
        UploadContent {
            space_id: ctx.space_id,
            quiz_id: ctx.quiz_id,
            initial_files: quiz.files,
            can_edit,
            current_section,
            show_navigation: false,
        }
    }
}

#[component]
pub fn UploadContent(
    space_id: ReadSignal<SpacePartition>,
    quiz_id: ReadSignal<SpaceQuizEntityType>,
    initial_files: Vec<File>,
    can_edit: bool,
    current_section: Signal<QuizCreatorSection>,
    #[props(default = true)] show_navigation: bool,
) -> Element {
    let tr: QuizCreatorTranslate = use_translate();
    let mut toast = use_toast();
    let mut files = use_signal(|| initial_files);
    let mut opened_menu = use_signal(|| Option::<String>::None);

    let on_save = move |_evt: Event<MouseData>| {
        if !can_edit {
            return;
        }
        let mut toast = toast;
        spawn(async move {
            let req = UpdateQuizRequest {
                files: Some(files()),
                ..Default::default()
            };
            if let Err(err) = update_quiz(space_id(), quiz_id(), req).await {
                error!("Failed to update quiz files: {:?}", err);
                toast.error(err);
            } else {
                let keys = space_page_actions_quiz_key(&space_id(), &quiz_id());
                invalidate_query(&keys);
                opened_menu.set(None);
            }
        });
    };

    rsx! {
        div { class: "flex w-full max-w-[1024px] flex-col gap-4",
            if can_edit {
                FileUploader {
                    accept: Some("*/*".to_string()),
                    on_upload_success: move |url: String| {
                        let mut next = files();
                        next.push(File {
                            id: url.clone(),
                            name: extract_filename_from_url(&url),
                            size: String::new(),
                            ext: extension_from_url(&url),
                            url: Some(url),
                        });
                        files.set(next);
                    },
                    div { class: "flex min-h-[220px] w-full flex-col items-center justify-center rounded-[12px] border border-dashed border-[#404040] bg-[#1A1A1A] px-6 py-10 text-center transition-colors hover:border-primary",
                        icons::ratel::Cloud {
                            width: "56",
                            height: "56",
                            class: "mb-4 text-[#8C8C8C] [&>path]:stroke-current",
                        }
                        p { class: "text-[24px]/[32px] font-bold text-white",
                            {tr.upload_drop_title}
                        }
                        Button {
                            style: ButtonStyle::Outline,
                            shape: ButtonShape::Square,
                            class: "mt-5 min-w-[120px] rounded-full border-white bg-white text-black hover:bg-white/90 hover:text-black",
                            "{tr.upload_cta}"
                        }
                        p { class: "mt-4 text-[13px]/[20px] font-medium text-[#A3A3A3]",
                            {tr.upload_supported_types}
                        }
                    }
                }
            }

            div { class: "flex flex-col gap-3",
                if files().is_empty() {
                    div { class: "flex min-h-[96px] items-center justify-center rounded-[12px] border border-[#262626] bg-[#1A1A1A] px-6 text-center",
                        p { class: "text-[15px]/[22px] font-medium text-[#8C8C8C]",
                            {tr.upload_empty}
                        }
                    }
                }
                for file in files().iter() {
                    {
                        let file = file.clone();
                        let file_id = file.id.clone();
                        let menu_file_id = file_id.clone();
                        let delete_file_id = file_id.clone();
                        let is_menu_open = opened_menu().as_ref() == Some(&file_id);
                        rsx! {
                            div {
                                key: "{file_id}",
                                class: "relative rounded-[12px] border border-[#262626] bg-[#1A1A1A] px-5 py-4",
                                div { class: "flex items-center justify-between gap-4",
                                    div { class: "flex min-w-0 items-center gap-4",
                                        div { class: "shrink-0 [&>svg]:size-10", {file_icon(&file.ext)} }
                                        div { class: "flex min-w-0 flex-col gap-1",
                                            p { class: "truncate text-[20px]/[28px] font-bold text-white max-tablet:text-[18px]/[24px]",
                                                "{file.name}"
                                            }
                                            if !file.size.is_empty() {
                                                p { class: "text-[13px]/[20px] font-medium text-[#8C8C8C]", "{file.size}" }
                                            }
                                        }
                                    }

                                    div { class: "flex items-center gap-2 shrink-0",
                                        if file.url.is_some() {
                                            Button {
                                                style: ButtonStyle::Outline,
                                                shape: ButtonShape::Square,
                                                class: "min-w-[88px] rounded-full border-white bg-white text-black hover:bg-white/90 hover:text-black",
                                                onclick: move |_| {
                                                    #[cfg(not(feature = "server"))]
                                                    if let Some(url) = &file.url {
                                                        let _ = crate::common::web_sys::window()
                                                            .and_then(|w| w.open_with_url_and_target(url, "_blank").ok());
                                                    }
                                                },
                                                {tr.upload_view}
                                            }
                                        }
                                        if can_edit {
                                            Button {
                                                size: ButtonSize::Icon,
                                                style: ButtonStyle::Text,
                                                class: "size-10 rounded-full border border-transparent text-white hover:bg-white/10",
                                                onclick: move |_| {
                                                    if opened_menu().as_ref() == Some(&menu_file_id) {
                                                        opened_menu.set(None);
                                                    } else {
                                                        opened_menu.set(Some(menu_file_id.clone()));
                                                    }
                                                },
                                                "..."
                                            }
                                        }
                                    }
                                }

                                if can_edit && is_menu_open {
                                    div { class: "mt-3 flex justify-end",
                                        Button {
                                            style: ButtonStyle::Outline,
                                            shape: ButtonShape::Square,
                                            class: "min-w-[96px] border-red-500 text-red-400 hover:bg-red-500/10 hover:text-red-300",
                                            onclick: move |_| {
                                                let mut next = files();
                                                next.retain(|f| f.id != delete_file_id);
                                                files.set(next);
                                                opened_menu.set(None);
                                            },
                                            {tr.upload_delete}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "flex w-full justify-end gap-3",
                if show_navigation {
                    Button {
                        style: ButtonStyle::Outline,
                        shape: ButtonShape::Square,
                        class: "min-w-[110px]",
                        onclick: move |_| current_section.set(QuizCreatorSection::Overview),
                        {tr.btn_back}
                    }
                }
                if can_edit {
                    Button {
                        style: ButtonStyle::Outline,
                        shape: ButtonShape::Square,
                        class: "min-w-[110px] inline-flex items-center justify-center gap-2 border-white text-white hover:text-white",
                        onclick: on_save,
                        crate::common::icons::other_devices::Save { class: "w-5 h-5 [&>path]:stroke-white [&>path]:fill-transparent" }
                        {tr.btn_save}
                    }
                }
                if show_navigation {
                    Button {
                        style: ButtonStyle::Primary,
                        shape: ButtonShape::Square,
                        class: "min-w-[110px] inline-flex items-center justify-center gap-2",
                        onclick: move |_| current_section.set(QuizCreatorSection::Quiz),
                        {tr.btn_next}
                        icons::arrows::ArrowRight {
                            width: "20",
                            height: "20",
                            class: "shrink-0 [&>path]:stroke-current",
                        }
                    }
                }
            }
        }
    }
}
