//! "Eligibility / 가입 요건 · TEAM-LEVEL CHECKS" tab.
//!
//! Mirrors `assets/design/sub-team/subteam-management-page.html` line
//! 392–431: card head with TEAM-LEVEL CHECKS subtitle, two requirement
//! cards (min-members / min-days) in a `req-grid`, and an inline note
//! pointing the host at the form/documents tabs for non-objective
//! requirements.
//!
//! Backend support today: both `min_sub_team_members` and
//! `min_sub_team_age_days` are wired through `UseSubTeamSettings` →
//! `update_sub_team_settings_handler` and persisted on the parent
//! `Team` row.

use crate::features::sub_team::{
    use_sub_team_settings, SubTeamTranslate, UpdateSubTeamSettingsRequest, UseSubTeamSettings,
};
use crate::*;

#[component]
pub fn RequirementsTab() -> Element {
    let tr: SubTeamTranslate = use_translate();
    let UseSubTeamSettings {
        settings,
        mut handle_update,
        ..
    } = use_sub_team_settings()?;

    let min_members = settings().min_sub_team_members;
    let min_age_days = settings().min_sub_team_age_days;

    rsx! {
        section { class: "card", id: "requirements",
            div { class: "card__head",
                h2 { class: "card__title", "{tr.req_card_title}" }
                span { class: "card__dash" }
                span { class: "card__meta", "{tr.req_card_meta}" }
            }

            div { class: "req-grid",
                // ── Card 1: 최소 멤버 수 (gold) — wired to backend
                div { class: "req-card",
                    div { class: "req-card__icon",
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
                            path { d: "M16 3.13a4 4 0 0 1 0 7.75" }
                        }
                    }
                    div { class: "req-card__body",
                        div { class: "req-card__title", "{tr.req_min_members_title}" }
                        div { class: "req-card__desc", "{tr.req_min_members_desc}" }
                    }
                    div { class: "req-card__input-wrap",
                        button {
                            class: "req-card__stepper",
                            onclick: move |_| {
                                let next = (min_members - 1).max(0);
                                handle_update
                                    .call(UpdateSubTeamSettingsRequest {
                                        is_parent_eligible: None,
                                        min_sub_team_members: Some(next),
                                        min_sub_team_age_days: None,
                                    });
                            },
                            "−"
                        }
                        input {
                            class: "req-card__value",
                            r#type: "number",
                            id: "min-members",
                            "data-testid": "sub-team-settings-min-members-input",
                            value: "{min_members}",
                            min: "0",
                            max: "99",
                            onchange: move |e| {
                                if let Ok(n) = e.value().parse::<i32>() {
                                    handle_update
                                        .call(UpdateSubTeamSettingsRequest {
                                            is_parent_eligible: None,
                                            min_sub_team_members: Some(n.max(0)),
                                            min_sub_team_age_days: None,
                                        });
                                }
                            },
                        }
                        button {
                            class: "req-card__stepper",
                            onclick: move |_| {
                                handle_update
                                    .call(UpdateSubTeamSettingsRequest {
                                        is_parent_eligible: None,
                                        min_sub_team_members: Some(min_members + 1),
                                        min_sub_team_age_days: None,
                                    });
                            },
                            "+"
                        }
                    }
                }

                // ── Card 2: 팀 생성 최소 기간 (teal) — wired to backend
                div { class: "req-card",
                    div {
                        class: "req-card__icon",
                        style: "background:rgba(110,237,216,0.06);border-color:rgba(110,237,216,0.22);color:var(--sub-team-teal);",
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
                    }
                    div { class: "req-card__body",
                        div {
                            class: "req-card__title",
                            style: "color:var(--sub-team-teal);",
                            "{tr.req_min_days_title}"
                        }
                        div { class: "req-card__desc", "{tr.req_min_days_desc}" }
                    }
                    div { class: "req-card__input-wrap",
                        button {
                            class: "req-card__stepper",
                            onclick: move |_| {
                                let next = (min_age_days - 1).max(0);
                                handle_update
                                    .call(UpdateSubTeamSettingsRequest {
                                        is_parent_eligible: None,
                                        min_sub_team_members: None,
                                        min_sub_team_age_days: Some(next),
                                    });
                            },
                            "−"
                        }
                        input {
                            class: "req-card__value",
                            r#type: "number",
                            id: "min-days",
                            "data-testid": "sub-team-settings-min-days-input",
                            value: "{min_age_days}",
                            min: "0",
                            max: "365",
                            onchange: move |e| {
                                if let Ok(n) = e.value().parse::<i32>() {
                                    handle_update
                                        .call(UpdateSubTeamSettingsRequest {
                                            is_parent_eligible: None,
                                            min_sub_team_members: None,
                                            min_sub_team_age_days: Some(n.clamp(0, 365)),
                                        });
                                }
                            },
                        }
                        button {
                            class: "req-card__stepper",
                            onclick: move |_| {
                                handle_update
                                    .call(UpdateSubTeamSettingsRequest {
                                        is_parent_eligible: None,
                                        min_sub_team_members: None,
                                        min_sub_team_age_days: Some((min_age_days + 1).min(365)),
                                    });
                            },
                            "+"
                        }
                    }
                }
            }

            // Inline note pointing the host at form (필수) + documents (필독).
            div { class: "inline-note", style: "margin-top:12px",
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    circle { cx: "12", cy: "12", r: "10" }
                    line {
                        x1: "12",
                        y1: "16",
                        x2: "12",
                        y2: "12",
                    }
                    line {
                        x1: "12",
                        y1: "8",
                        x2: "12.01",
                        y2: "8",
                    }
                }
                span { "{tr.req_inline_note}" }
            }
        }
    }
}
