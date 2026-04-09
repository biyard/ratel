use crate::common::*;
use crate::features::spaces::pages::actions::gamification::i18n::GamificationTranslate;

/// "LEVEL UP!" animation scene rendered when `new_level > old_level`.
///
/// Shows a gold-glow ring around the new level number, an old -> new
/// level transition, and a sunburst effect via the `.animate-level-glow`
/// CSS keyframe class.
#[component]
pub fn LevelUpScene(old_level: u32, new_level: u32) -> Element {
    let tr: GamificationTranslate = use_translate();

    if new_level <= old_level {
        return rsx! {};
    }

    rsx! {
        Col {
            main_axis_align: MainAxisAlign::Center,
            cross_axis_align: CrossAxisAlign::Center,
            class: "gap-4",
            "data-testid": "level-up-scene",

            // Header
            h2 { class: "text-3xl font-bold text-primary animate-xp-burst", "{tr.completion_level_up}" }

            // Old -> New level row
            Row {
                main_axis_align: MainAxisAlign::Center,
                cross_axis_align: CrossAxisAlign::Center,
                class: "gap-3 animate-slide-up",

                // Old level
                span { class: "text-2xl font-semibold text-foreground-muted",
                    "{tr.level_label} {old_level}"
                }

                // Arrow
                lucide_dioxus::ArrowRight { class: "w-6 h-6 text-foreground-muted" }

                // New level with glow
                div { class: "p-1 rounded-full animate-level-glow",
                    Badge {
                        color: BadgeColor::Green,
                        size: BadgeSize::Normal,
                        variant: BadgeVariant::Rounded,
                        "{tr.level_label} {new_level}"
                    }
                }
            }
        }
    }
}
