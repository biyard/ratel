use crate::common::*;
use crate::features::notifications::controllers::get_unread_count::get_unread_count_handler;
use std::time::Duration;

const POLL_INTERVAL_SECS: u64 = 60;

/// Returns a `Signal<i64>` that tracks the current user's unread notification
/// count. The signal is initialised to `0` and is refreshed every
/// `POLL_INTERVAL_SECS` seconds by a spawned async loop that calls
/// `get_unread_count_handler()`.
pub fn use_unread_count() -> Signal<i64> {
    use_hook(|| {
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

        count
    })
}
