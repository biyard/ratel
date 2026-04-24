use crate::features::spaces::pages::actions::types::SpaceActionSummary;
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub struct DependencyLock {
    pub locked: bool,
}

impl DependencyLock {
    pub fn none() -> Self {
        Self { locked: false }
    }
}

pub fn build_action_lookup(actions: &[SpaceActionSummary]) -> HashMap<String, (String, bool)> {
    actions
        .iter()
        .map(|a| (a.action_id.clone(), (a.title.clone(), a.user_participated)))
        .collect()
}

pub fn resolve_outstanding_actions(
    action: &SpaceActionSummary,
    all_actions: &[SpaceActionSummary],
) -> Vec<SpaceActionSummary> {
    if action.depends_on.is_empty() || action.dependencies_met {
        return Vec::new();
    }
    action
        .depends_on
        .iter()
        .filter_map(|dep_id| {
            all_actions
                .iter()
                .find(|a| &a.action_id == dep_id && !a.user_participated)
                .cloned()
        })
        .collect()
}

pub fn resolve_dependency_lock(
    action: &SpaceActionSummary,
    _lookup: &HashMap<String, (String, bool)>,
) -> DependencyLock {
    if action.depends_on.is_empty() || action.dependencies_met {
        return DependencyLock::none();
    }
    DependencyLock { locked: true }
}

pub fn open_locked_popup(
    popup: &mut crate::common::PopupService,
    space_id: crate::common::types::SpacePartition,
    outstanding: Vec<SpaceActionSummary>,
) {
    use crate::features::spaces::pages::index::action_dashboard::LockedDependenciesPopup;
    let mut popup_ref = *popup;
    popup.open(rsx! {
        LockedDependenciesPopup {
            space_id,
            outstanding,
            on_close: move |_| popup_ref.close(),
        }
    });
}
