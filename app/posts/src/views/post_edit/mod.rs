use crate::controllers::get_post::get_post_handler;
use crate::controllers::update_post::{update_post_handler, UpdatePostRequest};
use crate::controllers::{create_space_handler, CreateSpaceRequest};
use crate::*;
use common::components::{ButtonShape, ButtonSize, ButtonStyle, TiptapEditor};
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
pub fn PostEdit(post_pk: String) -> Element {
    let tr: PostEditTranslate = use_translate();
    let nav = use_navigator();

    let mut title = use_signal(String::new);
    let mut content = use_signal(String::new);
    let mut status = use_signal(|| EditorStatus::Idle);
    let mut last_saved = use_signal(|| (String::new(), String::new()));
    let mut initialized = use_signal(|| false);
    let mut skip_creating_space = use_signal(|| true);

    let post_pk_load = post_pk.clone();
    use_effect(move || {
        if initialized() {
            return;
        }
        initialized.set(true);
        let pk = post_pk_load.clone();
        spawn(async move {
            match get_post_handler(pk.parse().unwrap()).await {
                Ok(resp) => {
                    if let Some(post) = resp.post {
                        title.set(post.title.clone());
                        content.set(post.html_contents.clone());
                        last_saved.set((post.title, post.html_contents));
                    }
                }
                Err(e) => {
                    dioxus::logger::tracing::error!("Failed to load post: {:?}", e);
                }
            }
        });
    });

    // Auto-save: debounce by tracking an edit version counter.
    // Each edit increments save_version. use_effect fires on change,
    // waits 3 seconds, and only saves if no newer edits occurred.
    let mut save_version = use_signal(|| 0u64);
    let post_pk_save = post_pk.clone();
    use_effect(move || {
        let ver = save_version();
        if ver == 0 {
            return;
        }
        let pk = post_pk_save.clone();
        spawn(async move {
            #[cfg(feature = "web")]
            gloo_timers::future::sleep(std::time::Duration::from_secs(3)).await;
            #[cfg(feature = "server")]
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;

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
                pk.parse().unwrap(),
                UpdatePostRequest::Writing {
                    title: current_title.clone(),
                    content: current_content.clone(),
                },
            )
            .await
            {
                Ok(_) => {
                    last_saved.set((current_title, current_content));
                    status.set(EditorStatus::Saved);
                }
                Err(e) => {
                    dioxus::logger::tracing::error!("Auto-save failed: {:?}", e);
                    status.set(EditorStatus::Unsaved);
                }
            }
        });
    });

    let post_pk_publish = post_pk.clone();
    let handle_publish = move || {
        let pk = post_pk_publish.clone();
        let nav = nav.clone();
        async move {
            status.set(EditorStatus::Publishing);
            match update_post_handler(
                pk.parse().unwrap(),
                UpdatePostRequest::Publish {
                    title: title(),
                    content: content(),
                    image_urls: None,
                    publish: true,
                    visibility: None,
                },
            )
            .await
            {
                Ok(_) => {
                    let feed_pk: FeedPartition = pk.parse().unwrap();
                    nav.push(format!("/posts/{feed_pk}"));
                }
                Err(e) => {
                    dioxus::logger::tracing::error!("Publish failed: {:?}", e);
                    status.set(EditorStatus::Unsaved);
                }
            }
        }
    };

    let post_pk_create_space = post_pk.clone();
    let handle_create_space = move || {
        let pk = post_pk_create_space.clone();
        let nav = nav.clone();
        async move {
            status.set(EditorStatus::Publishing);

            let post_id: FeedPartition = pk.parse().unwrap();
            let current_title = title();
            let current_content = content();

            let publish_result = update_post_handler(
                post_id.clone(),
                UpdatePostRequest::Publish {
                    title: current_title,
                    content: current_content,
                    image_urls: None,
                    publish: true,
                    visibility: None,
                },
            )
            .await;

            if let Err(e) = publish_result {
                dioxus::logger::tracing::error!("Publish failed before space create: {:?}", e);
                status.set(EditorStatus::Unsaved);
                return;
            }

            match create_space_handler(CreateSpaceRequest { post_pk: post_id }).await {
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

    let title_len = title().len();
    let content_text_len = strip_html_tags(&content()).trim().len();
    let can_submit = !title().is_empty()
        && content_text_len >= CONTENT_MIN_LENGTH
        && status() != EditorStatus::Saving
        && status() != EditorStatus::Publishing;
    let action_label = tr.publish;

    rsx! {
        div { class: "flex flex-col gap-5 py-5 px-4 mx-auto w-full max-w-[906px]",
            h1 { class: "text-2xl font-bold text-text-primary", "{tr.page_title}" }

            // Title input
            div { class: "relative",
                input {
                    class: "w-full bg-transparent text-xl font-bold text-text-primary placeholder-text-tertiary outline-none border-b border-divider pb-2",
                    r#type: "text",
                    placeholder: "{tr.title_placeholder}",
                    maxlength: "{TITLE_MAX_LENGTH}",
                    value: "{title}",
                    oninput: move |e| {
                        let val = e.value();
                        if val.len() <= TITLE_MAX_LENGTH {
                            title.set(val);
                            status.set(EditorStatus::Unsaved);
                            save_version += 1;
                        }
                    },
                }
                div { class: "absolute right-3 top-1/2 -translate-y-1/2 text-sm text-text-tertiary",
                    "{title_len}/{TITLE_MAX_LENGTH}"
                }
            }

            // TiptapEditor
            div { class: "relative",
                TiptapEditor {
                    class: "w-full min-h-[400px] bg-post-input-bg border border-post-input-border rounded-md focus-within:border-ring focus-within:ring-1 focus-within:ring-ring/50",
                    content: content(),
                    editable: true,
                    placeholder: "{tr.content_placeholder}",
                    on_content_change: move |html: String| {
                        content.set(html);
                        status.set(EditorStatus::Unsaved);
                        save_version += 1;
                    },
                }
                if status() != EditorStatus::Idle {
                    div { class: "flex absolute left-3 bottom-3 gap-2 items-center py-1 px-2 text-xs rounded text-text-tertiary bg-card",
                        match status() {
                            EditorStatus::Saving => rsx! { "{tr.saving}" },
                            EditorStatus::Saved => rsx! { "{tr.all_changes_saved}" },
                            EditorStatus::Unsaved => rsx! { "{tr.unsaved_changes}" },
                            EditorStatus::Publishing => rsx! { "{tr.publishing}" },
                            EditorStatus::Idle => rsx! { "" },
                        }
                    }
                }
            }

            // Status + actions row
            div { class: "flex gap-4 justify-end items-center",
                label { class: "flex items-center gap-2 text-sm text-text-primary cursor-pointer",
                    input {
                        r#type: "checkbox",
                        checked: skip_creating_space(),
                        onchange: move |e| {
                            skip_creating_space.set(e.checked());
                        },
                    }
                    span { "{tr.skip_creating_space}" }
                }
                button {
                    class: "{ButtonStyle::Primary} {ButtonSize::default()} {ButtonShape::default()} min-w-[150px] text-base",
                    disabled: !can_submit,
                    onclick: move |_| {
                        if skip_creating_space() {
                            spawn(handle_publish());
                        } else {
                            spawn(handle_create_space());
                        }
                    },
                    {if status() == EditorStatus::Publishing { tr.publishing } else { action_label }}
                }
            }

            if status() == EditorStatus::Saving {
                div { class: "flex gap-2 justify-center items-center mt-4 text-sm text-text-tertiary",
                    div { class: "w-4 h-4 border-2 border-text-tertiary border-t-transparent rounded-full animate-spin" }
                    span { "{tr.saving}" }
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
