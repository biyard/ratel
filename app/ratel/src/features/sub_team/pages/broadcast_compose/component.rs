//! Broadcast announcement composer — new + edit flow.
//! Consumes `UseSubTeamBroadcastCompose`. POSTING_AS and VISIBILITY are
//! locked by the hook — this UI only edits title/body.

use crate::features::social::controllers::find_team::find_team_handler;
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
    // announcement_id via context for the hook
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

    let UseSubTeamBroadcastCompose {
        announcement_id,
        announcement,
        mut handle_save_new,
        mut handle_save_existing,
        mut handle_publish,
        mut handle_delete,
        ..
    } = use_sub_team_broadcast_compose()?;

    let existing = announcement();
    let initial_title = existing.as_ref().map(|a| a.title.clone()).unwrap_or_default();
    let initial_body = existing.as_ref().map(|a| a.body.clone()).unwrap_or_default();

    let mut title: Signal<String> = use_signal(|| initial_title);
    let mut body: Signal<String> = use_signal(|| initial_body);

    let current_id: Signal<Option<String>> = use_signal(move || announcement_id());

    let username_for_back = username.clone();
    let username_for_after = username.clone();

    let save_draft = move |_| {
        let t = title().clone();
        let b = body().clone();
        if let Some(id) = current_id() {
            handle_save_existing.call(
                id,
                UpdateSubTeamAnnouncementRequest {
                    title: Some(t),
                    body: Some(b),
                },
            );
        } else {
            // handle_save_new returns the new id. The hook exposes
            // `.value()` to read the last result after call completes;
            // for simplicity we optimistically flip to edit mode once
            // the caller bumps current_id via the action result.
            handle_save_new.call(CreateSubTeamAnnouncementRequest {
                title: t,
                body: b,
            });
        }
    };

    let publish = move |_| {
        if let Some(id) = current_id() {
            handle_publish.call(id);
            nav.push(Route::TeamSubTeamManagementPage {
                username: username_for_after.clone(),
            });
        }
    };

    let delete = move |_| {
        if let Some(id) = current_id() {
            handle_delete.call(id);
            nav.go_back();
        }
    };

    rsx! {
        div { class: "arena sub-team-broadcast-compose",
            div { class: "arena-topbar",
                div { class: "arena-topbar__left",
                    a {
                        class: "back-btn",
                        "aria-label": "Back",
                        onclick: move |_| {
                            nav.push(Route::TeamSubTeamManagementPage {
                                username: username_for_back.clone(),
                            });
                        },
                        lucide_dioxus::ChevronLeft { class: "w-4 h-4 [&>path]:stroke-current" }
                    }
                    div { class: "topbar-title",
                        span { class: "topbar-title__eyebrow", "Sub-team · Broadcast" }
                        span { class: "topbar-title__main", "{tr.broadcast_compose}" }
                    }
                }
                div { class: "arena-topbar__right",
                    button {
                        class: "topbar-btn",
                        "data-testid": "sub-team-broadcast-save-draft-btn",
                        onclick: save_draft,
                        lucide_dioxus::Save { class: "w-3 h-3 [&>path]:stroke-current" }
                        "{tr.broadcast_draft}"
                    }
                    button {
                        class: "topbar-btn topbar-btn--primary",
                        id: "publish-btn",
                        "data-testid": "sub-team-broadcast-publish-btn",
                        onclick: publish,
                        disabled: current_id().is_none(),
                        lucide_dioxus::Send { class: "w-3 h-3 [&>path]:stroke-current" }
                        "{tr.broadcast_publish}"
                    }
                }
            }

            // Broadcast banner
            div { class: "broadcast-banner",
                div { class: "broadcast-banner__inner",
                    div { class: "broadcast-banner__icon",
                        lucide_dioxus::Megaphone { class: "w-4 h-4 [&>path]:stroke-current" }
                    }
                    div { class: "broadcast-banner__body",
                        div { class: "broadcast-banner__label", "Broadcast mode · Sub-team" }
                        div { class: "broadcast-banner__text",
                            strong { "{team_display}" }
                            " Broadcast — all recognized sub-teams"
                        }
                    }
                }
            }

            div { class: "composer-page",
                main { class: "composer",
                    // Title
                    div {
                        input {
                            class: "title-input",
                            r#type: "text",
                            "data-testid": "sub-team-broadcast-title-input",
                            placeholder: "{tr.broadcast_title}",
                            value: "{title()}",
                            oninput: move |e| title.set(e.value()),
                        }
                        div { class: "title-divider" }
                    }

                    // Body editor (plain textarea — rich text deferred)
                    textarea {
                        class: "body-editor",
                        id: "body-editor",
                        "data-testid": "sub-team-broadcast-body-input",
                        placeholder: "{tr.broadcast_body}",
                        value: "{body()}",
                        oninput: move |e| body.set(e.value()),
                    }
                }

                aside { class: "side-panel",
                    // POSTING AS (locked)
                    div { class: "side-card",
                        div { class: "side-card__title",
                            lucide_dioxus::Users { class: "w-3 h-3 [&>path]:stroke-current" }
                            "Posting as"
                        }
                        div {
                            class: "locked-row",
                            title: "상위팀 고정 · 변경 불가",
                            div { class: "locked-row__avatar",
                                {team_display.chars().take(3).collect::<String>().to_uppercase()}
                            }
                            div { class: "locked-row__body",
                                div { class: "locked-row__name", "{team_display}" }
                                div { class: "locked-row__meta", "@{team_handle} · locked" }
                            }
                            span {
                                class: "locked-row__lock",
                                "aria-label": "Locked",
                                lucide_dioxus::Lock { class: "w-3 h-3 [&>path]:stroke-current" }
                            }
                        }
                    }

                    // VISIBILITY (locked to Broadcast)
                    div { class: "side-card",
                        div { class: "side-card__title",
                            lucide_dioxus::Eye { class: "w-3 h-3 [&>path]:stroke-current" }
                            "Visibility"
                        }
                        div { class: "broadcast-target",
                            div { class: "broadcast-target__row",
                                span { class: "broadcast-target__badge",
                                    lucide_dioxus::Megaphone { class: "w-3 h-3 [&>path]:stroke-current" }
                                    "Broadcast"
                                }
                                span { class: "broadcast-target__lock",
                                    lucide_dioxus::Lock { class: "w-3 h-3 [&>path]:stroke-current" }
                                    "locked"
                                }
                            }
                            div { class: "broadcast-target__note", "All recognized sub-teams" }
                        }
                    }

                    if current_id().is_some() {
                        button { class: "danger-row", onclick: delete,
                            lucide_dioxus::Trash2 { class: "w-3 h-3 [&>path]:stroke-current" }
                            "{tr.broadcast_delete}"
                        }
                    }
                }
            }
        }
    }
}
