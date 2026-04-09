use crate::common::*;
use crate::features::spaces::pages::actions::gamification::i18n::GamificationTranslate;
use crate::features::spaces::space_common::controllers::SpaceResponse;

/// Title region for the dungeon-hero header band.
///
/// Renders three stacked elements:
///   1. Mini "Dungeon · Floor N" label (uppercase, primary color)
///   2. Space title with primary→accent gradient text
///   3. Party stats row (explorer count, chapter count)
///
/// `chapter_count` drives the floor number (clamped to a minimum of 1) and the
/// chapter stat. Participant count is derived from `space.quota - space.remains`.
#[component]
pub fn TitleRegion(space: SpaceResponse, chapter_count: u32) -> Element {
    let tr: GamificationTranslate = use_translate();

    let floor_number = chapter_count.max(1);
    let participant_count = (space.quota - space.remains).max(0);

    rsx! {
        Col { class: "gap-2", "data-testid": "title-region",

            span {
                class: "text-xs font-semibold tracking-wider uppercase text-primary",
                "data-testid": "title-region-floor",
                "{tr.dungeon_label} · {tr.floor_label} {floor_number}"
            }

            h1 {
                class: "text-3xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-primary to-accent",
                "data-testid": "title-region-space-title",
                "{space.title}"
            }

            Row {
                cross_axis_align: CrossAxisAlign::Center,
                class: "gap-4 text-xs text-foreground-muted",
                "data-testid": "title-region-stats",

                span { class: "flex gap-1 items-center", "{participant_count} {tr.explorers_label}" }

                span { class: "flex gap-1 items-center", "{chapter_count} {tr.chapters_label}" }
            }
        }
    }
}
