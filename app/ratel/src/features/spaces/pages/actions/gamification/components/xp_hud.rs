use crate::common::*;
use crate::features::spaces::pages::actions::gamification::hooks::use_xp_hud;
use crate::features::spaces::pages::actions::gamification::i18n::GamificationTranslate;

/// Horizontal XP HUD strip for the dungeon-hero header.
///
/// Renders four tiles on a glass surface:
///   1. Level badge sphere (radial gradient avatar)
///   2. XP progress bar with level + XP label
///   3. Streak chip (gradient-filled badge)
///   4. Combo chip (gradient-filled badge)
#[component]
pub fn XpHud() -> Element {
    let tr: GamificationTranslate = use_translate();
    let state = use_xp_hud();

    let xp_total = state.xp.saturating_add(state.xp_to_next);
    let xp_max = xp_total.max(1) as f64;
    let xp_value = state.xp.max(0) as f64;

    rsx! {
        Card {
            variant: CardVariant::Glass,
            direction: CardDirection::Row,
            cross_axis_align: CrossAxisAlign::Center,
            class: "gap-4 w-full",
            "data-testid": "xp-hud",

            Avatar {
                size: AvatarImageSize::Medium,
                shape: AvatarShape::Sphere,
                "data-testid": "xp-hud-level",
                AvatarFallback {
                    span { class: "text-sm font-bold", "{state.level}" }
                }
            }

            Col { class: "flex-1 gap-1 min-w-0",
                Row {
                    main_axis_align: MainAxisAlign::Between,
                    cross_axis_align: CrossAxisAlign::Center,
                    class: "gap-2 w-full",
                    span { class: "text-xs font-semibold text-text-primary",
                        "{tr.level_label} {state.level}"
                    }
                    span { class: "text-xs tabular-nums text-foreground-muted",
                        "{state.xp} / {xp_total} {tr.xp_suffix}"
                    }
                }
                Progress {
                    value: xp_value,
                    max: xp_max,
                    "aria-label": "{tr.xp_progress_aria}",
                    ProgressIndicator {}
                }
            }

            Badge {
                color: BadgeColor::Orange,
                variant: BadgeVariant::Rounded,
                fill: BadgeFill::Gradient,
                "data-testid": "xp-hud-streak",
                "🔥 {state.streak_days} {tr.streak_suffix}"
            }

            Badge {
                color: BadgeColor::Purple,
                variant: BadgeVariant::Rounded,
                fill: BadgeFill::Gradient,
                "data-testid": "xp-hud-combo",
                "{tr.combo_label} ×{state.combo_multiplier:.1}"
            }
        }
    }
}
