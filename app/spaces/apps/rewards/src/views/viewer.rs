use crate::controllers::list_space_rewards;
use crate::i18n::SpaceRewardTranslate;
use crate::*;
use space_common::models::SpaceRewardResponse;

#[derive(Clone, PartialEq)]
enum ClaimStatus {
    Claimed,
    Available { current: i64, max: i64, unit: &'static str },
    Unlimited,
}

fn get_claim_status(reward: &SpaceRewardResponse) -> ClaimStatus {
    if reward.period == RewardPeriod::Once {
        return if reward.user_claims >= 1 {
            ClaimStatus::Claimed
        } else {
            ClaimStatus::Available {
                current: reward.user_claims,
                max: 1,
                unit: "claims",
            }
        };
    }

    match &reward.condition {
        RewardCondition::MaxUserClaims(max) => {
            if reward.user_claims >= *max {
                ClaimStatus::Claimed
            } else {
                ClaimStatus::Available {
                    current: reward.user_claims,
                    max: *max,
                    unit: "claims",
                }
            }
        }
        RewardCondition::MaxUserPoints(max) => {
            if reward.user_points >= *max {
                ClaimStatus::Claimed
            } else {
                ClaimStatus::Available {
                    current: reward.user_points,
                    max: *max,
                    unit: "points",
                }
            }
        }
        _ => ClaimStatus::Unlimited,
    }
}

fn behavior_label(behavior: &RewardUserBehavior, tr: &SpaceRewardTranslate) -> String {
    match behavior {
        RewardUserBehavior::RespondPoll => tr.respond_poll.to_string(),
    }
}

fn period_label(period: &RewardPeriod, tr: &SpaceRewardTranslate) -> String {
    match period {
        RewardPeriod::Once => tr.period_once.to_string(),
        RewardPeriod::Hourly => tr.period_hourly.to_string(),
        RewardPeriod::Daily => tr.period_daily.to_string(),
        RewardPeriod::Weekly => tr.period_weekly.to_string(),
        RewardPeriod::Monthly => tr.period_monthly.to_string(),
        RewardPeriod::Yearly => tr.period_yearly.to_string(),
        RewardPeriod::Unlimited => tr.period_unlimited.to_string(),
    }
}

#[component]
fn RewardViewCard(reward: SpaceRewardResponse, tr: SpaceRewardTranslate) -> Element {
    let status = get_claim_status(&reward);
    let is_claimed = status == ClaimStatus::Claimed;
    let per_claim_points = reward.points * reward.credits.max(1);

    let border_class = if is_claimed {
        "border-2 rounded-lg p-4 bg-card-bg border-btn-primary-bg/40"
    } else {
        "border-2 rounded-lg p-4 bg-card-bg border-card-border"
    };

    let icon_class = if is_claimed {
        "w-10 h-10 rounded-full flex items-center justify-center shrink-0"
    } else {
        "w-10 h-10 rounded-full flex items-center justify-center shrink-0 bg-btn-primary-bg/10"
    };

    rsx! {
        div { class: "{border_class}",
            div { class: "flex items-center gap-3",
                div { class: "{icon_class}",
                    if is_claimed {
                        span { class: "text-btn-primary-bg text-xl", "✓" }
                    } else {
                        span { class: "text-btn-primary-bg text-xl", "🎁" }
                    }
                }

                div { class: "flex-1 min-w-0",
                    div { class: "flex items-center gap-2",
                        h4 { class: "text-base font-semibold text-font-primary truncate",
                            {behavior_label(&reward.behavior, &tr)}
                        }
                        if is_claimed {
                            span { class: "px-2 py-0.5 text-xs rounded border border-yellow-500/60 bg-yellow-500/10 text-yellow-500",
                                {tr.claimed}
                            }
                        } else {
                            span { class: "px-2 py-0.5 text-xs rounded border border-btn-primary-bg/60 bg-btn-primary-bg/10 text-btn-primary-text",
                                {period_label(&reward.period, &tr)}
                            }
                        }
                    }
                    if !reward.description.is_empty() {
                        p { class: "text-sm text-font-secondary truncate",
                            {reward.description.clone()}
                        }
                    }
                }
            }

            match &status {
                ClaimStatus::Available { current, max, unit } => {
                    let pct = if *max > 0 { (*current as f64 / *max as f64 * 100.0).min(100.0) } else { 0.0 };
                    let unit_label = if *unit == "claims" { tr.claims_unit.to_string() } else { tr.points_unit.to_string() };
                    rsx! {
                        div { class: "mt-3 flex items-center gap-3",
                            div { class: "flex-1 h-2 bg-font-secondary/20 rounded-full overflow-hidden",
                                div {
                                    class: "h-full bg-font-secondary rounded-full transition-all duration-300",
                                    style: "width: {pct}%",
                                }
                            }
                            span { class: "text-xs text-font-secondary whitespace-nowrap",
                                "{current} / {max} {unit_label}"
                            }
                        }
                    }
                }
                ClaimStatus::Claimed => rsx! {
                    div { class: "mt-3 flex items-center gap-3",
                        div { class: "flex-1 h-2 rounded-full overflow-hidden",
                            div { class: "h-full bg-btn-primary-bg rounded-full w-full" }
                        }
                        span { class: "text-xs text-font-secondary whitespace-nowrap",
                            {tr.completed}
                        }
                    }
                },
                ClaimStatus::Unlimited => rsx! {
                    div { class: "mt-3 flex items-center gap-2",
                        span { class: "text-font-secondary", "∞" }
                        span { class: "text-xs text-font-secondary", {tr.no_limit} }
                    }
                },
            }

            div { class: "mt-3 flex items-center justify-between text-sm",
                span { class: "text-font-secondary",
                    "{tr.per_claim}: "
                    span { class: "font-semibold text-font-primary",
                        "{per_claim_points} P"
                    }
                }
                span { class: "text-font-secondary",
                    "{tr.earned}: "
                    span { class: "font-semibold text-btn-primary-text",
                        "{reward.user_points} P"
                    }
                }
            }
        }
    }
}

#[component]
pub fn ViewerPage(space_id: SpacePartition) -> Element {
    let tr: SpaceRewardTranslate = use_translate();
    let mut rewards = use_signal(Vec::<SpaceRewardResponse>::new);
    let mut did_load = use_signal(|| false);

    let space_id_clone = space_id.clone();
    use_effect(move || {
        if did_load() {
            return;
        }
        did_load.set(true);
        let space_id = space_id_clone.clone();
        spawn(async move {
            if let Ok(res) = list_space_rewards(space_id, None).await {
                rewards.set(res.items);
            }
        });
    });

    let reward_list = rewards();

    if reward_list.is_empty() && did_load() {
        return rsx! {
            div { class: "flex items-center justify-center p-8",
                div { class: "flex flex-col items-center gap-2",
                    span { class: "text-4xl", "🎁" }
                    p { class: "text-font-secondary", {tr.no_rewards} }
                }
            }
        };
    }

    rsx! {
        div { class: "flex flex-col gap-4 p-4 w-full max-w-[1024px]",
            h2 { class: "text-xl font-bold text-font-primary",
                {tr.viewer_title}
            }

            div { class: "flex flex-col gap-3",
                for reward in reward_list.iter() {
                    RewardViewCard {
                        key: "{reward.sk}",
                        reward: reward.clone(),
                        tr: tr.clone(),
                    }
                }
            }
        }
    }
}
