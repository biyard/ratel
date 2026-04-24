//! "Eligibility / 가입 요건" tab — consumes `UseSubTeamSettings`.
//!
//! Shows the parent-eligible toggle and the min-members stepper. Any
//! change calls `handle_update.call(UpdateSubTeamSettingsRequest { .. })`
//! which autosaves and updates the local `settings` signal.

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

    let is_on = settings().is_parent_eligible;
    let min_members = settings().min_sub_team_members;

    rsx! {
        // Activation hero
        section { class: "activation", id: "activation", "data-on": "{is_on}",
            div { class: "activation__icon",
                lucide_dioxus::Users { class: "w-6 h-6 [&>path]:stroke-current" }
            }
            div {
                div { class: "activation__label",
                    if is_on {
                        "Parent-eligible · ON"
                    } else {
                        "Parent-eligible · OFF"
                    }
                }
                div { class: "activation__title", "{tr.settings_is_parent_eligible}" }
            }
            label {
                class: "switch activation__switch",
                "data-testid": "sub-team-settings-eligibility-switch",
                input {
                    r#type: "checkbox",
                    id: "activation-toggle",
                    checked: is_on,
                    onchange: move |e| {
                        let v = e.checked();
                        handle_update
                            .call(UpdateSubTeamSettingsRequest {
                                is_parent_eligible: Some(v),
                                min_sub_team_members: None,
                            });
                    },
                }
                span { class: "switch__track" }
            }
        }

        // Requirements card
        section { class: "card", id: "requirements",
            div { class: "card__head",
                h2 { class: "card__title", "{tr.tab_requirements}" }
                span { class: "card__dash" }
            }
            div { class: "req-grid",
                div { class: "req-card",
                    div { class: "req-card__icon",
                        lucide_dioxus::Users { class: "w-4 h-4 [&>path]:stroke-current" }
                    }
                    div { class: "req-card__body",
                        div { class: "req-card__title", "{tr.settings_min_members}" }
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
                            onchange: move |e| {
                                if let Ok(n) = e.value().parse::<i32>() {
                                    handle_update
                                        .call(UpdateSubTeamSettingsRequest {
                                            is_parent_eligible: None,
                                            min_sub_team_members: Some(n.max(0)),
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
                                    });
                            },
                            "+"
                        }
                    }
                }
            }
        }
    }
}
