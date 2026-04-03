use crate::common::components::TiptapEditor;
use crate::common::hooks::use_infinite_query;
use crate::features::posts::components::VisibilityModal;
use crate::features::posts::controllers::get_post::get_post_handler;
use crate::features::posts::controllers::update_post::{update_post_handler, UpdatePostRequest};
use crate::features::posts::controllers::{
    create_category_handler, create_space_handler, list_categories_handler, CreateCategoryRequest,
    CreateSpaceRequest,
};
use crate::features::posts::models::Post;
use crate::features::posts::types::Visibility;
use crate::features::posts::*;
use dioxus::prelude::*;

translate! {
    PostEditTranslate;

    page_title: {
        en: "Create Post",
        ko: "게시물 작성",
    },
    title_placeholder: {
        en: "Title",
        ko: "제목",
    },
    content_placeholder: {
        en: "Type your script",
        ko: "내용을 입력하세요",
    },
    publish: {
        en: "Publish",
        ko: "게시",
    },

    go_to_space: {
        en: "Go to Space",
        ko: "스페이스로 이동",
    },
    category_placeholder: {
        en: "Category",
        ko: "카테고리",
    },
    publishing: {
        en: "Publishing...",
        ko: "게시 중...",
    },
    saving: {
        en: "Saving...",
        ko: "저장 중...",
    },
    all_changes_saved: {
        en: "All changes saved",
        ko: "모든 변경사항 저장됨",
    },
    unsaved_changes: {
        en: "Unsaved changes",
        ko: "저장되지 않은 변경사항",
    },
    skip_creating_space: {
        en: "Skip creating space",
        ko: "스페이스 만들기 건너뛰기",
    },
    create_category: {
        en: "Create",
        ko: "생성",
    },
}

const TITLE_MAX_LENGTH: usize = 50;
const CONTENT_MIN_LENGTH: usize = 10;

#[derive(Debug, Clone, PartialEq)]
enum EditorStatus {
    Idle,
    Saving,
    Saved,
    Unsaved,
    Publishing,
}

#[component]
pub fn PostEdit(post_id: FeedPartition) -> Element {
    let tr: PostEditTranslate = use_translate();
    let mut toast = use_toast();
    let nav = use_navigator();
    let mut popup = use_popup();
    let p1 = post_id.clone();
    let res = use_loader(move || {
        let post_id = p1.clone();
        async move { get_post_handler(post_id).await }
    })?();

    let post = res.post.unwrap_or_default();
    let post_space_pk = post.space_pk.clone();
    let existing_space_id = post_space_pk.and_then(|pk| match pk {
        Partition::Space(id) => Some(id),
        _ => None,
    });
    let has_existing_space = existing_space_id.is_some();
    let initial_categories = post.categories.clone();
    let Post {
        title,
        html_contents,
        ..
    } = post;

    let mut title = use_signal(move || title.clone());
    let mut content = use_signal(move || html_contents.clone());
    let mut status = use_signal(|| EditorStatus::Idle);

    let initial_categories_for_signal = initial_categories.clone();
    let mut last_saved =
        use_signal(move || (title(), content(), initial_categories_for_signal.clone()));
    let mut skip_creating_space = use_signal(move || !has_existing_space);

    // Category state - multiple categories
    let mut categories = use_signal(move || initial_categories.clone());
    let mut is_creating_category = use_signal(|| false);

    let categories_query = use_infinite_query(move |bookmark| list_categories_handler(bookmark))?;
    let categories_query_for_memo = categories_query.clone();
    let all_categories = use_memo(move || {
        categories_query_for_memo
            .items()
            .iter()
            .map(|c| c.name.clone())
            .collect::<Vec<String>>()
    });

    // Auto-save: debounce by tracking an edit version counter.
    // Each edit increments save_version. use_effect fires on change,
    // waits 3 seconds, and only saves if no newer edits occurred.
    let save_version = use_signal(|| 0u64);
    let v = post_id.clone();
    use_effect(move || {
        let ver = save_version();
        let current_title = title();
        let current_content = content();
        let current_cats = categories();
        let saved = last_saved();
        if ver == 0 {
            return;
        }
        let post_id = v.clone();
        spawn(async move {
            crate::common::utils::time::sleep(std::time::Duration::from_secs(3)).await;

            // Only save if version hasn't changed (no newer edits during wait)
            if save_version() != ver {
                return;
            }

            let (saved_title, saved_content, saved_categories) = saved;

            if current_title == saved_title
                && current_content == saved_content
                && current_cats == saved_categories
            {
                return;
            }

            status.set(EditorStatus::Saving);
            match update_post_handler(
                post_id,
                UpdatePostRequest::Writing {
                    title: current_title.clone(),
                    content: current_content.clone(),
                    categories: Some(current_cats.clone()),
                },
            )
            .await
            {
                Ok(Post {
                    title,
                    html_contents,
                    ..
                }) => {
                    last_saved.set((title, html_contents, current_cats));
                    status.set(EditorStatus::Saved);
                }
                Err(e) => {
                    dioxus::logger::tracing::error!("Auto-save failed: {:?}", e);
                    status.set(EditorStatus::Unsaved);
                }
            }
        });
    });

    let publish_post_id = post_id.clone();
    let publish = move |visibility: Visibility| {
        let post_id = publish_post_id.clone();
        spawn(async move {
            status.set(EditorStatus::Publishing);
            match update_post_handler(
                post_id.clone(),
                UpdatePostRequest::Publish {
                    title: title(),
                    content: content(),
                    image_urls: None,
                    publish: true,
                    visibility: Some(visibility),
                    categories: Some(categories()),
                },
            )
            .await
            {
                Ok(_) => {
                    nav.replace(format!("/posts/{post_id}"));
                }
                Err(e) => {
                    dioxus::logger::tracing::error!("Publish failed: {:?}", e);
                    status.set(EditorStatus::Unsaved);
                }
            }
        });
    };

    let p1 = post_id.clone();
    let handle_create_space = move || {
        let post_id = p1.clone();

        async move {
            status.set(EditorStatus::Publishing);

            let current_title = title();
            let current_content = content();

            let publish_result = update_post_handler(
                post_id.clone(),
                UpdatePostRequest::Publish {
                    title: current_title,
                    content: current_content,
                    image_urls: None,
                    publish: false,
                    visibility: None,
                    categories: Some(categories()),
                },
            )
            .await;

            if let Err(e) = publish_result {
                toast.error(e);
                status.set(EditorStatus::Unsaved);
                return;
            }

            match create_space_handler(CreateSpaceRequest { post_id }).await {
                Ok(resp) => {
                    nav.push(format!("/spaces/{}/dashboard", resp.space_id));
                }
                Err(e) => {
                    dioxus::logger::tracing::error!("Create space failed: {:?}", e);
                    status.set(EditorStatus::Unsaved);
                }
            }
        }
    };

    let title_len = title().chars().count();
    let content_text_len = strip_html_tags(&content()).trim().len();
    let can_submit = !title().is_empty()
        && content_text_len >= CONTENT_MIN_LENGTH
        && status() != EditorStatus::Saving
        && status() != EditorStatus::Publishing;

    let action_label = use_memo(move || {
        let status = status();
        let skip_space = skip_creating_space();

        match status {
            EditorStatus::Publishing => tr.publishing,
            _ => {
                if has_existing_space {
                    tr.go_to_space
                } else if skip_space {
                    tr.publish
                } else {
                    tr.go_to_space
                }
            }
        }
    });

    rsx! {
        Container {
            bottom_sheet: rsx! {
                div { class: "flex gap-4 justify-end items-center w-full",
                    label { class: "flex gap-2 items-center text-sm cursor-pointer text-text-primary",
                        input {
                            "data-testid": "skip-space-checkbox",
                            r#type: "checkbox",
                            checked: skip_creating_space(),
                            onchange: move |e| {
                                skip_creating_space.set(e.checked());
                            },
                        }
                        span { "{tr.skip_creating_space}" }
                    }
                    Button {
                        class: "text-base min-w-[150px]",
                        disabled: !can_submit,
                        onclick: move |_| {
                            if let Some(space_id) = &existing_space_id {
                                nav.push(format!("/spaces/{}/dashboard", space_id));
                            } else if skip_creating_space() {
                                let publish = publish.clone();
                                popup.open(rsx! {
                                    VisibilityModal {
                                        on_confirm: move |visibility: Visibility| {
                                            popup.close();
                                            publish(visibility);
                                        },
                                        on_cancel: move |_| {
                                            popup.close();
                                        },
                                    }
                                });
                            } else {
                                spawn(handle_create_space());
                            }
                        },
                        {action_label()}
                    }
                }

                if status() == EditorStatus::Saving {
                    div { class: "flex gap-2 justify-center items-center mt-4 text-sm text-text-tertiary",
                        div { class: "w-4 h-4 rounded-full border-2 animate-spin border-text-tertiary border-t-transparent" }
                        span { {tr.saving} }
                    }
                }
            },
            div { class: "flex flex-col gap-5 py-5 px-4 mx-auto w-full max-w-[906px]",
                h1 { class: "text-2xl font-bold text-text-primary", {tr.page_title} }

                // Title input
                div { class: "relative",
                    Input {
                        class: "pr-14 w-full",
                        placeholder: tr.title_placeholder,
                        maxlength: TITLE_MAX_LENGTH,
                        value: title,
                        oninput: move |e: Event<FormData>| {
                            let val = e.value();
                            if val.chars().count() <= TITLE_MAX_LENGTH {
                                title.set(val.clone());
                                mark_post_unsaved(
                                    &val,
                                    &content(),
                                    &categories(),
                                    last_saved,
                                    status,
                                    save_version,
                                );
                            }
                        },
                    }
                    div { class: "absolute right-3 top-1/2 text-sm -translate-y-1/2 pointer-events-none text-text-tertiary",
                        "{title_len}/{TITLE_MAX_LENGTH}"
                    }
                }

                // Multi-category tag input using SearchInput
                SearchInput {
                    tags: categories(),
                    suggestions: all_categories(),
                    placeholder: tr.category_placeholder,
                    create_label: tr.create_category,
                    creating: is_creating_category(),
                    data_testid: Some("category-search-input".to_string()),
                    on_add: move |tag: String| {
                        let mut current = categories();
                        let lower = tag.to_lowercase();
                        if !current.iter().any(|c| c.to_lowercase() == lower) {
                            current.push(tag);
                            categories.set(current.clone());
                            mark_post_unsaved(
                                &title(),
                                &content(),
                                &current,
                                last_saved,
                                status,
                                save_version,
                            );
                        }
                    },
                    on_remove: move |tag: String| {
                        let mut current = categories();
                        current.retain(|c| c != &tag);
                        categories.set(current.clone());
                        mark_post_unsaved(
                            &title(),
                            &content(),
                            &current,
                            last_saved,
                            status,
                            save_version,
                        );
                    },
                    on_create_new: move |new_cat: String| {
                        is_creating_category.set(true);
                        let mut cats_query = categories_query.clone();
                        spawn(async move {
                            match create_category_handler(CreateCategoryRequest {
                                    name: new_cat,
                                })
                                .await
                            {
                                Ok(cat) => {
                                    let name = cat.name;
                                    let mut current = categories();
                                    let lower = name.to_lowercase();
                                    if !current.iter().any(|c| c.to_lowercase() == lower) {
                                        current.push(name);
                                        categories.set(current.clone());
                                        mark_post_unsaved(
                                            &title(),
                                            &content(),
                                            &current,
                                            last_saved,
                                            status,
                                            save_version,
                                        );
                                    }
                                    cats_query.restart();
                                }
                                Err(e) => {
                                    toast.error(e);
                                }
                            }
                            is_creating_category.set(false);
                        });
                    },
                }

                // TiptapEditor
                TiptapEditor {
                    class: "w-full rounded-md border focus-within:ring-1 min-h-[400px] bg-post-input-bg border-post-input-border focus-within:border-ring focus-within:ring-ring/50",
                    content: content(),
                    editable: true,
                    placeholder: tr.content_placeholder,
                    on_content_change: move |html: String| {
                        content.set(html.clone());
                        mark_post_unsaved(
                            &title(),
                            &html,
                            &categories(),
                            last_saved,
                            status,
                            save_version,
                        );
                    },
                }
                if status() != EditorStatus::Idle {
                    div { class: "flex absolute bottom-3 left-3 gap-2 items-center py-1 px-2 text-xs rounded text-text-tertiary bg-card",
                        match status() {
                            EditorStatus::Saving => rsx! {
                                {tr.saving}
                            },
                            EditorStatus::Saved => rsx! {
                                {tr.all_changes_saved}
                            },
                            EditorStatus::Unsaved => rsx! {
                                {tr.unsaved_changes}
                            },
                            EditorStatus::Publishing => rsx! {
                                {tr.publishing}
                            },
                            EditorStatus::Idle => rsx! { "" },
                        }
                    }
                }
            
            }
        }
    }
}

fn strip_html_tags(html: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }
    result
}

fn mark_post_unsaved(
    current_title: &str,
    current_content: &str,
    current_categories: &[String],
    last_saved: Signal<(String, String, Vec<String>)>,
    mut status: Signal<EditorStatus>,
    mut save_version: Signal<u64>,
) {
    let (saved_title, saved_content, saved_categories) = last_saved();
    if current_title == saved_title
        && current_content == saved_content
        && current_categories == saved_categories
    {
        status.set(EditorStatus::Saved);
    } else {
        status.set(EditorStatus::Unsaved);
        *save_version.write() += 1;
    }
}
