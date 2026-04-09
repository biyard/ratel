use crate::common::*;
use crate::features::spaces::pages::actions::gamification::i18n::GamificationTranslate;

/// Horizontal XP formula breakdown strip.
///
/// Displays the multiplicative formula that produces the projected XP:
/// `base_points P base × participants explorers × ×combo combo × ×streak streak = total XP`
///
/// Each factor is rendered as a `Badge` chip with a distinctive color so the
/// user can quickly parse which multiplier contributes most.
#[component]
pub fn XpMathBreakdown(
    base_points: i64,
    participants: u32,
    combo: f32,
    streak: f32,
    total: i64,
) -> Element {
    let tr: GamificationTranslate = use_translate();

    rsx! {
        Row {
            main_axis_align: MainAxisAlign::Center,
            cross_axis_align: CrossAxisAlign::Center,
            class: "flex-wrap gap-2 w-full",
            "data-testid": "xp-math-breakdown",

            // Base points factor
            Badge { color: BadgeColor::Grey, size: BadgeSize::Normal,
                "{base_points} {tr.briefing_base}"
            }

            span { class: "text-foreground-muted", "\u{00d7}" }

            // Participant count factor
            Badge { color: BadgeColor::Orange, size: BadgeSize::Normal,
                "{participants} {tr.briefing_explorers}"
            }

            span { class: "text-foreground-muted", "\u{00d7}" }

            // Combo multiplier factor
            Badge { color: BadgeColor::Blue, size: BadgeSize::Normal,
                "\u{00d7}{combo:.1} {tr.briefing_combo}"
            }

            span { class: "text-foreground-muted", "\u{00d7}" }

            // Streak multiplier factor
            Badge { color: BadgeColor::Grey, size: BadgeSize::Normal,
                "\u{00d7}{streak:.1} {tr.briefing_streak}"
            }

            span { class: "text-lg font-bold text-foreground-muted", "=" }

            // Total XP result
            span { class: "text-2xl font-bold text-primary", "{total} XP" }
        }
    }
}
