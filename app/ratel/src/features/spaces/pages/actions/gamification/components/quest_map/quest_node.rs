use crate::common::*;
use crate::features::spaces::pages::actions::gamification::i18n::GamificationTranslate;
use crate::features::spaces::pages::actions::gamification::types::{QuestNodeStatus, QuestNodeView};
use crate::features::spaces::pages::actions::types::SpaceActionType;
use crate::features::spaces::Route;

/// A single node tile on the Quest Map.
///
/// Visual variants by status:
/// - `Active` — `GlassAccent` card with projected XP + action-type badge + "BEGIN" CTA
/// - `Cleared` — `Glass` card with checkmark + title + earned XP
/// - `Locked` — dashed-border container with lock icon + title
/// - `RoleGated` — similar to Locked but shows the role requirement text
#[component]
pub fn QuestNode(
    node: QuestNodeView,
    space_id: SpacePartition,
) -> Element {
    let tr: GamificationTranslate = use_translate();
    let nav = use_navigator();

    let action_type_label = match node.action_type {
        SpaceActionType::Poll => "Poll",
        SpaceActionType::TopicDiscussion => "Discussion",
        SpaceActionType::Follow => "Follow",
        SpaceActionType::Quiz => "Quiz",
    };

    let action_type_color = match node.action_type {
        SpaceActionType::Poll => BadgeColor::Orange,
        SpaceActionType::TopicDiscussion => BadgeColor::Blue,
        SpaceActionType::Follow => BadgeColor::Pink,
        SpaceActionType::Quiz => BadgeColor::Purple,
    };

    let node_id = node.id.clone();
    let action_type = node.action_type.clone();
    let space_id_clone = space_id.clone();

    let navigate_to_action = move |_: MouseEvent| {
        let route = match action_type {
            SpaceActionType::Poll => Route::PollActionPage {
                space_id: space_id_clone.clone(),
                poll_id: node_id.clone().into(),
            },
            SpaceActionType::TopicDiscussion => Route::DiscussionActionPage {
                space_id: space_id_clone.clone(),
                discussion_id: node_id.clone().into(),
            },
            SpaceActionType::Follow => Route::FollowActionPage {
                space_id: space_id_clone.clone(),
                follow_id: node_id.clone().into(),
            },
            SpaceActionType::Quiz => Route::QuizActionPage {
                space_id: space_id_clone.clone(),
                quiz_id: node_id.clone().into(),
            },
        };
        nav.push(route);
    };

    let begin_label = tr.quest_begin.to_string();
    let locked_label = tr.quest_locked.to_string();
    let cleared_label = tr.quest_cleared.to_string();
    let role_gated_label = tr.quest_role_gated.to_string();
    let xp_suffix = tr.xp_suffix.to_string();

    match node.status {
        QuestNodeStatus::Active => rsx! {
            Card {
                variant: CardVariant::GlassAccent,
                direction: CardDirection::Col,
                class: "gap-3 w-full cursor-pointer",
                "data-testid": "quest-node-active",
                onclick: navigate_to_action,

                Row {
                    main_axis_align: MainAxisAlign::Between,
                    cross_axis_align: CrossAxisAlign::Start,
                    class: "gap-2 w-full",
                    Badge {
                        color: action_type_color,
                        variant: BadgeVariant::Rounded,
                        {action_type_label}
                    }
                    span { class: "text-xs font-bold text-accent", "{node.projected_xp} {xp_suffix}" }
                }

                p { class: "w-full text-base font-semibold text-text-primary truncate",
                    "{node.title}"
                }

                Row { main_axis_align: MainAxisAlign::End, class: "w-full",
                    Badge {
                        color: BadgeColor::Green,
                        variant: BadgeVariant::Rounded,
                        {begin_label.as_str()}
                    }
                }
            }
        },

        QuestNodeStatus::Cleared => rsx! {
            Card {
                variant: CardVariant::Glass,
                direction: CardDirection::Col,
                class: "gap-3 w-full opacity-75",
                "data-testid": "quest-node-cleared",

                Row {
                    main_axis_align: MainAxisAlign::Between,
                    cross_axis_align: CrossAxisAlign::Start,
                    class: "gap-2 w-full",
                    Badge {
                        color: action_type_color,
                        variant: BadgeVariant::Rounded,
                        {action_type_label}
                    }
                    Row { class: "gap-1 items-center",
                        lucide_dioxus::CircleCheck { class: "w-4 h-4 text-accent" }
                        span { class: "text-xs text-foreground-muted", {cleared_label.as_str()} }
                    }
                }

                p { class: "w-full text-sm font-semibold text-text-primary truncate",
                    "{node.title}"
                }

                if let Some(quiz_result) = &node.quiz_result {
                    Row { class: "gap-2 text-xs text-foreground-muted",
                        span { "{quiz_result.score}/{quiz_result.total}" }
                        if quiz_result.passed {
                            Badge {
                                color: BadgeColor::Green,
                                variant: BadgeVariant::Rounded,
                                "PASS"
                            }
                        } else {
                            Badge {
                                color: BadgeColor::Red,
                                variant: BadgeVariant::Rounded,
                                "FAIL"
                            }
                        }
                    }
                }
            }
        },

        QuestNodeStatus::Locked => rsx! {
            div {
                class: "flex flex-col gap-3 p-4 w-full border-2 border-dashed opacity-60 rounded-[12px] border-border",
                "data-testid": "quest-node-locked",

                Row {
                    main_axis_align: MainAxisAlign::Between,
                    cross_axis_align: CrossAxisAlign::Center,
                    class: "gap-2 w-full",
                    Badge {
                        color: action_type_color,
                        variant: BadgeVariant::Rounded,
                        {action_type_label}
                    }
                    lucide_dioxus::Lock { class: "w-4 h-4 text-foreground-muted" }
                }

                p { class: "w-full text-sm font-semibold text-foreground-muted truncate",
                    "{node.title}"
                }

                span { class: "text-xs italic text-foreground-muted", {locked_label.as_str()} }
            }
        },

        QuestNodeStatus::RoleGated => rsx! {
            div {
                class: "flex flex-col gap-3 p-4 w-full border-2 border-dashed opacity-60 rounded-[12px] border-border",
                "data-testid": "quest-node-role-gated",

                Row {
                    main_axis_align: MainAxisAlign::Between,
                    cross_axis_align: CrossAxisAlign::Center,
                    class: "gap-2 w-full",
                    Badge {
                        color: action_type_color,
                        variant: BadgeVariant::Rounded,
                        {action_type_label}
                    }
                    lucide_dioxus::ShieldOff { class: "w-4 h-4 text-foreground-muted" }
                }

                p { class: "w-full text-sm font-semibold text-foreground-muted truncate",
                    "{node.title}"
                }

                span { class: "text-xs italic text-foreground-muted", {role_gated_label.as_str()} }
            }
        },
    }
}
