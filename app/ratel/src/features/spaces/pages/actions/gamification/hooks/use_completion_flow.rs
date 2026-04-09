use super::*;

use crate::features::spaces::pages::actions::gamification::types::XpGainResponse;

/// Tracks which phase of the completion animation the overlay is in.
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum CompletionState {
    /// No overlay visible.
    #[default]
    Idle,
    /// The overlay is on-screen; CSS keyframes handle the animation timing.
    Showing,
    /// The user dismissed the overlay (click / tap).
    Done,
}

/// V1 orchestrator: simply tracks whether the completion overlay is
/// visible. The animation timing is handled entirely by CSS keyframes
/// defined in `completion_overlay.css`. No JS timers are needed.
///
/// Returns `(state, trigger, dismiss)`:
/// - `state`   — read to decide what to render
/// - `trigger` — call with an `XpGainResponse` after a successful action submission
/// - `dismiss` — call when the user taps the overlay to close it
///
/// ```rust,ignore
/// let (state, trigger, dismiss) = use_completion_flow();
/// // after server responds with xp_gain:
/// trigger.call(xp_gain);
/// ```
pub fn use_completion_flow(
    response: Signal<Option<XpGainResponse>>,
) -> (Signal<CompletionState>, EventHandler<XpGainResponse>, EventHandler) {
    let mut state = use_signal(|| CompletionState::Idle);
    let mut response_w = response;

    let trigger = EventHandler::new(move |resp: XpGainResponse| {
        response_w.set(Some(resp));
        state.set(CompletionState::Showing);
    });

    let dismiss = EventHandler::new(move |_: ()| {
        state.set(CompletionState::Done);
    });

    (state, trigger, dismiss)
}
