use crate::common::*;
use crate::features::ai_moderator::controllers::*;

#[component]
pub fn MaterialList(
    space_id: ReadSignal<SpacePartition>,
    discussion_sk: String,
) -> Element {
    let tr: MaterialListTranslate = use_translate();
    let mut toast = use_toast();
    let disc_sk = discussion_sk.clone();

    let mut materials = use_server_future(move || {
        let disc_sk = disc_sk.clone();
        async move { list_materials(space_id(), disc_sk).await }
    })?;

    let items = materials
        .read()
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .cloned()
        .unwrap_or_default();

    rsx! {
        div { class: "flex flex-col gap-3 w-full",
            p { class: "font-semibold font-raleway text-[13px]/[16px] tracking-[-0.14px] text-web-font-neutral",
                {tr.reference_materials}
            }

            FileUploader {
                accept: ".pdf".to_string(),
                on_upload_meta: {
                    let discussion_sk = discussion_sk.clone();
                    move |meta: UploadedFileMeta| {
                        let space_id = space_id();
                        let discussion_sk = discussion_sk.clone();
                        spawn(async move {
                            let req = UploadMaterialRequest {
                                file_name: meta.name,
                                file_url: meta.url,
                            };
                            match upload_material(space_id, discussion_sk, req).await {
                                Ok(_) => {
                                    materials.restart();
                                }
                                Err(e) => {
                                    toast.error(e);
                                }
                            }
                        });
                    }
                },
                on_upload_success: move |_url: String| {},
                div { class: "flex gap-2 justify-center items-center py-3 px-4 w-full border border-dashed rounded-[8px] cursor-pointer border-border hover:border-primary/50 transition-colors",
                    icons::upload_download::Upload1 {
                        width: "16",
                        height: "16",
                        class: "[&>path]:stroke-foreground-muted",
                    }
                    span { class: "text-[13px] font-medium text-foreground-muted",
                        {tr.upload_pdf}
                    }
                }
            }

            if !items.is_empty() {
                div { class: "flex flex-col gap-2",
                    for item in items.iter() {
                        {
                            let material_id = item.material_id.clone();
                            let file_name = item.file_name.clone();
                            let discussion_sk = discussion_sk.clone();
                            rsx! {
                                div {
                                    key: "{material_id}",
                                    class: "flex gap-2 justify-between items-center py-2 px-3 rounded-[8px] bg-card-bg",
                                    div { class: "flex gap-2 items-center min-w-0",
                                        icons::files::Pdf {
                                            width: "16",
                                            height: "16",
                                        }
                                        span { class: "text-[13px] font-medium truncate text-text-primary",
                                            {file_name}
                                        }
                                    }
                                    button {
                                        class: "shrink-0 p-1 rounded transition-colors hover:bg-destructive/10",
                                        onclick: {
                                            let material_id = material_id.clone();
                                            let discussion_sk = discussion_sk.clone();
                                            move |_| {
                                                let material_id = material_id.clone();
                                                let space_id = space_id();
                                                let discussion_sk = discussion_sk.clone();
                                                spawn(async move {
                                                    match delete_material(space_id, discussion_sk, material_id).await {
                                                        Ok(_) => {
                                                            materials.restart();
                                                        }
                                                        Err(e) => {
                                                            toast.error(e);
                                                        }
                                                    }
                                                });
                                            }
                                        },
                                        icons::edit::Delete2 {
                                            width: "14",
                                            height: "14",
                                            class: "[&>path]:stroke-destructive",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

translate! {
    MaterialListTranslate;

    reference_materials: {
        en: "Reference Materials",
        ko: "참고 자료",
    },
    upload_pdf: {
        en: "Upload PDF",
        ko: "PDF 업로드",
    },
}
