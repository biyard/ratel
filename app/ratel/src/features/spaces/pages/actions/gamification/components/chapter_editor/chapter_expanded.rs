use crate::common::*;
use crate::features::spaces::pages::actions::gamification::components::chapter_editor::{
    extract_chapter_id, DagEditorCanvas,
};
use crate::features::spaces::pages::actions::gamification::controllers::chapters::*;
use crate::features::spaces::pages::actions::gamification::i18n::GamificationTranslate;
use crate::features::spaces::pages::actions::gamification::models::SpaceChapter;
use crate::features::spaces::pages::actions::gamification::types::ChapterBenefit;
use crate::features::spaces::pages::actions::models::SpaceAction;

/// Expanded inline-edit view for a single chapter.
///
/// Provides editable fields for name, actor role, completion benefit,
/// and a DAG editor canvas for the chapter's actions.
#[component]
pub fn ChapterExpanded(
    chapter: SpaceChapter,
    space_id: ReadSignal<SpacePartition>,
    on_collapse: EventHandler,
    on_saved: EventHandler,
) -> Element {
    let tr: GamificationTranslate = use_translate();
    let chapter_id = extract_chapter_id(&chapter.sk);

    let mut name_draft = use_signal(|| chapter.name.clone());
    let mut actor_role_draft = use_signal(|| chapter.actor_role);
    let mut benefit_draft = use_signal(|| chapter.completion_benefit.clone());

    let order_label = format!("{} {}", tr.ch_pill, chapter.order + 1);

    // Save handler: sends update to server (use_callback is Copy, safe in multiple closures)
    let chapter_id_signal = use_signal(|| chapter_id.clone());
    let save_chapter = use_callback(move |_: ()| {
        let sid = space_id();
        let cid = chapter_id_signal();
        let name = name_draft();
        let role = actor_role_draft();
        let benefit = benefit_draft();
        spawn(async move {
            let req = UpdateChapterRequest {
                name: Some(name),
                description: None,
                actor_role: Some(role),
                completion_benefit: Some(benefit),
            };
            match update_chapter(sid, cid, req).await {
                Ok(_) => {
                    on_saved.call(());
                }
                Err(e) => {
                    tracing::error!("Failed to update chapter: {e}");
                }
            }
        });
    });

    // Actor role display values
    let role_options = vec![
        (SpaceUserRole::Candidate, "Candidate"),
        (SpaceUserRole::Participant, "Participant"),
    ];

    // Benefit display values
    let benefit_options = vec![
        ("xp_only", tr.benefit_xp_only.to_string()),
        ("role_upgrade", tr.benefit_role_upgrade.to_string()),
        ("role_and_xp", tr.benefit_role_and_xp.to_string()),
    ];

    let current_benefit_key = match benefit_draft() {
        ChapterBenefit::XpOnly => "xp_only",
        ChapterBenefit::RoleUpgradeTo(_) => "role_upgrade",
        ChapterBenefit::RoleUpgradeAndXp(_) => "role_and_xp",
    };

    rsx! {
        Card {
            variant: CardVariant::GlassAccent,
            direction: CardDirection::Col,
            class: "gap-4 w-full",
            "data-testid": "chapter-expanded",

            // Header row: pill + collapse button
            Row {
                main_axis_align: MainAxisAlign::Between,
                cross_axis_align: CrossAxisAlign::Center,
                class: "w-full",

                Badge { color: BadgeColor::Blue, variant: BadgeVariant::Rounded, "{order_label}" }

                Button {
                    size: ButtonSize::Small,
                    style: ButtonStyle::Text,
                    shape: ButtonShape::Rounded,
                    onclick: move |_| on_collapse.call(()),
                    Row {
                        cross_axis_align: CrossAxisAlign::Center,
                        class: "gap-1",
                        lucide_dioxus::ChevronsUpDown { class: "w-4 h-4" }
                        span { "{tr.collapse}" }
                    }
                }
            }

            // Name field
            Col { class: "gap-1 w-full",
                Label { html_for: "chapter-name", "{tr.chapter_name}" }
                Input {
                    r#type: InputType::Text,
                    value: name_draft(),
                    placeholder: tr.chapter_name.to_string(),
                    oninput: move |e: FormEvent| {
                        name_draft.set(e.value());
                    },
                    onfocusout: move |_| {
                        save_chapter(());
                    },
                }
            }

            // Actor role + Benefit selectors row
            Row { class: "gap-4 w-full max-mobile:flex-col",
                // Actor role
                Col { class: "flex-1 gap-1",
                    Label { html_for: "actor-role", "{tr.actor_role_label}" }
                    div { class: "flex gap-2",
                        for (role , label) in role_options.iter() {
                            {
                                let role = *role;
                                let is_selected = actor_role_draft() == role;
                                rsx! {
                                    Button {
                                        size: ButtonSize::Small,
                                        style: if is_selected { ButtonStyle::Primary } else { ButtonStyle::Outline },
                                        shape: ButtonShape::Rounded,
                                        onclick: move |_| {
                                            actor_role_draft.set(role);
                                            save_chapter(());
                                        },
                                        "{label}"
                                    }
                                }
                            }
                        }
                    }
                }

                // Completion benefit
                Col { class: "flex-1 gap-1",
                    Label { html_for: "benefit", "{tr.benefit_label}" }
                    div { class: "flex flex-wrap gap-2",
                        for (key , label) in benefit_options.iter() {
                            {
                                let key = *key;
                                let is_selected = current_benefit_key == key;
                                rsx! {
                                    Button {
                                        size: ButtonSize::Small,
                                        style: if is_selected { ButtonStyle::Primary } else { ButtonStyle::Outline },
                                        shape: ButtonShape::Rounded,
                                        onclick: move |_| {
                                            let new_benefit = match key {
                                                "xp_only" => ChapterBenefit::XpOnly,
                                                "role_upgrade" => ChapterBenefit::RoleUpgradeTo(SpaceUserRole::Participant),
                                                "role_and_xp" => ChapterBenefit::RoleUpgradeAndXp(SpaceUserRole::Participant),
                                                _ => ChapterBenefit::XpOnly,
                                            };
                                            benefit_draft.set(new_benefit);
                                            save_chapter(());
                                        },
                                        "{label}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // DAG Editor placeholder — actions for this chapter
            // In V1 this renders a simplified list view of actions.
            // Full DAG editor wiring happens when list_actions_by_chapter
            // is available.
            Separator {}

            DagEditorCanvas {
                chapter_id: chapter_id.clone(),
                space_id,
                on_dependency_change: move |_dep: (String, Vec<String>)| {
                    // TODO: wire to update_space_action dependency endpoint
                    tracing::info!("Dependency changed — save pending");
                },
            }
        }
    }
}
