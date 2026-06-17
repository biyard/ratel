use crate::common::*;

#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;

#[cfg(target_arch = "wasm32")]
struct ScrollLockSnapshot {
    scroll_y: f64,
    previous_style: Option<String>,
}

#[cfg(target_arch = "wasm32")]
#[derive(Default)]
struct ScrollLockState {
    count: u32,
    snapshot: Option<ScrollLockSnapshot>,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static SCROLL_LOCK_STATE: RefCell<ScrollLockState> = RefCell::new(ScrollLockState::default());
}

#[cfg(target_arch = "wasm32")]
fn lock_page_scroll() -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    let Some(document) = window.document() else {
        return false;
    };
    let Some(body) = document.body() else {
        return false;
    };

    let scroll_y = window.scroll_y().ok().unwrap_or(0.0);
    let previous_style = body.get_attribute("style");

    let mut should_apply_lock = false;
    SCROLL_LOCK_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.count = state.count.saturating_add(1);
        if state.count == 1 {
            state.snapshot = Some(ScrollLockSnapshot {
                scroll_y,
                previous_style,
            });
            should_apply_lock = true;
        }
    });

    if should_apply_lock {
        // `position:fixed` resolves against the initial containing block (the
        // full viewport), bypassing `:root`'s safe-area padding. With the iOS
        // WKWebView running edge-to-edge (full-height viewport), a bare `top:0`
        // pushes the page under the status bar/notch (topbar jumps up). Pin the
        // top to the safe-area inset and size to the padded area so the locked
        // body occupies the same box as the unlocked one. env(...) is 0 on
        // web/Android, so this stays a no-op there.
        //
        // Adding `var(--vv-offset-top)` to `top` counters iOS's keyboard-
        // avoidance shift: when a modal input is focused, WKWebView moves the
        // visual viewport up (offsetTop > 0) and drags this fixed body with it,
        // so the topbar jumps under the status bar. The app.rs visualViewport
        // handler publishes that offset as `--vv-offset-top`; pushing the body
        // down by the same amount keeps it (topbar) visually pinned. The var is
        // unset/0 on web/Android.
        //
        // NOTE: this MUST be done via `top`, never `transform`. A transform on
        // the body establishes a containing block for its `position:fixed`
        // descendants (CSS spec), which re-anchors the modal popup to the body
        // box instead of the viewport — shoving the whole modal down. `top`
        // does not create a containing block, so the modal keeps its
        // viewport-relative centering and rides iOS's own offset (input stays
        // above the keyboard) while only the body/topbar is compensated.
        let _ = body.set_attribute(
            "style",
            &format!(
                "position:fixed;top:calc(env(safe-area-inset-top) - {scroll_y}px + var(--vv-offset-top, 0px));left:env(safe-area-inset-left);right:env(safe-area-inset-right);width:auto;height:calc(100dvh - env(safe-area-inset-top) - env(safe-area-inset-bottom));overflow:hidden;overscroll-behavior:none;"
            ),
        );
    }

    true
}

#[cfg(target_arch = "wasm32")]
fn unlock_page_scroll() {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Some(body) = document.body() else {
        return;
    };

    let snapshot = SCROLL_LOCK_STATE.with(|state| {
        let mut state = state.borrow_mut();
        if state.count == 0 {
            return None;
        }

        state.count -= 1;
        if state.count == 0 {
            state.snapshot.take()
        } else {
            None
        }
    });

    let Some(snapshot) = snapshot else {
        return;
    };

    if let Some(previous_style) = snapshot.previous_style {
        let _ = body.set_attribute("style", &previous_style);
    } else {
        let _ = body.remove_attribute("style");
    }
    window.scroll_to_with_x_and_y(0.0, snapshot.scroll_y);
}

pub fn use_scroll_lock(lock: bool) {
    #[cfg(target_arch = "wasm32")]
    {
        let mut is_locked = use_signal(|| false);
        let is_locked_for_drop = is_locked;

        use_effect(use_reactive((&lock,), move |(lock,)| {
            if lock && !is_locked() {
                if lock_page_scroll() {
                    is_locked.set(true);
                }
            } else if !lock && is_locked() {
                unlock_page_scroll();
                is_locked.set(false);
            }
        }));

        use_drop(move || {
            if is_locked_for_drop() {
                unlock_page_scroll();
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = lock;
    }
}
