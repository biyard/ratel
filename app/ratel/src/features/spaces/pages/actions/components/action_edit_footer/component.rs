use super::*;

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
/// Button clicks update `current_page` signal; the `.pager__track` in
/// each creator page reads the signal via inline `translateX` style to
/// slide the visible card. No JS or scroll-snap involved.
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

    let mut go_prev = move |_| {
        let cur = current_page();
        if cur == 0 {
            return;
        }
        current_page.set(cur - 1);
    };

    let mut go_next = move |_| {
        let cur = current_page();
        if cur + 1 >= total_pages {
            return;
        }
        current_page.set(cur + 1);
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
