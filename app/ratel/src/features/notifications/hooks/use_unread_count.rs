use crate::common::*;
use crate::features::notifications::controllers::get_unread_count::get_unread_count_handler;
use dioxus::core::provide_root_context;
use std::time::Duration;

const POLL_INTERVAL_SECS: u64 = 60;

// A root-scoped signal so every caller of `use_unread_count` — `NotificationBell`
// and `NotificationPanel` in particular — shares the same unread count. Writing
// 0 from `mark_all_read` in the panel then propagates to the bell instantly
// instead of waiting for the next poll tick.
#[derive(Clone, Copy)]
struct UnreadCountSignal(Signal<i64>);

/// Returns a `Signal<i64>` that tracks the current user's unread notification
/// count. The signal is initialised to `0`, shared across the component tree
/// via a root context, and refreshed every `POLL_INTERVAL_SECS` seconds by a
/// single spawned async loop.
pub fn use_unread_count() -> Signal<i64> {
    use_hook(|| {
        if let Some(ctx) = try_consume_context::<UnreadCountSignal>() {
            return ctx.0;
        }

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

        provide_root_context(UnreadCountSignal(count));
        count
    })
}
