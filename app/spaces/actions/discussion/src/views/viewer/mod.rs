use crate::controllers::{add_comment, get_discussion, list_comments};
use crate::*;
use space_common::types::space_page_actions_discussion_key;

#[component]
pub fn ViewerPage(space_id: SpacePartition, discussion_id: SpacePostEntityType) -> Element {
    let nav = navigator();
    let key = space_page_actions_discussion_key(&space_id, &discussion_id);
    let discussion_loader = use_query(&key, {
        let space_id = space_id.clone();
        let discussion_id = discussion_id.clone();
        move || get_discussion(space_id.clone(), discussion_id.clone())
    })?;

    let discussion = discussion_loader.read().clone();

    //FIXME: use InfiniteQuery
    let comments_loader = use_query(&key, {
        let space_id = space_id.clone();
        let discussion_id = discussion_id.clone();
        move || list_comments(space_id.clone(), discussion_id.clone(), None)
    })?;

    let comments = comments_loader.read().clone();

    let mut comment_input = use_signal(String::new);

    let on_back = move |_| {
        nav.go_back();
    };

    rsx! {
        div { class: "flex flex-col gap-5 w-full",
            // Back button
            button {
                class: "flex items-center gap-2 text-sm text-neutral-400 hover:text-white light:text-neutral-600 light:hover:text-neutral-900 transition-colors",
                onclick: on_back,
                "← Back"
            }
            div { class: "flex flex-col gap-5",
                // Title
                h1 { class: "text-2xl font-bold text-white light:text-neutral-900",
                    if discussion.title.is_empty() {
                        "Untitled Discussion"
                    } else {
                        "{discussion.title}"
                    }
                }

                // Author info
                div { class: "flex items-center gap-3 text-sm text-neutral-400 light:text-neutral-600",
                    if !discussion.author_profile_url.is_empty() {
                        img {
                            class: "w-6 h-6 rounded-full",
                            src: "{discussion.author_profile_url}",
                        }
                    }
                    span { class: "font-medium", "{discussion.author_display_name}" }
                    if !discussion.category_name.is_empty() {
                        span { class: "px-2 py-0.5 rounded bg-neutral-700 light:bg-neutral-200 text-xs",
                            "{discussion.category_name}"
                        }
                    }
                }

                // Content
                if !discussion.html_contents.is_empty() {
                    div {
                        class: "prose prose-invert light:prose max-w-none text-neutral-200 light:text-neutral-800",
                        dangerous_inner_html: "{discussion.html_contents}",
                    }
                }

                // Divider
                hr { class: "border-neutral-700 light:border-neutral-300" }
            }
            div { class: "flex flex-col gap-4",
                h2 { class: "text-lg font-bold text-white light:text-neutral-900",
                    "Comments"
                }

                // Add comment input
                div { class: "flex gap-2",
                    input {
                        class: "flex-1 px-4 py-2 rounded-lg bg-neutral-800 light:bg-neutral-100 border border-neutral-700 light:border-neutral-300 text-white light:text-neutral-900 text-sm placeholder-neutral-500",
                        placeholder: "Write a comment...",
                        value: "{comment_input}",
                        oninput: move |e| comment_input.set(e.value()),
                    }
                    button {
                        class: "px-4 py-2 rounded-lg bg-yellow-400 light:bg-yellow-500 text-neutral-900 text-sm font-bold hover:opacity-90 transition-opacity disabled:opacity-50",
                        disabled: comment_input().is_empty(),
                        onclick: {
                            let space_id = space_id.clone();
                            let discussion_id = discussion_id.clone();
                            move |_| {
                                let content = comment_input();
                                if content.is_empty() {
                                    return;
                                }
                                comment_input.set(String::new());
                                let space_id = space_id.clone();
                                let discussion_id = discussion_id.clone();
                                spawn(async move {
                                    let req = crate::controllers::AddCommentRequest {
                                        content,
                                    };
                                    match add_comment(space_id, discussion_id, req).await {
                                        Ok(_) => {}
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

                // Comments list
                div { class: "flex flex-col gap-3",
                    for comment in comments.iter() {
                        CommentItem {
                            key: "{comment.sk}",
                            content: comment.content.clone(),
                            author_display_name: comment.author_display_name.clone(),
                            author_profile_url: comment.author_profile_url.clone(),
                            likes: comment.likes,
                            replies: comment.replies,
                            liked: comment.liked,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CommentItem(
    content: String,
    author_display_name: String,
    author_profile_url: String,
    likes: u64,
    replies: u64,
    liked: bool,
) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2 p-3 rounded-lg border border-neutral-700 light:border-neutral-300 bg-neutral-800/50 light:bg-neutral-50",
            div { class: "flex items-center gap-2 text-sm",
                if !author_profile_url.is_empty() {
                    img {
                        class: "w-5 h-5 rounded-full",
                        src: "{author_profile_url}",
                    }
                }
                span { class: "font-medium text-white light:text-neutral-900", {author_display_name} }
            }
            p { class: "text-sm text-neutral-300 light:text-neutral-700", {content} }
            div { class: "flex items-center gap-4 text-xs text-neutral-500",
                span { class: if liked { "text-yellow-400" } else { "" }, "{likes} likes" }
                if replies > 0 {
                    span { "{replies} replies" }
                }
            }
        }
    }
}
