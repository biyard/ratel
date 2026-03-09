use crate::features::spaces::apps::file::components::{FileCard, FileUploadZone};
use crate::features::spaces::apps::file::controllers::{FileLinkInfo, UpdateSpaceFilesRequest};
use crate::features::spaces::apps::file::i18n::SpaceFileTranslate;
use crate::features::spaces::apps::file::types::FileLinkTarget;
use crate::features::spaces::apps::file::*;
use crate::features::spaces::space_common::hooks::use_space_role;

#[derive(Clone, Copy, PartialEq, Eq)]
enum FileTab {
    All,
    Overview,
    Boards,
}

fn is_image_ext(ext: &FileExtension) -> bool {
    matches!(ext, FileExtension::JPG | FileExtension::PNG)
}

fn is_video_ext(ext: &FileExtension) -> bool {
    matches!(
        ext,
        FileExtension::MP4 | FileExtension::MOV | FileExtension::MKV
    )
}

#[component]
pub fn FileManagerPage(space_id: SpacePartition) -> Element {
    let tr: SpaceFileTranslate = use_translate();

    let mut active_tab = use_signal(|| FileTab::All);
    let mut editing = use_signal(|| false);
    let mut files = use_signal(Vec::<File>::new);
    let mut original_files = use_signal(Vec::<File>::new);
    let mut file_links = use_signal(Vec::<FileLinkInfo>::new);
    let mut did_load = use_signal(|| false);
    let mut is_saving = use_signal(|| false);

    let space_id_for_load = space_id.clone();
    use_effect(move || {
        if did_load() {
            return;
        }
        did_load.set(true);

        let space_id = space_id_for_load.clone();

        spawn(async move {
            if let Ok(loaded_files) = get_space_files(space_id.clone()).await {
                files.set(loaded_files);
            }
            if let Ok(loaded_links) = list_file_links(space_id).await {
                file_links.set(loaded_links);
            }
        });
    });

    // Filter files by active tab
    let displayed_files: Vec<File> = {
        let all_files = files();
        let links = file_links();

        match active_tab() {
            FileTab::All => all_files,
            FileTab::Overview => {
                let linked_urls: Vec<String> = links
                    .iter()
                    .filter(|link| link.link_target == FileLinkTarget::Overview)
                    .map(|link| link.file_url.clone())
                    .collect();
                all_files
                    .into_iter()
                    .filter(|f| f.url.as_ref().is_some_and(|url| linked_urls.contains(url)))
                    .collect()
            }
            FileTab::Boards => {
                let linked_urls: Vec<String> = links
                    .iter()
                    .filter(|link| matches!(link.link_target, FileLinkTarget::Board(_)))
                    .map(|link| link.file_url.clone())
                    .collect();
                all_files
                    .into_iter()
                    .filter(|f| f.url.as_ref().is_some_and(|url| linked_urls.contains(url)))
                    .collect()
            }
        }
    };

    let image_files: Vec<File> = displayed_files
        .iter()
        .filter(|f| is_image_ext(&f.ext))
        .cloned()
        .collect();
    let video_files: Vec<File> = displayed_files
        .iter()
        .filter(|f| is_video_ext(&f.ext))
        .cloned()
        .collect();

    let tab_button_props = |tab: FileTab| -> (ButtonStyle, &'static str) {
        if active_tab() == tab {
            (
                ButtonStyle::Primary,
                "!px-4 !py-2 !text-sm !font-semibold !rounded-[8px]",
            )
        } else {
            (
                ButtonStyle::Outline,
                "!px-4 !py-2 !text-sm !font-semibold !rounded-[8px] hover:!bg-card-hover",
            )
        }
    };

    rsx! {
        div { class: "flex flex-col gap-5 w-full max-w-[1024px] text-web-font-primary pb-6",
            h3 { class: "font-bold font-raleway text-[24px]/[28px] tracking-[-0.24px] text-web-font-primary",
                {tr.page_title}
            }

            // Tabs + Edit/Save buttons
            div { class: "flex justify-between items-center flex-wrap gap-2",
                div { class: "flex gap-2",
                    {
                        let (style, class) = tab_button_props(FileTab::All);
                        rsx! {
                            Button {
                                class: class.to_string(),
                                style,
                                shape: ButtonShape::Square,
                                onclick: move |_| active_tab.set(FileTab::All),
                                {tr.tab_all}
                            }
                        }
                    }
                    {
                        let (style, class) = tab_button_props(FileTab::Overview);
                        rsx! {
                            Button {
                                class: class.to_string(),
                                style,
                                shape: ButtonShape::Square,
                                onclick: move |_| active_tab.set(FileTab::Overview),
                                {tr.tab_overview}
                            }
                        }
                    }
                    {
                        let (style, class) = tab_button_props(FileTab::Boards);
                        rsx! {
                            Button {
                                class: class.to_string(),
                                style,
                                shape: ButtonShape::Square,
                                onclick: move |_| active_tab.set(FileTab::Boards),
                                {tr.tab_boards}
                            }
                        }
                    }
                }

                div { class: "flex gap-2",
                    if editing() {
                        Button {
                            class: "!px-4 !py-2 !text-sm !font-semibold !rounded-[8px]".to_string(),
                            style: ButtonStyle::Primary,
                            shape: ButtonShape::Square,
                            disabled: is_saving(),
                            onclick: {
                                let space_id = space_id.clone();
                                move |_| {
                                    if is_saving() {
                                        return;
                                    }
                                    is_saving.set(true);
                                    let current_files = files();
                                    let space_id = space_id.clone();

                                    spawn(async move {
                                        match update_space_files(
                                                space_id,
                                                UpdateSpaceFilesRequest {
                                                    files: current_files,
                                                },
                                            )
                                            .await
                                        {
                                            Ok(updated) => {
                                                files.set(updated);
                                            }
                                            Err(err) => {
                                                error!("Failed to save files: {:?}", err);
                                            }
                                        }
                                        is_saving.set(false);
                                        editing.set(false);
                                    });
                                }
                            },
                            {tr.btn_save}
                        }
                        Button {
                            class: "!px-4 !py-2 !text-sm !font-semibold !rounded-[8px]".to_string(),
                            style: ButtonStyle::Outline,
                            shape: ButtonShape::Square,
                            onclick: move |_| {
                                files.set(original_files());
                                editing.set(false);
                            },
                            {tr.btn_discard}
                        }
                    } else {
                        Button {
                            class: "!px-4 !py-2 !text-sm !font-semibold !rounded-[8px] hover:!bg-card-hover".to_string(),
                            style: ButtonStyle::Outline,
                            shape: ButtonShape::Square,
                            onclick: move |_| {
                                original_files.set(files());
                                editing.set(true);
                            },
                            {tr.btn_edit}
                        }
                    }
                }
            }

            if editing() {
                FileUploadZone {
                    on_upload: move |file: File| {
                        let mut current = files();
                        current.push(file);
                        files.set(current);
                    },
                }
            }

            if displayed_files.is_empty() {
                div { class: "flex justify-center items-center w-full py-10 text-card-meta",
                    {tr.no_files}
                }
            } else {
                div { class: "flex flex-col gap-2.5",
                    for file in displayed_files.iter() {
                        FileCard {
                            key: "{file.id}",
                            file: file.clone(),
                            editable: editing(),
                            on_delete: move |file_id: String| {
                                let mut current = files();
                                current.retain(|f| f.id != file_id);
                                files.set(current);
                            },
                        }
                    }
                }
            }

            if !image_files.is_empty() && !editing() {
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 mt-4 pt-4 border-t border-separator",
                    for file in image_files.iter() {
                        img {
                            key: "img-{file.id}",
                            src: file.url.clone().unwrap_or_default(),
                            alt: "{file.name}",
                            class: "w-full object-contain rounded-lg border border-separator max-h-[500px] bg-black",
                        }
                    }
                }
            }

            if !video_files.is_empty() && !editing() {
                div { class: "flex flex-col gap-3 mt-4 pt-4 border-t border-separator",
                    for file in video_files.iter() {
                        video {
                            key: "video-{file.id}",
                            src: file.url.clone().unwrap_or_default(),
                            controls: true,
                            class: "w-full rounded-lg max-h-[480px]",
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn HomePage(space_id: SpacePartition) -> Element {
    let tr: SpaceFileTranslate = use_translate();
    let role = use_space_role()();

    if role == SpaceUserRole::Creator {
        rsx! {
            FileManagerPage { space_id }
        }
    } else {
        rsx! {
            div { class: "flex justify-center items-center w-full h-full text-web-font-primary",
                {tr.no_permission}
            }
        }
    }
}
