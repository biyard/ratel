use crate::common::*;
use crate::features::spaces::pages::actions::gamification::components::quest_map::{
    ChapterSection, CollapsedPastStrip,
};
use crate::features::spaces::pages::actions::gamification::hooks::use_quest_map;
use crate::features::spaces::pages::actions::gamification::i18n::GamificationTranslate;

/// Top-level Quest Map component for the participant view.
///
/// Loads the Quest Map via `use_quest_map` (suspends until data is
/// ready), then splits chapters into three groups:
///
/// 1. **Past** — all `is_complete == true`. Rendered as a
///    `CollapsedPastStrip` at the top.
/// 2. **Active / Current** — the first non-complete chapter. Shown
///    expanded with its node grid.
/// 3. **Locked** — remaining chapters that can't be acted on yet.
///    Each renders as a locked `ChapterSection`.
#[component]
pub fn QuestMap(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: GamificationTranslate = use_translate();
    let quest_map_loading_label = tr.quest_map_loading.to_string();
    let loader = use_quest_map(space_id)?;
    let quest_map = loader();

    let past_chapters: Vec<_> = quest_map
        .chapters
        .iter()
        .filter(|c| c.is_complete)
        .cloned()
        .collect();

    let active_chapter = quest_map.chapters.iter().find(|c| !c.is_complete).cloned();

    let locked_chapters: Vec<_> = quest_map
        .chapters
        .iter()
        .skip_while(|c| c.is_complete)
        .skip(1) // skip the active one
        .cloned()
        .collect();

    let space_id_val = space_id();

    if quest_map.chapters.is_empty() {
        return rsx! {
            Col { class: "items-center py-12 w-full",
                span { class: "text-sm italic text-foreground-muted",
                    {quest_map_loading_label.as_str()}
                }
            }
        };
    }

    rsx! {
        Col { class: "gap-6 w-full", "data-testid": "quest-map",

            // Past chapters (collapsed strip).
            if !past_chapters.is_empty() {
                CollapsedPastStrip {
                    past_chapters: past_chapters.clone(),
                    space_id: space_id_val.clone(),
                }
            }

            // Active chapter.
            if let Some(chapter) = active_chapter {
                ChapterSection {
                    chapter,
                    space_id: space_id_val.clone(),
                    is_active: true,
                }
            }

            // Locked chapters.
            for chapter in locked_chapters.iter() {
                ChapterSection {
                    key: "{chapter.id}",
                    chapter: chapter.clone(),
                    space_id: space_id_val.clone(),
                    is_active: false,
                }
            }
        }
    }
}
