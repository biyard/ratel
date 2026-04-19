use dioxus::prelude::*;

use super::i18n::PostEditTranslate;
use crate::common::components::editor::Editor as RichEditor;
use crate::features::auth::hooks::use_user_context;
use crate::features::posts::controllers::get_post::get_post_handler;
use crate::features::posts::controllers::update_post::{update_post_handler, UpdatePostRequest};
use crate::features::posts::controllers::{create_space_handler, CreateSpaceRequest};
use crate::features::posts::models::Post;
use crate::common::types::SpacePartition;
use crate::features::posts::types::Visibility;
use crate::features::posts::*;

const TITLE_MAX_LENGTH: usize = 80;
const CONTENT_MIN_LENGTH: usize = 10;
const DEFAULT_AVATAR: &str = "https://metadata.ratel.foundation/ratel/default-profile.png";

#[derive(Debug, Clone, PartialEq)]
enum EditorStatus {
    Idle,
    Saving,
    Saved,
    Unsaved,
    Publishing,
}

#[derive(Clone, Copy, PartialEq)]
enum PostKind {
    Post,
    Repost,
    Artwork,
}

#[component]
pub fn PostEdit(post_id: FeedPartition) -> Element {
    let tr: PostEditTranslate = use_translate();
    let mut toast = use_toast();
    let nav = use_navigator();
    let mut popup = use_popup();

    let loader_post_id = post_id.clone();
    let res = use_loader(move || {
        let post_id = loader_post_id.clone();
        async move { get_post_handler(post_id).await }
    })?();

    let post = res.post.unwrap_or_default();
    let existing_space_id = post.space_pk.clone().and_then(|pk| match pk {
        Partition::Space(id) => Some(id),
        _ => None,
    });
    let has_existing_space = existing_space_id.is_some();
    let initial_categories = post.categories.clone();
    let Post {
        title: init_title,
        html_contents,
        ..
    } = post;

    let user_ctx = use_user_context();
    let user_ctx_snapshot = user_ctx.read();
    let (user_name, user_handle, user_avatar) = user_ctx_snapshot
        .user
        .as_ref()
        .map(|u| {
            let display = if u.display_name.is_empty() {
                u.username.clone()
            } else {
                u.display_name.clone()
            };
            let avatar = if u.profile_url.is_empty() {
                DEFAULT_AVATAR.to_string()
            } else {
                u.profile_url.clone()
            };
            (display, u.username.clone(), avatar)
        })
        .unwrap_or_else(|| ("You".to_string(), "".to_string(), DEFAULT_AVATAR.to_string()));
    drop(user_ctx_snapshot);

    let user_label = if user_handle.is_empty() {
        user_name.clone()
    } else {
        format!("{user_name} · @{user_handle}")
    };

    let mut title = use_signal(move || init_title.clone());
    let mut content = use_signal(move || html_contents.clone());
    let mut status = use_signal(|| EditorStatus::Idle);

    let initial_categories_for_signal = initial_categories.clone();
    let mut last_saved =
        use_signal(move || (title(), content(), initial_categories_for_signal.clone()));

    let mut categories = use_signal(move || initial_categories.clone());
    let mut tag_input = use_signal(String::new);

    let mut post_kind = use_signal(|| PostKind::Post);
    let mut visibility = use_signal(|| Visibility::Public);
    let mut space_enabled = use_signal(move || has_existing_space);
    let mut drawer_open = use_signal(|| false);
    let mut as_dropdown_open = use_signal(|| false);

    let save_version = use_signal(|| 0u64);
    let autosave_post_id = post_id.clone();
    use_effect(move || {
        let ver = save_version();
        let current_title = title();
        let current_content = content();
        let current_cats = categories();
        let saved = last_saved();
        if ver == 0 {
            return;
        }
        let post_id = autosave_post_id.clone();
        spawn(async move {
            crate::common::utils::time::sleep(std::time::Duration::from_secs(3)).await;
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

    let content_text_chars = use_memo(move || strip_html(&content()).chars().count());
    let content_word_count = use_memo(move || {
        let plain = strip_html(&content());
        plain.split_whitespace().count()
    });
    let read_minutes =
        use_memo(move || std::cmp::max(1, (content_word_count() as f32 / 200.0).ceil() as usize));

    let can_submit = use_memo(move || {
        !title().is_empty()
            && content_text_chars() >= CONTENT_MIN_LENGTH
            && status() != EditorStatus::Saving
            && status() != EditorStatus::Publishing
    });

    let status_line = use_memo(move || match status() {
        EditorStatus::Idle => String::new(),
        EditorStatus::Saving => tr.saving.to_string(),
        EditorStatus::Saved => tr.all_changes_saved.to_string(),
        EditorStatus::Unsaved => tr.unsaved_changes.to_string(),
        EditorStatus::Publishing => tr.publishing.to_string(),
    });

    let autosave_label = use_memo(move || match status() {
        EditorStatus::Saved => format!("{} {}", tr.autosaved, tr.just_now),
        EditorStatus::Saving => tr.saving.to_string(),
        EditorStatus::Unsaved => tr.unsaved_changes.to_string(),
        EditorStatus::Publishing => tr.publishing.to_string(),
        EditorStatus::Idle => format!("{} {}", tr.autosaved, tr.just_now),
    });

    let publish_label = use_memo(move || {
        if space_enabled() || has_existing_space {
            tr.design_space.to_string()
        } else {
            tr.publish.to_string()
        }
    });

    let post_id_sig = use_signal(move || post_id.clone());
    let existing_space_id_sig = use_signal(move || existing_space_id.clone());

    let commit_publish = use_callback(move |_: ()| {
        if !can_submit() {
            return;
        }
        if has_existing_space {
            if let Some(space_id) = existing_space_id_sig.peek().clone() {
                nav.push(crate::Route::SpaceIndexPage {
                    space_id: SpacePartition(space_id),
                });
                return;
            }
        }
        let post_id = post_id_sig.peek().clone();
        if space_enabled() {
            spawn(async move {
                status.set(EditorStatus::Publishing);
                let publish_result = update_post_handler(
                    post_id.clone(),
                    UpdatePostRequest::Publish {
                        title: title(),
                        content: content(),
                        image_urls: None,
                        publish: true,
                        visibility: Some(visibility()),
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
                        nav.push(crate::Route::SpaceIndexPage {
                            space_id: resp.space_id,
                        });
                    }
                    Err(e) => {
                        crate::error!("create_space_handler failed: {e:?}");
                        toast.error(e);
                        status.set(EditorStatus::Unsaved);
                    }
                }
            });
            return;
        }
        let vis = visibility();
        spawn(async move {
            status.set(EditorStatus::Publishing);
            match update_post_handler(
                post_id.clone(),
                UpdatePostRequest::Publish {
                    title: title(),
                    content: content(),
                    image_urls: None,
                    publish: true,
                    visibility: Some(vis),
                    categories: Some(categories()),
                },
            )
            .await
            {
                Ok(_) => {
                    nav.replace(crate::Route::PostDetail { post_id });
                }
                Err(e) => {
                    dioxus::logger::tracing::error!("Publish failed: {:?}", e);
                    status.set(EditorStatus::Unsaved);
                }
            }
        });
    });

    let save_draft_post_id = post_id_sig;
    let commit_save_draft = use_callback(move |_: ()| {
        if status() == EditorStatus::Saving || status() == EditorStatus::Publishing {
            return;
        }
        let post_id = save_draft_post_id.peek().clone();
        let current_title = title();
        let current_content = content();
        let current_cats = categories();
        spawn(async move {
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
                    title: saved_title,
                    html_contents,
                    ..
                }) => {
                    last_saved.set((saved_title, html_contents, current_cats));
                    status.set(EditorStatus::Saved);
                }
                Err(e) => {
                    dioxus::logger::tracing::error!("Save draft failed: {:?}", e);
                    toast.error(e);
                    status.set(EditorStatus::Unsaved);
                }
            }
        });
    });

    let mut add_tag = move |raw: String| {
        let tag = raw.trim().to_string();
        if tag.is_empty() {
            return;
        }
        let mut current = categories();
        let lower = tag.to_lowercase();
        if current.iter().any(|c| c.to_lowercase() == lower) {
            return;
        }
        current.push(tag);
        categories.set(current.clone());
        mark_unsaved(&title(), &content(), &current, last_saved, status, save_version);
    };

    let mut remove_tag = move |tag: String| {
        let mut current = categories();
        current.retain(|c| c != &tag);
        categories.set(current.clone());
        mark_unsaved(&title(), &content(), &current, last_saved, status, save_version);
    };

    rsx! {
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "anonymous",
        }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Orbitron:wght@400;500;600;700;800;900&family=Outfit:wght@300;400;500;600;700&display=swap",
        }
        document::Link { rel: "preload", href: asset!("./style.css"), r#as: "style" }
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        document::Script { defer: true, src: asset!("./script.js") }

        div { class: "composer-arena",
            div { class: "arena-topbar",
                div { class: "arena-topbar__left",
                    button {
                        class: "back-btn",
                        "aria-label": tr.back,
                        onclick: move |_| {
                            nav.go_back();
                        },
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "15 18 9 12 15 6" }
                        }
                    }
                    div { class: "topbar-title",
                        span { class: "topbar-title__eyebrow", "{tr.page_eyebrow}" }
                        span { class: "topbar-title__main", "{tr.page_title}" }
                    }
                }
                div { class: "arena-topbar__right",
                    span { class: "autosave", "{autosave_label}" }
                    button {
                        class: "topbar-btn",
                        disabled: status() == EditorStatus::Saving || status() == EditorStatus::Publishing,
                        onclick: move |_| commit_save_draft.call(()),
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z" }
                            polyline { points: "17 21 17 13 7 13 7 21" }
                            polyline { points: "7 3 7 8 15 8" }
                        }
                        "{tr.save_draft}"
                    }
                    button {
                        class: "topbar-btn topbar-btn--primary",
                        "data-testid": "post-edit-publish-btn",
                        "data-space": space_enabled() || has_existing_space,
                        disabled: !can_submit(),
                        onclick: move |_| commit_publish.call(()),
                        if space_enabled() || has_existing_space {
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2.5",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                polyline { points: "13 17 18 12 13 7" }
                                polyline { points: "6 17 11 12 6 7" }
                            }
                        } else {
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2.5",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                line {
                                    x1: "22",
                                    y1: "2",
                                    x2: "11",
                                    y2: "13",
                                }
                                polygon { points: "22 2 15 22 11 13 2 9 22 2" }
                            }
                        }
                        span { "{publish_label}" }
                    }
                }
            }

            div { class: "composer-page",
                main { class: "composer",
                    div {
                        class: "type-segmented",
                        role: "tablist",
                        "aria-label": "Post type",
                        button {
                            class: "type-seg",
                            role: "tab",
                            "aria-selected": post_kind() == PostKind::Post,
                            onclick: move |_| post_kind.set(PostKind::Post),
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                                polyline { points: "14 2 14 8 20 8" }
                            }
                            "{tr.type_post}"
                        }
                        button {
                            class: "type-seg",
                            role: "tab",
                            "aria-selected": post_kind() == PostKind::Repost,
                            "aria-disabled": "true",
                            title: tr.coming_soon,
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                polyline { points: "17 1 21 5 17 9" }
                                path { d: "M3 11V9a4 4 0 0 1 4-4h14" }
                                polyline { points: "7 23 3 19 7 15" }
                                path { d: "M21 13v2a4 4 0 0 1-4 4H3" }
                            }
                            "{tr.type_repost}"
                            span { class: "type-seg__soon", "{tr.coming_soon}" }
                        }
                        button {
                            class: "type-seg",
                            role: "tab",
                            "aria-selected": post_kind() == PostKind::Artwork,
                            "aria-disabled": "true",
                            title: tr.coming_soon,
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                rect {
                                    x: "3",
                                    y: "3",
                                    width: "18",
                                    height: "18",
                                    rx: "2",
                                    ry: "2",
                                }
                                circle { cx: "8.5", cy: "8.5", r: "1.5" }
                                polyline { points: "21 15 16 10 5 21" }
                            }
                            "{tr.type_artwork}"
                            span { class: "type-seg__soon", "{tr.coming_soon}" }
                        }
                    }

                    div {
                        input {
                            class: "title-input",
                            r#type: "text",
                            placeholder: tr.title_placeholder,
                            value: "{title}",
                            maxlength: TITLE_MAX_LENGTH as i64,
                            oninput: move |e: Event<FormData>| {
                                let val = e.value();
                                if val.chars().count() <= TITLE_MAX_LENGTH {
                                    title.set(val.clone());
                                    mark_unsaved(
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
                        div { class: "title-divider" }
                    }

                    RichEditor {
                        class: "w-full",
                        content: content(),
                        editable: true,
                        placeholder: tr.body_placeholder.to_string(),
                        on_content_change: move |html: String| {
                            content.set(html.clone());
                            mark_unsaved(&title(), &html, &categories(), last_saved, status, save_version);
                        },
                    }
                }

                aside {
                    class: "side-panel",
                    id: "post-side-panel",
                    "data-open": drawer_open(),
                    div { class: "side-panel__head",
                        span { class: "side-panel__handle" }
                        span { class: "side-panel__title", "{tr.post_options}" }
                        button {
                            class: "side-panel__close",
                            "aria-label": tr.close_options,
                            onclick: move |_| drawer_open.set(false),
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                line {
                                    x1: "18",
                                    y1: "6",
                                    x2: "6",
                                    y2: "18",
                                }
                                line {
                                    x1: "6",
                                    y1: "6",
                                    x2: "18",
                                    y2: "18",
                                }
                            }
                        }
                    }

                    // Posting As
                    div { class: "side-card",
                        div { class: "side-card__title",
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" }
                                circle { cx: "12", cy: "7", r: "4" }
                            }
                            "{tr.posting_as}"
                        }
                        div {
                            class: "as-dropdown",
                            id: "as-dropdown",
                            "data-open": as_dropdown_open(),
                            button {
                                class: "as-dropdown__trigger",
                                id: "as-dropdown-trigger",
                                "aria-haspopup": "listbox",
                                "aria-expanded": as_dropdown_open(),
                                onclick: move |_| {
                                    let next = !as_dropdown_open();
                                    as_dropdown_open.set(next);
                                },
                                img {
                                    class: "as-avatar",
                                    src: "{user_avatar}",
                                    alt: "",
                                }
                                div { class: "as-text",
                                    span { class: "as-text__name", "{user_label}" }
                                    span { class: "as-text__meta", "{tr.personal_feed}" }
                                }
                                span { class: "as-dropdown__chevron",
                                    svg {
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        polyline { points: "6 9 12 15 18 9" }
                                    }
                                }
                            }
                            div { class: "as-dropdown__menu", role: "listbox",
                                div { class: "as-dropdown__group-label", "{tr.group_personal}" }
                                button {
                                    class: "as-dropdown__item",
                                    role: "option",
                                    "aria-selected": "true",
                                    onclick: move |_| as_dropdown_open.set(false),
                                    img {
                                        class: "as-avatar",
                                        src: "{user_avatar}",
                                        alt: "",
                                    }
                                    div { class: "as-text",
                                        span { class: "as-text__name", "{user_label}" }
                                        span { class: "as-text__meta", "{tr.personal_feed}" }
                                    }
                                    span { class: "as-dropdown__check",
                                        svg {
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "currentColor",
                                            stroke_width: "3",
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            polyline { points: "20 6 9 17 4 12" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Enable Space
                    div {
                        class: "side-card space-toggle",
                        "data-on": space_enabled(),
                        div { class: "space-toggle__head",
                            div { class: "side-card__title", style: "margin:0",
                                svg {
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    circle { cx: "12", cy: "12", r: "10" }
                                    path { d: "M2 12h20" }
                                    path { d: "M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z" }
                                }
                                "{tr.enable_space}"
                            }
                            button {
                                class: "switch",
                                role: "switch",
                                "aria-checked": space_enabled(),
                                "aria-label": tr.enable_space,
                                disabled: has_existing_space,
                                onclick: move |_| {
                                    if !has_existing_space {
                                        let next = !space_enabled();
                                        space_enabled.set(next);
                                    }
                                },
                            }
                        }
                        div { class: "space-toggle__hint", "{tr.space_hint}" }
                        div { class: "space-toggle__active",
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                polyline { points: "20 6 9 17 4 12" }
                            }
                            "{tr.space_active_hint}"
                        }
                    }

                    // Tags
                    div { class: "side-card",
                        div { class: "side-card__title",
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M20.59 13.41l-7.17 7.17a2 2 0 0 1-2.83 0L2 12V2h10l8.59 8.59a2 2 0 0 1 0 2.82z" }
                            }
                            "{tr.tags}"
                        }
                        div { class: "tag-input",
                            for tag in categories().iter().cloned() {
                                span { class: "tag", key: "{tag}",
                                    "{tag}"
                                    button {
                                        class: "tag__x",
                                        "aria-label": tr.remove_tag,
                                        onclick: {
                                            let t = tag.clone();
                                            move |_| remove_tag(t.clone())
                                        },
                                        svg {
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "currentColor",
                                            stroke_width: "2.5",
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            line {
                                                x1: "18",
                                                y1: "6",
                                                x2: "6",
                                                y2: "18",
                                            }
                                            line {
                                                x1: "6",
                                                y1: "6",
                                                x2: "18",
                                                y2: "18",
                                            }
                                        }
                                    }
                                }
                            }
                            input {
                                class: "tag-input__field",
                                "data-testid": "tag-input-field",
                                r#type: "text",
                                placeholder: tr.tag_placeholder,
                                value: "{tag_input}",
                                oninput: move |e: Event<FormData>| tag_input.set(e.value()),
                                onkeydown: move |e: Event<KeyboardData>| {
                                    if e.key() == Key::Enter {
                                        e.prevent_default();
                                        add_tag(tag_input());
                                        tag_input.set(String::new());
                                    }
                                },
                            }
                        }
                    }

                    // Visibility
                    div { class: "side-card",
                        div { class: "side-card__title",
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" }
                                circle { cx: "12", cy: "12", r: "3" }
                            }
                            "{tr.visibility}"
                        }
                        div { class: "vis-row",
                            button {
                                class: "vis-opt",
                                "aria-selected": matches!(visibility(), Visibility::Public),
                                onclick: move |_| visibility.set(Visibility::Public),
                                svg {
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    circle { cx: "12", cy: "12", r: "10" }
                                    line {
                                        x1: "2",
                                        y1: "12",
                                        x2: "22",
                                        y2: "12",
                                    }
                                    path { d: "M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z" }
                                }
                                span { "{tr.visibility_public}" }
                            }
                            button {
                                class: "vis-opt",
                                "aria-selected": matches!(visibility(), Visibility::Private),
                                onclick: move |_| visibility.set(Visibility::Private),
                                svg {
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    rect {
                                        x: "3",
                                        y: "11",
                                        width: "18",
                                        height: "11",
                                        rx: "2",
                                        ry: "2",
                                    }
                                    path { d: "M7 11V7a5 5 0 0 1 10 0v4" }
                                }
                                span { "{tr.visibility_private}" }
                            }
                        }
                    }

                    // Discard
                    button {
                        class: "danger-row",
                        onclick: move |_| {
                            popup.open(rsx! {
                                DiscardDraftConfirm { on_cancel: move |_| popup.close() }
                            });
                        },
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "3 6 5 6 21 6" }
                            path { d: "M19 6l-2 14a2 2 0 0 1-2 2H9a2 2 0 0 1-2-2L5 6" }
                            path { d: "M10 11v6" }
                            path { d: "M14 11v6" }
                        }
                        "{tr.discard_draft}"
                    }
                }
            }

            div {
                class: "drawer-backdrop",
                "data-open": drawer_open(),
                onclick: move |_| drawer_open.set(false),
            }

            div { class: "bottom-bar",
                div { class: "bottom-bar__left",
                    span { class: "bottom-bar__stat",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M4 19.5A2.5 2.5 0 0 1 6.5 17H20" }
                            path { d: "M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z" }
                        }
                        strong { "{content_word_count}" }
                        " {tr.stat_words} · "
                        strong { "{read_minutes} {tr.stat_min}" }
                        " {tr.stat_read}"
                    }
                    span { class: "bottom-bar__stat",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            circle { cx: "12", cy: "12", r: "10" }
                            polyline { points: "12 6 12 12 16 14" }
                        }
                        "{status_line}"
                    }
                }
                div { class: "bottom-bar__right",
                    button { class: "bottom-bar__btn bottom-bar__btn--desktop",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "9 11 12 14 22 4" }
                            path { d: "M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11" }
                        }
                        "{tr.spell_check}"
                    }
                    button {
                        class: "bottom-bar__btn bottom-bar__btn--mobile",
                        onclick: move |_| drawer_open.set(true),
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            line {
                                x1: "4",
                                y1: "21",
                                x2: "4",
                                y2: "14",
                            }
                            line {
                                x1: "4",
                                y1: "10",
                                x2: "4",
                                y2: "3",
                            }
                            line {
                                x1: "12",
                                y1: "21",
                                x2: "12",
                                y2: "12",
                            }
                            line {
                                x1: "12",
                                y1: "8",
                                x2: "12",
                                y2: "3",
                            }
                            line {
                                x1: "20",
                                y1: "21",
                                x2: "20",
                                y2: "16",
                            }
                            line {
                                x1: "20",
                                y1: "12",
                                x2: "20",
                                y2: "3",
                            }
                            line {
                                x1: "1",
                                y1: "14",
                                x2: "7",
                                y2: "14",
                            }
                            line {
                                x1: "9",
                                y1: "8",
                                x2: "15",
                                y2: "8",
                            }
                            line {
                                x1: "17",
                                y1: "16",
                                x2: "23",
                                y2: "16",
                            }
                        }
                        "{tr.options}"
                    }
                    button {
                        class: "bottom-bar__btn bottom-bar__btn--mobile bottom-bar__btn--primary",
                        "data-testid": "post-edit-publish-btn-mobile",
                        "data-space": space_enabled() || has_existing_space,
                        disabled: !can_submit(),
                        onclick: move |_| commit_publish.call(()),
                        if space_enabled() || has_existing_space {
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2.5",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                polyline { points: "13 17 18 12 13 7" }
                                polyline { points: "6 17 11 12 6 7" }
                            }
                        } else {
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2.5",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                line {
                                    x1: "22",
                                    y1: "2",
                                    x2: "11",
                                    y2: "13",
                                }
                                polygon { points: "22 2 15 22 11 13 2 9 22 2" }
                            }
                        }
                        span { "{publish_label}" }
                    }
                }
            }
        }
    }
}

#[component]
fn DiscardDraftConfirm(on_cancel: EventHandler<()>) -> Element {
    rsx! {
        div { class: "flex flex-col gap-4 p-6 rounded-xl min-w-[320px] bg-card-bg",
            h3 { class: "text-base font-semibold text-text-primary", "Discard this draft?" }
            p { class: "text-sm text-foreground-muted",
                "Discarding will permanently remove the current draft. This action cannot be undone."
            }
            div { class: "flex gap-2 justify-end",
                button {
                    class: "py-2 px-4 text-sm rounded-md border border-border text-text-primary",
                    onclick: move |_| on_cancel.call(()),
                    "Cancel"
                }
            }
        }
    }
}

fn strip_html(html: &str) -> String {
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

fn mark_unsaved(
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
