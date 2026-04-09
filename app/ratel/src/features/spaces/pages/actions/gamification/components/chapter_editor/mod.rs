use crate::common::*;
use crate::features::spaces::pages::actions::gamification::controllers::chapters::*;
use crate::features::spaces::pages::actions::gamification::i18n::GamificationTranslate;
use crate::features::spaces::pages::actions::gamification::models::SpaceChapter;
use crate::features::spaces::pages::actions::gamification::types::ChapterBenefit;

mod add_chapter_slot;
mod chapter_expanded;
mod chapter_row;
mod dag_editor_canvas;

pub use add_chapter_slot::*;
pub use chapter_expanded::*;
pub use chapter_row::*;
pub use dag_editor_canvas::*;

/// Top-level Chapter Editor component for the creator view.
///
/// Loads and displays all chapters for a space, allowing the creator
/// to expand/collapse, edit, reorder, add, and delete chapters.
#[component]
pub fn ChapterEditor(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: GamificationTranslate = use_translate();
    let mut chapters_resource =
        use_loader(move || async move { list_chapters(space_id()).await })?;

    let chapters = chapters_resource();

    let mut expanded_chapter_id: Signal<Option<String>> = use_signal(|| None);

    let handle_add = move |_| {
        let sid = space_id();
        spawn(async move {
            let req = CreateChapterRequest {
                name: "New Chapter".to_string(),
                actor_role: Some(SpaceUserRole::Participant),
                completion_benefit: Some(ChapterBenefit::XpOnly),
            };
            match create_chapter(sid, req).await {
                Ok(new_id) => {
                    expanded_chapter_id.set(Some(new_id));
                    chapters_resource.restart();
                }
                Err(e) => {
                    tracing::error!("Failed to create chapter: {e}");
                }
            }
        });
    };

    rsx! {
        Col { class: "gap-4 w-full", "data-testid": "chapter-editor",

            for chapter in chapters.iter() {
                {
                    let chapter_id = extract_chapter_id(&chapter.sk);
                    let is_expanded = expanded_chapter_id()
                        .as_ref()
                        .map(|id| id == &chapter_id)
                        .unwrap_or(false);

                    if is_expanded {
                        rsx! {
                            ChapterExpanded {
                                key: "{chapter_id}",
                                chapter: chapter.clone(),
                                space_id,
                                on_collapse: move |_| {
                                    expanded_chapter_id.set(None);
                                },
                                on_saved: move |_| {
                                    chapters_resource.restart();
                                },
                            }
                        }
                    } else {
                        let cid = chapter_id.clone();
                        let cid_del = chapter_id.clone();
                        rsx! {
                            ChapterRow {
                                key: "{chapter_id}",
                                chapter: chapter.clone(),
                                on_expand: move |_| {
                                    expanded_chapter_id.set(Some(cid.clone()));
                                },
                                on_delete: {
                                    let sid = space_id();
                                    move |_| {
                                        let sid = sid.clone();
                                        let cid = cid_del.clone();
                                        spawn(async move {
                                            match delete_chapter(sid, cid).await {
                                                Ok(_) => {
                                                    chapters_resource.restart();
                                                }
                                                Err(e) => {
                                                    tracing::error!("Failed to delete chapter: {e}");
                                                }
                                            }
                                        });
                                    }
                                },
                            }
                        }
                    }
                }
            }

            AddChapterSlot { on_add: handle_add }
        }
    }
}

/// Extract the chapter ID string from the EntityType sort key.
fn extract_chapter_id(sk: &EntityType) -> String {
    let sk_str = sk.to_string();
    // EntityType::SpaceChapter serializes as "SPACE_CHAPTER#{id}"
    if let Some(id) = sk_str.strip_prefix("SPACE_CHAPTER#") {
        id.to_string()
    } else {
        sk_str
    }
}
