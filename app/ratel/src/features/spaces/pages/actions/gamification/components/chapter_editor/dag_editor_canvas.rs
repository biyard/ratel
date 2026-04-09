use crate::common::*;
use crate::features::spaces::pages::actions::controllers::list_actions;
use crate::features::spaces::pages::actions::gamification::i18n::GamificationTranslate;
use crate::features::spaces::pages::actions::types::SpaceActionSummary;

/// Click-to-connect DAG editor for actions within a chapter.
///
/// V1 renders a simple vertical list of action nodes with dependency
/// chips. Click a node to select it as "source", then click another
/// to create a dependency (target depends on source). Existing edges
/// are shown as removable chips.
#[component]
pub fn DagEditorCanvas(
    chapter_id: String,
    space_id: ReadSignal<SpacePartition>,
    on_dependency_change: EventHandler<(String, Vec<String>)>,
) -> Element {
    let tr: GamificationTranslate = use_translate();

    // Load all actions for the space, then filter by chapter_id.
    let actions_resource =
        use_loader(move || async move { list_actions(space_id()).await })?;

    let all_actions = actions_resource();

    // Filter actions belonging to this chapter
    let chapter_actions: Vec<SpaceActionSummary> = all_actions
        .iter()
        .filter(|a| {
            a.chapter_id
                .as_ref()
                .map(|cid| cid == &chapter_id)
                .unwrap_or(false)
        })
        .cloned()
        .collect();

    let mut selected_source: Signal<Option<String>> = use_signal(|| None);

    if chapter_actions.is_empty() {
        return rsx! {
            div {
                class: "py-4 px-3 text-sm italic text-center text-foreground-muted",
                "data-testid": "dag-editor",
                "No actions assigned to this chapter yet."
            }
        };
    }

    rsx! {
        Col { class: "gap-3 w-full", "data-testid": "dag-editor",

            // Instruction banner
            if selected_source().is_some() {
                div { class: "py-2 px-3 text-xs text-center rounded-lg text-primary bg-primary/10",
                    "{tr.click_to_connect}"
                }
            }

            // Action nodes list
            for action in chapter_actions.iter() {
                {
                    let action_id = action.action_id.clone();
                    let action_title = action.title.clone();
                    let depends_on = action.depends_on.clone();
                    let is_source = selected_source()
                        .as_ref()
                        .map(|s| s == &action_id)
                        .unwrap_or(false);

                    // Build a name map for dependency labels
                    let name_map: Vec<(String, String)> = chapter_actions
                        .iter()
                        .map(|a| (a.action_id.clone(), a.title.clone()))
                        .collect();

                    rsx! {
                        div {
                            key: "{action_id}",
                            class: "flex flex-col gap-2 p-3 rounded-lg border transition-colors duration-150 cursor-pointer border-border aria-selected:border-primary aria-selected:bg-primary/5",
                            "aria-selected": is_source,
                            onclick: {
                                let action_id = action_id.clone();
                                let depends_on = depends_on.clone();
                                move |_| {
                                    if let Some(source_id) = selected_source() {
                                        if source_id != action_id {
                                            // Create dependency: action_id now depends on source_id
                                            let mut new_deps = depends_on.clone();
                                            if !new_deps.contains(&source_id) {
                                                new_deps.push(source_id);
                                            }

                                            // Node header

                                            // Dependency chips

                                            on_dependency_change.call((action_id.clone(), new_deps));
                                        }
                                        selected_source.set(None);
                                    } else {
                                        selected_source.set(Some(action_id.clone()));
                                    }
                                }
                            },

                            Row {
                                main_axis_align: MainAxisAlign::Between,
                                cross_axis_align: CrossAxisAlign::Center,
                                class: "w-full",

                                span { class: "text-sm font-semibold text-text-primary", "{action_title}" }

                                if is_source {
                                    Badge { color: BadgeColor::Blue, variant: BadgeVariant::Rounded, "Source" }
                                }
                            }

                            if !depends_on.is_empty() {
                                Row { class: "flex-wrap gap-1.5 items-center",

                                    span { class: "text-xs text-foreground-muted", "{tr.depends_on_label}:" }

                                    for dep_id in depends_on.iter() {
                                        {
                                            let dep_name = name_map
                                                .iter()
                                                .find(|(id, _)| id == dep_id)
                                                .map(|(_, name)| name.as_str())
                                                .unwrap_or(dep_id.as_str());

                                            let dep_id_remove = dep_id.clone();
                                            let action_id_remove = action_id.clone();
                                            let current_deps = depends_on.clone();

                                            rsx! {
                                                div { class: "flex gap-1 items-center py-0.5 px-2 text-xs rounded-full bg-card-bg text-text-primary",
                                                    span { "{dep_name}" }
                                                    button {
                                                        class: "cursor-pointer text-foreground-muted hover:text-destructive",
                                                        onclick: move |e: MouseEvent| {
                                                            e.stop_propagation();
                                                            let new_deps: Vec<String> = current_deps
                                                                .iter()
                                                                .filter(|d| **d != dep_id_remove)
                                                                .cloned()
                                                                .collect();
                                                            on_dependency_change.call((action_id_remove.clone(), new_deps));
                                                        },
                                                        "x"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
