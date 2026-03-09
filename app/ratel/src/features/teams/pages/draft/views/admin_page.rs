use super::super::*;
use dioxus::prelude::*;
use crate::features::posts::components::{CreatePostButton, TeamDrafts};

#[component]
pub fn AdminPage(teamname: String, team_pk: TeamPartition) -> Element {
    let team_pk_str = team_pk.to_string();

    rsx! {
        div { class: "flex relative flex-1 flex-row w-full",
            div { class: "flex flex-1 flex-row max-mobile:px-2.5 w-full",
                SuspenseBoundary {
                    fallback: |_| rsx! {
                        div { class: "text-center text-gray-400 py-4", "Loading drafts..." }
                    },
                    TeamDrafts { teamname: teamname.clone() }
                }
            }

            div { class: "h-fit w-fit max-tablet:fixed max-tablet:bottom-4 max-tablet:right-4 tablet:w-80 tablet:pl-4 tablet:static",
                CreatePostButton { team_pk: Some(team_pk_str.clone()) }
            }
        }
    }
}
