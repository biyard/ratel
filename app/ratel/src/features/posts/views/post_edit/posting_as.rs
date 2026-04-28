use dioxus::prelude::*;

use super::i18n::PostEditTranslate;
use crate::common::contexts::TeamItem;

const DEFAULT_AVATAR: &str = "https://metadata.ratel.foundation/ratel/default-profile.png";

#[component]
pub fn PostingAs(
    tr: PostEditTranslate,
    user_avatar: String,
    user_label: String,
    teams: Vec<TeamItem>,
    selected_team_pk: Option<String>,
    open: Signal<bool>,
    on_switch: Callback<Option<String>>,
) -> Element {
    let mut open = open;
    let selected_pk = selected_team_pk.clone();
    let teams_now = teams.clone();
    crate::debug!(
        "PostingAs: teams={:?} selected_team_pk={:?}",
        teams,
        selected_pk
    );

    let (trigger_avatar, trigger_name, trigger_meta, trigger_avatar_team) = match selected_pk
        .as_ref()
        .and_then(|pk| teams_now.iter().find(|t| t.pk.to_string() == *pk).cloned())
    {
        Some(team) => {
            let avatar = if team.profile_url.is_empty() {
                DEFAULT_AVATAR.to_string()
            } else {
                team.profile_url.clone()
            };
            let name = if team.nickname.is_empty() {
                team.username.clone()
            } else {
                team.nickname.clone()
            };
            (avatar, name, tr.team_meta.to_string(), true)
        }
        None => (
            user_avatar.clone(),
            user_label.clone(),
            tr.personal_feed.to_string(),
            false,
        ),
    };

    rsx! {
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
                "data-open": open(),
                button {
                    class: "as-dropdown__trigger",
                    id: "as-dropdown-trigger",
                    "aria-haspopup": "listbox",
                    "aria-expanded": open(),
                    onclick: move |_| {
                        let next = !open();
                        open.set(next);
                    },
                    img {
                        class: if trigger_avatar_team { "as-avatar as-avatar--team" } else { "as-avatar" },
                        src: "{trigger_avatar}",
                        alt: "",
                    }
                    div { class: "as-text",
                        span { class: "as-text__name", "{trigger_name}" }
                        span { class: "as-text__meta", "{trigger_meta}" }
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
                        "aria-selected": selected_pk.is_none(),
                        onclick: move |_| on_switch.call(None),
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
                    if !teams_now.is_empty() {
                        div { class: "as-dropdown__group-label", "{tr.group_teams}" }
                        for team in teams_now.iter().cloned() {
                            {
                                let team_pk = team.pk.to_string();
                                let team_pk_match = team_pk.clone();
                                let is_selected = selected_pk
                                    .as_ref()
                                    .map(|pk| pk == &team_pk_match)
                                    .unwrap_or(false);
                                let team_avatar = if team.profile_url.is_empty() {
                                    DEFAULT_AVATAR.to_string()
                                } else {
                                    team.profile_url.clone()
                                };
                                let team_name = if team.nickname.is_empty() {
                                    team.username.clone()
                                } else {
                                    team.nickname.clone()
                                };
                                rsx! {
                                    button {
                                        key: "{team_pk}",
                                        class: "as-dropdown__item",
                                        role: "option",
                                        "aria-selected": is_selected,
                                        onclick: move |_| on_switch.call(Some(team_pk.clone())),
                                        img { class: "as-avatar as-avatar--team", src: "{team_avatar}", alt: "" }
                                        div { class: "as-text",
                                            span { class: "as-text__name", "{team_name}" }
                                            span { class: "as-text__meta", "{tr.team_meta}" }
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
                    }
                }
            }
        }
    }
}
