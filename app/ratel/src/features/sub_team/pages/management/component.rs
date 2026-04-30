//! Top-level `TeamSubTeamManagementPage` component.

use crate::features::social::controllers::find_team::find_team_handler;
use crate::features::sub_team::SubTeamTranslate;
use crate::*;

use super::{BroadcastTab, DocsTab, FormTab, ListTab, QueueTab, RequirementsTab};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagementTab {
    Requirements,
    Documents,
    Roster,
    Queue,
    Broadcast,
}

#[component]
pub fn TeamSubTeamManagementPage(username: String) -> Element {
    let tr: SubTeamTranslate = use_translate();

    // Resolve username -> team pk. Until resolved, render a lightweight
    // placeholder — the hooks need TeamPartition to be installed in
    // context before they run.
    let username_for_load = username.clone();
    let team_resource = use_loader(move || {
        let name = username_for_load.clone();
        async move { find_team_handler(name).await }
    })?;

    let team_pk = team_resource().pk;

    let team_id: TeamPartition = team_pk
        .parse::<TeamPartition>()
        .unwrap_or(TeamPartition(String::new()));

    // Provide the resolved team id as context BEFORE any tab hook reads it.
    use_context_provider(|| team_id.clone());

    let mut active_tab: Signal<ManagementTab> = use_signal(|| ManagementTab::Requirements);

    rsx! {
        SeoMeta { title: "{tr.tab_requirements}" }

        div { class: "arena sub-team-management",
            div { class: "page page--wide", id: "page-root",

                // Autosave chip
                div { class: "u-flex u-justify-end",
                    span { class: "autosave-chip",
                        lucide_dioxus::Check { class: "w-3 h-3 [&>path]:stroke-current" }
                        "{tr.settings_autosaved}"
                    }
                }

                // Tabs nav
                nav { class: "tabs-nav", role: "tablist",
                    TabButton {
                        label: tr.tab_requirements.to_string(),
                        active: active_tab() == ManagementTab::Requirements,
                        testid: "sub-team-tab-requirements".to_string(),
                        onclick: move |_| active_tab.set(ManagementTab::Requirements),
                    }
                    TabButton {
                        label: tr.tab_documents.to_string(),
                        active: active_tab() == ManagementTab::Documents,
                        testid: "sub-team-tab-documents".to_string(),
                        onclick: move |_| active_tab.set(ManagementTab::Documents),
                    }
                    TabButton {
                        label: tr.tab_sub_teams.to_string(),
                        active: active_tab() == ManagementTab::Roster,
                        testid: "sub-team-tab-roster".to_string(),
                        onclick: move |_| active_tab.set(ManagementTab::Roster),
                    }
                    TabButton {
                        label: tr.tab_queue.to_string(),
                        active: active_tab() == ManagementTab::Queue,
                        testid: "sub-team-tab-queue".to_string(),
                        onclick: move |_| active_tab.set(ManagementTab::Queue),
                    }
                    TabButton {
                        label: tr.tab_broadcast.to_string(),
                        active: active_tab() == ManagementTab::Broadcast,
                        testid: "sub-team-tab-broadcast".to_string(),
                        onclick: move |_| active_tab.set(ManagementTab::Broadcast),
                    }
                }

                // Tab panels — only active tab mounts its controller hook.
                div {
                    class: "tab-panel",
                    "data-tab": "requirements",
                    "data-active": "{active_tab() == ManagementTab::Requirements}",
                    if active_tab() == ManagementTab::Requirements {
                        RequirementsTab {}
                        FormTab {}
                    }
                }
                div {
                    class: "tab-panel",
                    "data-tab": "documents",
                    "data-active": "{active_tab() == ManagementTab::Documents}",
                    if active_tab() == ManagementTab::Documents {
                        DocsTab { username: username.clone() }
                    }
                }
                div {
                    class: "tab-panel",
                    "data-tab": "roster",
                    "data-active": "{active_tab() == ManagementTab::Roster}",
                    if active_tab() == ManagementTab::Roster {
                        ListTab { username: username.clone() }
                    }
                }
                div {
                    class: "tab-panel",
                    "data-tab": "queue",
                    "data-active": "{active_tab() == ManagementTab::Queue}",
                    if active_tab() == ManagementTab::Queue {
                        QueueTab {}
                    }
                }
                div {
                    class: "tab-panel",
                    "data-tab": "broadcast",
                    "data-active": "{active_tab() == ManagementTab::Broadcast}",
                    if active_tab() == ManagementTab::Broadcast {
                        BroadcastTab { username: username.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn TabButton(
    label: String,
    active: bool,
    #[props(default)] testid: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: "tabs-nav__btn",
            role: "tab",
            "aria-selected": "{active}",
            "data-testid": "{testid}",
            onclick: move |e| onclick.call(e),
            "{label}"
        }
    }
}
