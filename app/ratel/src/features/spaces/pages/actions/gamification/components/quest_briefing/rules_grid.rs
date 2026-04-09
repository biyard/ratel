use crate::common::*;
use crate::features::spaces::pages::actions::gamification::i18n::GamificationTranslate;

/// 2x2 grid of rule cards summarising quest constraints.
///
/// Each cell shows an icon, a label, and a value. The four cells are:
/// 1. Time remaining  (clock icon)
/// 2. Retries allowed (refresh icon)
/// 3. Prerequisites   (check / x icon depending on `prerequisites_met`)
/// 4. Unlocks next    (arrow-right icon + quest title or "—")
#[component]
pub fn RulesGrid(
    time_remaining: String,
    retries: String,
    prerequisites_met: bool,
    #[props(default)] unlocks_next: Option<String>,
) -> Element {
    let tr: GamificationTranslate = use_translate();

    let prereq_label = if prerequisites_met {
        tr.briefing_met.to_string()
    } else {
        tr.briefing_not_met.to_string()
    };

    let unlocks_label = unlocks_next.unwrap_or_else(|| "\u{2014}".to_string());

    rsx! {
        div {
            class: "grid grid-cols-2 gap-3 w-full",
            "data-testid": "rules-grid",

            // Time remaining
            Card {
                variant: CardVariant::Outlined,
                direction: CardDirection::Col,
                class: "gap-2",
                Row { cross_axis_align: CrossAxisAlign::Center, class: "gap-2",
                    lucide_dioxus::Clock { class: "w-4 h-4 text-foreground-muted" }
                    span { class: "text-xs text-foreground-muted", "{tr.briefing_time_remaining}" }
                }
                span { class: "text-sm font-semibold text-text-primary", "{time_remaining}" }
            }

            // Retries allowed
            Card {
                variant: CardVariant::Outlined,
                direction: CardDirection::Col,
                class: "gap-2",
                Row { cross_axis_align: CrossAxisAlign::Center, class: "gap-2",
                    lucide_dioxus::RefreshCw { class: "w-4 h-4 text-foreground-muted" }
                    span { class: "text-xs text-foreground-muted", "{tr.briefing_retries}" }
                }
                span { class: "text-sm font-semibold text-text-primary", "{retries}" }
            }

            // Prerequisites
            Card {
                variant: CardVariant::Outlined,
                direction: CardDirection::Col,
                class: "gap-2",
                Row { cross_axis_align: CrossAxisAlign::Center, class: "gap-2",
                    if prerequisites_met {
                        lucide_dioxus::CircleCheck { class: "w-4 h-4 text-accent" }
                    } else {
                        lucide_dioxus::CircleX { class: "w-4 h-4 text-destructive" }
                    }
                    span { class: "text-xs text-foreground-muted", "{tr.briefing_prerequisites}" }
                }
                span { class: "text-sm font-semibold text-text-primary", "{prereq_label}" }
            }

            // Unlocks next
            Card {
                variant: CardVariant::Outlined,
                direction: CardDirection::Col,
                class: "gap-2",
                Row { cross_axis_align: CrossAxisAlign::Center, class: "gap-2",
                    lucide_dioxus::ArrowRight { class: "w-4 h-4 text-foreground-muted" }
                    span { class: "text-xs text-foreground-muted", "{tr.briefing_unlocks_next}" }
                }
                span { class: "text-sm font-semibold text-text-primary truncate", "{unlocks_label}" }
            }
        }
    }
}
