use super::*;

use crate::features::spaces::pages::actions::gamification::controllers::get_quest_map;
use crate::features::spaces::pages::actions::gamification::types::QuestMapResponse;

use dioxus::fullstack::{Loader, Loading};

/// Client-side data hook for the Quest Map component.
///
/// Wraps `get_quest_map` in a `use_loader` so the Quest Map component
/// can consume the response without any additional boilerplate. The hook
/// propagates via `?` to bubble the `Loading` suspension up to the
/// nearest Suspense boundary.
///
/// ```rust,ignore
/// fn MyComponent(space_id: ReadSignal<SpacePartition>) -> Element {
///     let loader = use_quest_map(space_id)?;
///     let data: &QuestMapResponse = &loader()?;
///     // ...
/// }
/// ```
#[allow(clippy::result_large_err)]
pub fn use_quest_map(
    space_id: ReadSignal<SpacePartition>,
) -> dioxus::prelude::Result<Loader<QuestMapResponse>, Loading> {
    use_loader(move || async move { get_quest_map(space_id()).await })
}
