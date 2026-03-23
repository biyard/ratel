use crate::common::chrono::{DateTime, Utc};
use crate::features::spaces::pages::actions::*;
use crate::features::spaces::space_common::hooks::{use_space, use_space_role};
use i18n::ActionCardTranslate;

mod i18n;

#[component]
pub fn ActionCard(action: SpaceActionSummary, space_id: SpacePartition) -> Element {
    let tr: ActionCardTranslate = use_translate();
    let nav = navigator();
    let mut toast = use_toast();
    let space = use_space();
    let role = use_space_role()();
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
        SpaceActionType::Follow => tr.action_type_follow,
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
    let quiz_score_text = match (action.quiz_score, action.quiz_total_score) {
        (Some(my_score), Some(total_score)) => Some(format!("{my_score}/{total_score}")),
        _ => None,
    };
    let quiz_result_label =
        action
            .quiz_passed
            .map(|passed| if passed { tr.quiz_pass } else { tr.quiz_failed });
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
    let is_closed = matches!(status, ActionStatus::Closed);
    let card_class = if is_closed {
        "flex flex-col w-full text-left border transition-colors cursor-pointer gap-[0.625rem] !rounded-[1rem] !bg-neutral-900 light:!bg-white !p-[0.9375rem] border-neutral-800 light:border-neutral-300 opacity-60"
    } else {
        "flex flex-col w-full text-left border transition-colors cursor-pointer gap-[0.625rem] !rounded-[1rem] !bg-neutral-900 light:!bg-white !p-[0.9375rem] border-neutral-800 light:border-neutral-300 light:hover:!bg-neutral-50 hover:!bg-neutral-800"
    };
    rsx! {
        SpaceCard {
            class: card_class.to_string(),
            onclick: {
                move |_| {
                    if role != SpaceUserRole::Creator {
                        let enable_action = matches!(status, ActionStatus::Ongoing)
                            && action.prerequisite;

                        if !enable_action {
                            let space_status = space().status;
                            if !matches!(space_status, Some(SpaceStatus::Started)) {
                                toast.error(crate::common::Error::SpaceNotStarted);
                                return;
                            }
                            if matches!(status, ActionStatus::Closed) {
                                toast.error(crate::common::Error::ActionEnded);
                                return;
                            }
                        }
                    }
                    let url = url.clone();
                    nav.push(url);
                }
            },

            // Top: badges + date
            div { class: "flex gap-3 justify-between items-start w-full max-mobile:flex-col",
                div { class: "flex gap-2 items-center",
                    Badge {
                        color: type_badge_color,
                        variant: BadgeVariant::Rounded,
                        {type_label}
                    }

                    if action.action_type != SpaceActionType::Follow {
                        Badge {
                            color: status_color,
                            variant: BadgeVariant::Rounded,
                            {status_label}
                        }
                    }

                    if action.prerequisite {
                        Badge {
                            color: BadgeColor::Red,
                            variant: BadgeVariant::Rounded,
                            {tr.prerequisite}
                        }
                    }
                }

                if let Some(period) = period {
                    span { class: "font-medium text-[0.75rem]/[1rem] text-neutral-500 max-mobile:self-end",
                        {period}
                    }
                }
            }

            // Title
            p {
                class: "w-full font-semibold text-[1.125rem]/[1.75rem] truncate",
                class: if action.title.is_empty() {
                    "text-foreground-muted italic"
                } else {
                    "text-text-primary"
                },
                {title}
            }

            // Description
            p {
                class: "flex-1 w-full min-h-0 font-medium break-words line-clamp-1 text-[0.75rem]/[1rem] text-neutral-300 light:text-neutral-600",
                class: if action.description.is_empty() { " text-white light:text-neutral-900" },
                dangerous_inner_html: description,
            }

            if has_stats {
                // Bottom: points + score (border-top as separator)
                div { class: "flex gap-2 items-center w-full border-t border-neutral-800 light:border-neutral-300 pt-[0.6875rem]",
                    if point_value != 0 {
                        span { class: "inline-flex items-center font-semibold text-white gap-[6px] h-[32px] px-[10px] rounded-[10px] bg-neutral-800/50 light:bg-neutral-100 light:text-neutral-900 text-[15px]/[22px]",
                            {points}
                        }
                    }
                    if score_value != 0 {
                        span { class: "inline-flex items-center font-semibold text-white gap-[6px] h-[32px] px-[10px] rounded-[10px] bg-neutral-800/50 light:bg-neutral-100 light:text-neutral-900 text-[15px]/[22px]",
                            {score}
                        }
                    }
                }
            }

            if action.action_type == SpaceActionType::Quiz && quiz_score_text.is_some() {
                div { class: "flex gap-2 justify-end mt-auto w-full",
                    if let Some(result) = quiz_result_label {
                        span {
                            class: "inline-flex items-center px-2.5 font-semibold h-[26px] rounded-[8px] text-[12px]/[18px]",
                            class: if action.quiz_passed == Some(true) { "bg-sp-act-quiz-pass-bg text-sp-act-quiz-pass-text" } else { "bg-sp-act-quiz-fail-bg text-sp-act-quiz-fail-text" },
                            {result}
                        }
                    }
                    if let Some(score_text) = quiz_score_text {
                        span { class: "inline-flex items-center px-2.5 font-semibold h-[26px] rounded-[8px] bg-primary/20 text-[12px]/[18px] text-primary",
                            {score_text}
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

fn resolve_action_status(action: &SpaceActionSummary, now_millis: i64) -> ActionStatus {
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
        SpaceActionType::Follow => BadgeColor::Pink,
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
