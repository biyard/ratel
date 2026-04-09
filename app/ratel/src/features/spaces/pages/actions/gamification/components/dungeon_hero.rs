use crate::common::*;
use crate::features::spaces::pages::actions::gamification::components::{
    LeaderboardRail, TitleRegion, XpHud,
};
use crate::features::spaces::space_common::hooks::use_space;

/// The persistent Dungeon Hero band that renders above every space sub-page.
///
/// Composes three sub-components on a glass-primary surface:
///   1. `TitleRegion` — "Dungeon · Floor N", space title, party stats
///   2. `LeaderboardRail` — top-3 avatars + YOU pill
///   3. `XpHud` — level badge, XP progress, streak/combo chips
#[component]
pub fn DungeonHero(space_id: ReadSignal<SpacePartition>) -> Element {
    // chapter_count is a static 2 until Phase 4 wires list_chapters.
    let chapter_count: u32 = 2;

    // SpaceResponse is loaded via the space context (already set up by SpaceLayout).
    let space = use_space()();

    rsx! {
        Card {
            variant: CardVariant::GlassPrimary,
            direction: CardDirection::Col,
            class: "gap-4 w-full",
            "data-testid": "dungeon-hero",

            // Top row: title region on the left, leaderboard rail on the right
            Row {
                main_axis_align: MainAxisAlign::Between,
                cross_axis_align: CrossAxisAlign::Start,
                class: "gap-4 w-full",
                TitleRegion { space, chapter_count }
                LeaderboardRail { space_id }
            }

            // Bottom row: XP HUD strip
            XpHud {}
        }
    }
}
