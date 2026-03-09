use super::{utils::*, PostDetailTranslate};
use crate::features::posts::controllers::dto::*;
use crate::features::posts::controllers::like_post::like_post_handler;
use crate::features::posts::controllers::{create_space_handler, delete_post_handler, CreateSpaceRequest};
use crate::features::posts::types::*;
use crate::features::posts::*;
use common::components::{Button, ButtonSize, ButtonStyle};
use dioxus::prelude::*;

#[component]
pub fn PostDetailHeader(detail: PostDetailResponse, post_pk: FeedPartition) -> Element {
    let t: PostDetailTranslate = use_translate();
    let nav = use_navigator();
    let user_ctx = ratel_auth::hooks::use_user_context();

    let post = match &detail.post {
        Some(p) => p.clone(),
        None => return rsx! {},
    };

    let mut optimistic_liked = use_signal(|| detail.is_liked);
    let mut optimistic_likes = use_signal(|| post.likes);
    let mut is_processing = use_signal(|| false);
    let mut menu_open = use_signal(|| false);

    let post_pk_for_like = post_pk.clone();
    let post_pk_for_create = post_pk.clone();
    let post_pk_for_delete = post_pk.clone();

    let post_user_pk = post.user_pk.clone();
    let admin_state = use_memo(move || {
        let permissions: TeamGroupPermissions = detail.permissions.into();
        let can_edit = permissions.contains(TeamGroupPermission::PostEdit);
        let can_delete = permissions.contains(TeamGroupPermission::PostDelete);
        let is_post_owner = user_ctx()
            .user
            .as_ref()
            .map(|user| user.pk == post_user_pk)
            .unwrap_or(false);
        (
            can_edit,
            can_delete,
            is_post_owner || (can_edit && can_delete),
        )
    });
    let (can_edit, can_delete, show_admin) = admin_state();

    let post_space_pk = post.space_pk.clone();
    let existing_space_id = use_memo(move || {
        post_space_pk.clone().and_then(|pk| match pk {
            Partition::Space(id) => Some(id),
            _ => None,
        })
    });

    let img_class = if post.author_type == ratel_auth::UserType::Team {
        "rounded-lg object-cover object-top w-6 h-6"
    } else {
        "rounded-full object-cover object-top w-6 h-6"
    };

    rsx! {
        div { class: "flex flex-col gap-2.5 w-full",
            div { class: "flex flex-row items-center w-full",
                div {
                    class: "inline-flex gap-2 items-center py-1.5 px-3 text-sm rounded-md cursor-pointer max-tablet:hidden text-text-primary",
                    aria_label: t.back,
                    onclick: move |_| {
                        nav.go_back();
                    },
                    icons::arrows::ArrowLeft { class: "[&>path]:stroke-back-icon" }
                }
                if show_admin {
                    div { class: "flex relative items-center ml-auto space-x-2.5",
                        if can_edit {
                            Button {
                                aria_label: t.edit,
                                style: ButtonStyle::Secondary,
                                onclick: move |_| {
                                    nav.push(format!("/posts/{post_pk}/edit"));
                                },
                                span { class: "inline-flex gap-2 items-center",
                                    icons::edit::Edit1 { class: "!size-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                                    {t.edit}
                                }
                            }
                            Button {
                                aria_label: t.create_space,
                                style: ButtonStyle::Primary,
                                onclick: move |_| {
                                    let nav = nav.clone();
                                    let post_pk_val = post_pk_for_create.clone();
                                    let existing_space_id = existing_space_id();
                                    spawn(async move {
                                        if let Some(space_id) = existing_space_id {
                                            nav.push(format!("/spaces/{space_id}/dashboard"));
                                            return;
                                        }
                                        match create_space_handler(CreateSpaceRequest {
                                                post_id: post_pk_val,
                                            })
                                            .await
                                        {
                                            Ok(resp) => {
                                                nav.push(format!("/spaces/{}/dashboard", resp.space_id));
                                            }
                                            Err(e) => {
                                                dioxus::logger::tracing::error!("Failed to create space: {:?}", e);
                                            }
                                        }
                                    });
                                },
                                span { class: "inline-flex gap-2 items-center",
                                    icons::home::Palace { class: "!size-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                                    {t.create_space}
                                }
                            }
                        }
                        if can_delete {
                            Button {
                                size: ButtonSize::Icon,
                                style: ButtonStyle::Text,
                                class: "p-1 rounded-full transition-colors focus:outline-none hover:bg-hover".to_string(),
                                onclick: move |_| {
                                    menu_open.set(!menu_open());
                                },
                                icons::validations::Extra { class: "size-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent [&>circle]:fill-icon-primary" }
                            }
                            if menu_open() {
                                div { class: "absolute right-0 top-full z-50 mt-2 w-40 rounded-md border border-divider bg-background",
                                    Button {
                                        size: ButtonSize::Inline,
                                        style: ButtonStyle::Text,
                                        class: "flex items-center py-2 px-4 w-full text-sm text-red-400 cursor-pointer hover:bg-hover"
                                            .to_string(),
                                        onclick: move |_| {
                                            let nav = nav.clone();
                                            let pk = post_pk_for_delete.clone();
                                            spawn(async move {
                                                let _ = delete_post_handler(pk, Some(false)).await;
                                                nav.go_back();
                                            });
                                        },
                                        span { class: "inline-flex items-center text-red-400",
                                            {t.delete}
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    div {}
                }
            }

            div { class: "flex flex-row justify-between",
                div { class: "flex gap-4 justify-end items-center w-full",
                    Button {
                        size: ButtonSize::Inline,
                        style: ButtonStyle::Text,
                        class: "flex gap-1 items-center transition-colors cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
                            .to_string(),
                        disabled: *is_processing.read(),
                        onclick: {
                            let post_pk_val = post_pk_for_like.clone();
                            move |_| {
                                if *is_processing.read() {
                                    return;
                                }
                                let new_like = !*optimistic_liked.read();
                                let previous_likes = *optimistic_likes.read();
                                let delta: i64 = if new_like { 1 } else { -1 };

                                optimistic_liked.set(new_like);
                                optimistic_likes.set((previous_likes + delta).max(0));
                                is_processing.set(true);

                                let pk = post_pk_val.clone();
                                spawn(async move {
                                    let _ = like_post_handler(pk, new_like).await;
                                    is_processing.set(false);
                                });
                            }
                        },
                        span { class: "inline-flex gap-1 items-center",
                            if optimistic_liked() {
                                icons::emoji::ThumbsUp { class: "size-5 [&>path]:fill-primary [&>path]:stroke-primary" }
                            } else {
                                icons::emoji::ThumbsUp { class: "size-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                            }
                            span { class: "text-[15px] text-text-primary",
                                {convert_number_to_string(*optimistic_likes.read())}
                            }
                        }
                    }
                    div { class: "flex gap-1 items-center",
                        icons::chat::SquareChat { class: "size-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                        span { class: "text-[15px] text-text-primary",
                            {convert_number_to_string(post.comments)}
                        }
                    }
                    div { class: "flex gap-1 items-center",
                        icons::links_share::Share1 { class: "size-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                        span { class: "text-[15px] text-text-primary",
                            {convert_number_to_string(post.shares)}
                        }
                    }
                }
            }

            h2 { class: "text-xl font-bold text-text-primary", {post.title} }

            div { class: "flex flex-row justify-between",
                div { class: "flex flex-row gap-2 justify-start items-center w-6 h-6 rounded-full",
                    if !post.author_profile_url.is_empty() {
                        img {
                            src: post.author_profile_url.clone(),
                            alt: post.author_display_name.clone(),
                            class: img_class,
                        }
                    } else {
                        div { class: "w-6 h-6 rounded-full bg-profile-bg" }
                    }
                    div { class: "font-semibold text-text-primary text-sm/[20px]",
                        {post.author_display_name}
                    }
                    icons::shapes::Badge2 { width: "16", height: "16", class: "" }
                }
                div { class: "font-light text-text-primary text-sm/[14px]",
                    {time_ago(post.created_at)}
                }
            }
        }
    }
}
