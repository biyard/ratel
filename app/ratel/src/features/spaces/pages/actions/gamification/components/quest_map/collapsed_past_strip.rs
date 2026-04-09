use crate::common::*;
use crate::features::spaces::pages::actions::gamification::components::quest_map::ChapterSection;
use crate::features::spaces::pages::actions::gamification::i18n::GamificationTranslate;
use crate::features::spaces::pages::actions::gamification::types::ChapterView;

/// Compact summary bar for past (fully-cleared) chapters.
///
/// Renders a single collapsed row showing the count of cleared chapters
/// and the total XP earned across them. Clicking the bar toggles an
/// expanded view that shows each past `ChapterSection` in sequence.
#[component]
pub fn CollapsedPastStrip(
    past_chapters: Vec<ChapterView>,
    space_id: SpacePartition,
) -> Element {
    let tr: GamificationTranslate = use_translate();
    let mut expanded = use_signal(|| false);

    if past_chapters.is_empty() {
        return rsx! {
            div {}
        };
    }

    let chapter_count = past_chapters.len();
    let total_xp: i64 = past_chapters.iter().map(|c| c.total_xp_earned).sum();

    let toggle_label = if expanded() {
        tr.collapse_past_chapters.to_string()
    } else {
        tr.expand_past_chapters.to_string()
    };

    let past_chapters_display = past_chapters.clone();
    let space_id_clone = space_id.clone();

    rsx! {
        Col { class: "gap-3 w-full", "data-testid": "collapsed-past-strip",

            // ── Summary row ──────────────────────────────────────────────────
            div {
                class: "flex flex-row gap-3 justify-between items-center p-3 w-full cursor-pointer rounded-[10px] bg-card-bg",
                onclick: move |_| expanded.set(!expanded()),

                Row { class: "gap-2 items-center",
                    lucide_dioxus::Trophy { class: "w-4 h-4 text-primary" }
                    span { class: "text-sm font-semibold text-text-primary",
                        "{chapter_count} {tr.past_chapters_summary}"
                    }
                    Badge {
                        color: BadgeColor::Green,
                        variant: BadgeVariant::Rounded,
                        "+{total_xp} {tr.xp_suffix}"
                    }
                }

                Row { class: "gap-2 items-center",
                    span { class: "text-xs text-foreground-muted", "{toggle_label}" }
                    if expanded() {
                        lucide_dioxus::ChevronUp { class: "w-4 h-4 text-foreground-muted" }
                    } else {
                        lucide_dioxus::ChevronDown { class: "w-4 h-4 text-foreground-muted" }
                    }
                }
            }

            // ── Expanded past chapters ───────────────────────────────────────
            if expanded() {
                Col { class: "gap-4 w-full",
                    for chapter in past_chapters_display.iter() {
                        ChapterSection {
                            key: "{chapter.id}",
                            chapter: chapter.clone(),
                            space_id: space_id_clone.clone(),
                            is_active: false,
                        }
                    }
                }
            }
        }
    }
}
