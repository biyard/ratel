//! Top-level `TeamSubTeamManagementPage` component.
//!
//! Shell that mirrors `assets/design/sub-team/subteam-management-page.html`:
//!
//!   1. Autosave chip (right-aligned)
//!   2. Activation hero — always visible, drives `is_parent_eligible`
//!   3. KPI row — recognized / pending / last broadcast counts
//!   4. Tabs nav with SVG icons + count badges
//!   5. Tab panels (each tab consumes its own controller hook)
//!
//! All three list-style controllers (`UseSubTeamSettings`, `UseSubTeamList`,
//! `UseSubTeamQueue`) are installed here so the KPI row and the activation
//! switch read from the same context the tabs reuse via `try_use_context`.

use crate::features::social::controllers::find_team::find_team_handler;
use crate::features::sub_team::models::SubTeamAnnouncementStatus;
use crate::features::sub_team::{
    use_sub_team_broadcast, use_sub_team_list, use_sub_team_queue, use_sub_team_settings,
    SubTeamTranslate, UpdateSubTeamSettingsRequest, UseSubTeamBroadcast, UseSubTeamList,
    UseSubTeamQueue, UseSubTeamSettings,
};
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

    // Install the controllers used both by the page shell (activation +
    // KPI row) and by the individual tabs.
    let UseSubTeamSettings {
        settings,
        mut handle_update,
        ..
    } = use_sub_team_settings()?;
    let UseSubTeamList { teams, .. } = use_sub_team_list()?;
    let UseSubTeamQueue { queue, .. } = use_sub_team_queue()?;
    // Install the broadcast controller here too — same pattern as the
    // other KPI sources. The `BroadcastTab` re-uses this context via
    // `try_use_context`, so we don't pay for a second loader when the
    // user clicks into the tab. We only need `announcements.items()`
    // to find the most recent `published_at` for the KPI cell.
    let UseSubTeamBroadcast { announcements, .. } = use_sub_team_broadcast()?;

    let is_on = settings().is_parent_eligible;
    let recognized_count = teams().items.len();
    let pending_count = queue.items().len();
    let last_broadcast_at: Option<i64> = announcements
        .items()
        .iter()
        .filter(|a| matches!(a.status, SubTeamAnnouncementStatus::Published))
        .filter_map(|a| a.published_at)
        .max();
    let last_broadcast_label = match last_broadcast_at {
        Some(ts) if ts > 0 => format_last_broadcast(ts, &tr),
        _ => tr.kpi_no_broadcast.to_string(),
    };

    let mut active_tab: Signal<ManagementTab> = use_signal(|| ManagementTab::Requirements);

    rsx! {
        SeoMeta { title: "{tr.tab_requirements}" }

        div { class: "sub-team-management",
            div {
                class: "page page--wide",
                id: "page-root",
                "data-activated": "{is_on}",

                // 1. Autosave chip
                div { class: "u-flex u-justify-end",
                    span { class: "autosave-chip",
                        lucide_dioxus::Check { class: "w-3 h-3 [&>path]:stroke-current" }
                        "{tr.settings_autosaved}"
                    }
                }

                // 2. Activation hero — always visible, drives `is_parent_eligible`
                section {
                    class: "activation",
                    id: "activation",
                    "data-on": "{is_on}",
                    div { class: "activation__icon",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M12 2l2.09 4.23 4.66.68-3.38 3.29.8 4.64L12 12.67l-4.17 2.17.8-4.64L5.25 6.91l4.66-.68L12 2z" }
                            circle { cx: "6", cy: "19", r: "3" }
                            circle { cx: "18", cy: "19", r: "3" }
                            path { d: "M9 19h6" }
                        }
                    }
                    div {
                        div {
                            class: "activation__label",
                            id: "activation-label",
                            if is_on {
                                "{tr.activation_label_on}"
                            } else {
                                "{tr.activation_label_off}"
                            }
                        }
                        div { class: "activation__title",
                            if is_on {
                                "{tr.activation_title_on}"
                            } else {
                                "{tr.activation_title_off}"
                            }
                        }
                        p { class: "activation__desc", "{tr.activation_desc}" }
                    }
                    label {
                        class: "switch activation__switch",
                        "data-testid": "sub-team-settings-eligibility-switch",
                        input {
                            r#type: "checkbox",
                            id: "activation-toggle",
                            checked: is_on,
                            onchange: move |e| {
                                handle_update
                                    .call(UpdateSubTeamSettingsRequest {
                                        is_parent_eligible: Some(e.checked()),
                                        min_sub_team_members: None,
                                        min_sub_team_age_days: None,
                                    });
                            },
                        }
                        span { class: "switch__track" }
                    }
                }

                // 3. KPI row — always visible above tabs.
                //
                // Inline `style=""` mirrors the scoped CSS rules in
                // main.css so the cards render regardless of CSS load
                // order / browser cache state. The classes stay in
                // place so future edits live in CSS only.
                div {
                    class: "kpi-row",
                    style: "display:grid;grid-template-columns:repeat(3,1fr);gap:12px;",
                    div {
                        class: "kpi",
                        style: "padding:14px 18px;border-radius:12px;background:var(--sub-team-glass);border:1px solid var(--sub-team-border);display:flex;flex-direction:column;gap:4px;min-width:0;",
                        span {
                            class: "kpi__label",
                            style: "font-size:9px;font-weight:700;letter-spacing:0.14em;text-transform:uppercase;color:var(--sub-team-dim);",
                            "{tr.kpi_recognized}"
                        }
                        span {
                            class: "kpi__value kpi__value--gold",
                            style: "font-size:22px;font-weight:800;letter-spacing:0.02em;background:linear-gradient(135deg,var(--sub-team-gold),#ffd24a);-webkit-background-clip:text;-webkit-text-fill-color:transparent;background-clip:text;",
                            "{recognized_count}"
                        }
                    }
                    div {
                        class: "kpi",
                        style: "padding:14px 18px;border-radius:12px;background:var(--sub-team-glass);border:1px solid var(--sub-team-border);display:flex;flex-direction:column;gap:4px;min-width:0;",
                        span {
                            class: "kpi__label",
                            style: "font-size:9px;font-weight:700;letter-spacing:0.14em;text-transform:uppercase;color:var(--sub-team-dim);",
                            "{tr.kpi_pending}"
                        }
                        span {
                            class: "kpi__value kpi__value--amber",
                            style: "font-size:22px;font-weight:800;letter-spacing:0.02em;color:var(--sub-team-amber);",
                            "{pending_count}"
                        }
                        if pending_count > 0 {
                            span {
                                class: "kpi__delta kpi__delta--amber",
                                style: "font-size:10px;letter-spacing:0.08em;color:var(--sub-team-amber);",
                                "{tr.kpi_pending_review}"
                            }
                        }
                    }
                    div {
                        class: "kpi",
                        style: "padding:14px 18px;border-radius:12px;background:var(--sub-team-glass);border:1px solid var(--sub-team-border);display:flex;flex-direction:column;gap:4px;min-width:0;",
                        span {
                            class: "kpi__label",
                            style: "font-size:9px;font-weight:700;letter-spacing:0.14em;text-transform:uppercase;color:var(--sub-team-dim);",
                            "{tr.kpi_last_broadcast}"
                        }
                        span {
                            class: "kpi__value kpi__value--purple",
                            style: "font-size:22px;font-weight:800;letter-spacing:0.02em;color:var(--sub-team-purple);",
                            "{last_broadcast_label}"
                        }
                    }
                }

                // 4. Tabs nav
                nav { class: "tabs-nav", role: "tablist",
                    TabButton {
                        label: tr.tab_requirements.to_string(),
                        active: active_tab() == ManagementTab::Requirements,
                        testid: "sub-team-tab-requirements".to_string(),
                        icon: rsx! {
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                polyline { points: "20 6 9 17 4 12" }
                            }
                        },
                        onclick: move |_| active_tab.set(ManagementTab::Requirements),
                    }
                    TabButton {
                        label: tr.tab_documents.to_string(),
                        active: active_tab() == ManagementTab::Documents,
                        testid: "sub-team-tab-documents".to_string(),
                        icon: rsx! {
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                                polyline { points: "14 2 14 8 20 8" }
                                line {
                                    x1: "9",
                                    y1: "13",
                                    x2: "15",
                                    y2: "13",
                                }
                                line {
                                    x1: "9",
                                    y1: "17",
                                    x2: "13",
                                    y2: "17",
                                }
                            }
                        },
                        onclick: move |_| active_tab.set(ManagementTab::Documents),
                    }
                    TabButton {
                        label: tr.tab_sub_teams.to_string(),
                        active: active_tab() == ManagementTab::Roster,
                        testid: "sub-team-tab-roster".to_string(),
                        badge: rsx! {
                            span { class: "tabs-nav__badge", "{recognized_count}" }
                        },
                        icon: rsx! {
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" }
                                circle { cx: "9", cy: "7", r: "4" }
                                path { d: "M23 21v-2a4 4 0 0 0-3-3.87" }
                            }
                        },
                        onclick: move |_| active_tab.set(ManagementTab::Roster),
                    }
                    TabButton {
                        label: tr.tab_queue.to_string(),
                        active: active_tab() == ManagementTab::Queue,
                        testid: "sub-team-tab-queue".to_string(),
                        badge: rsx! {
                            span { class: if pending_count > 0 { "tabs-nav__badge tabs-nav__badge--amber" } else { "tabs-nav__badge" },
                                "{pending_count}"
                            }
                        },
                        icon: rsx! {
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                circle { cx: "12", cy: "12", r: "10" }
                                polyline { points: "12 6 12 12 16 14" }
                            }
                        },
                        onclick: move |_| active_tab.set(ManagementTab::Queue),
                    }
                    TabButton {
                        label: tr.tab_broadcast.to_string(),
                        active: active_tab() == ManagementTab::Broadcast,
                        testid: "sub-team-tab-broadcast".to_string(),
                        icon: rsx! {
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M3 11l18-8v18l-18-8v-2z" }
                            }
                        },
                        onclick: move |_| active_tab.set(ManagementTab::Broadcast),
                    }
                }

                // 5. Tab panels — only active tab mounts its tab body.
                // Requirements + Form are always interactive regardless of
                // the activation switch — the switch only controls whether
                // the public apply page is reachable, not whether the admin
                // can configure rules. (`.gated` wrapper removed.)
                div {
                    class: "tab-panel",
                    "data-tab": "requirements",
                    "data-active": "{active_tab() == ManagementTab::Requirements}",
                    if active_tab() == ManagementTab::Requirements {
                        div { style: "display:flex;flex-direction:column;gap:16px;",
                            RequirementsTab {}
                            FormTab {}
                        }
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
                        BroadcastTab {
                            username: username.clone(),
                            team_display_name: {
                                let t = team_resource();
                                if t.nickname.is_empty() { t.username.clone() } else { t.nickname.clone() }
                            },
                        }
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
    icon: Element,
    #[props(default)] badge: Option<Element>,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: "tabs-nav__btn",
            role: "tab",
            "aria-selected": "{active}",
            "data-testid": "{testid}",
            onclick: move |e| onclick.call(e),
            {icon}
            "{label}"
            if let Some(b) = badge {
                {b}
            }
        }
    }
}

/// Format the "Last broadcast" KPI value. Today / Yesterday / N days
/// ago (localised) with a YYYY-MM-DD fallback past a week. The caller
/// passes the active `SubTeamTranslate` instance so the label switches
/// with locale.
fn format_last_broadcast(ts_ms: i64, tr: &SubTeamTranslate) -> String {
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let diff_days = ((now - ts_ms) / 86_400_000).max(0);
    match diff_days {
        0 => tr.broadcast_time_today.to_string(),
        1 => tr.broadcast_time_yesterday.to_string(),
        2..=6 => tr.broadcast_time_days_ago.replace("{n}", &diff_days.to_string()),
        _ => {
            use chrono::TimeZone;
            chrono::Utc
                .timestamp_millis_opt(ts_ms)
                .single()
                .map(|t| t.format("%Y-%m-%d").to_string())
                .unwrap_or_default()
        }
    }
}
