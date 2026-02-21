use crate::*;
use common::chrono::{DateTime, Utc};
use i18n::ActionCardTranslate;

mod i18n;

#[component]
pub fn ActionCard(action: SpaceAction, space_id: SpacePartition) -> Element {
    let tr: ActionCardTranslate = use_translate();
    let nav = navigator();
    let now = common::utils::time::get_now_timestamp_millis();

    let period = format_action_period(action.started_at, action.ended_at);
    let status = resolve_action_status(&action, now);
    let status_label = match status {
        ActionStatus::Draft => tr.status_draft,
        ActionStatus::Ongoing => tr.status_ongoing,
        ActionStatus::Closed => tr.status_closed,
    };
    let status_color = status_badge_color(status);
    let type_label = match action.action_type {
        SpaceActionType::StudyAndQuiz => tr.action_type_quiz,
        SpaceActionType::Poll => tr.action_type_poll,
        SpaceActionType::TopicDiscussion => tr.action_type_discussion,
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
    let action_id = action.action_id.clone();

    rsx! {
        button {
            class: "flex flex-col gap-[0.625rem] p-[0.9375rem] rounded-[1rem] border border-neutral-800 light:border-neutral-300 bg-neutral-900 light:bg-white hover:bg-neutral-800 light:hover:bg-neutral-50 transition-colors text-left w-full",
            onclick: move |_| {
                nav.push(Route::PollApp {
                    space_id: space_id.clone(),
                    rest: vec![action_id.clone()],
                });
            },

            // Top: badges + date
            div { class: "flex justify-between items-center gap-3 w-full",
                div { class: "flex items-center gap-2",
                    Badge {
                        color: type_badge_color,
                        variant: BadgeVariant::Rounded,
                        {type_label}
                    }
                    Badge { color: status_color, variant: BadgeVariant::Rounded, {status_label} }
                }

                if let Some(period) = period {
                    span { class: "text-[0.75rem]/[1rem] font-medium text-neutral-500",
                        {period}
                    }
                }
            }

            // Title (single line, clipped)
            div { class: "h-[1.75rem] overflow-hidden w-full",
                p { class: "text-[1.125rem]/[1.75rem] font-semibold text-white light:text-neutral-900",
                    {action.title}
                }
            }

            // Description
            p { class: "text-[0.75rem]/[1rem] font-medium text-neutral-300 light:text-neutral-600 break-words flex-1 min-h-0 w-full",
                {action.description}
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
        SpaceActionType::StudyAndQuiz => BadgeColor::Purple,
        SpaceActionType::Poll => BadgeColor::Orange,
        SpaceActionType::TopicDiscussion => BadgeColor::Blue,
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
