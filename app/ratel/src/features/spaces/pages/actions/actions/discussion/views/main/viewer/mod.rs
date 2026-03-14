mod context;

use crate::features::spaces::space_common::types::{
    space_page_actions_discussion_comments_key, space_page_actions_discussion_key,
};

use super::*;
use context::Context;
use context::*;

#[component]
pub fn ViewerMain(
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
) -> Element {
    let mut comment_input = use_signal(String::new);
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
                space_id: space_id(),
                discussion_id: discussion_id(),
                discussion,
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
    space_id: SpacePartition,
    discussion_id: SpacePostEntityType,
    discussion: SpacePost,
    can_comment: bool,
    is_creator: bool,
) -> Element {
    let mut comment_input = use_signal(String::new);
    let mut ctx = use_discussion_comment_context();
    let comments = ctx.comments.items();
    let more_comments = ctx.comments.more_element();

    rsx! {
        div { class: "flex flex-col gap-4",
            h2 { class: "text-lg font-bold text-white light:text-neutral-900",
                "Comments ({discussion.comments})"
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
                        disabled: comment_input().is_empty(),
                        onclick: {
                            let space_id = space_id.clone();
                            let discussion_id = discussion_id.clone();
                            move |_| {
                                let content = comment_input();
                                comment_input.set(String::new());
                                let space_id = space_id.clone();
                                let discussion_id = discussion_id.clone();
                                async move {
                                    if content.is_empty() {
                                        return;
                                    }
                                    let req = AddCommentRequest { content };
                                    match add_comment(space_id.clone(), discussion_id.clone(), req).await {
                                        Ok(comment) => {
                                            ctx.comments.insert(comment);
                                        }
                                        Err(e) => {
                                            error!("Failed to add comment: {:?}", e);
                                        }
                                    }
                                }
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
                        let comment_sk: SpacePostCommentEntityType = comment
                            .sk
                            .clone()
                            .try_into()
                            .unwrap_or_default();
                        rsx! {
                            CommentItem {
                                key: "{comment.sk}",
                                space_id: space_id.clone(),
                                discussion_id: discussion_id.clone(),
                                comment_sk,
                                content: comment.content.clone(),
                                author_pk: comment.author_pk.clone(),
                                author_display_name: comment.author_display_name.clone(),
                                author_profile_url: comment.author_profile_url.clone(),
                                likes: comment.likes,
                                replies: comment.replies,
                                liked: comment.liked,
                                is_creator,
                                can_interact: can_comment,
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
    space_id: SpacePartition,
    discussion_id: SpacePostEntityType,
    comment_sk: SpacePostCommentEntityType,
    content: String,
    author_pk: Partition,
    author_display_name: String,
    author_profile_url: String,
    likes: u64,
    replies: u64,
    liked: bool,
    is_creator: bool,
    can_interact: bool,
) -> Element {
    let mut show_reply_input = use_signal(|| false);
    let mut reply_input = use_signal(String::new);

    rsx! {
        div { class: "flex flex-col gap-2 p-3 rounded-lg border border-neutral-700 light:border-neutral-300 bg-neutral-800/50 light:bg-neutral-50",
            div { class: "flex justify-between items-center",
                div { class: "flex gap-2 items-center text-sm",
                    if !author_profile_url.is_empty() {
                        img {
                            class: "w-5 h-5 rounded-full",
                            src: "{author_profile_url}",
                        }
                    }
                    span { class: "font-medium text-white light:text-neutral-900",
                        {author_display_name}
                    }
                }
                if is_creator {
                    DeleteCommentButton {
                        space_id: space_id.clone(),
                        discussion_id: discussion_id.clone(),
                        comment_sk: comment_sk.clone(),
                    }
                }
            }
            p { class: "text-sm text-neutral-300 light:text-neutral-700", {content} }
            div { class: "flex gap-4 items-center text-xs text-neutral-500",
                if can_interact {
                    LikeButton {
                        space_id: space_id.clone(),
                        discussion_id: discussion_id.clone(),
                        comment_sk: comment_sk.clone(),
                        likes,
                        liked,
                    }
                    button {
                        class: "transition-colors hover:text-white",
                        onclick: move |_| show_reply_input.toggle(),
                        "Reply ({replies})"
                    }
                } else {
                    span { class: if liked { "text-yellow-400" } else { "" }, "♥ {likes}" }
                    if replies > 0 {
                        span { "{replies} replies" }
                    }
                }
            }
            if show_reply_input() {
                ReplyInput {
                    space_id: space_id.clone(),
                    discussion_id: discussion_id.clone(),
                    comment_sk: comment_sk.clone(),
                    reply_input,
                    show_reply_input,
                }
            }
        }
    }
}

#[component]
fn DeleteCommentButton(
    space_id: SpacePartition,
    discussion_id: SpacePostEntityType,
    comment_sk: SpacePostCommentEntityType,
) -> Element {
    use crate::features::spaces::pages::actions::actions::discussion::controllers::delete_comment;
    let mut query = use_query_store();

    rsx! {
        button {
            class: "text-xs text-red-400 transition-colors hover:text-red-300",
            onclick: {
                let space_id = space_id.clone();
                let discussion_id = discussion_id.clone();
                let comment_sk = comment_sk.clone();
                move |_| {
                    let space_id = space_id.clone();
                    let discussion_id = discussion_id.clone();
                    let comment_sk = comment_sk.clone();
                    async move {
                        match delete_comment(space_id.clone(), discussion_id.clone(), comment_sk)
                            .await
                        {
                            Ok(_) => {
                                let discussion_key = space_page_actions_discussion_key(
                                    &space_id,
                                    &discussion_id,
                                );
                                let comments_key = space_page_actions_discussion_comments_key(
                                    &space_id,
                                    &discussion_id,
                                );
                                query.invalidate(&discussion_key);
                                query.invalidate(&comments_key);
                            }
                            Err(e) => {
                                error!("Failed to delete comment: {:?}", e);
                            }
                        }
                    }
                }
            },
            "Delete"
        }
    }
}

#[component]
fn LikeButton(
    space_id: SpacePartition,
    discussion_id: SpacePostEntityType,
    comment_sk: SpacePostCommentEntityType,
    likes: u64,
    liked: bool,
) -> Element {
    let mut query = use_query_store();

    rsx! {
        button {
            class: "flex gap-1 items-center transition-colors hover:text-yellow-400",
            class: if liked { "text-yellow-400" } else { "" },
            onclick: {
                let space_id = space_id.clone();
                let discussion_id = discussion_id.clone();
                let comment_sk = comment_sk.clone();
                move |_| {
                    let space_id = space_id.clone();
                    let discussion_id = discussion_id.clone();
                    let comment_sk = comment_sk.clone();
                    async move {
                        let req = LikeCommentRequest { like: !liked };
                        match like_comment(
                                space_id.clone(),
                                discussion_id.clone(),
                                comment_sk,
                                req,
                            )
                            .await
                        {
                            Ok(_) => {
                                let comments_key = space_page_actions_discussion_comments_key(
                                    &space_id,
                                    &discussion_id,
                                );
                                query.invalidate(&comments_key);
                            }
                            Err(e) => {
                                error!("Failed to like comment: {:?}", e);
                            }
                        }
                    }
                }
            },
            if liked {
                "♥ {likes}"
            } else {
                "♡ {likes}"
            }
        }
    }
}

#[component]
fn ReplyInput(
    space_id: SpacePartition,
    discussion_id: SpacePostEntityType,
    comment_sk: SpacePostCommentEntityType,
    reply_input: Signal<String>,
    show_reply_input: Signal<bool>,
) -> Element {
    let mut reply_input = reply_input;
    let mut show_reply_input = show_reply_input;
    let mut ctx = use_discussion_comment_context();
    let mut query = use_query_store();

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
                disabled: reply_input().is_empty(),
                onclick: {
                    let space_id = space_id.clone();
                    let discussion_id = discussion_id.clone();
                    let comment_sk = comment_sk.clone();
                    move |_| {
                        let content = reply_input();
                        reply_input.set(String::new());
                        show_reply_input.set(false);
                        let space_id = space_id.clone();
                        let discussion_id = discussion_id.clone();
                        let comment_sk = comment_sk.clone();
                        async move {
                            if content.is_empty() {
                                return;
                            }
                            let req = ReplyCommentRequest { content };
                            match reply_comment(
                                    space_id.clone(),
                                    discussion_id.clone(),
                                    comment_sk,
                                    req,
                                )
                                .await
                            {
                                Ok(_) => {
                                    ctx.comments.restart();
                                    let discussion_key =
                                        space_page_actions_discussion_key(&space_id, &discussion_id);
                                    query.invalidate(&discussion_key);
                                }
                                Err(e) => {
                                    error!("Failed to reply: {:?}", e);
                                }
                            }
                        }
                    }
                },
                "Reply"
            }
        }
    }
}
