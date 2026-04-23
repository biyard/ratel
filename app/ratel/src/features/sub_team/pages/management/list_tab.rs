//! "Sub-teams / 하위팀 목록" tab — consumes `UseSubTeamList`.
//!
//! Renders a clickable list of recognized sub-teams, routing each row to
//! the sub-team detail page.

use crate::features::sub_team::{
    use_sub_team_list, SubTeamSummaryResponse, SubTeamTranslate, UseSubTeamList,
};
use crate::route::Route;
use crate::*;

#[component]
pub fn ListTab(username: String) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let UseSubTeamList { teams, .. } = use_sub_team_list()?;

    let data = teams();
    let items: Vec<SubTeamSummaryResponse> = data.items.clone();
    let truncated = data.truncated;

    rsx! {
        section { class: "card",
            div { class: "card__head",
                h2 { class: "card__title", "{tr.tab_sub_teams}" }
                span { class: "card__dash" }
                span { class: "card__meta", "{items.len()}" }
            }

            if items.is_empty() {
                div { class: "inline-note", "{tr.empty_list}" }
            } else {
                div { class: "roster",
                    for row in items.iter() {
                        SubTeamRow {
                            key: "{row.sub_team_id}",
                            row: row.clone(),
                            username: username.clone(),
                        }
                    }
                }
            }

            if truncated {
                div { class: "inline-note",
                    lucide_dioxus::Info { class: "w-3 h-3 [&>path]:stroke-current" }
                    "Showing first 50 sub-teams"
                }
            }
        }
    }
}

#[component]
fn SubTeamRow(row: SubTeamSummaryResponse, username: String) -> Element {
    let nav = use_navigator();
    let sub_team_id = row.sub_team_id.clone();
    let username_for_click = username.clone();

    let initials: String = row
        .display_name
        .chars()
        .take(2)
        .collect::<String>()
        .to_uppercase();

    rsx! {
        a {
            class: "roster-row",
            "data-testid": "sub-team-roster-row",
            onclick: move |_| {
                nav.push(Route::TeamSubTeamDetailPage {
                    username: username_for_click.clone(),
                    sub_team_id: sub_team_id.clone(),
                });
            },
            div { class: "avatar avatar--teal", "{initials}" }
            div { class: "roster-row__body",
                span { class: "roster-row__name", "{row.display_name}" }
                span { class: "roster-row__handle", "@{row.username}" }
            }
            div { class: "roster-row__metrics",
                span { class: "roster-row__metric",
                    strong { "{row.member_count}" }
                    " members"
                }
            }
            span { class: "roster-row__chev",
                lucide_dioxus::ChevronRight { class: "w-4 h-4 [&>path]:stroke-current" }
            }
        }
    }
}
