use crate::common::*;
use crate::features::profile::i18n::ProfileTranslate;
use crate::features::spaces::pages::actions::gamification::controllers::profile::GlobalProfileResponse;

/// Hero card displayed at the top of the global player profile page.
///
/// Shows the user's level in a large badge, their display name, an XP
/// progress bar toward the next level, and a streak chip.
#[component]
pub fn HeroPlayerCard(profile: GlobalProfileResponse, user_name: String) -> Element {
    let tr: ProfileTranslate = use_translate();

    let level = profile.level;
    let total_xp = profile.total_xp;

    // Compute XP thresholds for current and next level.
    // level = floor(sqrt(total_xp / 1000)) + 1
    // So for level L, threshold = (L-1)^2 * 1000
    let current_threshold = ((level.saturating_sub(1)) as i64).pow(2) * 1000;
    let next_threshold = (level as i64).pow(2) * 1000;
    let xp_in_level = (total_xp - current_threshold).max(0);
    let xp_needed = (next_threshold - current_threshold).max(1);
    let progress_pct = ((xp_in_level as f64 / xp_needed as f64) * 100.0).min(100.0);

    let tier_text = match level {
        1..=4 => "Novice",
        5..=9 => "Apprentice",
        10..=19 => "Journeyman",
        20..=49 => "Expert",
        _ => "Master",
    };

    rsx! {
        Card {
            variant: CardVariant::Outlined,
            direction: CardDirection::Col,
            class: "gap-4 p-6 w-full",
            "data-testid": "hero-player-card",

            Row {
                cross_axis_align: CrossAxisAlign::Center,
                class: "gap-4 w-full",

                // Level badge
                div {
                    class: "flex justify-center items-center w-16 h-16 rounded-full shrink-0",
                    style: "background: radial-gradient(circle at 30% 30%, rgba(252, 179, 0, 0.6), rgba(110, 237, 216, 0.3), transparent);",
                    span { class: "text-2xl font-black text-text-primary", "{tr.level_label}{level}" }
                }

                Col { class: "flex-1 gap-1 min-w-0",
                    // Name and tier
                    h2 { class: "text-lg font-bold truncate text-text-primary", "{user_name}" }
                    span { class: "text-sm font-semibold text-primary", "{tier_text}" }

                    // XP progress bar
                    Row {
                        main_axis_align: MainAxisAlign::Between,
                        cross_axis_align: CrossAxisAlign::Center,
                        class: "gap-2 w-full",
                        span { class: "text-xs tabular-nums text-foreground-muted",
                            "{xp_in_level} / {xp_needed} {tr.xp_suffix}"
                        }
                    }
                    Progress {
                        value: progress_pct,
                        max: 100.0,
                        "aria-label": "{tr.xp_to_next_level}",
                        ProgressIndicator {}
                    }
                }
            }

            // Streak chip
            if profile.current_streak > 0 {
                Row { main_axis_align: MainAxisAlign::Start, class: "gap-2",
                    Badge {
                        color: BadgeColor::Orange,
                        variant: BadgeVariant::Rounded,
                        "data-testid": "profile-streak",
                        "{profile.current_streak} {tr.streak_fire}"
                    }
                }
            }
        }
    }
}
