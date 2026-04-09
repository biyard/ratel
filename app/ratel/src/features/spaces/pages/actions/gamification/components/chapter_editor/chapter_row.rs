use crate::common::*;
use crate::features::spaces::pages::actions::gamification::components::chapter_editor::extract_chapter_id;
use crate::features::spaces::pages::actions::gamification::i18n::GamificationTranslate;
use crate::features::spaces::pages::actions::gamification::models::SpaceChapter;
use crate::features::spaces::pages::actions::gamification::types::ChapterBenefit;

/// Collapsed chapter row showing a summary of the chapter.
///
/// Displays: drag handle, chapter order pill, name, actor role badge,
/// completion benefit summary, edit button, and delete button.
#[component]
pub fn ChapterRow(
    chapter: SpaceChapter,
    on_expand: EventHandler,
    on_delete: EventHandler,
) -> Element {
    let tr: GamificationTranslate = use_translate();

    let order_label = format!("{} {}", tr.ch_pill, chapter.order + 1);

    let role_label = match chapter.actor_role {
        SpaceUserRole::Participant => "Participant",
        SpaceUserRole::Candidate => "Candidate",
        SpaceUserRole::Viewer => "Viewer",
        SpaceUserRole::Creator => "Creator",
    };

    let role_badge_color = match chapter.actor_role {
        SpaceUserRole::Participant => BadgeColor::Green,
        SpaceUserRole::Candidate => BadgeColor::Orange,
        _ => BadgeColor::Grey,
    };

    let benefit_label = match &chapter.completion_benefit {
        ChapterBenefit::XpOnly => tr.benefit_xp_only.to_string(),
        ChapterBenefit::RoleUpgradeTo(_) => tr.benefit_role_upgrade.to_string(),
        ChapterBenefit::RoleUpgradeAndXp(_) => tr.benefit_role_and_xp.to_string(),
    };

    rsx! {
        Card {
            variant: CardVariant::Glass,
            direction: CardDirection::Row,
            main_axis_align: MainAxisAlign::Between,
            cross_axis_align: CrossAxisAlign::Center,
            class: "gap-3 w-full",
            "data-testid": "chapter-row",

            // Left side: drag handle + pill + name + badges
            Row {
                cross_axis_align: CrossAxisAlign::Center,
                class: "flex-1 gap-3 min-w-0",

                // Drag handle
                div { class: "flex-shrink-0 cursor-grab text-foreground-muted",
                    lucide_dioxus::GripVertical { class: "w-5 h-5" }
                }

                // Chapter order pill
                Badge { color: BadgeColor::Blue, variant: BadgeVariant::Rounded, "{order_label}" }

                // Chapter name
                span { class: "text-sm font-semibold truncate text-text-primary", "{chapter.name}" }

                // Actor role badge
                Badge { color: role_badge_color, variant: BadgeVariant::Rounded, "{role_label}" }

                // Benefit summary
                span { class: "text-xs text-foreground-muted", "{benefit_label}" }
            }

            // Right side: edit + delete buttons
            Row {
                cross_axis_align: CrossAxisAlign::Center,
                class: "flex-shrink-0 gap-2",

                Button {
                    size: ButtonSize::Small,
                    style: ButtonStyle::Outline,
                    shape: ButtonShape::Rounded,
                    onclick: move |_| on_expand.call(()),
                    lucide_dioxus::Pencil { class: "w-4 h-4" }
                }

                Button {
                    size: ButtonSize::Small,
                    style: ButtonStyle::Outline,
                    shape: ButtonShape::Rounded,
                    onclick: move |_| on_delete.call(()),
                    "{tr.delete_chapter}"
                }
            }
        }
    }
}
