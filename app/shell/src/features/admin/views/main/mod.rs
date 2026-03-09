mod i18n;

use crate::features::admin::controllers::{
    CreateGlobalRewardRequest, RewardResponse, UpdateGlobalRewardRequest, create_reward,
    list_rewards, update_reward,
};
use crate::features::admin::models::reward_types::{
    ConditionType, RewardConditionExt, RewardPeriodExt, RewardUserBehaviorExt,
};
use crate::features::admin::*;
use common::RewardCondition;
use common::RewardPeriod;
use common::RewardUserBehavior;
use i18n::AdminRewardsTranslate;

#[component]
pub fn AdminMainPage() -> Element {
    let tr: AdminRewardsTranslate = use_translate();

    let mut rewards_resource = use_server_future(move || async move { list_rewards(None).await })?;
    let rewards_state = rewards_resource.value();

    let mut show_form = use_signal(|| false);
    let mut editing = use_signal(|| Option::<RewardResponse>::None);
    let mut is_submitting = use_signal(|| false);

    let mut form_behavior = use_signal(|| RewardUserBehavior::default());
    let mut form_point = use_signal(|| 0i64);
    let mut form_period = use_signal(|| RewardPeriod::default());
    let mut form_condition_type = use_signal(|| ConditionType::default());
    let mut form_condition_value = use_signal(|| 0i64);

    let open_create_form = move |_| {
        editing.set(None);
        form_behavior.set(RewardUserBehavior::default());
        form_point.set(0);
        form_period.set(RewardPeriod::default());
        form_condition_type.set(ConditionType::default());
        form_condition_value.set(0);
        show_form.set(true);
    };

    let mut open_edit_form = move |reward: RewardResponse| {
        let ct = reward.condition.condition_type();
        let cv = reward.condition.value().unwrap_or(0);
        form_behavior.set(reward.reward_behavior.clone());
        form_point.set(reward.point);
        form_period.set(reward.period.clone());
        form_condition_type.set(ct);
        form_condition_value.set(cv);
        editing.set(Some(reward));
        show_form.set(true);
    };

    let close_form = move |_| {
        show_form.set(false);
        editing.set(None);
    };

    let on_submit = move |_| {
        let is_edit = editing.read().is_some();
        let behavior = form_behavior.read().clone();
        let point = *form_point.read();
        let period = form_period.read().clone();
        let condition_type = form_condition_type.read().clone();
        let condition_value = *form_condition_value.read();
        let condition = RewardCondition::from_type_and_value(&condition_type, condition_value);
        let reward_resource = rewards_resource.clone();
        is_submitting.set(true);
        spawn(async move {
            let mut reward_resource = reward_resource.clone();
            let result = if is_edit {
                update_reward(UpdateGlobalRewardRequest {
                    behavior,
                    point,
                    period,
                    condition,
                })
                .await
            } else {
                create_reward(CreateGlobalRewardRequest {
                    behavior,
                    point,
                    period,
                    condition,
                })
                .await
            };

            is_submitting.set(false);
            match result {
                Ok(_) => {
                    show_form.set(false);
                    editing.set(None);
                    rewards_resource.restart();
                }
                Err(e) => {
                    error!("Failed to save reward: {:?}", e);
                }
            }
        });
    };

    if rewards_state.read().is_none() {
        return rsx! {
            div { class: "w-full max-w-desktop mx-auto px-4 py-8",
                div { class: "text-center text-text-primary", "{tr.loading}" }
            }
        };
    }

    let rewards: Vec<RewardResponse> = match rewards_state.read().as_ref() {
        Some(Ok(data)) => data.items.clone(),
        Some(Err(e)) => {
            return rsx! {
                div { class: "w-full max-w-desktop mx-auto px-4 py-8",
                    div { class: "text-center text-red-500", "{tr.error}: {e}" }
                }
            };
        }
        None => vec![],
    };

    let is_editing = editing.read().is_some();
    let show_form_val = *show_form.read();
    let is_submitting_val = *is_submitting.read();
    let current_condition_type = form_condition_type.read().clone();
    let show_condition_value = current_condition_type != ConditionType::None;

    rsx! {
        div { class: "w-full max-w-desktop mx-auto px-4 py-6",
            // Header
            div { class: "flex items-center justify-between mb-6",
                h1 { class: "text-2xl font-bold text-text-primary", "{tr.title}" }
                button {
                    class: "px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 transition-colors",
                    onclick: open_create_form,
                    "{tr.add_reward}"
                }
            }

            // Modal overlay
            if show_form_val {
                div {
                    class: "fixed inset-0 z-50 flex items-center justify-center bg-black/50",
                    onclick: close_form,
                    div {
                        class: "bg-bg border border-card-border rounded-lg p-6 w-full max-w-md mx-4",
                        onclick: move |e| e.stop_propagation(),

                        h2 { class: "text-lg font-semibold text-text-primary mb-4",
                            if is_editing {
                                "{tr.edit_reward}"
                            } else {
                                "{tr.add_reward}"
                            }
                        }

                        // Behavior select
                        div { class: "mb-4",
                            label { class: "block text-sm font-medium text-text-secondary mb-1",
                                "{tr.action_label}"
                            }
                            select {
                                class: "w-full px-3 py-2 rounded-lg bg-bg border border-card-border text-text-primary",
                                disabled: is_editing,
                                onchange: move |e| {
                                    if let Ok(b) = e.value().parse::<RewardUserBehavior>() {
                                        form_behavior.set(b);
                                    }
                                },
                                for behavior in RewardUserBehavior::all() {
                                    option {
                                        value: "{behavior}",
                                        selected: *form_behavior.read() == behavior,
                                        "{behavior.label()}"
                                    }
                                }
                            }
                        }

                        // Points input
                        div { class: "mb-4",
                            label { class: "block text-sm font-medium text-text-secondary mb-1",
                                "{tr.point}"
                            }
                            input {
                                r#type: "number",
                                class: "w-full px-3 py-2 rounded-lg bg-bg border border-card-border text-text-primary",
                                value: "{form_point}",
                                onchange: move |e| {
                                    if let Ok(v) = e.value().parse::<i64>() {
                                        form_point.set(v);
                                    }
                                },
                            }
                        }

                        // Period select
                        div { class: "mb-4",
                            label { class: "block text-sm font-medium text-text-secondary mb-1",
                                "{tr.period}"
                            }
                            select {
                                class: "w-full px-3 py-2 rounded-lg bg-bg border border-card-border text-text-primary",
                                onchange: move |e| {
                                    if let Ok(p) = e.value().parse::<RewardPeriod>() {
                                        form_period.set(p);
                                    }
                                },
                                for period in RewardPeriod::all() {
                                    option {
                                        value: "{period}",
                                        selected: *form_period.read() == period,
                                        "{period.label()}"
                                    }
                                }
                            }
                        }

                        // Condition select
                        div { class: "mb-4",
                            label { class: "block text-sm font-medium text-text-secondary mb-1",
                                "{tr.condition}"
                            }
                            select {
                                class: "w-full px-3 py-2 rounded-lg bg-bg border border-card-border text-text-primary",
                                onchange: move |e| {
                                    let val = e.value();
                                    let ct = match val.as_str() {
                                        "MaxClaims" => ConditionType::MaxClaims,
                                        "MaxPoints" => ConditionType::MaxPoints,
                                        "MaxUserClaims" => ConditionType::MaxUserClaims,
                                        "MaxUserPoints" => ConditionType::MaxUserPoints,
                                        _ => ConditionType::None,
                                    };
                                    form_condition_type.set(ct);
                                },
                                option {
                                    value: "None",
                                    selected: current_condition_type == ConditionType::None,
                                    "None"
                                }
                                option {
                                    value: "MaxClaims",
                                    selected: current_condition_type == ConditionType::MaxClaims,
                                    "Max Claims"
                                }
                                option {
                                    value: "MaxPoints",
                                    selected: current_condition_type == ConditionType::MaxPoints,
                                    "Max Points"
                                }
                                option {
                                    value: "MaxUserClaims",
                                    selected: current_condition_type == ConditionType::MaxUserClaims,
                                    "Max User Claims"
                                }
                                option {
                                    value: "MaxUserPoints",
                                    selected: current_condition_type == ConditionType::MaxUserPoints,
                                    "Max User Points"
                                }
                            }
                        }

                        // Condition value input (conditional)
                        if show_condition_value {
                            div { class: "mb-4",
                                label { class: "block text-sm font-medium text-text-secondary mb-1",
                                    "{tr.condition_value}"
                                }
                                input {
                                    r#type: "number",
                                    class: "w-full px-3 py-2 rounded-lg bg-bg border border-card-border text-text-primary",
                                    value: "{form_condition_value}",
                                    onchange: move |e| {
                                        if let Ok(v) = e.value().parse::<i64>() {
                                            form_condition_value.set(v);
                                        }
                                    },
                                }
                            }
                        }

                        // Buttons
                        div { class: "flex justify-end gap-3 mt-6",
                            button {
                                class: "px-4 py-2 rounded-lg border border-card-border text-text-primary hover:bg-card-border transition-colors",
                                onclick: close_form,
                                "{tr.cancel}"
                            }
                            button {
                                class: "px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 transition-colors",
                                disabled: is_submitting_val,
                                onclick: on_submit,
                                "{tr.save}"
                            }
                        }
                    }
                }
            }

            // Rewards table
            div { class: "bg-bg border border-card-border rounded-lg overflow-hidden",
                div { class: "px-4 py-3 border-b border-card-border",
                    span { class: "text-sm font-medium text-text-primary", "{tr.tab_rules}" }
                }

                if rewards.is_empty() {
                    div { class: "px-4 py-8 text-center text-text-secondary", "{tr.no_rewards}" }
                } else {
                    div { class: "overflow-x-auto",
                        table { class: "w-full text-sm",
                            thead {
                                tr { class: "border-b border-card-border bg-card-border/30",
                                    th { class: "px-4 py-3 text-left font-medium text-text-secondary",
                                        "{tr.action_label}"
                                    }
                                    th { class: "px-4 py-3 text-left font-medium text-text-secondary",
                                        "{tr.point}"
                                    }
                                    th { class: "px-4 py-3 text-left font-medium text-text-secondary",
                                        "{tr.period}"
                                    }
                                    th { class: "px-4 py-3 text-left font-medium text-text-secondary",
                                        "{tr.condition}"
                                    }
                                    th { class: "px-4 py-3 text-left font-medium text-text-secondary",
                                        "{tr.actions}"
                                    }
                                }
                            }
                            tbody {
                                for reward in rewards.iter() {
                                    {
                                        let reward_clone = reward.clone();
                                        rsx! {
                                            tr { class: "border-b border-card-border hover:bg-card-border/10 transition-colors",
                                                td { class: "px-4 py-3 text-text-primary", "{reward.reward_behavior.label()}" }
                                                td { class: "px-4 py-3 text-text-primary", "{reward.point}" }
                                                td { class: "px-4 py-3 text-text-primary", "{reward.period.label()}" }
                                                td { class: "px-4 py-3 text-text-primary", "{reward.condition.label()}" }
                                                td { class: "px-4 py-3",
                                                    button {
                                                        class: "text-blue-500 hover:text-blue-400 text-sm font-medium",
                                                        onclick: move |_| open_edit_form(reward_clone.clone()),
                                                        "{tr.edit}"
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
    }
}
