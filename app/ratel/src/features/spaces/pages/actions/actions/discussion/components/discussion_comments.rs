use crate::common::components::{
    Button, ButtonShape, ButtonSize, ButtonStyle, Input, InputVariant, TextArea,
};
use crate::common::hooks::use_infinite_query;
use crate::common::utils::time::time_ago;
use crate::features::spaces::pages::actions::actions::discussion::*;

translate! {
    DiscussionCommentsTranslate;

    comments: { en: "Comments", ko: "댓글" },
    write_comment: { en: "Write a comment...", ko: "댓글을 입력하세요..." },
    edited: { en: "(Edited)", ko: "(수정)" },
    edit: { en: "Edit", ko: "수정" },
    delete: { en: "Delete", ko: "삭제" },
    cancel: { en: "Cancel", ko: "취소" },
    complete_edit: { en: "Save", ko: "수정 완료" },
    write_reply: { en: "Write a reply...", ko: "답글을 입력하세요..." },
    responses: { en: "responses", ko: "응답" },
}

fn to_millis(ts: i64) -> i64 {
    if ts.abs() < 1_000_000_000_000 {
        ts.saturating_mul(1000)
    } else {
        ts
    }
}

#[component]
pub fn DiscussionComments(
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
    can_comment: bool,
    can_manage_comments: bool,
    current_user_pk: Option<String>,
) -> Element {
    let tr: DiscussionCommentsTranslate = use_translate();
    DiscussionCommentContext::init(space_id, discussion_id)?;
    let mut comment_input = use_signal(String::new);
    let mut ctx = use_discussion_comment_context();
    let comments = ctx.comments.items();
    let more_comments = ctx.comments.more_element();
    let comment_count: usize = comments
        .iter()
        .map(|comment| 1usize + (comment.replies as usize))
        .sum();

    rsx! {
        div { class: "flex flex-col gap-4",
            h2 { class: "text-lg font-bold text-text-primary", "{tr.comments} ({comment_count})" }
            if can_comment {
                div { class: "flex gap-2",
                    Input {
                        variant: InputVariant::Default,
                        class: "flex-1 h-10".to_string(),
                        placeholder: "{tr.write_comment}",
                        value: comment_input(),
                        oninput: move |e: Event<FormData>| comment_input.set(e.value()),
                    }
                    Button {
                        style: ButtonStyle::Primary,
                        shape: ButtonShape::Rounded,
                        size: ButtonSize::Icon,
                        class: "size-10 shrink-0 !p-0 inline-flex items-center justify-center".to_string(),
                        disabled: comment_input().trim().is_empty(),
                        onclick: {
                            move |_| {
                                let content = comment_input().trim().to_string();
                                if content.is_empty() {
                                    return;
                                }
                                comment_input.set(String::new());
                                let mut comments_query = ctx.comments;
                                spawn(async move {
                                    let req = AddCommentRequest { content };
                                    match add_comment(space_id(), discussion_id(), req).await {
                                        Ok(comment) => {
                                            comments_query.insert(comment);
                                        }
                                        Err(e) => {
                                            error!("Failed to add comment: {:?}", e);
                                        }
                                    }
                                });
                            }
                        },
                        if comment_input().trim().is_empty() {
                            span { class: "inline-flex items-center justify-center leading-none",
                                icons::chat::SquareChat { class: "size-5 [&>path]:stroke-btn-primary-disable-text [&>path]:fill-transparent" }
                            }
                        } else {
                            span { class: "inline-flex items-center justify-center leading-none",
                                icons::chat::SquareChat { class: "size-5 [&>path]:stroke-btn-primary-text [&>path]:fill-transparent" }
                            }
                        }
                    }
                }
            }
            div { class: "flex flex-col divide-y divide-divider",
                for comment in comments.iter() {
                    {
                        let comment = comment.clone();
                        let comment_sk: SpacePostCommentEntityType = comment.sk.clone().into();
                        let mut comments_query = ctx.comments;
                        rsx! {
                            div { key: "{comment.sk}", class: "py-3 first:pt-0 last:pb-0",
                                CommentItem {
                                    space_id,
                                    discussion_id,
                                    comment_sk,
                                    comment,
                                    can_comment,
                                    can_manage_comments,
                                    current_user_pk: current_user_pk.clone(),
                                    on_refresh_comments: move |_| comments_query.restart(),
                                }
                            }
                        }
                    }
                }
            }
            if ctx.comments.is_loading() {
                LoadingIndicator { max_width: "100px" }
            } else {
                {more_comments}
            }
        }
    }
}

#[component]
fn CommentItem(
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
    comment_sk: SpacePostCommentEntityType,
    comment: DiscussionCommentResponse,
    can_comment: bool,
    can_manage_comments: bool,
    current_user_pk: Option<String>,
    on_refresh_comments: EventHandler<()>,
) -> Element {
    let tr: DiscussionCommentsTranslate = use_translate();
    let comment_sk = use_signal(|| comment_sk);
    let comment_target = SpacePostCommentTargetEntityType::from(comment.sk.clone());
    let delete_target = comment_target.clone();
    let edit_target = comment_target.clone();
    let like_target = comment_target.clone();
    let original_content_for_edit = comment.content.clone();
    let original_content_for_cancel = comment.content.clone();
    let comment_time_label = time_ago(to_millis(comment.updated_at));
    let is_my_comment = current_user_pk
        .as_ref()
        .is_some_and(|pk| *pk == comment.author_pk.to_string());
    let can_manage_this_comment = can_manage_comments && is_my_comment;
    let mut show_action_menu = use_signal(|| false);
    let mut is_editing = use_signal(|| false);
    let mut edit_content = use_signal(|| comment.content.clone());
    let mut show_reply_input = use_signal(|| false);
    let mut show_replies = use_signal(|| false);
    let mut reply_input = use_signal(String::new);
    let mut reply_count = use_signal(|| comment.replies);
    use_effect(use_reactive(
        (&comment.sk, &comment.replies),
        move |(_, next_replies)| {
            reply_count.set(next_replies);
        },
    ));
    let mut replies_query = use_infinite_query(move |bookmark| {
        list_replies(space_id(), discussion_id(), comment_sk(), bookmark)
    })?;
    let replies = replies_query.items();
    let more_replies = replies_query.more_element();

    rsx! {
        div { class: "flex flex-col gap-3 rounded-xl bg-card px-4 py-3",
            div { class: "flex justify-between items-center",
                div { class: "flex gap-2 items-center text-sm",
                    if !comment.author_profile_url.is_empty() {
                        img {
                            class: "w-5 h-5 rounded-full",
                            src: "{comment.author_profile_url}",
                        }
                    }
                    span { class: "font-semibold text-text-primary", {comment.author_display_name} }
                    span { class: "text-xs text-text-secondary", "{comment_time_label}" }
                    if comment.updated_at > comment.created_at {
                        span { class: "text-xs text-text-secondary", "{tr.edited}" }
                    }
                }
                if can_manage_this_comment {
                    div { class: "relative",
                        Button {
                            size: ButtonSize::Icon,
                            style: ButtonStyle::Text,
                            class: "text-text-secondary hover:text-text-primary".to_string(),
                            onclick: move |_| show_action_menu.set(!show_action_menu()),
                            crate::common::icons::validations::Extra {
                                class: "size-4 [&>circle]:fill-current"
                            }
                        }
                        if show_action_menu() {
                            div { class: "absolute right-0 top-8 z-10 min-w-[110px] rounded-md bg-card p-1 shadow-lg",
                                Button {
                                    size: ButtonSize::Small,
                                    style: ButtonStyle::Text,
                                    class: "w-full !justify-start text-left text-xs text-text-primary hover:bg-transparent focus:bg-transparent rounded"
                                        .to_string(),
                                    onclick: move |_| {
                                        edit_content.set(original_content_for_edit.clone());
                                        is_editing.set(true);
                                        show_action_menu.set(false);
                                    },
                                    "{tr.edit}"
                                }
                                Button {
                                    size: ButtonSize::Small,
                                    style: ButtonStyle::Text,
                                    class: "w-full !justify-start text-left text-xs text-red-500 hover:bg-transparent focus:bg-transparent rounded"
                                        .to_string(),
                                    onclick: move |_| {
                                        let target = delete_target.clone();
                                        show_action_menu.set(false);
                                        spawn(async move {
                                            match delete_comment(space_id(), discussion_id(), target).await {
                                                Ok(_) => on_refresh_comments.call(()),
                                                Err(e) => error!("Failed to delete comment: {:?}", e),
                                            }
                                        });
                                    },
                                    "{tr.delete}"
                                }
                            }
                        }
                    }
                }
            }
            if is_editing() {
                div { class: "flex flex-col gap-2",
                    TextArea {
                        class: "w-full min-h-[84px] resize-none rounded-lg bg-input-box-bg border border-input-box-border px-3 py-2 text-sm text-text-primary outline-none"
                            .to_string(),
                        value: edit_content(),
                        oninput: move |e: Event<FormData>| edit_content.set(e.value()),
                    }
                    div { class: "flex justify-end gap-2",
                        Button {
                            style: ButtonStyle::Outline,
                            shape: ButtonShape::Square,
                            size: ButtonSize::Small,
                            onclick: move |_| {
                                edit_content.set(original_content_for_cancel.clone());
                                is_editing.set(false);
                            },
                            "{tr.cancel}"
                        }
                        Button {
                            style: ButtonStyle::Primary,
                            shape: ButtonShape::Square,
                            size: ButtonSize::Small,
                            disabled: edit_content().trim().is_empty(),
                            onclick: move |_| {
                                let content = edit_content().trim().to_string();
                                if content.is_empty() {
                                    return;
                                }
                                let target = edit_target.clone();
                                spawn(async move {
                                    let req = UpdateCommentRequest { content };
                                    match update_comment(space_id(), discussion_id(), target, req).await {
                                        Ok(_) => {
                                            is_editing.set(false);
                                            on_refresh_comments.call(());
                                        }
                                        Err(e) => error!("Failed to update comment: {:?}", e),
                                    }
                                });
                            },
                            "{tr.complete_edit}"
                        }
                    }
                }
            } else {
                p { class: "text-sm text-text-primary", {comment.content.clone()} }
            }
            div { class: "flex items-center justify-between text-xs text-text-secondary",
                Button {
                    size: ButtonSize::Inline,
                    style: ButtonStyle::Text,
                    class: "inline-flex items-center text-text-secondary hover:text-primary".to_string(),
                    onclick: move |_| {
                        if can_comment {
                            let is_open = show_replies() || show_reply_input();
                            if is_open {
                                show_replies.set(false);
                                show_reply_input.set(false);
                            } else {
                                show_replies.set(true);
                                show_reply_input.set(true);
                            }
                        } else {
                            show_replies.toggle();
                        }
                    },
                    span { class: "inline-flex items-center gap-1 leading-none",
                        icons::chat::SquareChat { class: "size-4 shrink-0 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
                        span { class: "font-normal text-text-secondary text-[12px]",
                            "{reply_count()} {tr.responses}"
                        }
                    }
                }
                LikeButton {
                    space_id,
                    discussion_id,
                    comment_sk: like_target,
                    likes: comment.likes,
                    liked: comment.liked,
                    on_changed: move |_| {
                        on_refresh_comments.call(());
                    },
                }
            }
            if can_comment && show_reply_input() {
                ReplyInput {
                    space_id,
                    discussion_id,
                    comment_sk,
                    reply_input,
                    show_reply_input,
                    on_success: move |_| {
                        reply_count.set(reply_count() + 1);
                        show_replies.set(true);
                        replies_query.restart();
                        on_refresh_comments.call(());
                    },
                }
            }
            if show_replies() && reply_count() > 0 {
                div { class: "ml-5 flex flex-col gap-2 pl-4",
                    for reply in replies.iter() {
                        {
                            let reply = reply.clone();
                            let mut replies_query = replies_query;
                            rsx! {
                                ReplyItem {
                                    key: "{reply.sk}",
                                    space_id,
                                    discussion_id,
                                    comment_sk: SpacePostCommentTargetEntityType::from(reply.sk.clone()),
                                    reply,
                                    can_manage_comments,
                                    current_user_pk: current_user_pk.clone(),
                                    on_refresh_comments: move |_| on_refresh_comments.call(()),
                                    on_refresh_replies: move |_| {
                                        replies_query.restart();
                                    },
                                    on_deleted: move |_| {
                                        reply_count.set(reply_count().saturating_sub(1));
                                        replies_query.restart();
                                        on_refresh_comments.call(());
                                    },
                                }
                            }
                        }
                    }
                    if replies_query.is_loading() {
                        LoadingIndicator { max_width: "80px" }
                    } else {
                        {more_replies}
                    }
                }
            }
        }
    }
}

#[component]
fn ReplyItem(
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
    comment_sk: SpacePostCommentTargetEntityType,
    reply: DiscussionCommentResponse,
    can_manage_comments: bool,
    current_user_pk: Option<String>,
    on_refresh_comments: EventHandler<()>,
    on_refresh_replies: EventHandler<()>,
    on_deleted: EventHandler<()>,
) -> Element {
    let tr: DiscussionCommentsTranslate = use_translate();
    let delete_target = comment_sk.clone();
    let edit_target = comment_sk.clone();
    let like_target = comment_sk.clone();
    let original_reply_for_edit = reply.content.clone();
    let original_reply_for_cancel = reply.content.clone();
    let reply_time_label = time_ago(to_millis(reply.updated_at));
    let is_my_reply = current_user_pk
        .as_ref()
        .is_some_and(|pk| *pk == reply.author_pk.to_string());
    let can_manage_this_reply = can_manage_comments && is_my_reply;
    let mut show_action_menu = use_signal(|| false);
    let mut is_editing = use_signal(|| false);
    let mut edit_content = use_signal(|| reply.content.clone());

    rsx! {
        div { class: "flex flex-col gap-2 rounded-lg border border-divider bg-card px-3 py-2.5",
            div { class: "flex items-center justify-between gap-2",
                div { class: "flex items-center gap-2 text-sm",
                    if !reply.author_profile_url.is_empty() {
                        img {
                            class: "size-4 rounded-full",
                            src: "{reply.author_profile_url}",
                        }
                    }
                    span { class: "font-semibold text-text-primary", {reply.author_display_name} }
                    span { class: "text-xs text-text-secondary", "{reply_time_label}" }
                    if reply.updated_at > reply.created_at {
                        span { class: "text-xs text-text-secondary", "{tr.edited}" }
                    }
                }
                if can_manage_this_reply {
                    div { class: "relative",
                        Button {
                            size: ButtonSize::Icon,
                            style: ButtonStyle::Text,
                            class: "text-text-secondary hover:text-text-primary".to_string(),
                            onclick: move |_| show_action_menu.set(!show_action_menu()),
                            crate::common::icons::validations::Extra {
                                class: "size-4 [&>circle]:fill-current"
                            }
                        }
                        if show_action_menu() {
                            div { class: "absolute right-0 top-8 z-10 min-w-[110px] rounded-md bg-card p-1 shadow-lg",
                                Button {
                                    size: ButtonSize::Small,
                                    style: ButtonStyle::Text,
                                    class: "w-full !justify-start text-left text-xs text-text-primary hover:bg-transparent focus:bg-transparent rounded"
                                        .to_string(),
                                    onclick: move |_| {
                                        edit_content.set(original_reply_for_edit.clone());
                                        is_editing.set(true);
                                        show_action_menu.set(false);
                                    },
                                    "{tr.edit}"
                                }
                                Button {
                                    size: ButtonSize::Small,
                                    style: ButtonStyle::Text,
                                    class: "w-full !justify-start text-left text-xs text-red-500 hover:bg-transparent focus:bg-transparent rounded"
                                        .to_string(),
                                    onclick: move |_| {
                                        let target = delete_target.clone();
                                        show_action_menu.set(false);
                                        spawn(async move {
                                            match delete_comment(space_id(), discussion_id(), target).await {
                                                Ok(_) => on_deleted.call(()),
                                                Err(e) => error!("Failed to delete reply: {:?}", e),
                                            }
                                        });
                                    },
                                    "{tr.delete}"
                                }
                            }
                        }
                    }
                }
            }
            if is_editing() {
                div { class: "flex flex-col gap-2",
                    TextArea {
                        class: "w-full min-h-[84px] resize-none rounded-lg bg-input-box-bg border border-input-box-border px-3 py-2 text-sm text-text-primary outline-none"
                            .to_string(),
                        value: edit_content(),
                        oninput: move |e: Event<FormData>| edit_content.set(e.value()),
                    }
                    div { class: "flex justify-end gap-2",
                        Button {
                            style: ButtonStyle::Outline,
                            shape: ButtonShape::Square,
                            size: ButtonSize::Small,
                            onclick: move |_| {
                                edit_content.set(original_reply_for_cancel.clone());
                                is_editing.set(false);
                            },
                            "{tr.cancel}"
                        }
                        Button {
                            style: ButtonStyle::Primary,
                            shape: ButtonShape::Square,
                            size: ButtonSize::Small,
                            disabled: edit_content().trim().is_empty(),
                            onclick: move |_| {
                                let content = edit_content().trim().to_string();
                                if content.is_empty() {
                                    return;
                                }
                                let target = edit_target.clone();
                                spawn(async move {
                                    let req = UpdateCommentRequest { content };
                                    match update_comment(space_id(), discussion_id(), target, req).await {
                                        Ok(_) => {
                                            is_editing.set(false);
                                            on_refresh_replies.call(());
                                            on_refresh_comments.call(());
                                        }
                                        Err(e) => error!("Failed to update reply: {:?}", e),
                                    }
                                });
                            },
                            "{tr.complete_edit}"
                        }
                    }
                }
            } else {
                p { class: "text-sm text-text-primary", {reply.content.clone()} }
            }
            div { class: "flex justify-end pt-1",
                LikeButton {
                    space_id,
                    discussion_id,
                    comment_sk: like_target,
                    likes: reply.likes,
                    liked: reply.liked,
                    on_changed: move |_| {
                        on_refresh_replies.call(());
                    },
                }
            }
        }
    }
}

#[component]
fn LikeButton(
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
    comment_sk: SpacePostCommentTargetEntityType,
    likes: u64,
    liked: bool,
    on_changed: EventHandler<()>,
) -> Element {
    let mut optimistic_liked = use_signal(|| liked);
    let mut optimistic_likes = use_signal(|| likes as i64);
    let mut is_processing = use_signal(|| false);

    use_effect(use_reactive(
        (&liked, &likes),
        move |(next_liked, next_likes)| {
            optimistic_liked.set(next_liked);
            optimistic_likes.set(next_likes as i64);
        },
    ));

    rsx! {
        Button {
            size: ButtonSize::Inline,
            style: ButtonStyle::Text,
            class: if optimistic_liked() { "inline-flex items-center gap-1.5 text-sm text-primary hover:text-primary"
                .to_string() } else { "inline-flex items-center gap-1.5 text-sm text-text-secondary hover:text-primary"
                .to_string() },
            disabled: is_processing(),
            onclick: {
                move |_| {
                    if is_processing() {
                        return;
                    }
                    let next_like = !optimistic_liked();
                    let prev_like = optimistic_liked();
                    let prev_likes = optimistic_likes();
                    let delta: i64 = if next_like { 1 } else { -1 };
                    optimistic_liked.set(next_like);
                    optimistic_likes.set((prev_likes + delta).max(0));
                    is_processing.set(true);

                    let comment_sk = comment_sk.clone();
                    spawn(async move {
                        let req = LikeCommentRequest {
                            like: next_like,
                        };
                        match like_comment(space_id(), discussion_id(), comment_sk, req).await {
                            Ok(_) => {
                                on_changed.call(());
                            }
                            Err(e) => {
                                optimistic_liked.set(prev_like);
                                optimistic_likes.set(prev_likes);
                                error!("Failed to like comment: {:?}", e);
                            }
                        }
                        is_processing.set(false);
                    });
                }
            },
            if optimistic_liked() {
                icons::emoji::ThumbsUp { class: "size-4 [&>path]:fill-primary [&>path]:stroke-primary" }
            } else {
                icons::emoji::ThumbsUp { class: "size-4 [&>path]:stroke-icon-primary [&>path]:fill-transparent" }
            }
            span { "{optimistic_likes()}" }
        }
    }
}

#[component]
fn ReplyInput(
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
    comment_sk: ReadSignal<SpacePostCommentEntityType>,
    reply_input: Signal<String>,
    show_reply_input: Signal<bool>,
    on_success: EventHandler<()>,
) -> Element {
    let tr: DiscussionCommentsTranslate = use_translate();
    let mut reply_input = reply_input;
    let mut show_reply_input = show_reply_input;

    rsx! {
        div { class: "mt-1 rounded-xl bg-card-bg-secondary p-3",
            TextArea {
                class: "h-20 w-full resize-none rounded-lg bg-input-box-bg border border-input-box-border px-3 py-2 text-sm text-text-primary outline-none placeholder:text-text-tertiary"
                    .to_string(),
                placeholder: "{tr.write_reply}",
                value: reply_input(),
                oninput: move |e: Event<FormData>| reply_input.set(e.value()),
            }
            div { class: "mt-2 flex justify-end",
                Button {
                    style: ButtonStyle::Primary,
                    shape: ButtonShape::Rounded,
                    size: ButtonSize::Icon,
                    class: "size-10 !p-0 inline-flex items-center justify-center".to_string(),
                    disabled: reply_input().trim().is_empty(),
                    onclick: {
                        move |_| {
                            let content = reply_input().trim().to_string();
                            if content.is_empty() {
                                return;
                            }
                            spawn(async move {
                                let req = ReplyCommentRequest { content };
                                match reply_comment(space_id(), discussion_id(), comment_sk(), req).await
                                {
                                    Ok(_) => {
                                        reply_input.set(String::new());
                                        show_reply_input.set(false);
                                        on_success.call(());
                                    }
                                    Err(e) => {
                                        error!("Failed to reply: {:?}", e);
                                    }
                                }
                            });
                        }
                    },
                    span { class: "inline-flex items-center justify-center leading-none",
                        icons::chat::SquareChat { class: "size-5 [&>path]:stroke-btn-primary-text [&>path]:fill-transparent" }
                    }
                }
            }
        }
    }
}
