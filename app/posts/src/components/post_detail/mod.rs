use crate::controllers::comments::add_comment::add_comment_handler;
use crate::controllers::comments::like_comment::like_comment_handler;
use crate::controllers::comments::reply_to_comment::reply_to_comment_handler;
use crate::controllers::dto::*;
use crate::controllers::like_post::like_post_handler;
use crate::controllers::{create_space_handler, delete_post_handler, CreateSpaceRequest};
use crate::models::PostArtworkMetadata;
use crate::types::*;
use crate::*;
use common::components::TiptapEditor;
use dioxus::prelude::*;

translate! {
    PostDetailTranslate;

    edit: {
        en: "Edit",
        ko: "편집하기",
    },

    create_space: {
        en: "Create a Space",
        ko: "스페이스 생성",
    },

    delete: {
        en: "Delete",
        ko: "삭제하기",
    },

    reply: {
        en: "reply",
        ko: "응답",
    },

    replies: {
        en: "replies",
        ko: "응답",
    },

    share_your_thoughts: {
        en: "Share your thoughts...",
        ko: "당신의 생각을 공유해주세요...",
    },

    comment_button: {
        en: "Comment",
        ko: "댓글",
    },

    reply_button: {
        en: "Reply",
        ko: "답글",
    },

    back: {
        en: "Back",
        ko: "뒤로",
    },

    cancel: {
        en: "Cancel",
        ko: "취소",
    },

    write_comment: {
        en: "Write a Comment",
        ko: "댓글 작성",
    },

    publish: {
        en: "Publish",
        ko: "게시",
    },

    publishing: {
        en: "Publishing...",
        ko: "게시 중...",
    },

    contents_hint: {
        en: "Type here, Use Markdown, BB code, or HTML to format.",
        ko: "여기에 마크다운 형태를 활용하여 설명을 입력해주세요.",
    },

    success_create_comment: {
        en: "Success to Create Comment",
        ko: "댓글을 성공적으로 게시했습니다.",
    },

    failed_create_comment: {
        en: "Failed to Create Comment. Please try again.",
        ko: "댓글 게시에 실패했습니다. 잠시 후 다시 시도해 주세요.",
    },

    success_delete_post: {
        en: "Success to Delete Post",
        ko: "게시물을 성공적으로 삭제했습니다.",
    },

    failed_delete_post: {
        en: "Failed to Delete Post. Please try again.",
        ko: "게시물 삭제에 실패했습니다. 잠시 후 다시 시도해주세요.",
    },
}

fn convert_number_to_string(n: i64) -> String {
    let suffixes = ["K", "M", "B"];
    let mut value = n as f64;
    let mut i = 0;

    while value >= 1000.0 && i < suffixes.len() {
        value /= 1000.0;
        i += 1;
    }

    if i == 0 {
        format!("{}", n)
    } else {
        format!("{} {}", value as i64, suffixes[i - 1])
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

#[component]
pub fn PostDetailHeader(detail: PostDetailResponse, post_pk: String) -> Element {
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

    let permissions: TeamGroupPermissions = detail.permissions.into();
    let can_edit = permissions.contains(TeamGroupPermission::PostEdit);
    let can_delete = permissions.contains(TeamGroupPermission::PostDelete);
    let is_post_owner = user_ctx()
        .user
        .as_ref()
        .map(|user| user.pk == post.user_pk)
        .unwrap_or(false);
    let show_admin = is_post_owner && (can_edit || can_delete);
    let existing_space_id = post.space_pk.clone().and_then(|pk| match pk {
        Partition::Space(id) => Some(id),
        _ => None,
    });

    let img_class = if post.author_type == ratel_auth::UserType::Team {
        "rounded-lg object-cover object-top w-6 h-6"
    } else {
        "rounded-full object-cover object-top w-6 h-6"
    };

    rsx! {
        div { class: "flex flex-col gap-2.5 w-full",
            // Back button row
            div { class: "flex flex-row justify-between items-center",
                button {
                    aria_label: "{t.back}",
                    onclick: move |_| {
                        nav.go_back();
                    },
                    icons::arrows::ArrowLeft { class: "[&>path]:stroke-back-icon" }
                }
                if show_admin {
                    div { class: "relative flex items-center space-x-2.5",
                        if can_edit {
                            button {
                                aria_label: "{t.edit}",
                                class: "rounded-md max-tablet:hidden text-sm px-3 py-1.5 text-text-primary bg-button-bg hover:bg-button-bg/80 inline-flex items-center gap-2",
                                onclick: move |_| {
                                    nav.push(format!("/posts/{post_pk}/edit"));
                                },
                                icons::edit::Edit1 { class: "!size-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                                "{t.edit}"
                            }
                            button {
                                aria_label: "{t.create_space}",
                                class: "max-tablet:hidden bg-submit-button-bg hover:bg-submit-button-bg/80 text-sm px-3 py-1.5 text-submit-button-text rounded-md inline-flex items-center gap-2",
                                onclick: move |_| {
                                    let nav = nav.clone();
                                    let post_pk_val = post_pk_for_create.clone();
                                    let existing_space_id = existing_space_id.clone();
                                    spawn(async move {
                                        if let Some(space_id) = existing_space_id {
                                            nav.push(format!("/spaces/{space_id}/dashboard"));
                                            return;
                                        }
                                        match create_space_handler(CreateSpaceRequest {
                                                post_pk: post_pk_val.parse().unwrap(),
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
                                icons::home::Palace { class: "!size-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                                "{t.create_space}"
                            }
                        }
                        if can_delete {
                            button {
                                class: "p-1 hover:bg-hover rounded-full focus:outline-none transition-colors",
                                onclick: move |_| {
                                    menu_open.set(!menu_open());
                                },
                                icons::validations::Extra { class: "size-6 [&>path]:stroke-icon-primary [&>circle]:stroke-icons-primary [&>path]:fill-transparent" }
                            }
                            if menu_open() {
                                div { class: "absolute right-0 top-full mt-2 w-40 border border-divider bg-background rounded-md z-50",
                                    button {
                                        class: "flex items-center w-full px-4 py-2 text-sm text-red-400 hover:bg-hover cursor-pointer",
                                        onclick: move |_| {
                                            let nav = nav.clone();
                                            let pk = post_pk_for_delete.clone();
                                            spawn(async move {
                                                let _ = delete_post_handler(pk.parse().unwrap(), Some(false)).await;
                                                nav.go_back();
                                            });
                                        },
                                        "{t.delete}"
                                    }
                                }
                            }
                        }
                    }
                } else {
                    div {}
                }
            }

            // Stats row
            div { class: "flex flex-row justify-between",
                div { class: "flex gap-4 justify-end items-center w-full",
                    // Like button
                    button {
                        class: "flex items-center gap-1 transition-colors cursor-pointer disabled:cursor-not-allowed disabled:opacity-50",
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
                                    let _ = like_post_handler(pk.parse().unwrap(), new_like).await;
                                    is_processing.set(false);
                                });
                            }
                        },
                        if *optimistic_liked.read() {
                            icons::emoji::ThumbsUp { class: "size-5 [&>path]:fill-primary [&>path]:stroke-icon-primary" }
                        } else {
                            icons::emoji::ThumbsUp { class: "size-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                        }
                        span { class: "text-[15px] text-text-primary",
                            "{convert_number_to_string(*optimistic_likes.read())}"
                        }
                    }
                    // Comments count
                    div { class: "flex gap-1 items-center",
                        icons::chat::SquareChat { class: "size-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                        span { class: "text-[15px] text-text-primary",
                            "{convert_number_to_string(post.comments)}"
                        }
                    }
                    // Shares count
                    div { class: "flex gap-1 items-center",
                        icons::links_share::Share1 { class: "size-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                        span { class: "text-[15px] text-text-primary",
                            "{convert_number_to_string(post.shares)}"
                        }
                    }
                }
            }

            // Title
            h2 { class: "text-xl font-bold text-text-primary", "{post.title}" }

            // Author and time
            div { class: "flex flex-row justify-between",
                div { class: "flex flex-row gap-2 justify-start items-center w-6 h-6 rounded-full",
                    if !post.author_profile_url.is_empty() {
                        img {
                            src: "{post.author_profile_url}",
                            alt: "{post.author_display_name}",
                            class: "{img_class}",
                        }
                    } else {
                        div { class: "rounded-full w-6 h-6 bg-profile-bg" }
                    }
                    div { class: "font-semibold text-text-primary text-sm/[20px]",
                        "{post.author_display_name}"
                    }
                    icons::shapes::Badge2 { width: "16", height: "16", class: "" }
                }
                div { class: "font-light text-text-primary text-sm/[14px]",
                    "{time_ago(post.created_at)}"
                }
            }
        }
    }
}

#[component]
pub fn PostContent(detail: PostDetailResponse) -> Element {
    let post = match &detail.post {
        Some(p) => p.clone(),
        None => return rsx! {},
    };

    if post.post_type == PostType::Artwork {
        let image_url = post.urls.first().cloned();
        let bg_color = detail
            .artwork_metadata
            .iter()
            .find(|m| m.trait_type == "background_color")
            .map(|m| m.value.clone())
            .unwrap_or_else(|| "#ffffff".to_string());

        rsx! {
            div { class: "flex flex-col w-full border rounded-[10px] bg-card-bg border-card-border px-4 py-5",
                div { class: "flex flex-col md:flex-row w-full min-h-[600px]",
                    // Image side
                    div { class: "flex justify-center items-center flex-1",
                        div {
                            class: "flex flex-col justify-center p-5",
                            style: "background-color: {bg_color};",
                            if let Some(url) = &image_url {
                                img {
                                    src: "{url}",
                                    alt: "{post.title}",
                                    class: "object-contain max-w-full max-h-[800px]",
                                }
                            } else {
                                div { class: "text-text-secondary", "No image available" }
                            }
                        }
                    }
                    // Metadata side
                    div { class: "flex flex-col gap-6 flex-1 p-8 bg-card",
                        div { class: "flex flex-col gap-1",
                            p { class: "text-sm text-text-secondary", "Artwork Name" }
                            h1 { class: "text-2xl font-bold text-text-primary", "{post.title}" }
                        }
                        if !detail.artwork_metadata.is_empty() {
                            ArtworkMetadataSection { metadata: detail.artwork_metadata.clone() }
                        }
                        if !post.html_contents.is_empty() {
                            div { class: "flex flex-col gap-2",
                                h2 { class: "text-lg font-semibold text-text-primary",
                                    "Description"
                                }
                                TiptapEditor {
                                    class: "w-full bg-transparent",
                                    content: post.html_contents.clone(),
                                    editable: false,
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        let image_url = post.urls.first().cloned().filter(|url| !url.is_empty());
        rsx! {
            div { class: "flex flex-col w-full border rounded-[10px] bg-card-bg border-card-border px-4 py-5",
                div { class: "flex flex-col gap-5 w-full",
                    TiptapEditor {
                        class: "w-full bg-transparent",
                        content: post.html_contents.clone(),
                        editable: false,
                    }
                    if let Some(url) = image_url {
                        div { class: "px-2 relative",
                            div { class: "aspect-video relative",
                                img {
                                    src: "{url}",
                                    alt: "Uploaded image",
                                    class: "object-cover w-full rounded-[8px]",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ArtworkMetadataSection(metadata: Vec<PostArtworkMetadata>) -> Element {
    let filtered: Vec<_> = metadata
        .iter()
        .filter(|m| m.trait_type != "background_color")
        .collect();

    if filtered.is_empty() {
        return rsx! {};
    }

    rsx! {
        div { class: "flex flex-col gap-4",
            h2 { class: "text-md font-semibold text-text-primary", "Artwork Metadata" }
            div { class: "flex flex-col gap-3",
                for item in filtered {
                    div {
                        key: "{item.trait_type}",
                        class: "flex justify-between items-start p-3 rounded-lg bg-background",
                        span { class: "text-sm font-medium text-text-secondary capitalize",
                            "{item.trait_type.replace('_', \" \")}"
                        }
                        span { class: "text-xs text-text-secondary font-semibold", "{item.value}" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn CommentSection(detail: PostDetailResponse, post_pk: String) -> Element {
    let t: PostDetailTranslate = use_translate();
    let comment_count = detail.post.as_ref().map(|p| p.comments).unwrap_or(0);
    let mut expand_comment = use_signal(|| false);
    let mut comment_text = use_signal(|| String::new());
    let mut is_submitting = use_signal(|| false);

    let comments: Vec<PostCommentResponse> = detail
        .comments
        .iter()
        .filter(|c| c.parent_comment_sk.is_none())
        .cloned()
        .collect();

    let reply_comments: Vec<PostCommentResponse> = detail
        .comments
        .iter()
        .filter(|c| c.parent_comment_sk.is_some())
        .cloned()
        .collect();

    let count_label = if comment_count == 1 {
        t.reply
    } else {
        t.replies
    };

    rsx! {
        div { id: "comments", class: "flex flex-col gap-2.5",
            // Comment count header
            div { class: "flex flex-row text-text-primary gap-2",
                icons::chat::SquareChat { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                span { class: "text-base/6 font-medium", "{comment_count} {count_label}" }
            }
            // Write a comment area
            if !*expand_comment.read() {
                button {
                    class: "flex flex-row w-full px-3.5 py-3 gap-2 bg-write-comment-box-bg border border-write-comment-box-border items-center rounded-lg hover:bg-write-comment-box-bg/80 hover:border-primary/50 transition-all duration-200 cursor-pointer group",
                    onclick: move |_| {
                        expand_comment.set(true);
                    },
                    icons::chat::SquareChat { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                    span { class: "text-write-comment-box-text text-[15px]/[24px] font-medium group-hover:text-primary transition-colors",
                        "{t.share_your_thoughts}"
                    }
                }
            }
            if *expand_comment.read() {
                div { class: "flex flex-col gap-2 p-4 border border-card-enable-border rounded-lg bg-card-bg-secondary",
                    textarea {
                        class: "w-full min-h-[80px] p-2 bg-transparent text-text-primary border border-divider rounded resize-none focus:outline-none focus:border-primary",
                        placeholder: "{t.share_your_thoughts}",
                        value: "{comment_text}",
                        oninput: move |e| {
                            comment_text.set(e.value());
                        },
                    }
                    div { class: "flex flex-row justify-end gap-2",
                        button {
                            class: "px-4 py-1.5 text-sm text-text-primary rounded hover:bg-divider cursor-pointer",
                            onclick: move |_| {
                                expand_comment.set(false);
                                comment_text.set(String::new());
                            },
                            "{t.cancel}"
                        }
                        button {
                            class: "px-4 py-1.5 text-sm text-white bg-primary rounded hover:bg-primary/80 cursor-pointer disabled:opacity-50",
                            disabled: comment_text.read().trim().is_empty() || *is_submitting.read(),
                            onclick: {
                                let pk = post_pk.clone();
                                move |_| {
                                    let content = comment_text.read().clone();
                                    if content.trim().is_empty() || *is_submitting.read() {
                                        return;
                                    }
                                    is_submitting.set(true);
                                    let pk = pk.clone();
                                    spawn(async move {
                                        let _ = add_comment_handler(pk.parse().unwrap(), content).await;
                                        comment_text.set(String::new());
                                        expand_comment.set(false);
                                        is_submitting.set(false);
                                        use_navigator()
                                            .push(crate::Route::PostDetail {
                                                post_pk: pk,
                                            });
                                    });
                                }
                            },
                            "{t.comment_button}"
                        }
                    }
                }
            }
            // Comment list
            for comment in comments {
                {
                    let replies: Vec<PostCommentResponse> = reply_comments
                        .iter()
                        .filter(|r| {
                            if let Some(parent) = &r.parent_comment_sk {
                                *parent == comment.sk
                            } else {
                                false
                            }
                        })
                        .cloned()
                        .collect();
                    rsx! {
                        CommentItem {
                            key: "{comment.sk}",
                            comment: comment.clone(),
                            post_pk: post_pk.clone(),
                            replies,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CommentItem(
    comment: PostCommentResponse,
    post_pk: String,
    replies: Vec<PostCommentResponse>,
) -> Element {
    let t: PostDetailTranslate = use_translate();
    let mut optimistic_liked = use_signal(|| comment.liked);
    let mut optimistic_likes = use_signal(|| comment.likes as i64);
    let mut is_processing = use_signal(|| false);
    let mut show_reply = use_signal(|| false);
    let mut show_replies = use_signal(|| false);
    let mut reply_text = use_signal(|| String::new());
    let mut is_reply_submitting = use_signal(|| false);

    let comment_sk_for_like = comment.sk.clone();
    let comment_sk_for_reply = comment.sk.clone();

    let img_class = "object-cover object-top w-10 h-10 rounded-full";

    let updated_secs = comment.updated_at * 1000;
    let reply_label = if comment.replies <= 1 {
        t.reply
    } else {
        t.replies
    };

    rsx! {
        div { class: "flex flex-col gap-3 pb-4",
            // Author row
            div { class: "flex flex-row justify-between items-start w-full",
                div { class: "flex flex-row gap-2 items-center",
                    if !comment.author_profile_url.is_empty() {
                        img {
                            src: "{comment.author_profile_url}",
                            alt: "{comment.author_display_name}",
                            class: "{img_class}",
                        }
                    } else {
                        div { class: "w-10 h-10 rounded-full bg-profile-bg" }
                    }
                    div { class: "flex flex-col gap-[2px]",
                        div { class: "font-semibold text-text-primary text-[15px] leading-[15px]",
                            "{comment.author_display_name}"
                        }
                        div { class: "font-semibold text-xs leading-[20px] text-text-primary",
                            "{time_ago(updated_secs)}"
                        }
                    }
                }
                div {}
            }
            // Content
            div { class: "flex flex-col gap-3 ml-12",
                TiptapEditor {
                    class: "w-full bg-transparent",
                    content: comment.content.clone(),
                    editable: false,
                }
            }
            // Actions row
            div { class: "flex flex-row gap-2 justify-between items-center w-full",
                div { class: "flex flex-row gap-5",
                    button {
                        aria_label: "Expand Replies",
                        class: "flex flex-row gap-2 justify-center items-center disabled:cursor-not-allowed text-primary",
                        disabled: comment.replies == 0,
                        onclick: move |_| {
                            let next = !*show_replies.read();
                            show_replies.set(next);
                        },
                        "{comment.replies} {reply_label}"
                        if comment.replies > 0 {
                            icons::arrows::ChevronDown { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                        }
                    }
                    button {
                        aria_label: "{t.reply_button}",
                        class: "flex gap-2 justify-center items-center cursor-pointer text-text-primary",
                        onclick: move |_| {
                            let current = *show_reply.read();
                            show_reply.set(!current);
                        },
                        icons::arrows::BendArrowRight { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                        "{t.reply}"
                    }
                }
                button {
                    aria_label: "Like Comment",
                    class: "flex flex-row gap-2 justify-center items-center",
                    disabled: *is_processing.read(),
                    onclick: {
                        let pk = post_pk.clone();
                        let sk = comment_sk_for_like.clone();
                        move |_| {
                            if *is_processing.read() {
                                return;
                            }
                            let new_like = !*optimistic_liked.read();
                            let prev = *optimistic_likes.read();
                            let delta: i64 = if new_like { 1 } else { -1 };

                            optimistic_liked.set(new_like);
                            optimistic_likes.set((prev + delta).max(0));
                            is_processing.set(true);

                            let pk = pk.clone();
                            let sk = sk.clone();
                            spawn(async move {
                                let _ = like_comment_handler(pk.parse().unwrap(), sk, new_like).await;
                                is_processing.set(false);
                            });
                        }
                    },
                    if *optimistic_liked.read() {
                        icons::emoji::ThumbsUp { class: "w-6 h-6 [&>path]:fill-primary [&>path]:stroke-icon-primary" }
                    } else {
                        icons::emoji::ThumbsUp { class: "w-6 h-6 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                    }
                    div { class: "font-medium text-base/[24px] text-comment-icon-text",
                        "{*optimistic_likes.read()}"
                    }
                }
            }
            // Inline reply form (match NewComment UI)
            if *show_reply.read() {
                div { class: "flex flex-col w-full bg-comment-box-bg border rounded-lg border-primary max-w-desktop ml-12 mt-2",
                    div { class: "flex flex-row justify-between items-center px-3 pt-3",
                        span { class: "text-sm font-medium text-text-primary", "{t.write_comment}" }
                        button {
                            aria_label: "{t.cancel}",
                            class: "p-1 rounded transition-colors hover:bg-foreground/10",
                            onclick: move |_| {
                                show_reply.set(false);
                                reply_text.set(String::new());
                            },
                            icons::arrows::DoubleArrowDown { class: "w-5 h-5 [&>path]:stroke-text-primary" }
                        }
                    }
                    div { class: "flex-1 w-full rounded-md transition-colors cursor-text hover:bg-foreground/5",
                        TiptapEditor {
                            class: "border-none",
                            content: reply_text.read().clone(),
                            editable: true,
                            placeholder: t.contents_hint,
                            on_content_change: move |val| {
                                reply_text.set(val);
                            },
                        }
                    }
                    div { class: "flex flex-row gap-2 justify-end items-center px-3 pt-3 pb-3 border-t border-divider",
                        button {
                            class: "flex items-center gap-2 px-3 py-1.5 rounded-full bg-primary text-black text-sm disabled:opacity-50",
                            disabled: reply_text.read().trim().is_empty() || *is_reply_submitting.read(),
                            onclick: {
                                let pk = post_pk.clone();
                                let sk = comment_sk_for_reply.clone();
                                move |_| {
                                    let content = reply_text.read().clone();
                                    if content.trim().is_empty() || *is_reply_submitting.read() {
                                        return;
                                    }
                                    is_reply_submitting.set(true);
                                    let pk = pk.clone();
                                    let sk = sk.clone();
                                    spawn(async move {
                                        let _ = reply_to_comment_handler(pk.parse().unwrap(), sk, content).await;
                                        reply_text.set(String::new());
                                        show_reply.set(false);
                                        is_reply_submitting.set(false);
                                        use_navigator()
                                            .push(crate::Route::PostDetail {
                                                post_pk: pk,
                                            });
                                    });
                                }
                            },
                            icons::chat::SquareChat { class: "w-5 h-5 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                            if *is_reply_submitting.read() {
                                "{t.publishing}"
                            } else {
                                "{t.publish}"
                            }
                        }
                    }
                }
            }
            // Reply list
            if *show_replies.read() && !replies.is_empty() {
                div { class: "flex flex-col gap-2.5",
                    for reply in &replies {
                        ReplyItem { key: "{reply.sk}", reply: reply.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn ReplyItem(reply: PostCommentResponse) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2 p-5 rounded-lg bg-reply-box border border-transparent",
            div { class: "flex flex-row gap-2 items-center",
                if !reply.author_profile_url.is_empty() {
                    img {
                        src: "{reply.author_profile_url}",
                        alt: "{reply.author_display_name}",
                        class: "rounded-full object-cover object-top w-10 h-10",
                    }
                } else {
                    div { class: "rounded-full w-10 h-10 bg-profile-bg" }
                }
                div { class: "flex flex-col gap-[2px]",
                    div { class: "font-semibold text-title-text text-[15px] leading-[15px]",
                        "{reply.author_display_name}"
                    }
                }
            }
            TiptapEditor {
                class: "w-full bg-transparent",
                content: reply.content.clone(),
                editable: false,
            }
        }
    }
}
