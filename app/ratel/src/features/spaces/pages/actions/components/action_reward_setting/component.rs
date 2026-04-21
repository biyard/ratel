use crate::features::spaces::pages::actions::*;
use crate::features::spaces::pages::actions::components::ActionCommonSettingsTranslate;
use crate::features::spaces::pages::actions::controllers::{
    UpdateSpaceActionRequest, update_space_action,
};

/// Self-contained Reward + Boost setting card styled to match the arena
/// config layout. Handles membership fetching, credit accounting, and
/// persists via `update_space_action::Credits`.
#[component]
pub fn ActionRewardSetting(
    space_id: ReadSignal<SpacePartition>,
    action_id: ReadSignal<String>,
    saved_credits: u64,
    action_status: Option<SpaceActionStatus>,
    #[props(default)] on_change: EventHandler<u64>,
) -> Element {
    let tr: ActionCommonSettingsTranslate = use_translate();
    let tr_reward: ArenaRewardTranslate = use_translate();
    let mut toast = crate::common::use_toast();
    let mut current_credits = use_signal(move || saved_credits);
    let mut enable_reward = use_signal(move || saved_credits > 0);

    let space = crate::features::spaces::space_common::hooks::use_space();
    let current_space = space();
    let reward_locked = crate::features::spaces::pages::actions::is_action_locked(
        current_space.status,
        action_status.as_ref(),
    );
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let personal_username = user_ctx
        .read()
        .user
        .as_ref()
        .map(|u| u.username.clone())
        .unwrap_or_default();
    let owner_username = current_space.author_username.clone();
    let team_detail =
        use_server_future(use_reactive((&owner_username,), |(username,)| async move {
            crate::features::social::controllers::find_team_handler(username.to_string()).await
        }))?;
    let team_detail_read = team_detail.read();
    let team_detail = team_detail_read
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .cloned();
    let is_team_author = current_space.author_type == crate::common::UserType::Team;
    let team_username = team_detail
        .as_ref()
        .map(|team| team.username.clone())
        .unwrap_or_else(|| current_space.author_username.clone());
    let is_team_space = is_team_author || team_detail.is_some();
    let upgrade_route = if is_team_space {
        format!("/{}/team-memberships", team_username)
    } else {
        format!("/{personal_username}/memberships")
    };
    let user_membership = crate::features::auth::hooks::use_user_membership();
    let team_membership = use_server_future(use_reactive(
        (&team_username, &is_team_space),
        |(team_username, is_team_space)| async move {
            if is_team_space && !team_username.is_empty() {
                crate::features::membership::controllers::get_team_membership_handler(
                    team_username.to_string(),
                )
                .await
                .map(Some)
            } else {
                Ok(None)
            }
        },
    ))?;
    let team_membership_read = team_membership.read();
    let team_membership = team_membership_read
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .and_then(|membership| membership.clone());
    let base_is_paid = if is_team_space {
        team_membership
            .as_ref()
            .is_some_and(|membership| !membership.tier.0.contains("Free"))
    } else {
        user_membership
            .as_ref()
            .is_some_and(|membership| membership.is_paid())
    };
    let base_max_credits = if is_team_space {
        team_membership.as_ref().map_or(0, |membership| {
            membership.max_credits_per_space.max(0) as u64
        })
    } else {
        user_membership.as_ref().map_or(0, |membership| {
            membership.max_credits_per_space.max(0) as u64
        })
    };
    let base_remaining_credits = if is_team_space {
        team_membership
            .as_ref()
            .map_or(0, |membership| membership.remaining_credits.max(0) as u64)
    } else {
        user_membership
            .as_ref()
            .map_or(0, |membership| membership.remaining_credits.max(0) as u64)
    };
    let mut remaining_credits = use_signal(move || base_remaining_credits);
    let mut auth_ctx = use_context::<crate::features::auth::context::Context>();
    use_effect(move || {
        if !is_team_space {
            let new_credits = auth_ctx
                .user_context
                .read()
                .membership
                .as_ref()
                .map_or(0, |m| m.remaining_credits.max(0) as u64);
            remaining_credits.set(new_credits);
        }
    });

    let available_credits = remaining_credits().saturating_add(current_credits());
    let credits_value = current_credits();
    let boost_multiplier = credits_value.max(1);
    let total_reward = boost_multiplier.saturating_mul(10_000);
    let nav = use_navigator();

    let save_credits = move |next_credits: u64| {
        spawn(async move {
            let previous = current_credits();
            let req = UpdateSpaceActionRequest::Credits {
                credits: next_credits,
            };
            match update_space_action(space_id(), action_id(), req).await {
                Ok(_) => {
                    let delta = next_credits as i64 - previous as i64;
                    current_credits.set(next_credits);
                    remaining_credits.set(
                        (remaining_credits().saturating_sub(delta.max(0) as u64))
                            .saturating_add((-delta).max(0) as u64),
                    );
                    if !is_team_space {
                        let mut user_ctx = auth_ctx.user_context.write();
                        if let Some(membership) = user_ctx.membership.as_mut() {
                            membership.remaining_credits =
                                (membership.remaining_credits - delta).max(0);
                        }
                    }
                    toast.info(tr.reward_updated.to_string());
                    on_change.call(next_credits);
                }
                Err(e) => {
                    toast.error(e);
                }
            }
        });
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "arena-reward",
            // ── Header strip ─────────
            div { class: "arena-reward__head",
                div { class: "arena-reward__head-title",
                    span { class: "arena-reward__head-title-text", "{tr_reward.reward}" }
                    span { class: "arena-reward__head-info",
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
                    }
                }
                if base_is_paid && available_credits > 0 {
                    div {
                        class: "switch",
                        role: "switch",
                        tabindex: "0",
                        "aria-checked": enable_reward(),
                        "aria-disabled": reward_locked,
                        "data-testid": "reward-setting-toggle",
                        onclick: move |_| {
                            if reward_locked {
                                toast.info(tr_reward.locked_started.to_string());
                                return;
                            }
                            let new_enabled = !enable_reward();
                            enable_reward.set(new_enabled);
                            let new_credits = if new_enabled { 1 } else { 0 };
                            save_credits(new_credits);
                        },
                        span { class: "switch__track",
                            span { class: "switch__thumb" }
                        }
                    }
                } else {
                    button {
                        class: "arena-reward__unlock",
                        r#type: "button",
                        onclick: {
                            let upgrade_route = upgrade_route.clone();
                            move |_| {
                                nav.push(upgrade_route.clone());
                            }
                        },
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            rect {
                                x: "3",
                                y: "11",
                                width: "18",
                                height: "11",
                                rx: "2",
                            }
                            path { d: "M7 11V7a5 5 0 0 1 10 0v4" }
                        }
                        "{tr_reward.unlock}"
                    }
                }
            }

            // ── Expanded body ─────────
            if enable_reward() {
                div { class: "arena-reward__grid",
                    // Default Reward column
                    div { class: "arena-reward__column",
                        span { class: "arena-reward__column-label", "{tr_reward.default_reward}" }
                        div { class: "arena-reward__display",
                            div { class: "arena-reward__boost-badge",
                                div { class: "arena-reward__boost-icon",
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
                                span { class: "arena-reward__boost-mult",
                                    "x{boost_multiplier} {tr_reward.boost_label}"
                                    svg {
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        path { d: "M5 12h14" }
                                        path { d: "M12 5l7 7-7 7" }
                                    }
                                }
                            }
                            div { class: "arena-reward__display-body",
                                span { class: "arena-reward__display-value",
                                    "{format_number(total_reward)}"
                                }
                                span { class: "arena-reward__display-label",
                                    "{tr_reward.action_point_reward}"
                                }
                            }
                        }
                    }

                    // Boost Multiplier column
                    div { class: "arena-reward__column",
                        span { class: "arena-reward__column-label", "{tr_reward.boost_multiplier}" }
                        div { class: "arena-reward__boost-tile",
                            div { class: "arena-reward__boost-row",
                                span { class: "arena-reward__boost-row-label",
                                    "{tr_reward.credit_usage}"
                                }
                                input {
                                    class: "arena-reward__credit-input",
                                    r#type: "number",
                                    min: "0",
                                    "data-testid": "reward-credit-input",
                                    readonly: reward_locked,
                                    disabled: reward_locked,
                                    value: "{credits_value}",
                                    oninput: move |evt: FormEvent| {
                                        if reward_locked {
                                            return;
                                        }
                                        let val = evt.value().parse::<u64>().unwrap_or(0);
                                        let limit = if base_max_credits > 0 {
                                            val.min(base_max_credits).min(available_credits)
                                        } else {
                                            val.min(available_credits)
                                        };
                                        save_credits(limit);
                                    },
                                }
                            }
                            if base_max_credits > 0 {
                                div { class: "arena-reward__boost-row",
                                    span { class: "arena-reward__boost-row-label",
                                        "{tr_reward.max_credits}"
                                    }
                                    span { class: "arena-reward__boost-row-value",
                                        "{format_number(base_max_credits)}"
                                    }
                                }
                            }
                            div { class: "arena-reward__boost-row",
                                span { class: "arena-reward__boost-row-label",
                                    "{tr_reward.remaining_credits}"
                                }
                                span { class: "arena-reward__boost-row-value arena-reward__boost-row-value--green",
                                    "{format_number(remaining_credits())}"
                                }
                            }
                        }
                    }
                }

                // Info notice
                div { class: "arena-reward__info",
                    div { class: "arena-reward__info-icon",
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
                    }
                    div { class: "arena-reward__info-body",
                        span { class: "arena-reward__info-title", "{tr_reward.point_boost}" }
                        ul { class: "arena-reward__info-list",
                            li { "{tr_reward.point_boost_line_one}" }
                            li { "{tr_reward.point_boost_line_two}" }
                            li { "{tr_reward.point_boost_line_three}" }
                        }
                    }
                }
            }
        }
    }
}

fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

translate! {
    ArenaRewardTranslate;

    reward: { en: "Reward", ko: "보상" },
    unlock: { en: "Unlock", ko: "잠금 해제" },
    default_reward: { en: "Default Reward", ko: "기본 보상" },
    action_point_reward: { en: "Action Point Reward", ko: "액션 포인트 보상" },
    boost_label: { en: "Boost", ko: "부스트" },
    boost_multiplier: { en: "Boost Multiplier Settings", ko: "부스트 배율 설정" },
    credit_usage: { en: "Credit Usage", ko: "크레딧 사용량" },
    max_credits: { en: "Max Credits", ko: "최대 크레딧" },
    remaining_credits: { en: "Remaining Credits", ko: "잔여 크레딧" },
    point_boost: { en: "Point & Boost", ko: "포인트 & 부스트" },
    point_boost_line_one: {
        en: "Points are rewarded to participants when they complete the action.",
        ko: "포인트는 참여자가 액션을 완료하면 지급됩니다.",
    },
    point_boost_line_two: {
        en: "Boost multiplies the base reward points by the set multiplier.",
        ko: "부스트는 기본 보상 포인트에 설정된 배율을 곱합니다.",
    },
    point_boost_line_three: {
        en: "Credits are consumed when boost is applied to an action.",
        ko: "액션에 부스트를 적용하면 크레딧이 소모됩니다.",
    },
    locked_started: {
        en: "Rewards cannot be changed after the action has started.",
        ko: "액션이 시작된 이후에는 보상을 변경할 수 없습니다.",
    },
}
