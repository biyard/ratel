use crate::common::{PopoverContent, PopoverRoot, PopoverTrigger};
use crate::features::spaces::pages::actions::controllers::{
    list_actions, update_space_action, UpdateSpaceActionRequest,
};
use crate::features::spaces::pages::actions::*;

#[component]
pub fn ActionDependencySelector(
    space_id: ReadSignal<SpacePartition>,
    action_id: ReadSignal<String>,
    initial_depends_on: Vec<String>,
    #[props(default)] on_changed: EventHandler<Vec<String>>,
) -> Element {
    let tr: ActionDependencySelectorTranslate = use_translate();
    let mut toast = use_toast();
    let mut depends_on = use_signal(|| initial_depends_on.clone());
    let mut menu_open = use_signal(|| false);
    let mut saving = use_signal(|| false);

    let actions_loader = use_loader(move || list_actions(space_id()))?;
    let actions = actions_loader();

    let current_action_id = action_id();
    let selected = depends_on();

    let blocked_by_cycle: std::collections::HashSet<String> = {
        use std::collections::HashSet;
        let mut blocked: HashSet<String> = HashSet::new();
        let mut frontier: Vec<String> = vec![current_action_id.clone()];
        while let Some(id) = frontier.pop() {
            for a in actions.iter() {
                if a.depends_on.contains(&id) && blocked.insert(a.action_id.clone()) {
                    frontier.push(a.action_id.clone());
                }
            }
        }
        blocked
    };

    let available: Vec<SpaceActionSummary> = actions
        .iter()
        .filter(|a| {
            a.action_id != current_action_id
                && !selected.contains(&a.action_id)
                && !matches!(a.action_type, SpaceActionType::Follow)
                && !blocked_by_cycle.contains(&a.action_id)
        })
        .cloned()
        .collect();

    let save = use_callback(move |next: Vec<String>| {
        if saving() {
            return;
        }
        saving.set(true);
        spawn(async move {
            let req = UpdateSpaceActionRequest::Dependencies {
                depends_on: next.clone(),
            };
            match update_space_action(space_id(), action_id(), req).await {
                Ok(_) => {
                    depends_on.set(next.clone());
                    on_changed.call(next);
                }
                Err(e) => {
                    toast.error(e);
                }
            }
            saving.set(false);
        });
    });

    let selected_view: Vec<SpaceActionSummary> = actions
        .iter()
        .filter(|a| selected.contains(&a.action_id))
        .cloned()
        .collect();

    let has_available = !available.is_empty();
    let has_selected = !selected_view.is_empty();

    rsx! {
        div { class: "flex flex-col gap-2 w-full",
            span { class: "font-medium text-[12px]/[16px] text-foreground-muted", "{tr.depends_on_hint}" }

            if !has_available && !has_selected {
                span { class: "font-medium italic text-[12px]/[16px] text-foreground-muted",
                    "{tr.no_actions_available}"
                }
            } else {
                div { class: "flex flex-wrap gap-2",
                    for dep in selected_view.iter() {
                        button {
                            key: "{dep.action_id}",
                            r#type: "button",
                            class: "inline-flex gap-1.5 items-center py-1 px-2.5 font-medium rounded-full bg-primary/10 text-[12px]/[16px] text-primary",
                            onclick: {
                                let dep_id = dep.action_id.clone();
                                let selected_list = selected.clone();
                                move |_| {
                                    let next: Vec<String> = selected_list
                                        .iter()
                                        .filter(|id| *id != &dep_id)
                                        .cloned()
                                        .collect();
                                    save.call(next);
                                }
                            },
                            span { "{dep.title}" }
                            span { "×" }
                        }
                    }

                    if has_available {
                        PopoverRoot {
                            open: menu_open(),
                            on_open_change: move |open| menu_open.set(open),
                            PopoverTrigger { class: "inline-flex gap-1 items-center py-1 px-2.5 font-medium rounded-full border border-separator text-[12px]/[16px] text-foreground-muted hover:bg-hover",
                                "{tr.add_dependency}"
                            }
                            PopoverContent {
                                div { class: "flex flex-col gap-1 p-2 min-w-[220px] border rounded-[10px] border-separator bg-popover shadow-lg",
                                    for action in available.iter() {
                                        button {
                                            key: "{action.action_id}",
                                            r#type: "button",
                                            class: "py-2 px-3 text-left rounded-[8px] text-[13px]/[18px] text-text-primary hover:bg-hover",
                                            onclick: {
                                                let aid = action.action_id.clone();
                                                let selected_list = selected.clone();
                                                move |_| {
                                                    let mut next = selected_list.clone();
                                                    next.push(aid.clone());
                                                    save.call(next);
                                                    menu_open.set(false);
                                                }
                                            },
                                            "{action.title}"
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

translate! {
    ActionDependencySelectorTranslate;

    depends_on_hint: {
        en: "Participants must complete the selected actions before this one unlocks.",
        ko: "선택한 액션을 모두 완료해야 이 액션을 참여할 수 있습니다."
    },
    add_dependency: { en: "+ Add dependency", ko: "+ 선행 액션 추가" },
    no_actions_available: {
        en: "No other actions exist in this space yet.",
        ko: "이 스페이스에 추가할 수 있는 다른 액션이 없습니다."
    },
}
