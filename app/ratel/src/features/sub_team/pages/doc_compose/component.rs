//! Sub-team document composer — new + edit. Consumes
//! `UseSubTeamDocCompose`. Mirrors
//! `assets/design/sub-team/subteam-doc-compose.html` (Phase 1 plain
//! textarea — rich text editor deferred).

use crate::features::social::controllers::find_team::find_team_handler;
use crate::features::sub_team::{
    use_sub_team_doc_compose, CreateSubTeamDocumentRequest, SubTeamTranslate,
    UpdateSubTeamDocumentRequest, UseSubTeamDocCompose,
};
use crate::route::Route;
use crate::*;

#[component]
pub fn TeamSubTeamDocComposePage(username: String) -> Element {
    render_compose(username, None)
}

#[component]
pub fn TeamSubTeamDocEditPage(username: String, doc_id: String) -> Element {
    render_compose(username, Some(doc_id))
}

fn render_compose(username: String, doc_id: Option<String>) -> Element {
    let tr: SubTeamTranslate = use_translate();

    let username_for_load = username.clone();
    let team_resource = use_loader(move || {
        let name = username_for_load.clone();
        async move { find_team_handler(name).await }
    })?;

    let team_pk = team_resource().pk;
    let team_id: TeamPartition = team_pk
        .parse::<TeamPartition>()
        .unwrap_or(TeamPartition(String::new()));
    use_context_provider(|| team_id);

    let doc_id_for_ctx = doc_id.clone();
    use_context_provider(move || doc_id_for_ctx.clone());

    rsx! {
        SeoMeta {
            title: if doc_id.is_some() { "{tr.doc_compose_title_edit}" } else { "{tr.doc_compose_title_new}" },
        }
        document::Stylesheet { href: asset!("./style.css") }
        DocComposeForm { username: username.clone() }
    }
}

#[component]
fn DocComposeForm(username: String) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let nav = use_navigator();

    let UseSubTeamDocCompose {
        doc_id,
        doc,
        mut handle_save_new,
        mut handle_save_existing,
        mut handle_delete,
        ..
    } = use_sub_team_doc_compose()?;

    let existing = doc();
    let initial_title = existing
        .as_ref()
        .map(|d| d.title.clone())
        .unwrap_or_default();
    let initial_body = existing
        .as_ref()
        .map(|d| d.body.clone())
        .unwrap_or_default();
    let initial_required = existing.as_ref().map(|d| d.required).unwrap_or(false);
    let initial_doc_id = doc_id();

    let mut title: Signal<String> = use_signal(|| initial_title);
    let mut body: Signal<String> = use_signal(|| initial_body);
    let mut required: Signal<bool> = use_signal(|| initial_required);

    let current_id: Signal<Option<String>> = use_signal(move || initial_doc_id.clone());

    let username_for_back = username.clone();
    let username_for_after = username.clone();
    let username_for_delete = username.clone();

    let save_action = move |_| {
        let t = title().clone();
        let b = body().clone();
        let r = required();
        if let Some(id) = current_id() {
            handle_save_existing.call(
                id,
                UpdateSubTeamDocumentRequest {
                    title: Some(t),
                    body: Some(b),
                    required: Some(r),
                    ..Default::default()
                },
            );
        } else {
            handle_save_new.call(CreateSubTeamDocumentRequest {
                title: t,
                body: b,
                required: r,
                ..Default::default()
            });
        }
        nav.push(Route::TeamSubTeamManagementPage {
            username: username_for_after.clone(),
        });
    };

    let delete_action = move |_| {
        if let Some(id) = current_id() {
            handle_delete.call(id);
        }
        nav.push(Route::TeamSubTeamManagementPage {
            username: username_for_delete.clone(),
        });
    };

    let body_len = body().chars().count();
    let word_count = body()
        .split_whitespace()
        .filter(|w| !w.is_empty())
        .count();

    rsx! {
        div { class: "arena sub-team-doc-compose",
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
                        span { class: "topbar-title__eyebrow", "{tr.doc_compose_eyebrow}" }
                        span { class: "topbar-title__main",
                            if current_id().is_some() {
                                "{tr.doc_compose_title_edit}"
                            } else {
                                "{tr.doc_compose_title_new}"
                            }
                        }
                    }
                }
                div { class: "arena-topbar__right",
                    button {
                        class: "topbar-btn topbar-btn--primary",
                        id: "publish-btn",
                        "data-testid": "sub-team-doc-save-btn",
                        onclick: save_action,
                        lucide_dioxus::Check { class: "w-3 h-3 [&>path]:stroke-current" }
                        "{tr.save}"
                    }
                }
            }

            div { class: "composer-page",
                main { class: "composer",
                    div {
                        input {
                            class: "title-input",
                            r#type: "text",
                            "data-testid": "sub-team-doc-title-input",
                            placeholder: "{tr.doc_compose_title_placeholder}",
                            value: "{title()}",
                            oninput: move |e| title.set(e.value()),
                        }
                        div { class: "title-divider" }
                    }
                    textarea {
                        class: "body-editor",
                        id: "body-editor",
                        "data-testid": "sub-team-doc-body-input",
                        placeholder: "{tr.doc_compose_body_placeholder}",
                        value: "{body()}",
                        oninput: move |e| body.set(e.value()),
                    }
                }

                aside { class: "side-panel",
                    div { class: "side-card",
                        div { class: "side-card__title",
                            lucide_dioxus::Check { class: "w-3 h-3 [&>path]:stroke-current" }
                            "{tr.required_reading}"
                        }
                        label {
                            class: "required-row",
                            "data-on": "{required()}",
                            div { class: "required-row__body",
                                div { class: "required-row__title",
                                    if required() {
                                        "{tr.doc_compose_required_on}"
                                    } else {
                                        "{tr.doc_compose_required_off}"
                                    }
                                }
                                div { class: "required-row__desc",
                                    "{tr.doc_compose_required_desc}"
                                }
                            }
                            input {
                                r#type: "checkbox",
                                class: "required-row__input",
                                "data-testid": "sub-team-doc-required-toggle",
                                checked: required(),
                                onchange: move |e| required.set(e.checked()),
                            }
                        }
                    }

                    div { class: "side-card",
                        div { class: "side-card__title",
                            lucide_dioxus::Clock { class: "w-3 h-3 [&>path]:stroke-current" }
                            "{tr.doc_compose_eyebrow}"
                        }
                        div { class: "meta-list",
                            div { class: "meta-row",
                                span { class: "meta-row__k", "{tr.doc_compose_word_count}" }
                                span { class: "meta-row__v", "{body_len}" }
                            }
                            div { class: "meta-row",
                                span { class: "meta-row__k", "Words" }
                                span { class: "meta-row__v", "{word_count}" }
                            }
                        }
                    }

                    if current_id().is_some() {
                        button { class: "danger-row", onclick: delete_action,
                            lucide_dioxus::Trash2 { class: "w-3 h-3 [&>path]:stroke-current" }
                            "{tr.delete_document}"
                        }
                    }
                }
            }
        }
    }
}
