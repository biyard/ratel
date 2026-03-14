use super::*;
use crate::common::components::{
    Button, ButtonShape, ButtonSize, ButtonStyle, SpaceCard, TiptapEditor,
};
use crate::common::icons::{edit::Edit1, other_devices::Save};
use crate::common::lucide_dioxus::Users;
use crate::features::posts::controllers::like_post::like_post_handler;
use crate::features::spaces::pages::apps::apps::file::components::{FileCard, FileUploadZone};
use crate::features::spaces::pages::apps::apps::file::UpdateSpaceFilesRequest;
use crate::features::spaces::space_common::hooks::use_space_query;

const DEFAULT_PROFILE_IMAGE: &str = "https://metadata.ratel.foundation/ratel/default-profile.png";

#[component]
pub fn OverviewContent(
    space_id: ReadSignal<SpacePartition>,
    #[props(default = false)] editable: bool,
) -> Element {
    let tr: OverviewTranslate = use_translate();
    let mut space_loader = use_space_query(&space_id())?;
    let space = space_loader.read().clone();

    let mut content = use_signal(|| space.content.clone());
    let mut is_editing = use_signal(|| false);
    let mut is_saving = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut files = use_signal(Vec::<File>::new);
    let mut is_like_processing = use_signal(|| false);
    let mut did_load = use_signal(|| false);

    use_effect(move || {
        if did_load() {
            return;
        }
        did_load.set(true);

        spawn(async move {
            if let Ok(loaded_files) =
                crate::features::spaces::pages::apps::apps::file::get_space_files(space_id()).await
            {
                files.set(loaded_files);
            }
        });
    });

    let allow_edit = editable && is_editing();

    rsx! {
        div { class: "mx-auto flex w-full justify-center px-4 pt-5",
            div { class: "flex w-full max-w-desktop flex-col gap-5",
                div { class: "flex items-center justify-between gap-2.5",
                    h1 { class: "flex-1 text-[28px]/[32px] font-bold text-text-primary",
                        "{space.title}"
                    }
                    div { class: "flex items-center gap-2",
                        if editable {
                            if !is_editing() {
                                Button {
                                    size: ButtonSize::Medium,
                                    style: ButtonStyle::Outline,
                                    shape: ButtonShape::Rounded,
                                    class: "inline-flex items-center gap-2",
                                    onclick: move |_| {
                                        if !is_saving() {
                                            is_editing.set(true);
                                        }
                                    },
                                    Edit1 { class: "size-4 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                                    "{tr.btn_edit}"
                                }
                            } else {
                                Button {
                                    size: ButtonSize::Medium,
                                    style: ButtonStyle::Outline,
                                    shape: ButtonShape::Rounded,
                                    class: "inline-flex items-center gap-2",
                                    loading: is_saving,
                                    onclick: {
                                        move |_| {
                                            if is_saving() {
                                                return;
                                            }
                                            is_saving.set(true);
                                            error.set(None);
                                            let space_pk = space_id();
                                            let html = content();
                                            let current_files = files();

                                            spawn(async move {
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
                                                        is_editing.set(false);
                                                    }
                                                    Err(err) => {
                                                        error.set(Some(err.to_string()));
                                                    }
                                                }

                                                is_saving.set(false);
                                            });
                                        }
                                    },
                                    Save { class: "size-4 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                                    "{tr.btn_save}"
                                }
                            }
                        }
                    }
                }

                div { class: "flex items-center justify-between border-y border-card-border py-4",
                    div { class: "flex min-w-0 items-center gap-2.5",
                        div { class: "flex flex-row w-fit items-center gap-[8px]",
                            {render_author_avatar(&space.author_profile_url, &space.author_display_name)}
                            div { class: "min-w-0 text-[14px]/[20px] font-semibold text-text-primary",
                                "{space.author_display_name}"
                            }
                        }
                        div { class: "shrink-0 text-[14px] font-light text-text-primary",
                            "{time_ago(space.created_at)}"
                        }
                    }
                    div { class: "flex items-center gap-5",
                        Button {
                            size: ButtonSize::Inline,
                            style: ButtonStyle::Text,
                            class: "inline-flex items-center gap-1 text-text-primary disabled:opacity-50".to_string(),
                            disabled: is_like_processing(),
                            onclick: move |_| {
                                if is_like_processing() {
                                    return;
                                }

                                is_like_processing.set(true);

                                let post_id = space.post_id.clone();
                                spawn(async move {
                                    let _ = like_post_handler(post_id, !space.liked).await;
                                    space_loader.restart();
                                    is_like_processing.set(false);
                                });
                            },
                            if space.liked {
                                icons::emoji::ThumbsUp { class: "size-5 [&>path]:fill-primary [&>path]:stroke-primary" }
                            } else {
                                icons::emoji::ThumbsUp { class: "size-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                            }
                            span { class: "text-[15px]", "{space.likes}" }
                        }
                        div { class: "inline-flex items-center gap-1 text-text-primary",
                            icons::chat::SquareChat { class: "size-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                            span { class: "text-[15px]", "{space.comments}" }
                        }
                        div { class: "inline-flex items-center gap-1 text-text-primary",
                            {render_visibility(&space.visibility)}
                        }
                    }
                }

                if let Some(message) = error() {
                    div { class: "text-sm text-red-500", "{tr.save_failed}: {message}" }
                }

                SpaceCard { class: "border-none !bg-transparent !p-0 shadow-none",
                    TiptapEditor {
                        class: "w-full h-fit [&>div]:border-0 [&>div]:bg-transparent [&_[data-tiptap-toolbar]]:hidden [&_[contenteditable='true']]:px-0 [&_[contenteditable='true']]:py-0 [&_[contenteditable='true']]:text-[15px]/[24px] [&_[contenteditable='true']]:tracking-[0.5px] [&_[contenteditable='true']]:text-[#D4D4D4]",
                        content: content(),
                        editable: allow_edit,
                        placeholder: tr.placeholder,
                        on_content_change: move |html: String| {
                            content.set(html);
                        },
                    }
                }

                div { class: "flex w-full flex-col gap-2.5",
                    for file in files().iter() {
                        FileCard {
                            key: "{file.id}",
                            file: file.clone(),
                            editable: allow_edit,
                            on_delete: move |file_id: String| {
                                let mut current = files();
                                current.retain(|f| f.id != file_id);
                                files.set(current);
                            },
                        }
                    }

                    if allow_edit {
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
}

fn render_author_avatar(profile_url: &str, display_name: &str) -> Element {
    let image_src = if profile_url.is_empty() {
        DEFAULT_PROFILE_IMAGE
    } else {
        profile_url
    };

    rsx! {
        img {
            class: "size-5 rounded-full object-cover",
            src: "{image_src}",
            alt: "{display_name}",
        }
    }
}

fn time_ago(timestamp_millis: i64) -> String {
    let now = chrono::Utc::now().timestamp_millis();
    let diff = now - timestamp_millis;

    if diff < 60 * 1000 {
        format!("{}s ago", diff / 1000)
    } else if diff < 3600 * 1000 {
        format!("{}m ago", diff / 1000 / 60)
    } else if diff < 86400 * 1000 {
        format!("{}h ago", diff / 1000 / 3600)
    } else if diff < 604800 * 1000 {
        format!("{}d ago", diff / 1000 / 86400)
    } else if diff < 31536000 * 1000 {
        format!("{}w ago", diff / 1000 / 604800)
    } else {
        format!("{}y ago", diff / 1000 / 31536000)
    }
}

fn visibility_label(visibility: &SpaceVisibility) -> &'static str {
    match visibility {
        SpaceVisibility::Private => "Private",
        SpaceVisibility::Public => "Public",
        SpaceVisibility::Team(_) => "Team",
    }
}

fn render_visibility(visibility: &SpaceVisibility) -> Element {
    let label = visibility_label(visibility);

    match visibility {
        SpaceVisibility::Public => rsx! {
            icons::security::Unlock1 { class: "size-5 shrink-0 [&>path]:stroke-icon-primary [&>path]:fill-transparent [&>rect]:stroke-icon-primary [&>rect]:fill-transparent [&>circle]:stroke-icon-primary [&>circle]:fill-transparent" }
            span { class: "text-[15px]", "{label}" }
        },
        _ => rsx! {
            icons::security::Lock1 { class: "size-5 shrink-0 [&>path]:stroke-icon-primary [&>path]:fill-transparent [&>rect]:stroke-icon-primary [&>rect]:fill-transparent [&>circle]:stroke-icon-primary [&>circle]:fill-transparent" }
            span { class: "text-[15px]", "{label}" }
        },
    }
}
