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
                    class: "flex gap-2 items-center text-sm transition-colors hover:text-white text-neutral-400 light:text-neutral-600 light:hover:text-neutral-900",
                    onclick: on_back,
                    "← Back"
                }
                div { class: "flex gap-2",
                    button {
                        class: "py-2 px-4 text-sm font-bold text-red-500 rounded-lg border border-red-500 transition-colors hover:bg-red-500/10",
                        onclick: on_delete,
                        "Delete"
                    }
                    button {
                        class: "py-2 px-4 text-sm font-bold bg-yellow-400 rounded-lg transition-opacity hover:opacity-90 light:bg-yellow-500 text-neutral-900",
                        onclick: on_save,
                        "Save"
                    }
                }
            }

            // Editor form
            div { class: "flex flex-col gap-4",
                // Title
                div { class: "flex flex-col gap-1",
                    label { class: "text-sm font-medium text-neutral-400 light:text-neutral-600",
                        "Title"
                    }
                    input {
                        class: "py-3 px-4 w-full text-base text-white rounded-lg border bg-neutral-800 light:bg-neutral-100 border-neutral-700 light:border-neutral-300 light:text-neutral-900 placeholder-neutral-500",
                        placeholder: "Enter discussion title...",
                        value: "{title}",
                        oninput: move |e| title.set(e.value()),
                    }
                }

                // Category
                div { class: "flex flex-col gap-1",
                    label { class: "text-sm font-medium text-neutral-400 light:text-neutral-600",
                        "Category"
                    }
                    input {
                        class: "py-3 px-4 w-full text-sm text-white rounded-lg border bg-neutral-800 light:bg-neutral-100 border-neutral-700 light:border-neutral-300 light:text-neutral-900 placeholder-neutral-500",
                        placeholder: "Enter category (optional)...",
                        value: "{category_name}",
                        oninput: move |e| category_name.set(e.value()),
                    }
                }

                // Content
                div { class: "flex flex-col gap-1",
                    label { class: "text-sm font-medium text-neutral-400 light:text-neutral-600",
                        "Content"
                    }
                    textarea {
                        class: "py-3 px-4 w-full text-sm text-white rounded-lg border resize-y min-h-[300px] bg-neutral-800 light:bg-neutral-100 border-neutral-700 light:border-neutral-300 light:text-neutral-900 placeholder-neutral-500",
                        placeholder: "Write your discussion content...",
                        value: "{html_contents}",
                        oninput: move |e| html_contents.set(e.value()),
                    }
                }
            }
        }
    }
}
