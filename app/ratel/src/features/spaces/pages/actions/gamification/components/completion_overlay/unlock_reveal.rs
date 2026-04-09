use crate::common::*;
use crate::features::spaces::pages::actions::gamification::i18n::GamificationTranslate;

/// Unlock reveal panel shown when completing an action unlocks new quests.
///
/// Renders a lock-open icon + "New quest unlocked" header + the list of
/// newly-available action titles. Skips rendering entirely when the
/// `unlocked_actions` list is empty.
#[component]
pub fn UnlockReveal(unlocked_actions: Vec<String>) -> Element {
    let tr: GamificationTranslate = use_translate();

    if unlocked_actions.is_empty() {
        return rsx! {};
    }

    rsx! {
        Col {
            main_axis_align: MainAxisAlign::Center,
            cross_axis_align: CrossAxisAlign::Center,
            class: "gap-3 animate-slide-up",
            "data-testid": "unlock-reveal",

            // Icon + header
            Row {
                main_axis_align: MainAxisAlign::Center,
                cross_axis_align: CrossAxisAlign::Center,
                class: "gap-2",

                lucide_dioxus::LockOpen { class: "w-5 h-5 text-accent" }
                span { class: "text-lg font-bold text-accent", "{tr.completion_quest_unlocked}" }
            }

            // List of unlocked action titles
            Col {
                main_axis_align: MainAxisAlign::Start,
                cross_axis_align: CrossAxisAlign::Center,
                class: "gap-1",

                for title in unlocked_actions.iter() {
                    Badge {
                        color: BadgeColor::Grey,
                        variant: BadgeVariant::Rounded,
                        "{title}"
                    }
                }
            }
        }
    }
}
