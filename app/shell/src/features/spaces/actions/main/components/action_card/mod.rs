use crate::features::spaces::actions::main::*;
use crate::common::chrono::{DateTime, Utc};
use i18n::ActionCardTranslate;

mod i18n;

#[component]
pub fn ActionCard(action: SpaceAction, space_id: SpacePartition) -> Element {
    let tr: ActionCardTranslate = use_translate();
    let nav = navigator();
    let now = crate::common::utils::time::get_now_timestamp_millis();

    let period = format_action_period(action.started_at, action.ended_at);
    let status = resolve_action_status(&action, now);
    let status_label = match status {
        ActionStatus::Draft => tr.status_draft,
        ActionStatus::Ongoing => tr.status_ongoing,
        ActionStatus::Closed => tr.status_closed,
    };
    let status_color = status_badge_color(status);
    let type_label = match action.action_type {
        // SpaceActionType::StudyAndQuiz => tr.action_type_quiz,
        SpaceActionType::Poll => tr.action_type_poll,
        SpaceActionType::TopicDiscussion => tr.action_type_discussion,
        SpaceActionType::Subscription => tr.action_type_subscription,
        SpaceActionType::Quiz => tr.action_type_quiz,
    };
    let type_badge_color = action_type_badge_color(&action.action_type);
    let point_value = action.total_point.unwrap_or_default();
    let points = format!("{} P", format_with_commas(point_value));
    let score_value = action.total_score.unwrap_or_default();
    let score = if score_value >= 0 {
        format!("+{score_value}")
    } else {
        score_value.to_string()
    };
    let has_stats = point_value != 0 || score_value != 0;
    let url = action.get_url(&space_id);

    let title = if action.title.is_empty() {
        tr.untitled.to_string()
    } else {
        action.title.clone()
    };

    let description = if action.description.is_empty() {
        tr.no_description.to_string()
    } else {
        action.description.clone()
    };
    rsx! {
        SpaceCard {
            class: "flex flex-col gap-[0.625rem] w-full cursor-pointer text-left !rounded-[1rem] !bg-neutral-900 light:!bg-white !p-[0.9375rem] border border-neutral-800 light:border-neutral-300 hover:!bg-neutral-800 light:hover:!bg-neutral-50 transition-colors"
                .to_string(),
            onclick: move |_| {
                let url = url.clone();
                nav.push(url);
            },

            // Top: badges + date
            div { class: "flex justify-between items-center gap-3 w-full",
                div { class: "flex items-center gap-2",
                    Badge {
                        color: type_badge_color,
                        variant: BadgeVariant::Rounded,
                        {type_label}
                    }

                    if action.action_type != SpaceActionType::Subscription {
                        Badge {
                            color: status_color,
                            variant: BadgeVariant::Rounded,
                            {status_label}
                        }
                    }
                
                }

                if let Some(period) = period {
                    span { class: "text-[0.75rem]/[1rem] font-medium text-neutral-500",
                        {period}
                    }
                }
            }

            // Title
            p {
                class: "text-[1.125rem]/[1.75rem] font-semibold truncate w-full",
                class: if action.title.is_empty() { "text-neutral-500 italic" } else { "text-white light:text-neutral-900" },
                {title}
            }

            // Description
            p {
                class: "text-[0.75rem]/[1rem] font-medium text-neutral-300 light:text-neutral-600 break-words flex-1 min-h-0 w-full",
                class: if action.description.is_empty() { " text-white light:text-neutral-900" },
                {description}
            }

            if has_stats {
                // Bottom: points + score (border-top as separator)
                div { class: "flex items-center gap-2 w-full border-t border-neutral-800 light:border-neutral-300 pt-[0.6875rem]",
                    if point_value != 0 {
                        span { class: "inline-flex items-center gap-[6px] h-[32px] px-[10px] rounded-[10px] bg-neutral-800/50 light:bg-neutral-100 text-white light:text-neutral-900 text-[15px]/[22px] font-semibold",
                            {points}
                        }
                    }
                    if score_value != 0 {
                        span { class: "inline-flex items-center gap-[6px] h-[32px] px-[10px] rounded-[10px] bg-neutral-800/50 light:bg-neutral-100 text-white light:text-neutral-900 text-[15px]/[22px] font-semibold",
                            {score}
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum ActionStatus {
    Draft,
    Ongoing,
    Closed,
}

fn resolve_action_status(action: &SpaceAction, now_millis: i64) -> ActionStatus {
    match (action.started_at, action.ended_at) {
        (Some(started_at), Some(ended_at)) => {
            if now_millis < started_at {
                ActionStatus::Draft
            } else if now_millis <= ended_at {
                ActionStatus::Ongoing
            } else {
                ActionStatus::Closed
            }
        }
        (Some(started_at), None) => {
            if now_millis < started_at {
                ActionStatus::Draft
            } else {
                ActionStatus::Ongoing
            }
        }
        _ => ActionStatus::Draft,
    }
}

fn status_badge_color(status: ActionStatus) -> BadgeColor {
    match status {
        ActionStatus::Draft => BadgeColor::Grey,
        ActionStatus::Ongoing => BadgeColor::Green,
        ActionStatus::Closed => BadgeColor::Red,
    }
}

fn action_type_badge_color(action_type: &SpaceActionType) -> BadgeColor {
    match action_type {
        // SpaceActionType::StudyAndQuiz => BadgeColor::Purple,
        SpaceActionType::Poll => BadgeColor::Orange,
        SpaceActionType::TopicDiscussion => BadgeColor::Blue,
        SpaceActionType::Subscription => BadgeColor::Pink,
        SpaceActionType::Quiz => BadgeColor::Purple,
    }
}

fn format_action_period(started_at: Option<i64>, ended_at: Option<i64>) -> Option<String> {
    let (Some(started_at), Some(ended_at)) = (started_at, ended_at) else {
        return None;
    };

    let start = DateTime::<Utc>::from_timestamp_millis(started_at)?;
    let end = DateTime::<Utc>::from_timestamp_millis(ended_at)?;

    Some(format!(
        "{} - {}, {}",
        start.format("%b %-d"),
        end.format("%b %-d"),
        end.format("%Y")
    ))
}

fn format_with_commas(value: i64) -> String {
    let negative = value.is_negative();
    let digits = value.abs().to_string();
    let mut formatted = String::with_capacity(digits.len() + (digits.len() / 3));

    for (idx, ch) in digits.chars().rev().enumerate() {
        if idx != 0 && idx % 3 == 0 {
            formatted.push(',');
        }
        formatted.push(ch);
    }

    let mut formatted: String = formatted.chars().rev().collect();
    if negative {
        formatted.insert(0, '-');
    }

    formatted
}
