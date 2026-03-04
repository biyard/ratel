use crate::*;

#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;

#[cfg(target_arch = "wasm32")]
struct ScrollLockSnapshot {
    scroll_y: f64,
    position: Option<String>,
    top: Option<String>,
    left: Option<String>,
    right: Option<String>,
    width: Option<String>,
    overflow: Option<String>,
    overscroll_behavior: Option<String>,
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
fn read_style_property(style: &web_sys::CssStyleDeclaration, property: &str) -> Option<String> {
    style
        .get_property_value(property)
        .ok()
        .and_then(|value| {
            let value = value.trim();
            if value.is_empty() {
                None
            } else {
                Some(value.to_string())
            }
        })
}

#[cfg(target_arch = "wasm32")]
fn restore_style_property(
    style: &web_sys::CssStyleDeclaration,
    property: &str,
    value: &Option<String>,
) {
    if let Some(value) = value {
        let _ = style.set_property(property, value);
    } else {
        let _ = style.remove_property(property);
    }
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

    let body_style = body.style();
    let scroll_y = window.scroll_y().ok().unwrap_or(0.0);

    let mut should_apply_lock = false;
    SCROLL_LOCK_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.count = state.count.saturating_add(1);
        if state.count == 1 {
            state.snapshot = Some(ScrollLockSnapshot {
                scroll_y,
                position: read_style_property(&body_style, "position"),
                top: read_style_property(&body_style, "top"),
                left: read_style_property(&body_style, "left"),
                right: read_style_property(&body_style, "right"),
                width: read_style_property(&body_style, "width"),
                overflow: read_style_property(&body_style, "overflow"),
                overscroll_behavior: read_style_property(&body_style, "overscroll-behavior"),
            });
            should_apply_lock = true;
        }
    });

    if should_apply_lock {
        let _ = body_style.set_property("position", "fixed");
        let _ = body_style.set_property("top", &format!("-{scroll_y}px"));
        let _ = body_style.set_property("left", "0");
        let _ = body_style.set_property("right", "0");
        let _ = body_style.set_property("width", "100%");
        let _ = body_style.set_property("overflow", "hidden");
        let _ = body_style.set_property("overscroll-behavior", "none");
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

    let body_style = body.style();

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

    restore_style_property(&body_style, "position", &snapshot.position);
    restore_style_property(&body_style, "top", &snapshot.top);
    restore_style_property(&body_style, "left", &snapshot.left);
    restore_style_property(&body_style, "right", &snapshot.right);
    restore_style_property(&body_style, "width", &snapshot.width);
    restore_style_property(&body_style, "overflow", &snapshot.overflow);
    restore_style_property(
        &body_style,
        "overscroll-behavior",
        &snapshot.overscroll_behavior,
    );
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
