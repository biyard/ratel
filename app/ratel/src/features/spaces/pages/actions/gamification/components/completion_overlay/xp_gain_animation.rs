use crate::common::*;
use crate::features::spaces::pages::actions::gamification::i18n::GamificationTranslate;

/// Centred "+{xp_earned} XP" burst animation with combo/streak badge chips.
///
/// The grow-and-fade animation is driven by the `.animate-xp-burst` CSS class
/// (keyframes in `completion_overlay.css`). No JS timers are used in V1;
/// the animation plays once via `animation-fill-mode: forwards`.
#[component]
pub fn XpGainAnimation(
    xp_earned: i64,
    combo: f32,
    streak: f32,
) -> Element {
    let tr: GamificationTranslate = use_translate();

    rsx! {
        Col {
            main_axis_align: MainAxisAlign::Center,
            cross_axis_align: CrossAxisAlign::Center,
            class: "gap-4",
            "data-testid": "xp-gain-animation",

            // Hero XP number — large burst
            div { class: "animate-xp-burst",
                span { class: "text-6xl font-bold text-primary", "+{xp_earned}" }
                span { class: "ml-2 text-2xl font-bold text-primary", "{tr.xp_suffix}" }
            }

            // Subtitle
            p { class: "text-sm text-foreground-muted animate-slide-up", "{tr.completion_xp_earned}" }

            // Multiplier badges
            Row {
                main_axis_align: MainAxisAlign::Center,
                cross_axis_align: CrossAxisAlign::Center,
                class: "flex-wrap gap-2 animate-slide-up",

                if combo > 1.0 {
                    Badge {
                        color: BadgeColor::Blue,
                        variant: BadgeVariant::Rounded,
                        "\u{00d7}{combo:.1} {tr.combo_label}"
                    }
                }

                if streak > 1.0 {
                    Badge {
                        color: BadgeColor::Orange,
                        variant: BadgeVariant::Rounded,
                        "\u{00d7}{streak:.1} {tr.streak_suffix}"
                    }
                }
            }
        }
    }
}
