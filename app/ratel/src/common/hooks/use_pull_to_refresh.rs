//! Reusable mobile pull-to-refresh hook.
//!
//! Call it from any page that has its OWN vertical scroll container, passing
//! the container's CSS selector and a refresh closure that re-runs the page's
//! loaders. The closure must be `Copy` (closures that capture only `Copy`
//! `Loader`/`InfiniteQuery` handles satisfy this) so it can be moved into the
//! background recv task.
//!
//! ```ignore
//! use_pull_to_refresh(".home-arena__scroll", move || {
//!     hot_spaces.restart();
//!     my_spaces.restart();
//! });
//! ```
//!
//! Compiled only into the Tauri (`tauri-web`, non-`fullstack`) build; on the
//! web/SSR build it's a no-op so desktop browsers never get pull-to-refresh.

#[cfg(all(feature = "tauri-web", not(feature = "fullstack")))]
pub fn use_pull_to_refresh(selector: &'static str, on_refresh: impl FnMut() + Copy + 'static) {
    use crate::*;

    // `use_future` runs the task AFTER the render commits and ties it to the
    // component's lifecycle. The previous implementation spawned the task and
    // ran `document::eval` from inside `use_hook` — i.e. DURING render — which
    // re-enters the reactive runtime and panics with "closure invoked
    // recursively or after being dropped" (a wasm trap in release/panic=abort).
    use_future(move || async move {
        let js = include_str!("pull_to_refresh.js").replace("__PTR_SCROLL_SEL__", selector);
        let mut runner = document::eval(&js);
        let mut on_refresh = on_refresh;
        while runner.recv::<bool>().await.is_ok() {
            on_refresh();
            let _ = document::eval("window.__ratelPtrDone && window.__ratelPtrDone();");
        }
    });
}

#[cfg(not(all(feature = "tauri-web", not(feature = "fullstack"))))]
pub fn use_pull_to_refresh(_selector: &'static str, _on_refresh: impl FnMut() + Copy + 'static) {}

// ── Layout-level pull-to-refresh ──────────────────────────────────────────
//
// Some pages share a single, persistent scroll container owned by a layout
// (e.g. `SocialLayout`'s outlet wrapper). Per-page `use_pull_to_refresh` can't
// be used there — the container element survives child route changes, so only
// the first page would bind. Instead the LAYOUT installs the gesture once and
// the active child page registers its refresh callback through this context.
//
// Layout:  let r = use_provide_page_refresh();
//          use_pull_to_refresh(".social-scroll", move || r.run());
// Page:    use_register_refresh(move || { feed.refresh(); });
use crate::*;

#[derive(Clone, Copy)]
pub struct PageRefresh {
    cb: Signal<Option<Callback<()>>>,
}

impl PageRefresh {
    /// Invoke the currently-registered page refresh (no-op if none).
    pub fn run(&self) {
        if let Some(cb) = self.cb.peek().as_ref() {
            cb.call(());
        }
    }
}

/// Installed by a layout that owns a shared scroll container. Child pages
/// register their refresh via [`use_register_refresh`].
pub fn use_provide_page_refresh() -> PageRefresh {
    use_context_provider(|| PageRefresh {
        cb: Signal::new(None),
    })
}

/// Register the current page's refresh closure into the ancestor
/// [`PageRefresh`] context so the layout's pull-to-refresh can trigger it.
/// No-op (apart from the context write) when no layout provided the context.
pub fn use_register_refresh(on_refresh: impl FnMut() + Copy + 'static) {
    // Call `use_effect` UNCONDITIONALLY (the context lookup goes inside the
    // effect, not around it) so the hook order is identical on every render —
    // a conditional hook is what corrupts Dioxus's positional hook list.
    let reg = try_use_context::<PageRefresh>();
    use_effect(move || {
        if let Some(reg) = reg {
            let mut cb = reg.cb;
            cb.set(Some(Callback::new(move |_| {
                let mut f = on_refresh;
                f();
            })));
        }
    });
}
