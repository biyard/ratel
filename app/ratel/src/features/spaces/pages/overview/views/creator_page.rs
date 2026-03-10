use super::*;
use crate::common::{
    components::{FileUploader, SpaceCard, TiptapEditor},
    icons::{edit::Edit1, other_devices::Save},
};
use crate::features::spaces::pages::apps::apps::file::UpdateSpaceFilesRequest;
use crate::features::spaces::pages::apps::apps::file::components::{FileCard, FileUploadZone};
use crate::features::spaces::space_common::hooks::use_space_query;

#[component]
pub fn CreatorPage(space_id: SpacePartition) -> Element {
    let tr: OverviewTranslate = use_translate();
    let space_loader = use_space_query(&space_id)?;
    let space = space_loader.read().clone();

    let mut content = use_signal(|| space.content.clone());
    let mut editable = use_signal(|| false);
    let mut is_saving = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut header_image = use_signal(|| space.urls.first().cloned());
    let mut files = use_signal(Vec::<File>::new);
    let mut did_load = use_signal(|| false);

    let space_id_for_load = space_id.clone();
    use_effect(move || {
        if did_load() {
            return;
        }
        did_load.set(true);

        let space_id = space_id_for_load.clone();
        spawn(async move {
            if let Ok(loaded_files) = crate::features::spaces::pages::apps::apps::file::get_space_files(space_id).await {
                files.set(loaded_files);
            }
        });
    });

    rsx! {
        div { class: "flex flex-col w-full gap-2.5",
            // Header image
            if let Some(url) = header_image() {
                div { class: "relative w-full aspect-video overflow-hidden rounded-lg",
                    img { class: "w-full h-full object-cover", src: "{url}" }
                    if editable() {
                        button {
                            class: "absolute top-2 right-2 w-8 h-8 bg-black/60 hover:bg-black/80 text-white rounded-full flex items-center justify-center cursor-pointer",
                            onclick: move |_| {
                                header_image.set(None);
                            },
                            "×"
                        }
                    }
                }
            } else if editable() {
                FileUploader {
                    accept: "image/*",
                    on_upload_success: move |url: String| {
                        header_image.set(Some(url));
                    },
                    div { class: "w-full aspect-video border-2 border-dashed border-separator rounded-lg flex items-center justify-center cursor-pointer hover:border-btn-primary-bg transition-colors",
                        p { class: "text-sm text-card-meta", "{tr.upload_image}" }
                    }
                }
            }

            // Content card with edit/save toggle
            SpaceCard { class: "flex flex-col rounded-lg !p-6".to_string(),
                div { class: "flex items-center justify-end flex-shrink-0",
                    div { class: "flex items-center gap-3",
                        if !editable() {
                            button {
                                class: "cursor-pointer w-5 h-5 [&>path]:stroke-1",
                                aria_label: tr.btn_edit,
                                onclick: move |_| {
                                    if !is_saving() {
                                        editable.set(true);
                                    }
                                },
                                Edit1 { class: "w-5 h-5 [&>path]:stroke-1 [&>path]:stroke-white" }
                            }
                        } else {
                            button {
                                class: "cursor-pointer w-5 h-5 [&>path]:stroke-1",
                                aria_label: tr.btn_save,
                                onclick: {
                                    let space_id = space_id.clone();
                                    move |_| {
                                        if is_saving() {
                                            return;
                                        }
                                        is_saving.set(true);
                                        error.set(None);
                                        let space_pk = space_id.clone();
                                        let html = content();
                                        let current_files = files();

                                        spawn(async move {
                                            // Save content
                                            match crate::features::spaces::pages::overview::controllers::update_space_content(
                                                    space_pk.clone(),
                                                    crate::features::spaces::pages::overview::controllers::UpdateContentRequest {
                                                        content: html,
                                                    },
                                                )
                                                .await
                                            {
                                                Ok(_) => {}
                                                Err(err) => {
                                                    error.set(Some(err.to_string()));
                                                }
                                            }

                                            // Save files
                                            match crate::features::spaces::pages::apps::apps::file::update_space_files(
                                                    space_pk,
                                                    UpdateSpaceFilesRequest {
                                                        files: current_files,
                                                    },
                                                )
                                                .await
                                            {
                                                Ok(updated) => {
                                                    files.set(updated);
                                                    editable.set(false);
                                                }
                                                Err(err) => {
                                                    error.set(Some(err.to_string()));
                                                }
                                            }

                                            is_saving.set(false);
                                        });
                                    }
                                },
                                Save { class: "w-5 h-5 [&>path]:stroke-1 [&>path]:stroke-white" }
                            }
                        }
                    }
                }

                // Error display
                if let Some(message) = error() {
                    div { class: "text-red-500 text-sm mt-2", "{tr.save_failed}: {message}" }
                }

                // TiptapEditor
                div { class: "flex flex-col w-full min-h-0 flex-1 overflow-hidden",
                    TiptapEditor {
                        class: "w-full h-fit [&>div]:bg-transparent [&_[data-tiptap-toolbar]]:bg-transparent",
                        content: content(),
                        editable: editable(),
                        placeholder: tr.placeholder,
                        on_content_change: move |html: String| {
                            content.set(html);
                        },
                    }
                }
            }

            // File management section
            div { class: "flex flex-col gap-2.5 w-full",
                for file in files().iter() {
                    FileCard {
                        key: "{file.id}",
                        file: file.clone(),
                        editable: editable(),
                        on_delete: move |file_id: String| {
                            let mut current = files();
                            current.retain(|f| f.id != file_id);
                            files.set(current);
                        },
                    }
                }

                // Upload zone (edit mode only)
                if editable() {
                    FileUploadZone {
                        on_upload: move |file: File| {
                            let mut current = files();
                            current.push(file);
                            files.set(current);
                        },
                    }
                }
            }
        }
    }
}
