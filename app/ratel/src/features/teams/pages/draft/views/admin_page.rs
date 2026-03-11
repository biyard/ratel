use super::super::*;
use crate::features::posts::components::{CreatePostButton, TeamDrafts};
use crate::*;

#[component]
pub fn AdminPage(teamname: String, team_pk: TeamPartition) -> Element {
    let team_pk_str = team_pk.to_string();

    rsx! {
        div { class: "flex relative flex-row flex-1 w-full",
            div { class: "flex flex-row flex-1 w-full max-mobile:px-2.5",
                SuspenseBoundary {
                    TeamDrafts { teamname: teamname.clone() }
                }
            }

            div { class: "h-fit w-fit max-tablet:fixed max-tablet:bottom-4 max-tablet:right-4 tablet:w-80 tablet:pl-4 tablet:static",
                CreatePostButton { team_pk: Some(team_pk_str.clone()) }
            }
        }
    }
}
