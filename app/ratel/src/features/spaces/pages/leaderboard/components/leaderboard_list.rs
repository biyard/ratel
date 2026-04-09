use crate::features::activity::controllers::RankingEntryResponse;
use crate::features::spaces::pages::leaderboard::i18n::LeaderboardTranslate;

use super::*;

/// Scrollable leaderboard list showing ranked entries.
///
/// Highlights the current user's row with a gold gradient background and
/// a "YOU" badge next to their name.
#[component]
pub fn LeaderboardList(
    entries: Vec<RankingEntryResponse>,
    #[props(default)] current_user_pk: Option<String>,
) -> Element {
    let tr: LeaderboardTranslate = use_translate();

    if entries.is_empty() {
        return rsx! {
            Col {
                main_axis_align: MainAxisAlign::Center,
                cross_axis_align: CrossAxisAlign::Center,
                class: "py-12 w-full",
                span { class: "text-sm italic text-foreground-muted", "{tr.no_entries}" }
            }
        };
    }

    rsx! {
        Col { class: "gap-1 w-full", "data-testid": "leaderboard-list",

            // Header row
            Row {
                main_axis_align: MainAxisAlign::Between,
                cross_axis_align: CrossAxisAlign::Center,
                class: "py-2 px-4 text-xs font-semibold text-foreground-muted",

                span { class: "w-10 text-center", "{tr.rank_label}" }
                span { class: "flex-1", "" }
                span { class: "w-20 text-right", "{tr.score_label}" }
            }

            for entry in entries.iter() {
                {
                    let is_current_user = current_user_pk
                        .as_ref()
                        .is_some_and(|pk| pk == &entry.user_pk);

                    let row_class = if is_current_user {
                        "px-4 py-3 rounded-lg border bg-primary/10 border-primary/20"
                    } else {
                        "px-4 py-3 rounded-lg hover:bg-card-bg"
                    };

                    rsx! {
                        Row {
                            key: "{entry.user_pk}",
                            main_axis_align: MainAxisAlign::Between,
                            cross_axis_align: CrossAxisAlign::Center,
                            class: "{row_class}",
                            "data-testid": "leaderboard-row-{entry.rank}",

                            // Rank number
                            span { class: "w-10 text-sm font-semibold text-center text-text-primary", "#{entry.rank}" }

                            // Avatar + Name
                            Row {
                                cross_axis_align: CrossAxisAlign::Center,
                                class: "flex-1 gap-3 min-w-0",

                                Avatar { size: AvatarImageSize::Small,
                                    if !entry.avatar.is_empty() {
                                        AvatarImage { src: "{entry.avatar}", alt: "{entry.name}" }
                                    }
                                    AvatarFallback { "{entry.name.chars().next().unwrap_or('?')}" }
                                }

                                Row { cross_axis_align: CrossAxisAlign::Center, class: "gap-2 min-w-0",

                                    span { class: "text-sm font-medium truncate text-text-primary", "{entry.name}" }

                                    if is_current_user {
                                        Badge { color: BadgeColor::Orange, size: BadgeSize::Small, "{tr.you_label}" }
                                    }
                                }
                            }

                            // Score
                            span { class: "w-20 text-sm font-semibold text-right text-foreground-muted",
                                "{entry.total_score} XP"
                            }
                        }
                    }
                }
            }
        }
    }
}
