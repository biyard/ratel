use crate::common::*;
use crate::features::spaces::pages::actions::gamification::i18n::GamificationTranslate;
use crate::features::spaces::pages::actions::gamification::types::*;
use crate::features::spaces::pages::actions::types::SpaceActionType;

mod rules_grid;
mod xp_math_breakdown;

pub use rules_grid::*;
pub use xp_math_breakdown::*;

/// Full-screen QuestBriefing overlay shown before the user enters an active quest.
///
/// Displays:
/// - Header row with action-type pill, chapter pill, title, and close button
/// - Hero XP number (large, centred, gold)
/// - XP math breakdown strip
/// - 2x2 rules grid (time, retries, prerequisites, unlocks)
/// - Bottom CTA row (Cancel / BEGIN)
#[component]
pub fn QuestBriefing(
    node: QuestNodeView,
    on_begin: EventHandler,
    on_cancel: EventHandler,
) -> Element {
    let tr: GamificationTranslate = use_translate();

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

    // Derive display values for the rules grid.
    let time_remaining = match node.ended_at {
        Some(_ts) => "\u{221e}".to_string(), // placeholder until countdown logic
        None => "\u{221e}".to_string(),
    };
    let retries = "\u{221e}".to_string(); // no retry limit for V1
    let prerequisites_met = node.depends_on.is_empty();
    let unlocks_next: Option<String> = None; // no sibling lookup in V1

    rsx! {
        Card {
            variant: CardVariant::GlassAccent,
            direction: CardDirection::Col,
            class: "gap-6 p-6 w-full max-w-lg",
            "data-testid": "quest-briefing",

            // ── Header row ──────────────────────────────────────────
            Row {
                main_axis_align: MainAxisAlign::Between,
                cross_axis_align: CrossAxisAlign::Center,
                class: "gap-2 w-full",

                Row {
                    cross_axis_align: CrossAxisAlign::Center,
                    class: "flex-wrap gap-2",
                    Badge {
                        color: action_type_color,
                        variant: BadgeVariant::Rounded,
                        {action_type_label}
                    }
                    Badge {
                        color: BadgeColor::Grey,
                        variant: BadgeVariant::Rounded,
                        "{node.chapter_id}"
                    }
                }

                Button {
                    size: ButtonSize::Icon,
                    style: ButtonStyle::Text,
                    onclick: move |_| on_cancel.call(()),
                    "aria-label": "close",
                    lucide_dioxus::X { class: "w-5 h-5" }
                }
            }

            // ── Title ───────────────────────────────────────────────
            h2 { class: "w-full text-lg font-bold text-text-primary", "{node.title}" }

            // ── Hero XP number ──────────────────────────────────────
            div { class: "py-2 w-full text-center",
                span { class: "text-5xl font-bold text-primary", "{node.projected_xp}" }
                span { class: "ml-2 text-foreground-muted", "{tr.briefing_xp_at_stake}" }
            }

            // ── XP math breakdown ───────────────────────────────────
            XpMathBreakdown {
                base_points: node.base_points,
                participants: 1, // placeholder until participant count is wired
                combo: 1.0,
                streak: 1.0,
                total: node.projected_xp,
            }

            // ── Rules grid ──────────────────────────────────────────
            RulesGrid {
                time_remaining,
                retries,
                prerequisites_met,
                unlocks_next,
            }

            // ── Bottom CTA ──────────────────────────────────────────
            Row { main_axis_align: MainAxisAlign::End, class: "gap-3 w-full",
                Button {
                    style: ButtonStyle::Secondary,
                    onclick: move |_| on_cancel.call(()),
                    "{tr.briefing_cancel}"
                }
                Button {
                    style: ButtonStyle::Primary,
                    onclick: move |_| on_begin.call(()),
                    "data-testid": "briefing-begin-btn",
                    "{tr.briefing_begin}"
                }
            }
        }
    }
}
