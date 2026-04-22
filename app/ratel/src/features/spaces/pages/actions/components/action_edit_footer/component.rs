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
///
/// Provided by each creator page at the top via
/// [`ActionEditSaveBus::provide`]; consumed by individual cards via
/// [`try_use_action_edit_save_bus`]. Optional so cards without
/// debounced state can ignore it.
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

/// Sticky footer for action-editor pages — Previous / Next / Save.
///
/// Layout-wise, the editor renders a horizontal scroll-snap `.pager`
/// holding 2-3 setting "pages" per action type. Previously the only
/// way to switch pages was to swipe horizontally and the only save
/// signal was a per-card autosave indicator, which left users without
/// a clear "I'm done" affordance. This footer makes both explicit.
///
/// The parent owns `current_page` (Signal<usize>) and supplies
/// `total_pages`. Previous/Next mutate `current_page` and call into JS
/// to scroll the pager to the target page. Save bumps the
/// [`ActionEditSaveBus`] for cards to flush.
#[component]
pub fn ActionEditFooter(
    current_page: Signal<usize>,
    total_pages: usize,
) -> Element {
    let tr: ActionEditFooterTranslate = use_translate();
    let mut toast = use_toast();
    let bus = try_use_action_edit_save_bus();
    let mut current_page = current_page;

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

        footer { class: "action-edit-footer", role: "contentinfo",
            div {
                class: "action-edit-footer__pages",
                "aria-label": "{tr.pages_aria}",
                "{cur + 1} / {total_pages}"
            }
            div { class: "action-edit-footer__buttons",
                button {
                    class: "btn btn--ghost",
                    "data-testid": "footer-prev-btn",
                    disabled: prev_disabled,
                    onclick: move |e| go_prev(e),
                    "{tr.previous}"
                }
                button {
                    class: "btn btn--secondary",
                    "data-testid": "footer-next-btn",
                    disabled: next_disabled,
                    onclick: move |e| go_next(e),
                    "{tr.next}"
                }
                button {
                    class: "btn btn--primary",
                    "data-testid": "footer-save-btn",
                    onclick: on_save,
                    "{tr.save}"
                }
            }
        }
    }
}
