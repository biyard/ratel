//! Broadcast announcement composer — full post-composer style.
//! Reuses the shared rich-text Editor; mirrors the right-panel layout
//! of `features/posts/views/post_edit` (posting-as / space-activation /
//! tags / discard draft) but drops crosspost. Every signal flows
//! through `UseSubTeamBroadcastCompose`. Auto-save is driven by a
//! `use_effect` here that debounces ~500 ms then upserts the Draft
//! row via the controller hook actions.

use crate::common::components::editor::Editor as RichEditor;
use crate::common::components::file_uploader::{FileUploader, UploadedFileMeta};
use crate::features::social::controllers::find_team::find_team_handler;
use crate::features::sub_team::hooks::BroadcastDraftStatus;
use crate::features::sub_team::{
    use_sub_team_broadcast_compose, CreateSubTeamAnnouncementRequest, SubTeamTranslate,
    UpdateSubTeamAnnouncementRequest, UseSubTeamBroadcastCompose,
};
use crate::route::Route;
use crate::*;

#[component]
pub fn TeamSubTeamBroadcastComposePage(username: String) -> Element {
    render_compose(username, None)
}

#[component]
pub fn TeamSubTeamBroadcastEditPage(username: String, announcement_id: String) -> Element {
    render_compose(username, Some(announcement_id))
}

fn render_compose(username: String, announcement_id: Option<String>) -> Element {
    let tr: SubTeamTranslate = use_translate();

    let username_for_load = username.clone();
    let team_resource = use_loader(move || {
        let name = username_for_load.clone();
        async move { find_team_handler(name).await }
    })?;

    let team_data = team_resource();
    let team_display = if team_data.nickname.is_empty() {
        team_data.username.clone()
    } else {
        team_data.nickname.clone()
    };
    let team_handle = team_data.username.clone();
    let team_pk = team_data.pk.clone();
    let team_id: TeamPartition = team_pk
        .parse::<TeamPartition>()
        .unwrap_or(TeamPartition(String::new()));
    use_context_provider(|| team_id);
    let announcement_id_for_ctx = announcement_id.clone();
    use_context_provider(move || announcement_id_for_ctx.clone());

    rsx! {
        SeoMeta { title: "{tr.broadcast_compose}" }
        ComposeForm {
            username: username.clone(),
            team_display: team_display.clone(),
            team_handle: team_handle.clone(),
        }
    }
}

#[component]
fn ComposeForm(username: String, team_display: String, team_handle: String) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let nav = use_navigator();

    let mut ctx = use_sub_team_broadcast_compose()?;
    let UseSubTeamBroadcastCompose {
        mut announcement_id,
        mut title,
        mut html_contents,
        mut tags,
        mut attachments,
        mut space_enabled,
        space_type,
        mut draft_status,
        mut last_saved_at,
        mut handle_save_new,
        mut handle_save_existing,
        ..
    } = ctx;

    // Local-only signal for the tag-input field. `tags` is the committed
    // chip list; this holds whatever the user is typing before Enter.
    // Mobile drawer state — same pattern as post_edit. Click "옵션" in the
    // bottom-bar to slide the side-panel up from below.
    let mut drawer_open = use_signal(|| false);
    let mut tag_draft: Signal<String> = use_signal(String::new);
    // Korean IME composition guard. Enter pressed while the IME is still
    // composing the trailing hangul character would otherwise fire the
    // tag-commit handler twice: once with the in-progress text, then a
    // second time after composition resolves and the leftover character
    // reappears in the input. Track composition state explicitly and
    // ignore Enter while it is active — matches the
    // `common/components/search_input` pattern.
    let mut tag_is_composing: Signal<bool> = use_signal(|| false);

    // ── Autosave effect ──────────────────────────────────────────────
    // Subscribe to every editor signal; on change, bump an internal
    // version counter (via `.peek()` + `.set()` so the effect doesn't
    // subscribe to itself) and spawn a 500 ms debounced task that
    // only writes if it is still the latest version. `save_version`
    // is read with `.peek()` inside the spawn too — otherwise its
    // mutation would chain back into another effect run and lock the
    // page in an infinite render loop.
    let save_version = use_signal(|| 0u64);
    let mut seeded = use_signal(|| false);
    use_effect({
        let mut save_version = save_version;
        move || {
            // Touch every tracked signal so this effect re-runs on edits.
            let t = title();
            let h = html_contents();
            let tg = tags();
            let at = attachments();
            let se = space_enabled();
            let st = space_type();
            // First run after the loader seeds the editor: skip the save
            // (otherwise we'd echo the loaded draft back to the server).
            if !*seeded.peek() {
                seeded.set(true);
                return;
            }
            // Bump the debounce version WITHOUT subscribing to it.
            let v = *save_version.peek() + 1;
            save_version.set(v);
            draft_status.set(BroadcastDraftStatus::Dirty);

            spawn(async move {
                crate::common::utils::time::sleep(std::time::Duration::from_millis(500))
                    .await;
                // Stale — a newer keystroke superseded this one.
                if *save_version.peek() != v {
                    return;
                }
                // Skip when there's nothing meaningful to save.
                if t.is_empty() && h.is_empty() && tg.is_empty() && at.is_empty() && !se {
                    return;
                }
                draft_status.set(BroadcastDraftStatus::Saving);
                let current_id = announcement_id.peek().clone();
                match current_id {
                    Some(id) => {
                        handle_save_existing.call(
                            id,
                            UpdateSubTeamAnnouncementRequest {
                                title: Some(t),
                                body: None,
                                html_contents: Some(h),
                                tags: Some(tg),
                                attachments: Some(at),
                                space_enabled: Some(se),
                                space_type: st,
                            },
                        );
                    }
                    None => {
                        handle_save_new.call(CreateSubTeamAnnouncementRequest {
                            title: t,
                            body: String::new(),
                            html_contents: h,
                            tags: tg,
                            attachments: at,
                            space_enabled: se,
                            space_type: st,
                        });
                    }
                }
                draft_status.set(BroadcastDraftStatus::Saved);
                last_saved_at.set(Some(
                    crate::common::utils::time::get_now_timestamp_millis(),
                ));
            });
        }
    });

    // After `handle_save_new` resolves with the new id, promote it into
    // `announcement_id` so subsequent autosaves use the existing-update
    // path instead of creating duplicate rows. Subscribe ONLY to the
    // action's value signal — peek `announcement_id` to avoid this
    // effect's set chaining back into itself.
    use_effect(move || {
        if let Some(Ok(sig)) = handle_save_new.value() {
            let new_id = sig();
            if announcement_id.peek().is_none() && !new_id.is_empty() {
                announcement_id.set(Some(new_id));
            }
        }
    });

    let username_for_publish = username.clone();
    let username_for_discard = username.clone();
    let _ = username;
    // Await the publish HTTP request BEFORE navigating. Using `Action::call`
    // here would detach the future from this component; the nav.push that
    // follows would then unmount the component and drop the in-flight
    // request, leaving the draft stuck in `작성중 · DRAFTS`.
    let publish = use_callback(move |_| {
        let username = username_for_publish.clone();
        spawn(async move {
            if let Some(id) = announcement_id() {
                if let Err(e) = ctx.publish_announcement(id).await {
                    tracing::error!("publish announcement failed: {e}");
                    return;
                }
            }
            nav.push(Route::TeamSubTeamManagementPage { username });
        });
    });

    let discard = move |_| {
        let username = username_for_discard.clone();
        async move {
            if let Some(id) = announcement_id() {
                if let Err(e) = ctx.delete_announcement(id).await {
                    tracing::error!("delete announcement failed: {e}");
                    return;
                }
            }
            nav.push(Route::TeamSubTeamManagementPage { username });
        }
    };

    let status_text = match draft_status() {
        BroadcastDraftStatus::Idle => tr.broadcast_status_idle.to_string(),
        BroadcastDraftStatus::Dirty => tr.broadcast_status_dirty.to_string(),
        BroadcastDraftStatus::Saving => tr.broadcast_status_saving.to_string(),
        BroadcastDraftStatus::Saved => tr.broadcast_status_saved.to_string(),
        BroadcastDraftStatus::Error => tr.broadcast_status_error.to_string(),
    };
    let _ = last_saved_at;

    let initial_html = html_contents();

    rsx! {
        div { class: "sub-team-broadcast-compose",
            // ── Topbar ─────────────────────────────────────────────
            div { class: "arena-topbar",
                div { class: "arena-topbar__left",
                    div {
                        class: "back-btn",
                        role: "button",
                        "aria-label": "Back",
                        onclick: move |_| {
                            nav.go_back();
                        },
                        lucide_dioxus::ChevronLeft { class: "w-4 h-4 [&>path]:stroke-current" }
                    }
                    div { class: "topbar-title",
                        span { class: "topbar-title__eyebrow", "{tr.broadcast_compose}" }
                        span { class: "topbar-title__main", "{tr.broadcast_compose}" }
                    }
                }
                div { class: "arena-topbar__right",
                    span {
                        class: "autosave-chip",
                        "data-state": match draft_status() {
                            BroadcastDraftStatus::Saved => "saved",
                            BroadcastDraftStatus::Saving => "saving",
                            BroadcastDraftStatus::Error => "error",
                            _ => "idle",
                        },
                        span { class: "autosave-chip__dot" }
                        "{status_text}"
                    }
                    div {
                        class: "topbar-btn topbar-btn--primary",
                        id: "publish-btn",
                        "data-testid": "sub-team-broadcast-publish-btn",
                        role: "button",
                        onclick: publish,
                        "aria-disabled": announcement_id().is_none() || title().is_empty(),
                        lucide_dioxus::Send { class: "w-3 h-3 [&>path]:stroke-current" }
                        "{tr.broadcast_publish}"
                    }
                }
            }

            div { class: "composer-page",
                // ── Main column ───────────────────────────────────
                main { class: "composer",
                    div {
                        input {
                            class: "title-input",
                            r#type: "text",
                            "data-testid": "sub-team-broadcast-title-input",
                            placeholder: "{tr.broadcast_title_placeholder}",
                            value: "{title()}",
                            oninput: move |e| {
                                title.set(e.value());
                                draft_status.set(BroadcastDraftStatus::Dirty);
                            },
                        }
                        div { class: "title-divider" }
                    }

                    RichEditor {
                        key: "{announcement_id().unwrap_or_default()}",
                        class: "sub-team-broadcast-body",
                        content: initial_html,
                        placeholder: tr.broadcast_body_placeholder.to_string(),
                        on_content_change: move |v: String| {
                            html_contents.set(v);
                            draft_status.set(BroadcastDraftStatus::Dirty);
                        },
                    }
                }

                // ── Right panel ───────────────────────────────────
                aside {
                    class: "side-panel",
                    "data-open": drawer_open(),
                    // Mobile drawer head — handle + title + close button.
                    // Hidden on desktop via base `.side-panel__head { display: none }`.
                    div { class: "side-panel__head",
                        span { class: "side-panel__handle" }
                        span { class: "side-panel__title", "{tr.sub_team_options_drawer_title}" }
                        button {
                            class: "side-panel__close",
                            "aria-label": tr.sub_team_options_close,
                            onclick: move |_| drawer_open.set(false),
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                line { x1: "18", y1: "6", x2: "6", y2: "18" }
                                line { x1: "6", y1: "6", x2: "18", y2: "18" }
                            }
                        }
                    }

                    // Posting as (locked — parent team).
                    div { class: "side-card",
                        div { class: "side-card__title",
                            lucide_dioxus::Users { class: "w-3 h-3 [&>path]:stroke-current" }
                            "{tr.broadcast_posting_as}"
                        }
                        div { class: "locked-row",
                            div { class: "locked-row__avatar",
                                {team_display.chars().take(3).collect::<String>().to_uppercase()}
                            }
                            div { class: "locked-row__body",
                                div { class: "locked-row__name", "{team_display}" }
                                div { class: "locked-row__meta", "@{team_handle}" }
                            }
                            span {
                                class: "locked-row__lock",
                                "aria-label": "Locked",
                                lucide_dioxus::Lock { class: "w-3 h-3 [&>path]:stroke-current" }
                            }
                        }
                    }

                    // Space activation — reuses post_edit's `.side-card.space-toggle`
                    // structure 1:1 (CSS already lives in main.css §8296+). No
                    // space-type chooser here: the type is chosen later inside
                    // the Space designer, matching post_edit's flow.
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
                                "{tr.broadcast_enable_space}"
                            }
                            // The global `.switch` rule that wins the cascade
                            // (main.css §18498) expects two child elements —
                            // `.switch__track` + `.switch__thumb` — and
                            // doesn't paint a ::after pseudo. Empty
                            // `<button class="switch">` renders as an empty
                            // oval with no knob; render the children
                            // explicitly so the toggle is visible.
                            button {
                                class: "switch",
                                r#type: "button",
                                role: "switch",
                                "aria-checked": space_enabled(),
                                "aria-label": tr.broadcast_enable_space,
                                "data-testid": "sub-team-broadcast-space-toggle",
                                onclick: move |_| {
                                    let next = !space_enabled();
                                    space_enabled.set(next);
                                    draft_status.set(BroadcastDraftStatus::Dirty);
                                },
                                span { class: "switch__track",
                                    span { class: "switch__thumb" }
                                }
                            }
                        }
                        div { class: "space-toggle__hint", "{tr.broadcast_space_hint}" }
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
                            "{tr.broadcast_space_active_hint}"
                        }
                    }

                    // Tags — reuses post_edit's `.tag-input` / `.tag` / `.tag__x`
                    // structure (CSS in main.css §8318+).
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
                            "{tr.broadcast_section_tags}"
                        }
                        div {
                            class: "tag-input",
                            "data-testid": "sub-team-broadcast-tag-input",
                            for tag in tags().iter().cloned() {
                                span {
                                    class: "tag",
                                    key: "{tag}",
                                    "data-testid": "sub-team-broadcast-tag",
                                    "data-tag-value": "{tag}",
                                    "{tag}"
                                    button {
                                        class: "tag__x",
                                        "data-testid": "sub-team-broadcast-tag-remove",
                                        "data-tag-value": "{tag}",
                                        "aria-label": tr.broadcast_remove_tag,
                                        onclick: {
                                            let t = tag.clone();
                                            move |_| {
                                                let mut cur = tags();
                                                cur.retain(|x| x != &t);
                                                tags.set(cur);
                                                draft_status.set(BroadcastDraftStatus::Dirty);
                                            }
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
                                "data-testid": "sub-team-broadcast-tag-input-field",
                                r#type: "text",
                                placeholder: tr.broadcast_tag_placeholder,
                                value: "{tag_draft()}",
                                oninput: move |e: Event<FormData>| tag_draft.set(e.value()),
                                oncompositionstart: move |_| tag_is_composing.set(true),
                                oncompositionend: move |_| tag_is_composing.set(false),
                                onkeydown: move |e: Event<KeyboardData>| {
                                    // Korean IME: skip Enter while composing
                                    // the trailing character — otherwise it
                                    // leaks into a duplicate tag.
                                    if tag_is_composing() {
                                        return;
                                    }
                                    if e.key() == Key::Enter {
                                        e.prevent_default();
                                        let v = tag_draft().trim().to_string();
                                        if !v.is_empty() {
                                            let mut cur = tags();
                                            if !cur.contains(&v) {
                                                cur.push(v);
                                                tags.set(cur);
                                                draft_status.set(BroadcastDraftStatus::Dirty);
                                            }
                                            tag_draft.set(String::new());
                                        }
                                    }
                                },
                            }
                        }
                    }

                    // Attachments — reuses the FileUploader primitive used
                    // by sub-team docs. Each upload is appended to the
                    // `attachments` signal, triggering autosave.
                    div { class: "side-card",
                        div { class: "side-card__title",
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48" }
                            }
                            "{tr.broadcast_section_attachments}"
                        }

                        if attachments().is_empty() {
                            div { class: "space-toggle__hint", "{tr.broadcast_attachments_none}" }
                        } else {
                            div { class: "attachment-list",
                                for (idx, f) in attachments().iter().cloned().enumerate() {
                                    div {
                                        class: "file-row",
                                        key: "{f.id}-{idx}",
                                        div { class: "file-row__icon",
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
                                        }
                                        div { class: "file-row__body",
                                            div { class: "file-row__name", "{f.name}" }
                                            div { class: "file-row__meta", "{f.size}" }
                                        }
                                        button {
                                            class: "file-row__remove",
                                            r#type: "button",
                                            "aria-label": tr.broadcast_remove_tag,
                                            onclick: move |_| {
                                                attachments
                                                    .with_mut(|v| {
                                                        if idx < v.len() {
                                                            v.remove(idx);
                                                        }
                                                    });
                                                draft_status.set(BroadcastDraftStatus::Dirty);
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
                            }
                        }

                        FileUploader {
                            class: "file-dropzone".to_string(),
                            accept: ".pdf,.docx,.pptx,.xlsx,.png,.jpg,.jpeg".to_string(),
                            on_upload_success: move |_: String| {},
                            on_upload_meta: move |uploaded: UploadedFileMeta| {
                                let UploadedFileMeta { url, name, size } = uploaded;
                                let uploaded_name = if name.trim().is_empty() {
                                    url.split('/').next_back().unwrap_or("file").to_string()
                                } else {
                                    name
                                };
                                let ext = FileExtension::from_name_or_url(&uploaded_name, &url);
                                attachments
                                    .with_mut(|v| {
                                        v.push(File {
                                            id: url.clone(),
                                            name: uploaded_name,
                                            size,
                                            ext,
                                            url: Some(url),
                                            uploader_name: None,
                                            uploader_profile_url: None,
                                            uploaded_at: Some(
                                                crate::common::utils::time::get_now_timestamp_millis(),
                                            ),
                                        });
                                    });
                                draft_status.set(BroadcastDraftStatus::Dirty);
                            },
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                                polyline { points: "17 8 12 3 7 8" }
                                line {
                                    x1: "12",
                                    y1: "3",
                                    x2: "12",
                                    y2: "15",
                                }
                            }
                            span { class: "file-dropzone__label",
                                "{tr.broadcast_attachments_upload_title}"
                                small { "{tr.broadcast_attachments_upload_hint}" }
                            }
                        }
                    }

                    // Discard draft — fires `handle_delete` then routes
                    // back to the broadcast tab via the SPA `nav.push`.
                    if announcement_id().is_some() {
                        div {
                            class: "danger-row",
                            "data-testid": "sub-team-broadcast-discard-btn",
                            role: "button",
                            onclick: discard,
                            lucide_dioxus::Trash2 { class: "w-3 h-3 [&>path]:stroke-current" }
                            "{tr.broadcast_discard_draft}"
                        }
                    }
                }
            }

            // Mobile drawer backdrop — dims the page when the side-panel
            // is slid up. Click anywhere outside to close.
            div {
                class: "drawer-backdrop",
                "data-open": drawer_open(),
                onclick: move |_| drawer_open.set(false),
            }

            // Bottom bar (mobile-only). Mirrors the post_edit pattern:
            // 옵션 button toggles the drawer, primary button publishes.
            div { class: "bottom-bar",
                div { class: "bottom-bar__right",
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
                            line { x1: "4", y1: "21", x2: "4", y2: "14" }
                            line { x1: "4", y1: "10", x2: "4", y2: "3" }
                            line { x1: "12", y1: "21", x2: "12", y2: "12" }
                            line { x1: "12", y1: "8", x2: "12", y2: "3" }
                            line { x1: "20", y1: "21", x2: "20", y2: "16" }
                            line { x1: "20", y1: "12", x2: "20", y2: "3" }
                            line { x1: "1", y1: "14", x2: "7", y2: "14" }
                            line { x1: "9", y1: "8", x2: "15", y2: "8" }
                            line { x1: "17", y1: "16", x2: "23", y2: "16" }
                        }
                        "{tr.sub_team_options}"
                    }
                    div {
                        class: "bottom-bar__btn bottom-bar__btn--mobile bottom-bar__btn--primary",
                        "data-testid": "sub-team-broadcast-publish-btn-mobile",
                        role: "button",
                        onclick: publish,
                        "aria-disabled": announcement_id().is_none() || title().is_empty(),
                        lucide_dioxus::Send { class: "w-3 h-3 [&>path]:stroke-current" }
                        "{tr.broadcast_publish}"
                    }
                }
            }
        }
    }
}

