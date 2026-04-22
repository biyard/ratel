use crate::common::*;
use crate::features::notifications::controllers::get_unread_count::get_unread_count_handler;
use std::time::Duration;

const POLL_INTERVAL_SECS: u64 = 60;

// Shared wrapper so every caller of `use_unread_count` — `NotificationBell`
// and `NotificationPanel` in particular — reads the same signal. Writing
// 0 from `mark_all_read` in the panel propagates to the bell instantly
// rather than waiting for the next poll tick.
#[derive(Clone, Copy)]
struct UnreadCountSignal(Signal<i64>);

/// Returns a `Signal<i64>` that tracks the current user's unread
/// notification count. Initialized to `0` and refreshed every
/// `POLL_INTERVAL_SECS` seconds by a single spawned async loop.
///
/// The whole initialization runs inside `use_hook`, which guarantees the
/// closure executes exactly once per caller scope and registers a single
/// stable hook slot. Replacing this wrapper with a bare
/// `if try_consume_context { early return } else { use_context_provider(...) }`
/// would add one hook slot on first render and zero on subsequent renders
/// of the same scope, which Dioxus rejects with
/// "rules of hooks" panic — see the bootstrap-panic we hit on 2026-04-22.
pub fn use_unread_count() -> Signal<i64> {
    use_hook(|| {
        // Ancestor scope already installed — reuse their signal.
        if let Some(ctx) = try_consume_context::<UnreadCountSignal>() {
            return ctx.0;
        }

        // Create the signal in the scope that first called this hook and
        // install as scope-local context so descendants can reach it. We
        // deliberately don't use `provide_root_context` here — that would
        // survive logout → login cycles while pointing at a signal whose
        // owning scope has already been dropped.
        let mut count = Signal::new(0i64);
        spawn(async move {
            loop {
                match get_unread_count_handler().await {
                    Ok(resp) => count.set(resp.count),
                    Err(e) => debug!("use_unread_count poll failed: {e}"),
                }
                crate::common::utils::time::sleep(Duration::from_secs(POLL_INTERVAL_SECS)).await;
            }
        });
        provide_context(UnreadCountSignal(count));
        count
    })
}
