mod i18n;

use crate::common::ListResponse;
use crate::common::RewardCondition;
use crate::common::RewardPeriod;
use crate::common::RewardUserBehavior;
use crate::common::hooks::use_infinite_query;
use crate::common::utils::format::format_with_commas;
use crate::features::admin::controllers::{
    AnalyzeQuotaResponse, CreateGlobalRewardRequest, GrantEnterpriseMembershipRequest,
    MembershipGrantTargetType, RewardResponse, UpdateAnalyzeQuotaRequest, UpdateGlobalRewardRequest,
    create_reward, get_analyze_quota, grant_enterprise_membership, list_enterprise_memberships,
    list_rewards, update_analyze_quota, update_reward,
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
                    enterprise_memberships_query.refresh();
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
        div { class: "admin-arena__container",
            // ── Analyze quota section (arena card) ──────────
            AnalyzeQuotaSection {}

            // ── Reward management section (arena card) ──────────
            section { class: "admin-arena__section",
                div { class: "admin-arena__section-actions",
                    span { class: "admin-arena__section-actions-title", "{tr.title}" }
                    button {
                        class: "admin-arena__btn",
                        r#type: "button",
                        onclick: open_create_form,
                        "{tr.add_reward}"
                    }
                }

                // Modal overlay (reward create / edit)
                if show_form_val {
                    div {
                        class: "admin-arena__modal-overlay",
                        onclick: close_form,
                        div {
                            class: "admin-arena__modal",
                            onclick: move |e| e.stop_propagation(),

                            h2 { class: "admin-arena__modal-title",
                                if is_editing {
                                    "{tr.edit_reward}"
                                } else {
                                    "{tr.add_reward}"
                                }
                            }

                            // Behavior select
                            div { class: "admin-arena__form admin-arena__form--stretch mb-4",
                                label { class: "admin-arena__form-label", "{tr.action_label}" }
                                select {
                                    class: "admin-arena__form-select",
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
                            div { class: "admin-arena__form admin-arena__form--stretch mb-4",
                                label { class: "admin-arena__form-label", "{tr.point}" }
                                input {
                                    r#type: "number",
                                    class: "admin-arena__form-input",
                                    value: "{form_point}",
                                    onchange: move |e| {
                                        if let Ok(v) = e.value().parse::<i64>() {
                                            form_point.set(v);
                                        }
                                    },
                                }
                            }

                            // Period select
                            div { class: "admin-arena__form admin-arena__form--stretch mb-4",
                                label { class: "admin-arena__form-label", "{tr.period}" }
                                select {
                                    class: "admin-arena__form-select",
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
                            div { class: "admin-arena__form admin-arena__form--stretch mb-4",
                                label { class: "admin-arena__form-label", "{tr.condition}" }
                                select {
                                    class: "admin-arena__form-select",
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
                                div { class: "admin-arena__form admin-arena__form--stretch mb-4",
                                    label { class: "admin-arena__form-label", "{tr.condition_value}" }
                                    input {
                                        r#type: "number",
                                        class: "admin-arena__form-input",
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
                            div { class: "admin-arena__modal-actions",
                                button {
                                    class: "admin-arena__btn admin-arena__btn--ghost",
                                    onclick: close_form,
                                    "{tr.cancel}"
                                }
                                button {
                                    class: "admin-arena__btn",
                                    disabled: is_submitting_val,
                                    onclick: on_submit,
                                    "{tr.save}"
                                }
                            }
                        }
                    }
                }

                // Rewards table
                div { class: "admin-arena__table-wrap",
                    div { class: "admin-arena__table-head", "{tr.tab_rules}" }

                    if rewards.is_empty() {
                        div { class: "admin-arena__table-empty", "{tr.no_rewards}" }
                    } else {
                        table { class: "admin-arena__table",
                            thead {
                                tr {
                                    th { "{tr.action_label}" }
                                    th { "{tr.point}" }
                                    th { "{tr.period}" }
                                    th { "{tr.condition}" }
                                    th { "{tr.actions}" }
                                }
                            }
                            tbody {
                                for reward in rewards.iter() {
                                    {
                                        let reward_clone = reward.clone();
                                        rsx! {
                                            tr {
                                                td { "{reward.reward_behavior.label()}" }
                                                td { "{format_with_commas(reward.point)}" }
                                                td { "{reward.period.label()}" }
                                                td { "{reward.condition.label()}" }
                                                td {
                                                    button {
                                                        class: "admin-arena__table-link",
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

            // ── Enterprise membership grant section (arena card) ─
            section { class: "admin-arena__section",
                div { class: "admin-arena__section-head",
                    span { class: "admin-arena__section-title", "{tr.enterprise_title}" }
                }
                p { class: "admin-arena__section-description", "{tr.enterprise_description}" }

                div { class: "admin-arena__grant-grid",
                    div { class: "admin-arena__form-field",
                        label { class: "admin-arena__form-label", "{tr.target_type}" }
                        select {
                            class: "admin-arena__form-select",
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

                    div { class: "admin-arena__form-field",
                        label { class: "admin-arena__form-label", "{tr.username}" }
                        input {
                            class: "admin-arena__form-input",
                            r#type: "text",
                            value: "{grant_username_value}",
                            placeholder: tr.username_placeholder,
                            oninput: move |e: Event<FormData>| grant_username.set(e.value()),
                        }
                    }

                    button {
                        class: "admin-arena__btn",
                        r#type: "button",
                        disabled: grant_submitting_value,
                        onclick: on_grant_enterprise,
                        "{tr.grant_enterprise}"
                    }
                }

                if let Some((message, ok)) = grant_message_value {
                    p { class: if ok { "admin-arena__status admin-arena__status--ok mt-4" } else { "admin-arena__status admin-arena__status--err mt-4" },
                        "{message}"
                    }
                }

                div { class: "admin-arena__subsection",
                    h3 { class: "admin-arena__subsection-title", "{tr.enterprise_granted_list}" }

                    if enterprise_memberships.is_empty() {
                        if enterprise_memberships_query.is_loading() {
                            div { class: "admin-arena__table-empty", "{tr.loading}" }
                        } else {
                            div { class: "admin-arena__table-empty", "{tr.enterprise_granted_empty}" }
                        }
                    } else {
                        div { class: "admin-arena__table-wrap",
                            table { class: "admin-arena__table",
                                thead {
                                    tr {
                                        th { "{tr.target_type}" }
                                        th { "{tr.username}" }
                                        th { "{tr.granted_credits}" }
                                        th { "{tr.max_credit}" }
                                    }
                                }
                                tbody {
                                    for item in enterprise_memberships {
                                        tr { key: "{item.username}",
                                            td {
                                                match item.target_type {
                                                    MembershipGrantTargetType::User => tr.target_user.to_string(),
                                                    MembershipGrantTargetType::Team => tr.target_team.to_string(),
                                                }
                                            }
                                            td { "{item.username}" }
                                            td { "{format_with_commas(item.remaining_credits)}" }
                                            td { "{format_with_commas(item.max_credits_per_space)}" }
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

/// Tunes the `AnalyzeQuotaConfig.non_enterprise_limit` from the admin
/// page. Lazily loads the row, falls back to the hardcoded default
/// (currently 2) when no row exists yet, and upserts on save.
#[component]
fn AnalyzeQuotaSection() -> Element {
    let mut quota_resource = use_server_future(move || async move { get_analyze_quota().await })?;
    let value = quota_resource.value();
    let mut input_limit = use_signal(|| 2i64);
    let mut saving = use_signal(|| false);
    let mut message = use_signal::<Option<(String, bool)>>(|| None);

    // Sync the input with the loaded value once the resource resolves.
    // Re-syncs whenever the resource reloads (e.g. after a save).
    let mut last_loaded = use_signal::<Option<i64>>(|| None);
    if let Some(Ok(resp)) = value.read().as_ref() {
        if last_loaded.read().as_ref() != Some(&resp.non_enterprise_limit) {
            last_loaded.set(Some(resp.non_enterprise_limit));
            input_limit.set(resp.non_enterprise_limit);
        }
    }

    let on_save = move |_| {
        let limit = input_limit();
        if limit < 0 {
            message.set(Some(("0 이상의 값을 입력하세요".to_string(), false)));
            return;
        }
        saving.set(true);
        message.set(None);
        spawn(async move {
            let result = update_analyze_quota(UpdateAnalyzeQuotaRequest {
                non_enterprise_limit: limit,
            })
            .await;
            saving.set(false);
            match result {
                Ok(_) => {
                    message.set(Some(("저장되었습니다".to_string(), true)));
                    quota_resource.restart();
                }
                Err(err) => {
                    message.set(Some((format!("저장 실패: {err}"), false)));
                }
            }
        });
    };

    let exists = matches!(value.read().as_ref(), Some(Ok(r)) if r.exists);
    let saving_val = saving();
    let message_val = message();

    rsx! {
        section { class: "admin-arena__section",
            div { class: "admin-arena__section-head",
                span { class: "admin-arena__section-title", "분석 페이지 생성 한도" }
                span { class: "admin-arena__section-meta",
                    if exists {
                        "DB 저장됨"
                    } else {
                        "기본값 사용 중"
                    }
                }
            }
            div { class: "admin-arena__form admin-arena__form--stretch",
                label {
                    class: "admin-arena__form-label",
                    r#for: "analyze-quota-limit",
                    "스페이스당 한도 (Non-Enterprise)"
                }
                div { class: "admin-arena__form-row",
                    input {
                        id: "analyze-quota-limit",
                        class: "admin-arena__form-input",
                        r#type: "number",
                        min: "0",
                        value: "{input_limit}",
                        oninput: move |evt| {
                            if let Ok(v) = evt.value().parse::<i64>() {
                                input_limit.set(v);
                            }
                        },
                    }
                    button {
                        class: "admin-arena__btn",
                        r#type: "button",
                        disabled: saving_val,
                        onclick: on_save,
                        if saving_val {
                            "저장 중…"
                        } else {
                            "저장"
                        }
                    }
                }
                span { class: "admin-arena__form-hint",
                    "Enterprise 등급은 무제한 — 이 값과 무관"
                }
                if let Some((text, ok)) = message_val {
                    span { class: if ok { "admin-arena__status admin-arena__status--ok" } else { "admin-arena__status admin-arena__status--err" },
                        "{text}"
                    }
                }
            }
        }
    }
}
