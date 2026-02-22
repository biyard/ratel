use crate::controllers::get_post::get_post_handler;
use crate::controllers::update_post::{update_post_handler, UpdatePostRequest};
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
    let handle_publish = move |_| {
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

    let title_len = title().len();
    let content_text_len = strip_html_tags(&content()).trim().len();
    let can_publish = !title().is_empty()
        && content_text_len >= CONTENT_MIN_LENGTH
        && status() != EditorStatus::Saving
        && status() != EditorStatus::Publishing;

    rsx! {
        div { class: "flex flex-col gap-6 w-full max-w-[906px] mx-auto py-6 px-4",
            h1 { class: "text-2xl font-bold text-text-primary", "{tr.page_title}" }

            // Title input
            div { class: "flex flex-col gap-1",
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
                div { class: "text-xs text-text-tertiary text-right",
                    "{title_len}/{TITLE_MAX_LENGTH}"
                }
            }

            // TiptapEditor
            TiptapEditor {
                class: "w-full min-h-[400px]",
                content: content(),
                editable: true,
                placeholder: "{tr.content_placeholder}",
                on_content_change: move |html: String| {
                    content.set(html);
                    status.set(EditorStatus::Unsaved);
                    save_version += 1;
                },
            }

            // Status + actions row
            div { class: "flex items-center justify-between",
                // Status indicator
                span { class: "text-sm text-text-tertiary",
                    match status() {
                        EditorStatus::Saving => rsx! { "{tr.saving}" },
                        EditorStatus::Saved => rsx! { "{tr.all_changes_saved}" },
                        EditorStatus::Unsaved => rsx! { "{tr.unsaved_changes}" },
                        EditorStatus::Publishing => rsx! { "{tr.saving}" },
                        EditorStatus::Idle => rsx! { "" },
                    }
                }
                button {
                    class: "{ButtonStyle::Primary} {ButtonSize::default()} {ButtonShape::default()}",
                    disabled: !can_publish,
                    onclick: handle_publish,
                    "{tr.publish}"
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
