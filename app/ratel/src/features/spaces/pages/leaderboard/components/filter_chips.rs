use crate::features::spaces::pages::leaderboard::i18n::LeaderboardTranslate;

use super::*;

/// Filter chip row for the leaderboard page.
///
/// V1: renders visual-only chips for "All", "Chapter", and time-window
/// filters. Active filtering logic will be wired in a later phase.
#[component]
pub fn FilterChips(
    #[props(default)] chapter_id: Option<String>,
    #[props(default)] window: Option<String>,
) -> Element {
    let tr: LeaderboardTranslate = use_translate();

    let active_window = window.as_deref().unwrap_or("all_time");

    rsx! {
        Row {
            cross_axis_align: CrossAxisAlign::Center,
            class: "flex-wrap gap-2",
            "data-testid": "leaderboard-filter-chips",

            // Source filter chips
            Badge {
                color: if chapter_id.is_none() { BadgeColor::Blue } else { BadgeColor::Grey },
                variant: BadgeVariant::Rounded,
                "{tr.filter_all}"
            }

            Badge {
                color: if chapter_id.is_some() { BadgeColor::Blue } else { BadgeColor::Grey },
                variant: BadgeVariant::Rounded,
                "{tr.filter_chapter}"
            }

            // Spacer
            div { class: "mx-1 w-px h-4 bg-separator" }

            // Time window chips
            Badge {
                color: if active_window == "all_time" { BadgeColor::Purple } else { BadgeColor::Grey },
                variant: BadgeVariant::Rounded,
                "{tr.window_all_time}"
            }

            Badge {
                color: if active_window == "weekly" { BadgeColor::Purple } else { BadgeColor::Grey },
                variant: BadgeVariant::Rounded,
                "{tr.window_weekly}"
            }

            Badge {
                color: if active_window == "monthly" { BadgeColor::Purple } else { BadgeColor::Grey },
                variant: BadgeVariant::Rounded,
                "{tr.window_monthly}"
            }
        }
    }
}
