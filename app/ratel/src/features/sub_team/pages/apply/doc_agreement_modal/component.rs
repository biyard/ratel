//! Overlay modal shown when a user clicks a required document row on the
//! apply page. Presents the full body + a 동의하기 (agree) / cancel footer.

use crate::features::sub_team::{ApplyContextDocument, SubTeamTranslate};
use crate::*;

#[component]
pub fn DocAgreementModal(
    open: bool,
    doc: ApplyContextDocument,
    already_agreed: bool,
    index: usize,
    total: usize,
    on_cancel: EventHandler<()>,
    on_agree: EventHandler<()>,
) -> Element {
    let tr: SubTeamTranslate = use_translate();
    // Reference-only docs are shown for the applicant to read but
    // never require an agreement — render the modal in read-only
    // mode (no Agree button, just Close).
    let is_required = doc.required;

    rsx! {
        div {
            class: "modal-backdrop sub-team-apply-doc-modal",
            "data-open": "{open}",
            role: "dialog",
            "aria-modal": "true",
            onclick: move |evt| {
                evt.stop_propagation();
                on_cancel.call(());
            },
            div {
                class: "modal doc-modal",
                onclick: move |evt| {
                    evt.stop_propagation();
                },
                div { class: "doc-modal__head",
                    span { class: "doc-modal__icon",
                        lucide_dioxus::FileText { class: "w-4 h-4 [&>path]:stroke-current" }
                    }
                    div { class: "doc-modal__title-wrap",
                        div { class: "doc-modal__kicker", "{tr.doc_modal_eyebrow}" }
                        div { class: "doc-modal__title", "{doc.title}" }
                    }
                    button {
                        r#type: "button",
                        class: "doc-modal__close-x",
                        "aria-label": "{tr.cancel}",
                        onclick: move |_| on_cancel.call(()),
                        lucide_dioxus::X { class: "w-4 h-4 [&>path]:stroke-current" }
                    }
                }
                div { class: "doc-modal__body",
                    // SAFETY: `doc.body` is rich-text produced by the
                    // parent admin via the docs composer (Tiptap →
                    // server-side allowlist sanitize in
                    // `update_sub_team_doc_handler`), so the HTML reaching
                    // here has been stripped of executable script. Render
                    // it as rich content rather than escaped text so
                    // `<b>` / `<div>` formatting actually renders.
                    div {
                        class: "doc-modal__content",
                        dangerous_inner_html: "{doc.body}",
                    }
                    // Surface attachments the parent admin attached when
                    // composing this doc — applicants need to be able to
                    // download the source files before agreeing.
                    if !doc.attachments.is_empty() {
                        div { class: "doc-modal__attachments",
                            for file in doc.attachments.iter() {
                                {
                                    let href = file.url.clone().unwrap_or_default();
                                    rsx! {
                                        a {
                                            class: "doc-modal__attachment",
                                            href: "{href}",
                                            target: "_blank",
                                            rel: "noopener noreferrer",
                                            lucide_dioxus::Paperclip { class: "w-3 h-3 [&>path]:stroke-current" }
                                            span { class: "doc-modal__attachment-name", "{file.name}" }
                                            span { class: "doc-modal__attachment-size", "{file.size}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "doc-modal__notice",
                        lucide_dioxus::Info { class: "w-4 h-4 [&>path]:stroke-current" }
                        div { class: "doc-modal__notice-text", "{tr.doc_modal_notice}" }
                    }
                }
                div { class: "doc-modal__foot",
                    div { class: "doc-modal__foot-left", "{index} / {total}" }
                    div { class: "doc-modal__foot-actions",
                        button {
                            r#type: "button",
                            class: "doc-modal__cancel",
                            onclick: move |_| on_cancel.call(()),
                            if is_required {
                                "{tr.doc_modal_cancel}"
                            } else {
                                "{tr.apply_close}"
                            }
                        }
                        if is_required {
                            button {
                                r#type: "button",
                                class: "doc-modal__agree-btn",
                                "data-agreed": "{already_agreed}",
                                "data-testid": "doc-agreement-agree-btn",
                                disabled: already_agreed,
                                onclick: move |_| on_agree.call(()),
                                lucide_dioxus::Check { class: "w-3 h-3 [&>path]:stroke-current" }
                                if already_agreed {
                                    "{tr.doc_modal_agreed}"
                                } else {
                                    "{tr.doc_modal_agree}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
