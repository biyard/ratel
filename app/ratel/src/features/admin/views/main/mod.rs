mod i18n;

use crate::common::ListResponse;
use crate::common::RewardCondition;
use crate::common::RewardPeriod;
use crate::common::RewardUserBehavior;
use crate::common::hooks::use_infinite_query;
use crate::common::utils::format::format_with_commas;
use crate::features::admin::controllers::{
    CreateGlobalRewardRequest, GrantEnterpriseMembershipRequest, MembershipGrantTargetType,
    RewardResponse, UpdateGlobalRewardRequest, create_reward, grant_enterprise_membership,
    list_enterprise_memberships, list_rewards, update_reward,
};
use crate::features::admin::models::reward_types::{
    ConditionType, RewardConditionExt, RewardPeriodExt, RewardUserBehaviorExt,
};
use crate::features::admin::*;
use i18n::AdminRewardsTranslate;

#[component]
pub fn AdminMainPage() -> Element {
    let tr: AdminRewardsTranslate = use_translate();

    let mut rewards_resource = use_server_future(move || async move { list_rewards().await })?;
    let rewards_state = rewards_resource.value();
    let mut enterprise_memberships_query =
        use_infinite_query::<
            String,
            crate::features::admin::controllers::EnterpriseMembershipGrantListItem,
            ListResponse<crate::features::admin::controllers::EnterpriseMembershipGrantListItem>,
            _,
        >(move |bookmark| async move { list_enterprise_memberships(bookmark).await })?;

    let mut show_form = use_signal(|| false);
    let mut editing = use_signal(|| Option::<RewardResponse>::None);
    let mut is_submitting = use_signal(|| false);

    let mut form_behavior = use_signal(|| RewardUserBehavior::default());
    let mut form_point = use_signal(|| 0i64);
    let mut form_period = use_signal(|| RewardPeriod::default());
    let mut form_condition_type = use_signal(|| ConditionType::default());
    let mut form_condition_value = use_signal(|| 0i64);
    let mut grant_target_type = use_signal(MembershipGrantTargetType::default);
    let mut grant_username = use_signal(String::new);
    let mut grant_submitting = use_signal(|| false);
    let mut grant_message = use_signal(|| Option::<(String, bool)>::None);

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
        is_submitting.set(true);
        spawn(async move {
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

    let on_grant_enterprise = move |_| {
        let username = grant_username.read().trim().to_string();
        let target_type = grant_target_type();

        if username.is_empty() {
            grant_message.set(Some((tr.username_placeholder.to_string(), false)));
            return;
        }

        grant_submitting.set(true);
        grant_message.set(None);

        spawn(async move {
            let result = grant_enterprise_membership(GrantEnterpriseMembershipRequest {
                username,
                target_type,
            })
            .await;

            grant_submitting.set(false);
            match result {
                Ok(_) => {
                    grant_message.set(Some((tr.grant_success.to_string(), true)));
                    grant_username.set(String::new());
                    enterprise_memberships_query.restart();
                }
                Err(err) => {
                    error!("Failed to grant enterprise membership: {:?}", err);
                    grant_message.set(Some((format!("{}: {}", tr.grant_failed, err), false)));
                }
            }
        });
    };

    if rewards_state.read().is_none() {
        return rsx! {
            div { class: "py-8 px-4 mx-auto w-full max-w-desktop",
                div { class: "text-center text-text-primary", "{tr.loading}" }
            }
        };
    }

    let rewards: Vec<RewardResponse> = match rewards_state.read().as_ref() {
        Some(Ok(data)) => data.items.clone(),
        Some(Err(e)) => {
            return rsx! {
                div { class: "py-8 px-4 mx-auto w-full max-w-desktop",
                    div { class: "text-center text-red-500", "{tr.error}: {e}" }
                }
            };
        }
        None => vec![],
    };
    let enterprise_memberships = enterprise_memberships_query.items();

    let is_editing = editing.read().is_some();
    let show_form_val = *show_form.read();
    let is_submitting_val = *is_submitting.read();
    let current_condition_type = form_condition_type.read().clone();
    let show_condition_value = current_condition_type != ConditionType::None;
    let grant_target_type_value = grant_target_type();
    let grant_username_value = grant_username();
    let grant_submitting_value = grant_submitting();
    let grant_message_value = grant_message();

    rsx! {
        div { class: "py-6 px-4 mx-auto w-full max-w-desktop",
            // Header
            div { class: "flex justify-between items-center mb-6",
                h1 { class: "text-2xl font-bold text-text-primary", "{tr.title}" }
                button {
                    class: "py-2 px-4 text-white bg-blue-600 rounded-lg transition-colors hover:bg-blue-700 disabled:opacity-50",
                    onclick: open_create_form,
                    "{tr.add_reward}"
                }
            }

            // Modal overlay
            if show_form_val {
                div {
                    class: "flex fixed inset-0 z-50 justify-center items-center bg-black/50",
                    onclick: close_form,
                    div {
                        class: "p-6 mx-4 w-full max-w-md rounded-lg border bg-bg border-card-border",
                        onclick: move |e| e.stop_propagation(),

                        h2 { class: "mb-4 text-lg font-semibold text-text-primary",
                            if is_editing {
                                "{tr.edit_reward}"
                            } else {
                                "{tr.add_reward}"
                            }
                        }

                        // Behavior select
                        div { class: "mb-4",
                            label { class: "block mb-1 text-sm font-medium text-text-secondary",
                                "{tr.action_label}"
                            }
                            select {
                                class: "py-2 px-3 w-full rounded-lg border bg-bg border-card-border text-text-primary",
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
                            label { class: "block mb-1 text-sm font-medium text-text-secondary",
                                "{tr.point}"
                            }
                            input {
                                r#type: "number",
                                class: "py-2 px-3 w-full rounded-lg border bg-bg border-card-border text-text-primary",
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
                            label { class: "block mb-1 text-sm font-medium text-text-secondary",
                                "{tr.period}"
                            }
                            select {
                                class: "py-2 px-3 w-full rounded-lg border bg-bg border-card-border text-text-primary",
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
                            label { class: "block mb-1 text-sm font-medium text-text-secondary",
                                "{tr.condition}"
                            }
                            select {
                                class: "py-2 px-3 w-full rounded-lg border bg-bg border-card-border text-text-primary",
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
                                label { class: "block mb-1 text-sm font-medium text-text-secondary",
                                    "{tr.condition_value}"
                                }
                                input {
                                    r#type: "number",
                                    class: "py-2 px-3 w-full rounded-lg border bg-bg border-card-border text-text-primary",
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
                        div { class: "flex gap-3 justify-end mt-6",
                            button {
                                class: "py-2 px-4 rounded-lg border transition-colors border-card-border text-text-primary hover:bg-card-border",
                                onclick: close_form,
                                "{tr.cancel}"
                            }
                            button {
                                class: "py-2 px-4 text-white bg-blue-600 rounded-lg transition-colors hover:bg-blue-700 disabled:opacity-50",
                                disabled: is_submitting_val,
                                onclick: on_submit,
                                "{tr.save}"
                            }
                        }
                    }
                }
            }

            // Rewards table
            div { class: "overflow-hidden rounded-lg border bg-bg border-card-border",
                div { class: "py-3 px-4 border-b border-card-border",
                    span { class: "text-sm font-medium text-text-primary", "{tr.tab_rules}" }
                }

                if rewards.is_empty() {
                    div { class: "py-8 px-4 text-center text-text-secondary", "{tr.no_rewards}" }
                } else {
                    div { class: "overflow-x-auto",
                        table { class: "w-full text-sm",
                            thead {
                                tr { class: "border-b border-card-border bg-card-border/30",
                                    th { class: "py-3 px-4 font-medium text-left text-text-secondary",
                                        "{tr.action_label}"
                                    }
                                    th { class: "py-3 px-4 font-medium text-left text-text-secondary",
                                        "{tr.point}"
                                    }
                                    th { class: "py-3 px-4 font-medium text-left text-text-secondary",
                                        "{tr.period}"
                                    }
                                    th { class: "py-3 px-4 font-medium text-left text-text-secondary",
                                        "{tr.condition}"
                                    }
                                    th { class: "py-3 px-4 font-medium text-left text-text-secondary",
                                        "{tr.actions}"
                                    }
                                }
                            }
                            tbody {
                                for reward in rewards.iter() {
                                    {
                                        let reward_clone = reward.clone();
                                        rsx! {
                                            tr { class: "border-b transition-colors border-card-border hover:bg-card-border/10",
                                                td { class: "py-3 px-4 text-text-primary", "{reward.reward_behavior.label()}" }
                                                td { class: "py-3 px-4 text-text-primary",
                                                    "{format_with_commas(reward.point)}"
                                                }
                                                td { class: "py-3 px-4 text-text-primary", "{reward.period.label()}" }
                                                td { class: "py-3 px-4 text-text-primary", "{reward.condition.label()}" }
                                                td { class: "py-3 px-4",
                                                    button {
                                                        class: "text-sm font-medium text-blue-500 hover:text-blue-400",
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

            div { class: "mt-6 rounded-lg border bg-bg border-card-border",
                div { class: "py-3 px-4 border-b border-card-border",
                    span { class: "text-sm font-medium text-text-primary", "{tr.enterprise_title}" }
                }
                div { class: "p-4",
                    p { class: "mb-4 text-sm text-text-secondary", "{tr.enterprise_description}" }

                    div { class: "grid grid-cols-1 gap-4 md:grid-cols-[180px_1fr_auto]",
                        div {
                            label { class: "block mb-1 text-sm font-medium text-text-secondary",
                                "{tr.target_type}"
                            }
                            select {
                                class: "py-2 px-3 w-full rounded-lg border bg-bg border-card-border text-text-primary",
                                value: match grant_target_type_value {
                                    MembershipGrantTargetType::User => "user",
                                    MembershipGrantTargetType::Team => "team",
                                },
                                onchange: move |e| {
                                    let value = e.value();
                                    if value == "team" {
                                        grant_target_type.set(MembershipGrantTargetType::Team);
                                    } else {
                                        grant_target_type.set(MembershipGrantTargetType::User);
                                    }
                                },
                                option { value: "user", "{tr.target_user}" }
                                option { value: "team", "{tr.target_team}" }
                            }
                        }

                        div {
                            label { class: "block mb-1 text-sm font-medium text-text-secondary",
                                "{tr.username}"
                            }
                            Input {
                                value: grant_username_value,
                                placeholder: tr.username_placeholder,
                                oninput: move |e: Event<FormData>| grant_username.set(e.value()),
                            }
                        }

                        div { class: "flex items-end",
                            Button {
                                style: ButtonStyle::Primary,
                                disabled: grant_submitting_value,
                                onclick: on_grant_enterprise,
                                "{tr.grant_enterprise}"
                            }
                        }
                    }

                    if let Some((message, ok)) = grant_message_value {
                        p {
                            class: if ok {
                                "mt-4 text-sm text-primary"
                            } else {
                                "mt-4 text-sm text-destructive"
                            },
                            "{message}"
                        }
                    }

                    div { class: "mt-6 pt-6 border-t border-card-border",
                        h3 { class: "mb-3 text-sm font-medium text-text-primary",
                            "{tr.enterprise_granted_list}"
                        }

                        if enterprise_memberships.is_empty() {
                            if enterprise_memberships_query.is_loading() {
                                p { class: "text-sm text-text-secondary", "{tr.loading}" }
                            } else {
                                p { class: "text-sm text-text-secondary",
                                    "{tr.enterprise_granted_empty}"
                                }
                            }
                        } else {
                            div { class: "overflow-x-auto rounded-lg border border-card-border",
                                table { class: "w-full text-sm",
                                    thead {
                                        tr { class: "border-b border-card-border bg-card-border/30",
                                            th { class: "py-3 px-4 font-medium text-left text-text-secondary",
                                                "{tr.target_type}"
                                            }
                                            th { class: "py-3 px-4 font-medium text-left text-text-secondary",
                                                "{tr.username}"
                                            }
                                            th { class: "py-3 px-4 font-medium text-left text-text-secondary",
                                                "{tr.granted_credits}"
                                            }
                                            th { class: "py-3 px-4 font-medium text-left text-text-secondary",
                                                "{tr.max_credit}"
                                            }
                                        }
                                    }
                                    tbody {
                                        for item in enterprise_memberships {
                                            tr { key: "{item.username}", class: "border-b border-card-border last:border-b-0",
                                                td { class: "py-3 px-4 text-text-primary",
                                                    match item.target_type {
                                                        MembershipGrantTargetType::User => tr.target_user.to_string(),
                                                        MembershipGrantTargetType::Team => tr.target_team.to_string(),
                                                    }
                                                }
                                                td { class: "py-3 px-4 text-text-primary", "{item.username}" }
                                                td { class: "py-3 px-4 text-text-primary",
                                                    "{format_with_commas(item.remaining_credits)}"
                                                }
                                                td { class: "py-3 px-4 text-text-primary",
                                                    "{format_with_commas(item.max_credits_per_space)}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            if enterprise_memberships_query.has_more() {
                                {enterprise_memberships_query.more_element()}
                            }
                        }
                    }
                }
            }
        }
    }
}
