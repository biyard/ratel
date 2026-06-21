use crate::common::types::FeedPartition;
use crate::common::*;
use crate::features::posts::components::ai_draft::i18n::AiDraftTranslate;
use crate::features::posts::controllers::generate_ai_draft::{
    generate_ai_draft_handler, GenerateAiDraftRequest, GenerateAiDraftResponse,
};
use crate::features::posts::types::{AiDraftLanguage, AiDraftTemplate};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AiDraftStep {
    Picker,
    Form,
    Loading,
    Error,
}

#[component]
pub fn AiDraftModal(
    post_id: FeedPartition,
    initial_language: AiDraftLanguage,
    on_close: EventHandler<()>,
    on_success: EventHandler<GenerateAiDraftResponse>,
) -> Element {
    let tr: AiDraftTranslate = use_translate();

    let mut step = use_signal(|| AiDraftStep::Picker);
    let topic = use_signal(String::new);
    let background = use_signal(String::new);
    let feedback_request = use_signal(String::new);
    let participation_notes = use_signal(String::new);
    let language = use_signal(|| initial_language);
    let mut error_msg = use_signal(|| Option::<String>::None);

    // Per-attempt nonce. When the user clicks Cancel we bump this; the
    // spawned task captures the value at launch time and bails out if it
    // doesn't match by the time the request resolves — so an in-flight AI
    // call can no longer overwrite the editor after the modal was closed.
    // The server-side allowance may still flip (one-shot is keyed on
    // success, not on the client honouring the result), but the editor
    // stays in the user's chosen state.
    let mut attempt_nonce = use_signal(|| 0u64);

    let required_filled = use_memo(move || {
        !topic.read().trim().is_empty()
            && !background.read().trim().is_empty()
            && !feedback_request.read().trim().is_empty()
    });

    let post_id_value = post_id;

    let do_generate = use_callback(move |_: ()| {
        // Bump the nonce so that *if* an earlier attempt's request is still
        // in flight, its eventual result is treated as cancelled.
        let this_nonce = {
            let n = attempt_nonce.peek().wrapping_add(1);
            n
        };
        attempt_nonce.set(this_nonce);
        step.set(AiDraftStep::Loading);
        error_msg.set(None);
        let topic_val = topic.read().clone();
        let background_val = background.read().clone();
        let feedback_val = feedback_request.read().clone();
        let notes_val = {
            let n = participation_notes.read().clone();
            if n.trim().is_empty() { None } else { Some(n) }
        };
        let lang_val = *language.read();
        let pid = post_id_value.clone();
        spawn(async move {
            let req = GenerateAiDraftRequest {
                template: AiDraftTemplate::OpinionGathering,
                topic: topic_val,
                background: background_val,
                feedback_request: feedback_val,
                participation_notes: notes_val,
                language: lang_val,
            };
            let result = generate_ai_draft_handler(pid, req).await;
            // If the user cancelled (or fired a retry) while we were in
            // flight, the nonce has moved on. Drop the result on the
            // floor and don't touch on_success / error_msg / step.
            if *attempt_nonce.read() != this_nonce {
                return;
            }
            match result {
                Ok(resp) => {
                    on_success.call(resp);
                }
                Err(e) => {
                    error_msg.set(Some(format!("{e}")));
                    step.set(AiDraftStep::Error);
                }
            }
        });
    });

    // Cancel: bumping the nonce makes any in-flight spawn discard its
    // result before touching the editor or the modal state. The caller
    // is expected to also close the modal after invoking this.
    let cancel_in_flight = use_callback(move |_: ()| {
        let next = attempt_nonce.peek().wrapping_add(1);
        attempt_nonce.set(next);
    });

    let current_step = *step.read();
    let picker_active = matches!(current_step, AiDraftStep::Picker);
    let form_active = matches!(current_step, AiDraftStep::Form);
    let loading_active = matches!(current_step, AiDraftStep::Loading);
    let error_active = matches!(current_step, AiDraftStep::Error);

    rsx! {
        div { class: "ai-scrim",
            section {
                class: "ai-modal",
                role: "dialog",
                aria_modal: "true",
                "aria-labelledby": "ai-modal-title",
                "data-testid": "ai-draft-modal",

                header { class: "ai-modal__head",
                    div { class: "ai-modal__head-icon",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M12 3l1.6 4.4L18 9l-4.4 1.6L12 15l-1.6-4.4L6 9l4.4-1.6L12 3z" }
                            path { d: "M19 14l.8 2.2L22 17l-2.2.8L19 20l-.8-2.2L16 17l2.2-.8L19 14z" }
                        }
                    }
                    div { class: "ai-modal__head-text",
                        div { class: "ai-modal__eyebrow", "{tr.modal_eyebrow}" }
                        h2 { class: "ai-modal__title", id: "ai-modal-title", "{tr.modal_title}" }
                    }
                    button {
                        class: "ai-modal__close",
                        r#type: "button",
                        aria_label: "Close",
                        onclick: move |_| on_close.call(()),
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2.5",
                            stroke_linecap: "round",
                            path { d: "M6 6l12 12M18 6L6 18" }
                        }
                    }
                }

                // Stepper
                div { class: "ai-stepper", aria_label: "Progress",
                    div {
                        class: "ai-step",
                        "data-active": if picker_active { "true" } else { "false" },
                        "data-done": if !picker_active { "true" } else { "false" },
                        div { class: "ai-step__dot", "1" }
                        div { class: "ai-step__label", "{tr.step_picker}" }
                    }
                    div { class: "ai-step__bar" }
                    div {
                        class: "ai-step",
                        "data-active": if form_active { "true" } else { "false" },
                        "data-done": if loading_active || error_active { "true" } else { "false" },
                        div { class: "ai-step__dot", "2" }
                        div { class: "ai-step__label", "{tr.step_form}" }
                    }
                    div { class: "ai-step__bar" }
                    div {
                        class: "ai-step",
                        "data-active": if loading_active || error_active { "true" } else { "false" },
                        "data-done": "false",
                        div { class: "ai-step__dot", "3" }
                        div { class: "ai-step__label", "{tr.step_generate}" }
                    }
                }

                // Body — pick one pane
                div { class: "ai-modal__body",
                    if picker_active {
                        PickerPane { tr: tr.clone() }
                    } else if form_active {
                        FormPane {
                            tr: tr.clone(),
                            topic,
                            background,
                            feedback_request,
                            participation_notes,
                            language,
                        }
                    } else if loading_active {
                        LoadingPane { tr: tr.clone() }
                    } else {
                        ErrorPane { tr: tr.clone(), error_msg }
                    }
                }

                // Footer — buttons depend on step
                footer { class: "ai-modal__foot",
                    if picker_active {
                        div { class: "ai-modal__foot-info",
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                circle { cx: "12", cy: "12", r: "9" }
                                path { d: "M12 8v4M12 16h.01" }
                            }
                            span { "{tr.foot_picker_info}" }
                        }
                        div { class: "ai-modal__foot-actions",
                            button {
                                class: "ai-btn-base ai-btn-base--ghost",
                                r#type: "button",
                                onclick: move |_| on_close.call(()),
                                "{tr.btn_cancel}"
                            }
                            button {
                                class: "ai-btn-base ai-btn-base--primary",
                                r#type: "button",
                                "data-testid": "ai-modal-next",
                                onclick: move |_| step.set(AiDraftStep::Form),
                                span { "{tr.btn_next}" }
                                svg {
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2.5",
                                    stroke_linecap: "round",
                                    path { d: "M5 12h14M13 6l6 6-6 6" }
                                }
                            }
                        }
                    } else if form_active {
                        div { class: "ai-modal__foot-info", "{tr.foot_form_info}" }
                        div { class: "ai-modal__foot-actions",
                            button {
                                class: "ai-btn-base",
                                r#type: "button",
                                onclick: move |_| step.set(AiDraftStep::Picker),
                                svg {
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2.5",
                                    stroke_linecap: "round",
                                    path { d: "M19 12H5M11 18l-6-6 6-6" }
                                }
                                span { "{tr.btn_back}" }
                            }
                            button {
                                class: "ai-btn-base ai-btn-base--primary",
                                r#type: "button",
                                "data-testid": "ai-modal-generate",
                                disabled: !*required_filled.read(),
                                onclick: move |_| do_generate.call(()),
                                svg {
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    path { d: "M12 3l1.6 4.4L18 9l-4.4 1.6L12 15l-1.6-4.4L6 9l4.4-1.6L12 3z" }
                                }
                                span { "{tr.btn_generate}" }
                            }
                        }
                    } else if loading_active {
                        div { class: "ai-modal__foot-info", "{tr.foot_loading_info}" }
                        div { class: "ai-modal__foot-actions",
                            button {
                                class: "ai-btn-base",
                                r#type: "button",
                                onclick: move |_| {
                                    // Discard the in-flight task's result before
                                    // closing so a late success can't overwrite
                                    // the editor after Cancel.
                                    cancel_in_flight.call(());
                                    on_close.call(());
                                },
                                "{tr.btn_cancel}"
                            }
                        }
                    } else {
                        div { class: "ai-modal__foot-info", "{tr.foot_error_info}" }
                        div { class: "ai-modal__foot-actions",
                            button {
                                class: "ai-btn-base ai-btn-base--ghost",
                                r#type: "button",
                                onclick: move |_| on_close.call(()),
                                "{tr.btn_close}"
                            }
                            button {
                                class: "ai-btn-base ai-btn-base--primary",
                                r#type: "button",
                                "data-testid": "ai-modal-retry",
                                onclick: move |_| do_generate.call(()),
                                svg {
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2.5",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    path { d: "M3 12a9 9 0 0 1 15-6.7L21 8M21 3v5h-5M21 12a9 9 0 0 1-15 6.7L3 16M3 21v-5h5" }
                                }
                                span { "{tr.btn_retry}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PickerPane(tr: AiDraftTranslate) -> Element {
    rsx! {
        div {
            div { class: "ai-pane-intro",
                div { class: "ai-pane-intro__title", "{tr.picker_title}" }
                div { class: "ai-pane-intro__sub", "{tr.picker_sub}" }
            }
            div { class: "ai-template-grid", role: "radiogroup",
                button {
                    class: "ai-template-card",
                    "aria-selected": "true",
                    role: "radio",
                    "data-testid": "ai-template-opinion",
                    div { class: "ai-template-card__icon",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M3 14l4-4 4 4 4-7 6 8" }
                            circle { cx: "7", cy: "10", r: "1.2" }
                        }
                    }
                    div { class: "ai-template-card__text",
                        div { class: "ai-template-card__title-row",
                            div { class: "ai-template-card__title", "{tr.template_opinion_title}" }
                            span { class: "ai-template-card__pill", "{tr.template_pill_available}" }
                        }
                        div { class: "ai-template-card__desc", "{tr.template_opinion_desc}" }
                    }
                    div { class: "ai-template-card__check",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "3",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M5 12l5 5 9-11" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FormPane(
    tr: AiDraftTranslate,
    topic: Signal<String>,
    background: Signal<String>,
    feedback_request: Signal<String>,
    participation_notes: Signal<String>,
    language: Signal<AiDraftLanguage>,
) -> Element {
    rsx! {
        div {
            div { class: "ai-pane-intro",
                div { class: "ai-pane-intro__title", "{tr.form_title}" }
                div { class: "ai-pane-intro__sub", "{tr.form_sub}" }
            }
            form { class: "ai-form", onsubmit: move |e| e.prevent_default(),
                div { class: "ai-form-field",
                    label { class: "ai-form-field__label", r#for: "ai-topic",
                        span { "{tr.field_topic_label}" }
                        span { class: "ai-req", "{tr.required_mark}" }
                    }
                    div { class: "ai-form-field__hint", "{tr.field_topic_hint}" }
                    input {
                        id: "ai-topic",
                        r#type: "text",
                        value: "{topic}",
                        placeholder: "{tr.field_topic_placeholder}",
                        "data-testid": "ai-form-topic",
                        oninput: move |e| topic.set(e.value()),
                    }
                }
                div { class: "ai-form-field",
                    label { class: "ai-form-field__label", r#for: "ai-background",
                        span { "{tr.field_background_label}" }
                        span { class: "ai-req", "{tr.required_mark}" }
                    }
                    div { class: "ai-form-field__hint", "{tr.field_background_hint}" }
                    textarea {
                        id: "ai-background",
                        rows: "3",
                        value: "{background}",
                        placeholder: "{tr.field_background_placeholder}",
                        "data-testid": "ai-form-background",
                        oninput: move |e| background.set(e.value()),
                    }
                }
                div { class: "ai-form-field",
                    label { class: "ai-form-field__label", r#for: "ai-feedback",
                        span { "{tr.field_feedback_label}" }
                        span { class: "ai-req", "{tr.required_mark}" }
                    }
                    div { class: "ai-form-field__hint", "{tr.field_feedback_hint}" }
                    textarea {
                        id: "ai-feedback",
                        rows: "3",
                        value: "{feedback_request}",
                        placeholder: "{tr.field_feedback_placeholder}",
                        "data-testid": "ai-form-feedback",
                        oninput: move |e| feedback_request.set(e.value()),
                    }
                }
                div { class: "ai-form-field",
                    label { class: "ai-form-field__label", r#for: "ai-notes",
                        span { "{tr.field_notes_label}" }
                    }
                    div { class: "ai-form-field__hint", "{tr.field_notes_hint}" }
                    textarea {
                        id: "ai-notes",
                        rows: "2",
                        value: "{participation_notes}",
                        placeholder: "{tr.field_notes_placeholder}",
                        "data-testid": "ai-form-notes",
                        oninput: move |e| participation_notes.set(e.value()),
                    }
                }
                div { class: "ai-form-field",
                    div { class: "ai-lang-select",
                        label { class: "ai-form-field__label", r#for: "ai-lang",
                            "{tr.field_language_label}"
                        }
                        select {
                            id: "ai-lang",
                            value: match *language.read() {
                                AiDraftLanguage::Ko => "ko",
                                AiDraftLanguage::En => "en",
                            },
                            "data-testid": "ai-form-language",
                            oninput: move |e| {
                                let next = if e.value() == "en" {
                                    AiDraftLanguage::En
                                } else {
                                    AiDraftLanguage::Ko
                                };
                                language.set(next);
                            },
                            option { value: "ko", "{tr.field_language_ko}" }
                            option { value: "en", "{tr.field_language_en}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LoadingPane(tr: AiDraftTranslate) -> Element {
    rsx! {
        div { class: "ai-pane-loading",
            div { class: "ai-gen-orb",
                div { class: "ai-gen-orb__inner",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M12 3l1.6 4.4L18 9l-4.4 1.6L12 15l-1.6-4.4L6 9l4.4-1.6L12 3z" }
                    }
                }
            }
            div { class: "ai-gen-status",
                div { class: "ai-gen-status__title", "{tr.loading_title}" }
                div { class: "ai-gen-status__sub", "{tr.loading_sub}" }
            }
        }
    }
}

#[component]
fn ErrorPane(tr: AiDraftTranslate, error_msg: Signal<Option<String>>) -> Element {
    rsx! {
        div {
            div { class: "ai-err-card",
                div { class: "ai-err-card__icon",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M12 9v4M12 17h.01" }
                        path { d: "M10.3 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" }
                    }
                }
                div { class: "ai-err-card__text",
                    div { class: "ai-err-card__title", "{tr.error_title}" }
                    div { class: "ai-err-card__msg",
                        {error_msg.read().clone().unwrap_or_else(|| tr.error_default_msg.to_string())}
                    }
                }
            }
            div { class: "ai-err-summary", "{tr.error_summary}" }
        }
    }
}
