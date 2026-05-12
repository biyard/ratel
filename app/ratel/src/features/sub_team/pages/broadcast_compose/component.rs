//! Broadcast announcement composer — full post-composer style.
//! Reuses the shared rich-text Editor; mirrors the right-panel layout
//! of `features/posts/views/post_edit` (posting-as / space-activation /
//! tags / discard draft) but drops crosspost. Every signal flows
//! through `UseSubTeamBroadcastCompose`. Auto-save is driven by a
//! `use_effect` here that debounces ~500 ms then upserts the Draft
//! row via the controller hook actions.

use crate::common::components::editor::Editor as RichEditor;
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
    // `nav` is intentionally unused here. All inter-page navigation
    // out of this composer is intentionally routed through anchor
    // `href` attributes (FIXME: dioxus 0.7 reconciler bug — see notes
    // on the back/publish/discard elements below).
    let _ = use_navigator;

    let UseSubTeamBroadcastCompose {
        mut announcement_id,
        mut title,
        mut html_contents,
        mut tags,
        mut space_enabled,
        space_type,
        mut draft_status,
        mut last_saved_at,
        mut handle_save_new,
        mut handle_save_existing,
        mut handle_publish,
        mut handle_delete,
        ..
    } = use_sub_team_broadcast_compose()?;

    // Local-only signal for the tag-input field. `tags` is the committed
    // chip list; this holds whatever the user is typing before Enter.
    let mut tag_draft: Signal<String> = use_signal(String::new);

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
                if t.is_empty() && h.is_empty() && tg.is_empty() && !se {
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

    let username_for_back = username.clone();
    let username_for_after = username.clone();
    let _ = username;

    let publish = move |_| {
        if let Some(id) = announcement_id() {
            handle_publish.call(id);
        }
        // Navigation is handled by the anchor `href` on the publish
        // element itself (FIXME: dioxus reconciler workaround — see
        // RSX comment below).
    };

    let discard = move |_| {
        if let Some(id) = announcement_id() {
            handle_delete.call(id);
        }
        // Navigation is handled by the anchor `href` on the discard
        // element itself (FIXME: see above).
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
                    // FIXME: anchor `href` (full-page reload) instead of
                    // `nav.go_back()` because SPA navigation between
                    // compose ↔ management hits the dioxus 0.7
                    // reconciler ElementId-reclaim bug. Switch back to
                    // a button + `nav.go_back()` once the reconciler
                    // bug is resolved.
                    a {
                        class: "back-btn",
                        "aria-label": "Back",
                        href: "/{username_for_back}/sub-teams/manage",
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
                    // FIXME: anchor `href` (full-page reload) instead of
                    // `nav.push` for the same reconciler workaround
                    // documented on the back button above. The publish
                    // server call is fired synchronously via
                    // `handle_publish.call(id)` before the browser
                    // navigation kicks in; the spawned action survives
                    // long enough to enqueue the HTTP request thanks to
                    // the underlying tokio runtime.
                    a {
                        class: "topbar-btn topbar-btn--primary",
                        id: "publish-btn",
                        "data-testid": "sub-team-broadcast-publish-btn",
                        href: "/{username_for_after}/sub-teams/manage",
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
                aside { class: "side-panel",
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
                        div { class: "tag-input", "data-testid": "sub-team-broadcast-tag-input",
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
                                            line { x1: "18", y1: "6", x2: "6", y2: "18" }
                                            line { x1: "6", y1: "6", x2: "18", y2: "18" }
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
                                onkeydown: move |e: Event<KeyboardData>| {
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

                    // Discard draft. FIXME: anchor `href` (full-page
                    // reload) instead of `nav.go_back()` to dodge the
                    // dioxus 0.7 reconciler ElementId-reclaim bug.
                    if announcement_id().is_some() {
                        a {
                            class: "danger-row",
                            "data-testid": "sub-team-broadcast-discard-btn",
                            href: "/{username_for_back}/sub-teams/manage",
                            onclick: discard,
                            lucide_dioxus::Trash2 { class: "w-3 h-3 [&>path]:stroke-current" }
                            "{tr.broadcast_discard_draft}"
                        }
                    }

                    // Hidden anchor to keep `username_for_back` consumed
                    // for the FIXME-tagged future hard-nav fallback.
                    span { class: "u-hidden", "{username_for_back}" }
                }
            }
        }
    }
}

