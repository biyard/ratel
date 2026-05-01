use crate::common::*;
use crate::features::cross_posting::i18n::BlueskyConnectModalTranslate;

/// Bluesky app-password connect modal. Controlled by the parent's
/// `Signal<bool>` via the `data-open` attribute — no JS open/close
/// handlers, the CSS transitions handle the visual reveal.
///
/// On submit, calls back to the parent with `(handle, app_password)`;
/// the parent owns the `UseCrossPosting::handle_connect_bluesky` action
/// and decides whether to close the modal optimistically.
#[component]
pub fn BlueskyConnectModal(
    open: Signal<bool>,
    on_submit: EventHandler<(String, String)>,
) -> Element {
    let mut handle = use_signal(String::new);
    let mut app_password = use_signal(String::new);
    let t: BlueskyConnectModalTranslate = use_translate();

    let mut close = move || {
        open.set(false);
        handle.set(String::new());
        app_password.set(String::new());
    };

    let submit_disabled = handle().trim().is_empty() || app_password().trim().is_empty();

    rsx! {
        div {
            class: "bsky-modal-backdrop",
            "data-open": "{open()}",
            onclick: move |_| close(),

            div {
                class: "bsky-modal",
                role: "dialog",
                "aria-labelledby": "bsky-modal-title",
                // Stop click-through so clicking inside the modal does
                // not close it (only the backdrop click does).
                onclick: move |e| e.stop_propagation(),

                header { class: "bsky-modal__head",
                    span { class: "bsky-modal__logo",
                        svg { "viewBox": "0 0 24 24", "fill": "currentColor",
                            path { "d": "M12 10.5c-1.3-2.5-4.9-7.2-8.2-9.5C.7-1.2 0 .5 0 1.5c0 1.1.6 9.1 1 10.3.8 4 5.5 5.1 9.6 4.4-6.5 1.1-12.3 3.5-4.7 11.4 2.5 2.6 3.5-2.6 4-5.2.5 2.6 1.6 7.8 4 5.2 7.7-7.9 1.9-10.4-4.6-11.5 4 .7 8.8-.4 9.6-4.4.4-1.2 1-9.2 1-10.3 0-1-.7-2.7-3.8-.5-3.3 2.3-6.9 7-8.2 9.6z" }
                        }
                    }
                    div { class: "bsky-modal__head-body",
                        div {
                            id: "bsky-modal-title",
                            class: "bsky-modal__head-title",
                            "{t.title}"
                        }
                        div { class: "bsky-modal__head-sub", "{t.subtitle}" }
                    }
                    button {
                        class: "bsky-modal__close",
                        "aria-label": "{t.close}",
                        onclick: move |_| close(),
                        svg {
                            "viewBox": "0 0 24 24",
                            "fill": "none",
                            "stroke": "currentColor",
                            "stroke-width": "2",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            line {
                                "x1": "18",
                                "y1": "6",
                                "x2": "6",
                                "y2": "18",
                            }
                            line {
                                "x1": "6",
                                "y1": "6",
                                "x2": "18",
                                "y2": "18",
                            }
                        }
                    }
                }

                div { class: "bsky-modal__body",
                    div { class: "bsky-modal__info",
                        svg {
                            "viewBox": "0 0 24 24",
                            "fill": "none",
                            "stroke": "currentColor",
                            "stroke-width": "2",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            circle { "cx": "12", "cy": "12", "r": "10" }
                            line {
                                "x1": "12",
                                "y1": "16",
                                "x2": "12",
                                "y2": "12",
                            }
                            line {
                                "x1": "12",
                                "y1": "8",
                                "x2": "12.01",
                                "y2": "8",
                            }
                        }
                        div { "{t.info}" }
                    }

                    div { class: "bsky-field",
                        label { class: "bsky-field__label", "{t.label_handle}" }
                        input {
                            class: "bsky-field__input",
                            r#type: "text",
                            placeholder: "{t.placeholder_handle}",
                            value: "{handle}",
                            oninput: move |e| handle.set(e.value()),
                        }
                        span { class: "bsky-field__hint", "{t.hint_handle}" }
                    }

                    div { class: "bsky-field",
                        label { class: "bsky-field__label", "{t.label_app_password}" }
                        input {
                            class: "bsky-field__input",
                            r#type: "password",
                            placeholder: "{t.placeholder_app_password}",
                            value: "{app_password}",
                            oninput: move |e| app_password.set(e.value()),
                        }
                        span { class: "bsky-field__hint", "{t.hint_app_password}" }
                    }
                }

                footer { class: "bsky-modal__foot",
                    span { class: "bsky-modal__foot-hint", "{t.foot_hint}" }
                    div { class: "bsky-modal__actions",
                        button {
                            class: "bsky-modal__btn bsky-modal__btn--cancel",
                            onclick: move |_| close(),
                            "{t.btn_cancel}"
                        }
                        button {
                            class: "bsky-modal__btn bsky-modal__btn--primary",
                            disabled: submit_disabled,
                            onclick: move |_| {
                                let h = handle().trim().to_string();
                                let p = app_password().trim().to_string();
                                if !h.is_empty() && !p.is_empty() {
                                    on_submit.call((h, p));
                                }
                            },
                            "{t.btn_connect}"
                        }
                    }
                }
            }
        }
    }
}
