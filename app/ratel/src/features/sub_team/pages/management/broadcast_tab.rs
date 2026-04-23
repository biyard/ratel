//! "Broadcast / 전체 공지 관리" tab — consumes `UseSubTeamBroadcast`.
//!
//! Shows a prominent "Compose broadcast" CTA, a list of drafts, and a
//! list of published announcements. Each draft has an "Edit" link that
//! routes to `TeamSubTeamBroadcastEditPage { announcement_id }`.

use crate::features::sub_team::models::SubTeamAnnouncementStatus;
use crate::features::sub_team::{
    use_sub_team_broadcast, SubTeamAnnouncementResponse, SubTeamTranslate, UseSubTeamBroadcast,
};
use crate::route::Route;
use crate::*;

#[component]
pub fn BroadcastTab(username: String) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let UseSubTeamBroadcast {
        mut announcements,
        mut handle_delete,
        mut handle_publish,
        ..
    } = use_sub_team_broadcast()?;

    let items = announcements.items();
    let drafts: Vec<SubTeamAnnouncementResponse> = items
        .iter()
        .filter(|a| a.status == SubTeamAnnouncementStatus::Draft)
        .cloned()
        .collect();
    let published: Vec<SubTeamAnnouncementResponse> = items
        .iter()
        .filter(|a| a.status == SubTeamAnnouncementStatus::Published)
        .cloned()
        .collect();

    let nav = use_navigator();
    let username_for_cta = username.clone();

    rsx! {
        // Compose CTA
        a {
            class: "bc-cta",
            onclick: move |_| {
                nav.push(Route::TeamSubTeamBroadcastComposePage {
                    username: username_for_cta.clone(),
                });
            },
            div { class: "bc-cta__icon",
                lucide_dioxus::Send { class: "w-5 h-5 [&>path]:stroke-current" }
            }
            div { class: "bc-cta__body",
                div { class: "bc-cta__label", "{tr.broadcast_compose}" }
                div { class: "bc-cta__sub", "{tr.broadcast_publish}" }
            }
            lucide_dioxus::ChevronRight { class: "bc-cta__arrow [&>path]:stroke-current" }
        }

        // Drafts
        section { class: "card",
            div { class: "card__head",
                h2 { class: "card__title", "{tr.broadcast_draft}" }
                span { class: "card__dash" }
                span { class: "card__meta", "{drafts.len()}" }
            }
            if drafts.is_empty() {
                div { class: "inline-note", "{tr.empty_list}" }
            } else {
                div { class: "u-col u-gap-10", id: "drafts-list",
                    for draft in drafts.iter() {
                        DraftRow {
                            key: "{draft.id}",
                            item: draft.clone(),
                            username: username.clone(),
                            on_delete: move |id| handle_delete.call(id),
                            on_publish: move |id| handle_publish.call(id),
                        }
                    }
                }
            }
        }

        // Published
        section { class: "card",
            div { class: "card__head",
                h2 { class: "card__title", "{tr.broadcast_published}" }
                span { class: "card__dash" }
                span { class: "card__meta", "{published.len()}" }
            }
            if published.is_empty() {
                div { class: "inline-note", "{tr.empty_list}" }
            } else {
                div { class: "u-col u-gap-10",
                    for item in published.iter() {
                        div { class: "bc-item", key: "{item.id}",
                            div { class: "bc-item__top",
                                span { class: "bc-item__kind",
                                    lucide_dioxus::Megaphone { class: "w-3 h-3 [&>path]:stroke-current" }
                                    "{tr.broadcast_published}"
                                }
                                span { class: "bc-item__time",
                                    "{item.published_at.unwrap_or(item.updated_at)}"
                                }
                            }
                            div { class: "bc-item__title-text", "{item.title}" }
                            div { class: "bc-item__meta",
                                span { "{item.fan_out_count} 팀" }
                            }
                        }
                    }
                }
            }
            {announcements.more_element()}
        }
    }
}

#[component]
fn DraftRow(
    item: SubTeamAnnouncementResponse,
    username: String,
    on_delete: EventHandler<String>,
    on_publish: EventHandler<String>,
) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let nav = use_navigator();
    let announcement_id = item.id.clone();
    let edit_username = username.clone();
    let edit_id = announcement_id.clone();
    let delete_id = announcement_id.clone();
    let publish_id = announcement_id.clone();

    rsx! {
        div { class: "draft-row",
            div { class: "draft-row__dot" }
            div { class: "draft-row__body",
                div { class: "draft-row__title", "{item.title}" }
                div { class: "draft-row__meta",
                    span { "updated {item.updated_at}" }
                    span { "Broadcast" }
                }
            }
            a {
                class: "draft-row__edit",
                onclick: move |_| {
                    nav.push(Route::TeamSubTeamBroadcastEditPage {
                        username: edit_username.clone(),
                        announcement_id: edit_id.clone(),
                    });
                },
                lucide_dioxus::Pencil { class: "w-3 h-3 [&>path]:stroke-current" }
                "{tr.edit}"
            }
            button {
                class: "draft-row__edit",
                onclick: move |_| on_publish.call(publish_id.clone()),
                lucide_dioxus::Send { class: "w-3 h-3 [&>path]:stroke-current" }
                "{tr.broadcast_publish}"
            }
            button {
                class: "draft-row__del",
                "aria-label": "Delete",
                onclick: move |_| on_delete.call(delete_id.clone()),
                lucide_dioxus::Trash2 { class: "w-3 h-3 [&>path]:stroke-current" }
            }
        }
    }
}
