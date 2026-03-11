use crate::common::components::{ButtonShape, ButtonSize, ButtonStyle, InputVariant, TiptapEditor};
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

    let Post {
        title,
        html_contents,
        ..
    } = res.post.unwrap_or_default();

    let mut title = use_signal(move || title.clone());
    let mut content = use_signal(move || html_contents.clone());
    let mut status = use_signal(|| EditorStatus::Idle);

    let mut last_saved = use_signal(move || (title(), content()));
    let mut skip_creating_space = use_signal(|| true);

    // Category state
    let mut category = use_signal(|| "".to_string());
    let mut category_input = use_signal(|| "".to_string());
    let mut show_category_dropdown = use_signal(|| false);
    let mut is_creating_category = use_signal(|| false);
    let mut category_error: Signal<Option<String>> = use_signal(|| None);

    let mut categories_query = use_infinite_query(move |bookmark| list_categories_handler(bookmark))?;
    let mut extra_categories = use_signal(|| vec![]);
    let categories = use_memo(move || {
        let mut cats: Vec<String> = categories_query.items().iter().map(|c| c.name.clone()).collect();
        cats.extend(extra_categories());
        cats
    });

    // Auto-save: debounce by tracking an edit version counter.
    // Each edit increments save_version. use_effect fires on change,
    // waits 3 seconds, and only saves if no newer edits occurred.
    let mut save_version = use_signal(|| 0u64);
    let v = post_id.clone();
    use_effect(move || {
        let ver = save_version();
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

            let current_title = title();
            let current_content = content();
            let (saved_title, saved_content) = last_saved();

            if current_title == saved_title && current_content == saved_content {
                return;
            }

            status.set(EditorStatus::Saving);
            match update_post_handler(
                post_id,
                UpdatePostRequest::Writing {
                    title: current_title.clone(),
                    content: current_content.clone(),
                },
            )
            .await
            {
                Ok(Post {
                    title,
                    html_contents,
                    ..
                }) => {
                    last_saved.set((title, html_contents));
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
                },
            )
            .await
            {
                Ok(_) => {
                    nav.push(format!("/posts/{post_id}"));
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
                if skip_space {
                    tr.publish
                } else {
                    tr.go_to_space
                }
            }
        }
    });

    rsx! {
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
                            title.set(val);
                            status.set(EditorStatus::Unsaved);
                            save_version += 1;
                        }
                    },
                }
                div { class: "absolute right-3 top-1/2 text-sm -translate-y-1/2 pointer-events-none text-text-tertiary",
                    "{title_len}/{TITLE_MAX_LENGTH}"
                }
            }

            // Category selector
            div { class: "relative w-full",
                Input {
                    class: "w-full",
                    variant: InputVariant::Default,
                    placeholder: "Category",
                    value: category_input,
                    oninput: move |e: Event<FormData>| {
                        category_input.set(e.value());
                        show_category_dropdown.set(true);
                    },
                }

                if show_category_dropdown() {
                    div {
                        class: "absolute z-20 mt-2 w-full rounded-md border shadow-md bg-card border-post-input-border max-h-60 overflow-y-auto",

                        for cat in categories()
                            .iter()
                            .filter(|c| c.to_lowercase().contains(&category_input().to_lowercase()))
                            .cloned()
                        {
                            div {
                                key: "{cat}",
                                class: "px-3 py-2 cursor-pointer hover:bg-muted text-sm text-text-primary",
                                onclick: {
                                    let cat = cat.clone();
                                    move |_| {
                                        category.set(cat.clone());
                                        category_input.set(cat.clone());
                                        show_category_dropdown.set(false);
                                    }
                                },
                                "{cat}"
                            }
                        }

                        if !category_input().is_empty()
                            && !categories().contains(&category_input())
                        {
                            div {
                                class: "px-3 py-2 cursor-pointer text-primary text-sm hover:bg-muted",
                                onclick: move |_| {
                                    if is_creating_category() { return; }
                                    is_creating_category.set(true);

                                    let new_cat = category_input();
                                    spawn(async move {
                                        match create_category_handler(CreateCategoryRequest {
                                            name: new_cat,
                                        })
                                        .await
                                        {
                                            Ok(cat) => {
                                                let name = cat.name;
                                                category.set(name.clone());
                                                category_input.set(name.clone());
                                                extra_categories.write().push(name);
                                                show_category_dropdown.set(false);
                                            }
                                            Err(e) => {
                                                category_error.set(Some(e.to_string()));
                                            }
                                        }
                                        is_creating_category.set(false);
                                    });
                                },
                                if is_creating_category() {
                                    "Creating..."
                                } else {
                                    "Create \"{category_input()}\""
                                }
                            }
                        }
                    }
                }
            }

            // TiptapEditor
            TiptapEditor {
                class: "w-full rounded-md border focus-within:ring-1 min-h-[400px] bg-post-input-bg border-post-input-border focus-within:border-ring focus-within:ring-ring/50",
                content: content(),
                editable: true,
                placeholder: tr.content_placeholder,
                on_content_change: move |html: String| {
                    content.set(html);
                    status.set(EditorStatus::Unsaved);
                    save_version += 1;
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

            // Status + actions row
            div { class: "flex gap-4 justify-end items-center",
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
                        if skip_creating_space() {
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
