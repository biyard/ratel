use crate::common::*;
use crate::common::hooks::use_infinite_query;
use crate::features::posts::controllers::create_post::create_post_handler;
use crate::features::posts::controllers::delete_post::delete_post_handler;
use crate::features::posts::controllers::dto::PostResponse;
use crate::features::posts::controllers::list_user_drafts::list_team_drafts_handler;
use crate::features::social::pages::team_arena::{use_team_arena, TeamArenaTab};
use crate::route::Route;

mod admin_page;
mod viewer_page;

#[allow(unused_imports)]
use admin_page::*;
#[allow(unused_imports)]
use viewer_page::*;

use super::controllers::get_team_drafts_permission_handler;
use super::i18n::TeamDraftTranslate;

#[component]
pub fn Home(username: ReadSignal<String>) -> Element {
    let tr: TeamDraftTranslate = use_translate();
    let nav = use_navigator();
    let mut toast = use_toast();

    // Sync arena topbar tab.
    let mut arena = use_team_arena();
    use_effect(move || arena.active_tab.set(TeamArenaTab::Drafts));

    // Permission gate: only TeamEdit/TeamAdmin see the arena view.
    let perm_resource = use_loader(move || async move {
        Ok::<_, super::Error>(
            get_team_drafts_permission_handler(username())
                .await
                .map_err(|e| e.to_string()),
        )
    })?;
    let perm_data = perm_resource.read();
    let perm_ctx = match perm_data.as_ref() {
        Ok(ctx) => ctx.clone(),
        Err(_) => {
            return rsx! {
                document::Link { rel: "stylesheet", href: asset!("./style.css") }
                ViewerPage { username: username() }
            }
        }
    };
    let can_edit = perm_ctx.role.is_admin_or_owner();
    if !can_edit {
        return rsx! {
            document::Link { rel: "stylesheet", href: asset!("./style.css") }
            ViewerPage { username: username() }
        };
    }

    let team_pk_str = perm_ctx.team_pk.0.clone();

    // Drafts list — paginated infinite query. After delete, call
    // `drafts_query.refresh()` to reload from the first page.
    let mut drafts_query = use_infinite_query(move |bookmark| async move {
        list_team_drafts_handler(username(), bookmark).await
    })?;
    let drafts: Vec<PostResponse> = drafts_query.items();
    let count = drafts.len();
    let drafts_more = drafts_query.more_element();

    let mut delete_target = use_signal(|| Option::<FeedPartition>::None);

    let on_new_post = {
        let team_pk_str = team_pk_str.clone();
        move |_| {
            let team_pk_str = team_pk_str.clone();
            let nav = nav;
            spawn(async move {
                let team_id = team_pk_str.parse::<TeamPartition>().ok();
                match create_post_handler(team_id).await {
                    Ok(resp) => {
                        let post_pk: FeedPartition = resp.post_pk.into();
                        nav.push(Route::PostEdit { post_id: post_pk });
                    }
                    Err(e) => {
                        toast.error(e);
                    }
                }
            });
        }
    };

    let on_confirm_delete = move |_: MouseEvent| {
        let Some(pk) = delete_target() else { return };
        spawn(async move {
            match delete_post_handler(pk, None).await {
                Ok(_) => {
                    toast.info(tr.delete_success);
                    delete_target.set(None);
                    drafts_query.refresh();
                }
                Err(e) => {
                    toast.error(e);
                }
            }
        });
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div { class: "td-section-label",
            span { class: "td-section-label__dash" }
            span { class: "td-section-label__title",
                "Team "
                strong { "{tr.drafts_label}" }
            }
            span { class: "td-section-label__dash" }
        }

        div { class: "td-page",

            div { class: "td-page-header",
                div { class: "td-page-header__left",
                    h1 { class: "td-page-header__title", "{tr.drafts_title}" }
                    span { class: "td-page-header__sub",
                        strong { "{count}" }
                        " {tr.drafts_subhead}"
                    }
                }
                button {
                    class: "td-btn-primary",
                    r#type: "button",
                    onclick: on_new_post,
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2.5",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        line {
                            x1: "12",
                            y1: "5",
                            x2: "12",
                            y2: "19",
                        }
                        line {
                            x1: "5",
                            y1: "12",
                            x2: "19",
                            y2: "12",
                        }
                    }
                    "{tr.new_post}"
                }
            }

            if drafts.is_empty() {
                div { class: "td-empty-state",
                    div { class: "td-empty-state__icon",
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
                    div { class: "td-empty-state__title", "{tr.empty_title}" }
                    div { class: "td-empty-state__desc", "{tr.empty_desc}" }
                }
            } else {
                div { class: "td-grid",
                    for draft in drafts.iter().cloned() {
                        DraftCard {
                            key: "{draft.pk}",
                            draft: draft.clone(),
                            on_delete: move |pk: FeedPartition| delete_target.set(Some(pk)),
                        }
                    }
                    {drafts_more}
                }
            }
        }

        if delete_target().is_some() {
            div {
                class: "td-modal-overlay",
                onclick: move |_| delete_target.set(None),
                div {
                    class: "td-modal",
                    onclick: move |e: Event<MouseData>| e.stop_propagation(),
                    div { class: "td-modal__header",
                        span { class: "td-modal__title", "{tr.delete_title}" }
                        button {
                            class: "td-modal__close",
                            r#type: "button",
                            aria_label: "Close",
                            onclick: move |_| delete_target.set(None),
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
                    div { class: "td-modal__body",
                        p { class: "td-modal__desc",
                            "{tr.delete_desc_pre}"
                            strong { "{tr.delete_desc_strong}" }
                            "{tr.delete_desc_post}"
                        }
                        div { class: "td-modal__actions",
                            button {
                                class: "td-modal__cancel",
                                r#type: "button",
                                onclick: move |_| delete_target.set(None),
                                "{tr.cancel}"
                            }
                            button {
                                class: "td-modal__confirm-danger",
                                r#type: "button",
                                onclick: on_confirm_delete,
                                "{tr.confirm}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DraftCard(draft: PostResponse, on_delete: EventHandler<FeedPartition>) -> Element {
    let tr: TeamDraftTranslate = use_translate();
    let nav = use_navigator();

    let title = if draft.title.trim().is_empty() {
        tr.untitled.to_string()
    } else {
        draft.title.clone()
    };
    let preview = strip_html(&draft.html_contents);
    let updated = format_relative(draft.updated_at, &tr);

    let author_initial = draft
        .author_display_name
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "?".to_string());
    let author_name = if draft.author_display_name.is_empty() {
        draft.author_username.clone()
    } else {
        draft.author_display_name.clone()
    };
    let post_pk = draft.pk.clone();
    let post_pk_for_card = draft.pk.clone();

    rsx! {
        div {
            class: "td-card",
            onclick: move |_| {
                nav.push(Route::PostEdit {
                    post_id: post_pk_for_card.clone(),
                });
            },
            div { class: "td-card__head",
                div { class: "td-card__meta-left",
                    span { class: "td-badge", "{tr.badge_draft}" }
                    span { class: "td-updated", "{updated}" }
                }
                button {
                    class: "td-action",
                    r#type: "button",
                    aria_label: "Delete draft",
                    onclick: move |e: Event<MouseData>| {
                        e.stop_propagation();
                        on_delete.call(post_pk.clone());
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
                        path { d: "M10 11v6M14 11v6" }
                        path { d: "M9 6V4a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v2" }
                    }
                }
            }
            h3 { class: "td-card__title", "{title}" }
            if !preview.is_empty() {
                p { class: "td-card__preview", "{preview}" }
            }
            div { class: "td-card__foot",
                div { class: "td-author",
                    if !draft.author_profile_url.is_empty() {
                        img {
                            class: "td-author__avatar",
                            src: "{draft.author_profile_url}",
                            alt: "{author_name}",
                        }
                    } else {
                        div { class: "td-author__avatar", "{author_initial}" }
                    }
                    span { class: "td-author__name", "{author_name}" }
                }
                span { class: "td-edit-hint",
                    "{tr.edit}"
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2.5",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        line {
                            x1: "5",
                            y1: "12",
                            x2: "19",
                            y2: "12",
                        }
                        polyline { points: "12 5 19 12 12 19" }
                    }
                }
            }
        }
    }
}

fn strip_html(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut in_tag = false;
    for ch in raw.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            c if !in_tag => out.push(c),
            _ => {}
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn format_relative(updated_ms: i64, tr: &TeamDraftTranslate) -> String {
    if updated_ms == 0 {
        return "—".to_string();
    }
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let diff_secs = ((now - updated_ms).max(0)) / 1000;
    if diff_secs < 60 {
        tr.updated_just_now.to_string()
    } else if diff_secs < 3600 {
        format!("{} {}", diff_secs / 60, tr.updated_minutes)
    } else if diff_secs < 86_400 {
        format!("{} {}", diff_secs / 3600, tr.updated_hours)
    } else {
        format!("{} {}", diff_secs / 86_400, tr.updated_days)
    }
}
