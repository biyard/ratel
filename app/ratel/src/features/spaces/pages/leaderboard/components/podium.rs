use crate::features::activity::controllers::RankingEntryResponse;

use super::*;

/// Podium component showing the top 3 leaderboard entries.
///
/// Renders a 3-column layout with the 1st-place entry in the center (tallest),
/// 2nd on the left, and 3rd on the right. Each podium slot shows a sphere
/// avatar (gold/teal/bronze) with the user's name and score.
#[component]
pub fn Podium(entries: Vec<RankingEntryResponse>) -> Element {
    if entries.is_empty() {
        return rsx! {};
    }

    let first = entries.first().cloned();
    let second = entries.get(1).cloned();
    let third = entries.get(2).cloned();

    rsx! {
        Row {
            main_axis_align: MainAxisAlign::Center,
            cross_axis_align: CrossAxisAlign::End,
            class: "gap-4 py-6 w-full",
            "data-testid": "leaderboard-podium",

            // 2nd place (left)
            if let Some(entry) = second {
                PodiumSlot {
                    entry,
                    rank: 2,
                    shape: AvatarShape::SphereTeal,
                    pedestal_class: "h-16 bg-gradient-to-t from-accent/20 to-accent/5",
                }
            }

            // 1st place (center, tallest)
            if let Some(entry) = first {
                PodiumSlot {
                    entry,
                    rank: 1,
                    shape: AvatarShape::Sphere,
                    pedestal_class: "h-24 bg-gradient-to-t from-primary/20 to-primary/5",
                }
            }

            // 3rd place (right)
            if let Some(entry) = third {
                PodiumSlot {
                    entry,
                    rank: 3,
                    shape: AvatarShape::SphereBronze,
                    pedestal_class: "h-12 bg-gradient-to-t from-[#cd7f32]/20 to-[#cd7f32]/5",
                }
            }
        }
    }
}

#[component]
fn PodiumSlot(
    entry: RankingEntryResponse,
    rank: u32,
    shape: AvatarShape,
    pedestal_class: String,
) -> Element {
    let rank_label = match rank {
        1 => "1st",
        2 => "2nd",
        3 => "3rd",
        _ => "",
    };

    rsx! {
        Col {
            cross_axis_align: CrossAxisAlign::Center,
            class: "flex-1 gap-2 max-w-[140px]",
            "data-testid": "podium-slot-{rank}",

            Avatar {
                size: if rank == 1 { AvatarImageSize::Large } else { AvatarImageSize::Medium },
                shape,
                if !entry.avatar.is_empty() {
                    AvatarImage { src: "{entry.avatar}", alt: "{entry.name}" }
                }
                AvatarFallback { "{entry.name.chars().next().unwrap_or('?')}" }
            }

            Col { cross_axis_align: CrossAxisAlign::Center, class: "gap-0.5",

                span { class: "text-xs font-bold text-primary", "{rank_label}" }

                span { class: "max-w-full text-sm font-semibold truncate text-text-primary",
                    "{entry.name}"
                }

                span { class: "text-xs text-foreground-muted", "{entry.total_score} XP" }
            }

            // Pedestal bar
            div { class: "w-full rounded-t-lg {pedestal_class}" }
        }
    }
}
