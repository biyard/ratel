use super::*;

#[component]
pub fn DiscussionActionEditorPage(
    space_id: SpacePartition,
    discussion_id: SpacePostEntityType,
) -> Element {
    let nav = navigator();
    let ctx = use_discussion_context();

    let discussion = ctx.discussion().post;
    let mut title = use_signal(|| discussion.title.clone());
    let mut html_contents = use_signal(|| discussion.html_contents.clone());
    let mut category_name = use_signal(|| discussion.category_name.clone());

    // Initialize fields from loaded data

    let on_back = move |_| {
        nav.go_back();
    };

    let on_save = {
        let space_id = space_id.clone();
        let discussion_id = discussion_id.clone();
        let nav = nav.clone();
        move |_| {
            let space_id = space_id.clone();
            let discussion_id = discussion_id.clone();
            let nav = nav.clone();
            let t = title();
            let h = html_contents();
            let c = category_name();
            spawn(async move {
                let req = UpdateDiscussionRequest {
                    title: Some(t),
                    html_contents: Some(h),
                    category_name: if c.is_empty() { None } else { Some(c) },
                    started_at: None,
                    ended_at: None,
                };
                match update_discussion(space_id, discussion_id, req).await {
                    Ok(_) => {
                        nav.go_back();
                    }
                    Err(e) => {
                        error!("Failed to update discussion: {:?}", e);
                    }
                }
            });
        }
    };

    let on_delete = {
        let space_id = space_id.clone();
        let discussion_id = discussion_id.clone();
        let nav = nav.clone();
        move |_| {
            let space_id = space_id.clone();
            let discussion_id = discussion_id.clone();
            let nav = nav.clone();
            spawn(async move {
                match delete_discussion(space_id.clone(), discussion_id).await {
                    Ok(_) => {
                        nav.push(Route::SpaceActionsPage {
                            space_id: space_id.clone(),
                        });
                    }
                    Err(e) => {
                        error!("Failed to delete discussion: {:?}", e);
                    }
                }
            });
        }
    };

    rsx! {
        div { class: "flex flex-col gap-5 w-full",
            // Header
            div { class: "flex justify-between items-center",
                button {
                    class: "flex gap-2 items-center text-sm transition-colors text-foreground-muted hover:text-text-primary",
                    onclick: on_back,
                    "← Back"
                }
                div { class: "flex gap-2",
                    button {
                        class: "py-2 px-4 text-sm font-bold text-destructive rounded-lg border border-destructive transition-colors hover:bg-destructive/10",
                        onclick: on_delete,
                        "Delete"
                    }
                    button {
                        class: "py-2 px-4 text-sm font-bold bg-btn-primary-bg rounded-lg transition-colors hover:bg-btn-primary-hover-bg text-btn-primary-text",
                        onclick: on_save,
                        "Save"
                    }
                }
            }

            // Editor form
            div { class: "flex flex-col gap-4",
                // Title
                div { class: "flex flex-col gap-1",
                    label { class: "text-sm font-medium text-foreground-muted",
                        "Title"
                    }
                    input {
                        class: "py-3 px-4 w-full text-base text-text-primary rounded-lg border bg-discussion-editor-bg border-discussion-editor-border placeholder-discussion-editor-placeholder",
                        placeholder: "Enter discussion title...",
                        value: "{title}",
                        oninput: move |e| title.set(e.value()),
                    }
                }

                // Category
                {
                    let categories_res = use_server_future(move || list_categories(space_id.clone()))?;
                    let available_categories = categories_res.read().as_ref()
                        .and_then(|r| r.as_ref().ok())
                        .cloned()
                        .unwrap_or_default();

                    rsx! {
                        div { class: "flex flex-col gap-1",
                            label { class: "text-sm font-medium text-foreground-muted",
                                "Category"
                            }
                            SearchInput {
                                tags: if category_name().is_empty() { vec![] } else { vec![category_name()] },
                                suggestions: available_categories,
                                placeholder: "Select category (optional)...",
                                on_add: move |tag: String| {
                                    category_name.set(tag);
                                },
                                on_remove: move |_tag: String| {
                                    category_name.set(String::new());
                                },
                            }
                        }
                    }
                }

                // Content
                div { class: "flex flex-col gap-1",
                    label { class: "text-sm font-medium text-foreground-muted",
                        "Content"
                    }
                    textarea {
                        class: "py-3 px-4 w-full text-sm text-text-primary rounded-lg border resize-y min-h-[300px] bg-discussion-editor-bg border-discussion-editor-border placeholder-discussion-editor-placeholder",
                        placeholder: "Write your discussion content...",
                        value: "{html_contents}",
                        oninput: move |e| html_contents.set(e.value()),
                    }
                }
            }
        }
    }
}
