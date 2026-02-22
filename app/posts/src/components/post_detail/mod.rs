use crate::controllers::comments::add_comment::add_comment_handler;
use crate::controllers::comments::like_comment::like_comment_handler;
use crate::controllers::comments::reply_to_comment::reply_to_comment_handler;
use crate::controllers::dto::*;
use crate::controllers::like_post::like_post_handler;
use crate::models::PostArtworkMetadata;
use crate::types::*;
use crate::*;
use dioxus::prelude::*;

translate! {
    PostDetailTranslate;

    reply: {
        en: "reply",
        ko: "답글",
    },

    replies: {
        en: "replies",
        ko: "답글",
    },

    share_your_thoughts: {
        en: "Share your thoughts...",
        ko: "의견을 남겨주세요...",
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

    let post = match &detail.post {
        Some(p) => p.clone(),
        None => return rsx! {},
    };

    let mut optimistic_liked = use_signal(|| detail.is_liked);
    let mut optimistic_likes = use_signal(|| post.likes);
    let mut is_processing = use_signal(|| false);

    let post_pk_for_like = post_pk.clone();

    let img_class = if post.author_type == ratel_auth::UserType::Team {
        "w-8 h-8 rounded-sm object-cover"
    } else {
        "w-8 h-8 rounded-full object-cover"
    };

    rsx! {
        div { class: "flex flex-col gap-2.5 w-full",
            // Back button row
            div { class: "flex flex-row justify-between items-center",
                button {
                    class: "p-1 cursor-pointer",
                    aria_label: "{t.back}",
                    onclick: move |_| {
                        nav.go_back();
                    },
                    icons::arrows::ArrowLeft { class: "[&>path]:stroke-text-primary" }
                }
            }
            // Stats row
            div { class: "flex flex-row justify-end gap-4 items-center w-full",
                // Like button
                button {
                    class: "flex items-center gap-1 cursor-pointer disabled:cursor-not-allowed disabled:opacity-50",
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
                        icons::emoji::ThumbsUp { class: "w-5 h-5 [&>path]:fill-primary [&>path]:stroke-primary" }
                    } else {
                        icons::emoji::ThumbsUp { class: "w-5 h-5 [&>path]:stroke-icon" }
                    }
                    span { class: "text-[15px] text-text-primary",
                        "{convert_number_to_string(*optimistic_likes.read())}"
                    }
                }
                // Comments count
                div { class: "flex gap-1 items-center",
                    icons::chat::SquareChat { class: "w-5 h-5 [&>path]:stroke-icon" }
                    span { class: "text-[15px] text-text-primary",
                        "{convert_number_to_string(post.comments)}"
                    }
                }
                // Shares count
                div { class: "flex gap-1 items-center",
                    icons::links_share::Share1 { class: "w-5 h-5 [&>path]:stroke-icon" }
                    span { class: "text-[15px] text-text-primary",
                        "{convert_number_to_string(post.shares)}"
                    }
                }
            }
            // Title
            h2 { class: "text-xl font-bold text-text-primary",
                "{post.title}"
            }
            // Author and time
            div { class: "flex flex-row justify-between items-center",
                div { class: "flex flex-row gap-2 items-center",
                    if !post.author_profile_url.is_empty() {
                        img {
                            src: "{post.author_profile_url}",
                            alt: "{post.author_display_name}",
                            class: "{img_class}",
                        }
                    } else {
                        div { class: "w-8 h-8 rounded-full bg-profile-bg" }
                    }
                    span { class: "font-semibold text-sm text-text-primary",
                        "{post.author_display_name}"
                    }
                }
                span { class: "font-light text-sm text-text-primary",
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
            div { class: "flex flex-col md:flex-row w-full min-h-[600px] border rounded-[10px] bg-card-bg-secondary border-card-enable-border overflow-hidden",
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
                div { class: "flex flex-col gap-6 flex-1 p-8 bg-card-bg-secondary",
                    div { class: "flex flex-col gap-1",
                        p { class: "text-sm text-text-secondary", "Artwork Name" }
                        h1 { class: "text-2xl font-bold text-text-primary", "{post.title}" }
                    }
                    if !detail.artwork_metadata.is_empty() {
                        ArtworkMetadataSection { metadata: detail.artwork_metadata.clone() }
                    }
                    if !post.html_contents.is_empty() {
                        div { class: "flex flex-col gap-2",
                            h2 { class: "text-lg font-semibold text-text-primary", "Description" }
                            div { class: "text-text-primary",
                                div { dangerous_inner_html: "{post.html_contents}" }
                            }
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
            div { class: "flex flex-col w-full border rounded-[10px] bg-card-bg-secondary border-card-enable-border",
                div { class: "break-all text-desc-text",
                    div { class: "px-5 py-5 border-none",
                        div {
                            dangerous_inner_html: "{post.html_contents}",
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
                        span { class: "text-xs text-text-secondary font-semibold",
                            "{item.value}"
                        }
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
        div { id: "comments", class: "flex flex-col gap-4",
            // Comment count header
            div { class: "flex flex-row gap-2 items-center text-text-primary",
                icons::chat::SquareChat { class: "w-6 h-6 [&>path]:stroke-text-primary" }
                span { class: "text-base font-medium",
                    "{comment_count} {count_label}"
                }
            }
            // Write a comment area
            if !*expand_comment.read() {
                button {
                    class: "flex flex-row w-full px-3.5 py-3 gap-2 bg-write-comment-box-bg border border-write-comment-box-border items-center rounded-lg hover:bg-write-comment-box-bg/80 hover:border-primary/50 transition-all duration-200 cursor-pointer group",
                    onclick: move |_| {
                        expand_comment.set(true);
                    },
                    icons::chat::SquareChat { class: "w-6 h-6 [&>path]:stroke-write-comment-box-icon group-hover:[&>path]:stroke-primary" }
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
                                        use_navigator().push(crate::Route::PostDetail { post_pk: pk });
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
    let mut reply_text = use_signal(|| String::new());
    let mut is_reply_submitting = use_signal(|| false);

    let comment_sk_for_like = comment.sk.clone();
    let comment_sk_for_reply = comment.sk.clone();

    let img_class = "w-8 h-8 rounded-full object-cover";

    let updated_secs = comment.updated_at * 1000;

    rsx! {
        div { class: "flex flex-col gap-2 p-4 border border-card-enable-border rounded-lg bg-card-bg-secondary",
            // Author row
            div { class: "flex flex-row justify-between items-center",
                div { class: "flex flex-row gap-2 items-center",
                    if !comment.author_profile_url.is_empty() {
                        img {
                            src: "{comment.author_profile_url}",
                            alt: "{comment.author_display_name}",
                            class: "{img_class}",
                        }
                    } else {
                        div { class: "w-8 h-8 rounded-full bg-profile-bg" }
                    }
                    span { class: "font-semibold text-sm text-text-primary",
                        "{comment.author_display_name}"
                    }
                }
                span { class: "font-light text-xs text-text-secondary",
                    "{time_ago(updated_secs)}"
                }
            }
            // Content
            div { class: "text-sm text-text-primary pl-10",
                "{comment.content}"
            }
            // Actions row
            div { class: "flex flex-row gap-4 pl-10",
                // Like button
                button {
                    class: "flex items-center gap-1 cursor-pointer text-sm",
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
                        icons::emoji::ThumbsUp { class: "w-4 h-4 [&>path]:fill-primary [&>path]:stroke-primary" }
                    } else {
                        icons::emoji::ThumbsUp { class: "w-4 h-4" }
                    }
                    span { class: "text-text-primary",
                        "{convert_number_to_string(*optimistic_likes.read())}"
                    }
                }
                // Reply button
                button {
                    class: "flex items-center gap-1 cursor-pointer text-sm text-text-primary hover:text-primary",
                    onclick: move |_| {
                        let current = *show_reply.read();
                        show_reply.set(!current);
                    },
                    icons::chat::SquareChat { class: "w-4 h-4" }
                    "{t.reply_button}"
                    if comment.replies > 0 {
                        span { class: "text-text-secondary",
                            " ({comment.replies})"
                        }
                    }
                }
            }
            // Inline reply form
            if *show_reply.read() {
                div { class: "flex flex-col gap-2 pl-10 mt-2",
                    textarea {
                        class: "w-full min-h-[60px] p-2 bg-transparent text-text-primary border border-divider rounded resize-none focus:outline-none focus:border-primary text-sm",
                        placeholder: "{t.share_your_thoughts}",
                        value: "{reply_text}",
                        oninput: move |e| {
                            reply_text.set(e.value());
                        },
                    }
                    div { class: "flex flex-row justify-end gap-2",
                        button {
                            class: "px-3 py-1 text-xs text-text-primary rounded hover:bg-divider cursor-pointer",
                            onclick: move |_| {
                                show_reply.set(false);
                                reply_text.set(String::new());
                            },
                            "{t.cancel}"
                        }
                        button {
                            class: "px-3 py-1 text-xs text-white bg-primary rounded hover:bg-primary/80 cursor-pointer disabled:opacity-50",
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
                                        use_navigator().push(crate::Route::PostDetail { post_pk: pk });
                                    });
                                }
                            },
                            "{t.reply_button}"
                        }
                    }
                }
            }
            // Reply list
            if !replies.is_empty() {
                div { class: "flex flex-col gap-2 pl-10 mt-2 border-l-2 border-divider",
                    for reply in &replies {
                        ReplyItem {
                            key: "{reply.sk}",
                            reply: reply.clone(),
                            post_pk: post_pk.clone(),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ReplyItem(reply: PostCommentResponse, post_pk: String) -> Element {
    let mut optimistic_liked = use_signal(|| reply.liked);
    let mut optimistic_likes = use_signal(|| reply.likes as i64);
    let mut is_processing = use_signal(|| false);

    let reply_sk = reply.sk.clone();
    let updated_secs = reply.updated_at * 1000;

    rsx! {
        div { class: "flex flex-col gap-1 p-3",
            div { class: "flex flex-row justify-between items-center",
                div { class: "flex flex-row gap-2 items-center",
                    if !reply.author_profile_url.is_empty() {
                        img {
                            src: "{reply.author_profile_url}",
                            alt: "{reply.author_display_name}",
                            class: "w-6 h-6 rounded-full object-cover",
                        }
                    } else {
                        div { class: "w-6 h-6 rounded-full bg-profile-bg" }
                    }
                    span { class: "font-semibold text-xs text-text-primary",
                        "{reply.author_display_name}"
                    }
                }
                span { class: "font-light text-xs text-text-secondary",
                    "{time_ago(updated_secs)}"
                }
            }
            div { class: "text-sm text-text-primary pl-8",
                "{reply.content}"
            }
            div { class: "flex flex-row gap-4 pl-8",
                button {
                    class: "flex items-center gap-1 cursor-pointer text-xs",
                    disabled: *is_processing.read(),
                    onclick: {
                        let pk = post_pk.clone();
                        let sk = reply_sk.clone();
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
                        icons::emoji::ThumbsUp { class: "w-3.5 h-3.5 [&>path]:fill-primary [&>path]:stroke-primary" }
                    } else {
                        icons::emoji::ThumbsUp { class: "w-3.5 h-3.5" }
                    }
                    span { class: "text-text-primary",
                        "{convert_number_to_string(*optimistic_likes.read())}"
                    }
                }
            }
        }
    }
}
