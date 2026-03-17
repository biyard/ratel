mod context;

use super::*;
use crate::common::hooks::use_infinite_query;
use context::Context;
use context::*;

#[component]
pub fn ViewerMain(
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
) -> Element {
    let role = use_space_role()();
    let can_comment = matches!(role, SpaceUserRole::Creator | SpaceUserRole::Participant);
    let ctx = use_discussion_context();
    let discussion = ctx.discussion().post;
    Context::init(space_id, discussion_id)?;

    rsx! {
        div { class: "flex flex-col gap-5 w-full",
            div { class: "flex justify-between items-center",
                button {
                    class: "flex gap-2 items-center text-sm transition-colors hover:text-white text-neutral-400 light:text-neutral-600 light:hover:text-neutral-900",
                    onclick: move |_| {
                        navigator().go_back();
                    },
                    "← Back"
                }
            }
            DiscussionContent { discussion: discussion.clone() }
            DiscussionComments {
                space_id,
                discussion_id,
                can_comment,
                is_creator: role.is_admin(),
            }
        }
    }
}

#[component]
pub fn DiscussionContent(discussion: SpacePost) -> Element {
    rsx! {
        div { class: "flex flex-col gap-5",
            h1 { class: "text-2xl font-bold text-white light:text-neutral-900",
                if discussion.title.is_empty() {
                    "Untitled Discussion"
                } else {
                    "{discussion.title}"
                }
            }
            div { class: "flex gap-3 items-center text-sm text-neutral-400 light:text-neutral-600",
                if !discussion.author_profile_url.is_empty() {
                    img {
                        class: "w-6 h-6 rounded-full",
                        src: "{discussion.author_profile_url}",
                    }
                }
                span { class: "font-medium", "{discussion.author_display_name}" }
                if !discussion.category_name.is_empty() {
                    span { class: "py-0.5 px-2 text-xs rounded bg-neutral-700 light:bg-neutral-200",
                        "{discussion.category_name}"
                    }
                }
            }
            if !discussion.html_contents.is_empty() {
                div {
                    class: "max-w-none prose prose-invert light:prose text-neutral-200 light:text-neutral-800",
                    dangerous_inner_html: "{discussion.html_contents}",
                }
            }
            hr { class: "border-neutral-700 light:border-neutral-300" }
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[component]
pub fn DiscussionComments(
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
    can_comment: bool,
    is_creator: bool,
) -> Element {
    let mut comment_input = use_signal(String::new);
    let mut ctx = use_discussion_comment_context();
    let comments = ctx.comments.items();
    let more_comments = ctx.comments.more_element();
    let comment_count = comments.len();

    rsx! {
        div { class: "flex flex-col gap-4",
            h2 { class: "text-lg font-bold text-white light:text-neutral-900",
                "Comments ({comment_count})"
            }
            if can_comment {
                div { class: "flex gap-2",
                    input {
                        class: "flex-1 py-2 px-4 text-sm text-white rounded-lg border bg-neutral-800 light:bg-neutral-100 border-neutral-700 light:border-neutral-300 light:text-neutral-900 placeholder-neutral-500",
                        placeholder: "Write a comment...",
                        value: "{comment_input}",
                        oninput: move |e| comment_input.set(e.value()),
                    }
                    button {
                        class: "py-2 px-4 text-sm font-bold bg-yellow-400 rounded-lg transition-opacity hover:opacity-90 disabled:opacity-50 light:bg-yellow-500 text-neutral-900",
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
                        "Send"
                    }
                }
            }
            div { class: "flex flex-col gap-3",
                for comment in comments.iter() {
                    {
                        let comment = comment.clone();
                        let comment_sk: SpacePostCommentEntityType = comment.sk.clone().into();
                        let mut comments_query = ctx.comments;
                        rsx! {
                            CommentItem {
                                key: "{comment.sk}",
                                space_id,
                                discussion_id,
                                comment_sk,
                                comment,
                                is_creator,
                                can_comment,
                                on_refresh_comments: move |_| comments_query.restart(),
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
}

#[component]
fn CommentItem(
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
    comment_sk: SpacePostCommentEntityType,
    comment: DiscussionCommentResponse,
    is_creator: bool,
    can_comment: bool,
    on_refresh_comments: EventHandler<()>,
) -> Element {
    let comment_sk = use_signal(|| comment_sk);
    let mut show_reply_input = use_signal(|| false);
    let mut show_replies = use_signal(|| comment.replies > 0);
    let mut reply_input = use_signal(String::new);
    let mut reply_count = use_signal(|| comment.replies);
    let mut replies_query = use_infinite_query(move |bookmark| {
        list_replies(space_id(), discussion_id(), comment_sk(), bookmark)
    })?;
    let replies = replies_query.items();
    let more_replies = replies_query.more_element();

    rsx! {
        div { class: "flex flex-col gap-2 p-3 rounded-lg border border-neutral-700 light:border-neutral-300 bg-neutral-800/50 light:bg-neutral-50",
            div { class: "flex justify-between items-center",
                div { class: "flex gap-2 items-center text-sm",
                    if !comment.author_profile_url.is_empty() {
                        img {
                            class: "w-5 h-5 rounded-full",
                            src: "{comment.author_profile_url}",
                        }
                    }
                    span { class: "font-medium text-white light:text-neutral-900",
                        {comment.author_display_name}
                    }
                }
                if is_creator {
                    DeleteCommentButton {
                        space_id,
                        discussion_id,
                        comment_sk,
                        on_deleted: move |_| {
                            on_refresh_comments.call(());
                        },
                    }
                }
            }
            p { class: "text-sm text-neutral-300 light:text-neutral-700", {comment.content.clone()} }
            div { class: "flex gap-4 items-center text-xs text-neutral-500",
                LikeButton {
                    space_id,
                    discussion_id,
                    comment_sk: SpacePostCommentTargetEntityType::from(comment.sk.clone()),
                    likes: comment.likes,
                    liked: comment.liked,
                    on_changed: move |_| {
                        on_refresh_comments.call(());
                    },
                }
                if reply_count() > 0 {
                    button {
                        class: "transition-colors hover:text-white",
                        onclick: move |_| show_replies.toggle(),
                        if show_replies() {
                            "Hide replies"
                        } else {
                            "Replies ({reply_count()})"
                        }
                    }
                }
                if can_comment {
                    button {
                        class: "transition-colors hover:text-white",
                        onclick: move |_| show_reply_input.toggle(),
                        "Reply"
                    }
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
                div { class: "ml-4 flex flex-col gap-2",
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
                                    on_refresh_replies: move |_| {
                                        replies_query.restart();
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
    on_refresh_replies: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2 rounded-lg border border-neutral-700/50 bg-neutral-900/40 p-2.5 light:border-neutral-200 light:bg-neutral-100",
            div { class: "flex items-center gap-2 text-sm",
                if !reply.author_profile_url.is_empty() {
                    img {
                        class: "size-4 rounded-full",
                        src: "{reply.author_profile_url}",
                    }
                }
                span { class: "font-medium text-white light:text-neutral-900",
                    {reply.author_display_name}
                }
            }
            p { class: "text-sm text-neutral-300 light:text-neutral-700", {reply.content.clone()} }
            div { class: "flex justify-end",
                LikeButton {
                    space_id,
                    discussion_id,
                    comment_sk: comment_sk.clone(),
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
fn DeleteCommentButton(
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
    comment_sk: ReadSignal<SpacePostCommentEntityType>,
    on_deleted: EventHandler<()>,
) -> Element {
    use crate::features::spaces::pages::actions::actions::discussion::controllers::delete_comment;

    rsx! {
        button {
            class: "text-xs text-red-400 transition-colors hover:text-red-300",
            onclick: {
                move |_| {
                    spawn(async move {
                        match delete_comment(space_id(), discussion_id(), comment_sk())
                            .await
                        {
                            Ok(_) => {
                                on_deleted.call(());
                            }
                            Err(e) => {
                                error!("Failed to delete comment: {:?}", e);
                            }
                        }
                    });
                }
            },
            "Delete"
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
        button {
            class: "flex gap-1 items-center transition-colors hover:text-yellow-400",
            class: if optimistic_liked() { "text-yellow-400" } else { "" },
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
                        match like_comment(
                                space_id(),
                                discussion_id(),
                                comment_sk,
                                req,
                            )
                            .await
                        {
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
                "♥ {optimistic_likes()}"
            } else {
                "♡ {optimistic_likes()}"
            }
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
    let mut reply_input = reply_input;
    let mut show_reply_input = show_reply_input;

    rsx! {
        div { class: "flex gap-2 mt-1",
            input {
                class: "flex-1 py-1.5 px-3 text-xs text-white rounded-lg border bg-neutral-700 light:bg-neutral-100 border-neutral-600 light:border-neutral-300 light:text-neutral-900 placeholder-neutral-500",
                placeholder: "Write a reply...",
                value: "{reply_input}",
                oninput: move |e| reply_input.set(e.value()),
            }
            button {
                class: "py-1.5 px-3 text-xs font-bold bg-yellow-400 rounded-lg hover:opacity-90 disabled:opacity-50 light:bg-yellow-500 text-neutral-900",
                disabled: reply_input().trim().is_empty(),
                onclick: {
                    move |_| {
                        let content = reply_input().trim().to_string();
                        if content.is_empty() {
                            return;
                        }
                        spawn(async move {
                            let req = ReplyCommentRequest { content };
                            match reply_comment(
                                    space_id(),
                                    discussion_id(),
                                    comment_sk(),
                                    req,
                                )
                                .await
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
                "Reply"
            }
        }
    }
}
