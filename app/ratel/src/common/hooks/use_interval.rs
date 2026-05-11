use crate::common::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Repeatedly runs `callback` every `interval_ms` milliseconds while the
/// component is mounted. On web, skips a tick while the tab is hidden
/// (`document.hidden`) so polling pauses for backgrounded tabs and resumes
/// automatically when the tab returns. The underlying task is tied to the
/// current scope via `use_future`, so it is cancelled when the component
/// unmounts.
///
/// **Server / mobile targets are a no-op** — server has no client-side
/// refresh loop, and mobile (Android/iOS) lacks the `gloo_timers`
/// dependency (web-only in `Cargo.toml`). The hook signature stays the
/// same so callers don't need cfg gating; it just doesn't tick on those
/// targets.
pub fn use_interval<F>(interval_ms: u32, callback: F)
where
    F: FnMut() + 'static,
{
    let cb: Rc<RefCell<F>> = Rc::new(RefCell::new(callback));

    use_future(move || {
        let _cb = cb.clone();
        let _interval_ms = interval_ms;
        async move {
            #[cfg(feature = "web")]
            loop {
                gloo_timers::future::TimeoutFuture::new(_interval_ms).await;
                if !is_tab_hidden() {
                    (_cb.borrow_mut())();
                }
            }
        }
    });
}

#[cfg(feature = "web")]
fn is_tab_hidden() -> bool {
    web_sys::window()
        .and_then(|w| w.document())
        .map(|d| d.hidden())
        .unwrap_or(false)
}
