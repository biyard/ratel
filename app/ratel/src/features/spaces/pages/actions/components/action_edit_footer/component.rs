use super::*;

#[cfg(not(feature = "server"))]
use wasm_bindgen::prelude::*;

#[cfg(not(feature = "server"))]
#[wasm_bindgen(js_namespace = ["window", "ratel", "actionEditor"])]
extern "C" {
    #[wasm_bindgen(js_name = goToPage)]
    fn js_go_to_page(index: usize);
}

fn go_to_page(index: usize) {
    #[cfg(not(feature = "server"))]
    js_go_to_page(index);
    #[cfg(feature = "server")]
    let _ = index;
}

/// Cross-card "flush pending saves now" bus.
///
/// Cards inside the action editor pager (`ContentCard`, `ConfigCard`,
/// `QuestionsCard`, etc.) drive their own debounced autosave. The
/// footer's Save button bumps `flush_tick`, and any card observing it
/// via `use_effect` should fire its pending update immediately rather
/// than wait out the debounce.
#[derive(Clone, Copy)]
pub struct ActionEditSaveBus {
    pub flush_tick: Signal<u64>,
}

impl ActionEditSaveBus {
    pub fn provide() -> Self {
        use_context_provider(|| Self {
            flush_tick: Signal::new(0u64),
        })
    }
}

pub fn try_use_action_edit_save_bus() -> Option<ActionEditSaveBus> {
    try_consume_context()
}

/// Sticky bottom footer for action-editor pages — Previous / counter /
/// Next / Save in one row.
///
/// `current_page` is bidirectional: button clicks call into JS to
/// smooth-scroll the pager AND update the signal; the JS scroll
/// listener in `script.js` calls back via a window-attached closure
/// (`__ratel_aef_set_page`) when the user swipes the pager manually,
/// keeping the disabled state of Previous/Next in sync with the actual
/// scroll position.
#[component]
pub fn ActionEditFooter(
    current_page: Signal<usize>,
    total_pages: usize,
    /// Kebab-case action type — drives the per-action accent color via
    /// `data-action-type` selectors. One of: `"poll"`, `"quiz"`,
    /// `"discussion"`, `"follow"`.
    #[props(default)]
    action_type_key: String,
) -> Element {
    let tr: ActionEditFooterTranslate = use_translate();
    let mut toast = use_toast();
    let bus = try_use_action_edit_save_bus();
    let mut current_page = current_page;

    // Register a window-attached callback so the JS scroll listener in
    // `script.js` can push the active page index back into Rust whenever
    // the user swipes the pager. `Closure::forget` intentionally leaks
    // a single closure for the lifetime of the editor session — only
    // one callback is ever live so the leak is bounded.
    #[cfg(not(feature = "server"))]
    use_hook(move || {
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;
        let closure = Closure::wrap(Box::new(move |idx: f64| {
            let new_page = idx.max(0.0) as usize;
            if new_page != *current_page.peek() {
                current_page.set(new_page);
            }
        }) as Box<dyn FnMut(f64)>);
        if let Some(window) = web_sys::window() {
            let _ = js_sys::Reflect::set(
                &window,
                &JsValue::from_str("__ratel_aef_set_page"),
                closure.as_ref().unchecked_ref(),
            );
        }
        closure.forget();
    });

    let mut go_prev = move |_| {
        let cur = current_page();
        if cur == 0 {
            return;
        }
        let next = cur - 1;
        current_page.set(next);
        go_to_page(next);
    };

    let mut go_next = move |_| {
        let cur = current_page();
        if cur + 1 >= total_pages {
            return;
        }
        let next = cur + 1;
        current_page.set(next);
        go_to_page(next);
    };

    let on_save = move |_| {
        if let Some(b) = bus {
            let mut tick = b.flush_tick;
            tick.with_mut(|t| *t = t.wrapping_add(1));
        }
        toast.info(tr.save_toast.to_string());
    };

    let cur = current_page();
    let prev_disabled = cur == 0;
    let next_disabled = cur + 1 >= total_pages;

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        document::Script { defer: true, src: asset!("./script.js") }

        footer {
            class: "action-edit-footer",
            role: "contentinfo",
            "data-action-type": action_type_key,
            div {
                class: "action-edit-footer__pages",
                "aria-label": "{tr.pages_aria}",
                "{cur + 1} / {total_pages}"
            }
            div { class: "action-edit-footer__buttons",
                button {
                    class: "aef-btn aef-btn--ghost",
                    "data-testid": "footer-prev-btn",
                    disabled: prev_disabled,
                    onclick: move |e| go_prev(e),
                    "{tr.previous}"
                }
                button {
                    class: "aef-btn aef-btn--secondary",
                    "data-testid": "footer-next-btn",
                    disabled: next_disabled,
                    onclick: move |e| go_next(e),
                    "{tr.next}"
                }
                button {
                    class: "aef-btn aef-btn--primary",
                    "data-testid": "footer-save-btn",
                    onclick: on_save,
                    "{tr.save}"
                }
            }
        }
    }
}
