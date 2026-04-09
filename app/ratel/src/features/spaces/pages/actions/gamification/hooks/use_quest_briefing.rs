use super::*;

/// Gate hook: tracks whether the QuestBriefing overlay should be shown.
///
/// On first mount `show_briefing` is `true`. Calling the dismiss handler
/// sets it to `false` for the rest of the component's lifetime.
///
/// ```rust,ignore
/// let (show_briefing, dismiss) = use_quest_briefing();
/// if show_briefing {
///     rsx! { QuestBriefing { on_begin: dismiss, on_cancel: dismiss, node } }
/// }
/// ```
pub fn use_quest_briefing() -> (bool, EventHandler) {
    let mut show = use_signal(|| true);
    let dismiss = EventHandler::new(move |_: ()| {
        show.set(false);
    });
    (show(), dismiss)
}
