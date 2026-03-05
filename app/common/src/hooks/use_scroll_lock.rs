use crate::*;

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
        let _ = body.set_attribute(
            "style",
            &format!(
                "position:fixed;top:-{scroll_y}px;left:0;right:0;width:100%;overflow:hidden;overscroll-behavior:none;"
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
