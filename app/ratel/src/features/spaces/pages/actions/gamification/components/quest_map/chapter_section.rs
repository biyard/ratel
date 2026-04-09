use crate::common::*;
use crate::features::spaces::pages::actions::gamification::components::quest_map::{DagCanvas, QuestNode};
use crate::features::spaces::pages::actions::gamification::i18n::GamificationTranslate;
use crate::features::spaces::pages::actions::gamification::types::{ChapterView, QuestNodeStatus};

/// Renders a single chapter on the Quest Map.
///
/// A chapter has three display modes:
/// 1. **Active** — visible header + node grid + DAG canvas. The user can
///    interact with `Active` nodes inside.
/// 2. **Passed** — collapsed header only (all nodes are `Cleared`). The
///    user can expand it to revisit the nodes.
/// 3. **Locked** — header with a lock badge + hint text. No nodes shown
///    until the previous chapter is complete.
#[component]
pub fn ChapterSection(
    chapter: ChapterView,
    space_id: SpacePartition,
    #[props(default = false)] is_active: bool,
) -> Element {
    let tr: GamificationTranslate = use_translate();
    let mut expanded = use_signal(|| is_active);

    let cleared_count = chapter
        .nodes
        .iter()
        .filter(|n| n.status == QuestNodeStatus::Cleared)
        .count();
    let total = chapter.nodes.len();

    // Determine the chapter state.
    let is_locked = chapter.nodes.is_empty()
        || chapter
            .nodes
            .iter()
            .all(|n| n.status == QuestNodeStatus::Locked || n.status == QuestNodeStatus::RoleGated)
            && !chapter.is_complete;

    // Header badge label + color.
    let (badge_label, badge_color) = if chapter.is_complete {
        (tr.chapter_passed_badge.to_string(), BadgeColor::Green)
    } else if is_locked {
        (tr.chapter_locked_badge.to_string(), BadgeColor::Grey)
    } else {
        (tr.chapter_active_badge.to_string(), BadgeColor::Blue)
    };

    let chapter_nodes = chapter.nodes.clone();
    let chapter_space_id = space_id.clone();
    let unlock_hint = tr.chapter_unlock_hint.to_string();

    rsx! {
        Col { class: "gap-2 w-full", "data-testid": "chapter-section",

            // ── Header ────────────────────────────────────────────────────────
            div {
                class: "flex flex-row gap-3 justify-between items-center p-3 w-full cursor-pointer rounded-[10px] bg-card-bg",
                onclick: move |_| {
                    if !is_locked {
                        expanded.set(!expanded());
                    }
                },

                Row { class: "gap-2 items-center",
                    Badge { color: badge_color, variant: BadgeVariant::Rounded, "{badge_label}" }
                    span { class: "text-sm font-semibold text-text-primary", "{chapter.name}" }
                }

                Row { class: "gap-3 items-center",
                    span { class: "text-xs text-foreground-muted", "{cleared_count}/{total}" }
                    if !is_locked {
                        if expanded() {
                            lucide_dioxus::ChevronUp { class: "w-4 h-4 text-foreground-muted" }
                        } else {
                            lucide_dioxus::ChevronDown { class: "w-4 h-4 text-foreground-muted" }
                        }
                    } else {
                        lucide_dioxus::Lock { class: "w-4 h-4 text-foreground-muted" }
                    }
                }
            }

            // ── Body ──────────────────────────────────────────────────────────
            if is_locked {
                // Locked chapter hint.
                div { class: "py-3 px-4 border border-dashed rounded-[10px] border-border",
                    span { class: "text-xs italic text-foreground-muted", {unlock_hint.as_str()} }
                }
            } else if expanded() {
                Col { class: "gap-4 w-full",
                    // DAG connector SVG.
                    if chapter_nodes.iter().any(|n| !n.depends_on.is_empty()) {
                        DagCanvas { nodes: chapter_nodes.clone() }
                    }

                    // Node grid: 1 column on mobile, 2 on tablet+.
                    div { class: "grid grid-cols-1 gap-3 w-full tablet:grid-cols-2",
                        for node in chapter_nodes.iter() {
                            QuestNode {
                                key: "{node.id}",
                                node: node.clone(),
                                space_id: chapter_space_id.clone(),
                            }
                        }
                    }
                }
            }
        }
    }
}
