use crate::common::*;
use crate::features::spaces::pages::actions::gamification::hooks::use_completion_flow;
use crate::features::spaces::pages::actions::gamification::hooks::CompletionState;
use crate::features::spaces::pages::actions::gamification::i18n::GamificationTranslate;
use crate::features::spaces::pages::actions::gamification::types::XpGainResponse;

mod level_up_scene;
mod unlock_reveal;
mod xp_gain_animation;

pub use level_up_scene::*;
pub use unlock_reveal::*;
pub use xp_gain_animation::*;

/// CSS keyframe animations for the completion overlay.
pub const COMPLETION_OVERLAY_CSS: Asset = asset!("/assets/completion_overlay.css");

/// Full-screen completion overlay driven by `use_completion_flow`.
///
/// When `response` is `Some`, renders:
/// 1. XP burst animation (always)
/// 2. Level-up scene (if `new_level > old_level`)
/// 3. Chapter complete badge (if `chapter_completed`)
/// 4. Role upgrade badge (if `role_upgraded.is_some()`)
/// 5. Unlock reveal (if `unlocked_actions` is non-empty)
/// 6. "Tap to continue" hint
///
/// The overlay dismisses on click and transitions to `CompletionState::Done`.
#[component]
pub fn CompletionOverlay(response: Signal<Option<XpGainResponse>>) -> Element {
    let tr: GamificationTranslate = use_translate();
    let (state, _trigger, dismiss) = use_completion_flow(response);

    let resp = response();
    let is_showing = state() == CompletionState::Showing && resp.is_some();

    if !is_showing {
        return rsx! {};
    }

    let resp = resp.unwrap();

    rsx! {
        // Load CSS keyframes
        document::Link { rel: "stylesheet", href: COMPLETION_OVERLAY_CSS }

        // Backdrop overlay
        div {
            class: "flex fixed inset-0 flex-col gap-6 justify-center items-center cursor-pointer z-[100] bg-background/80 backdrop-blur-sm",
            "data-testid": "completion-overlay",
            onclick: move |_| dismiss.call(()),

            // ── XP burst (always shown) ────────────────────────
            XpGainAnimation {
                xp_earned: resp.xp_earned,
                combo: resp.combo_multiplier,
                streak: resp.streak_multiplier,
            }

            // ── Level-up scene (conditional) ───────────────────
            LevelUpScene { old_level: resp.old_level, new_level: resp.new_level }

            // ── Chapter complete badge (conditional) ───────────
            if resp.chapter_completed {
                div { class: "animate-slide-up",
                    Badge {
                        color: BadgeColor::Purple,
                        variant: BadgeVariant::Rounded,
                        size: BadgeSize::Normal,
                        "{tr.completion_chapter_complete}"
                    }
                }
            }

            // ── Role upgrade badge (conditional) ───────────────
            if resp.role_upgraded.is_some() {
                div { class: "animate-slide-up",
                    Badge {
                        color: BadgeColor::Green,
                        variant: BadgeVariant::Rounded,
                        size: BadgeSize::Normal,
                        "{tr.completion_role_upgraded}"
                    }
                }
            }

            // ── Unlock reveal (conditional) ────────────────────
            UnlockReveal { unlocked_actions: resp.unlocked_actions.clone() }

            // ── Dismiss hint ───────────────────────────────────
            p { class: "mt-4 text-xs text-foreground-muted animate-slide-up",
                "{tr.completion_tap_dismiss}"
            }
        }
    }
}
