use super::*;

translate! {
    DeleteSpacePopupTranslate;

    title: {
        en: "Delete Space?",
        ko: "스페이스를 삭제하시겠습니까?",
    },
    confirm_pre: {
        en: "This will permanently delete ",
        ko: "다음 스페이스가 영구적으로 삭제됩니다: ",
    },
    confirm_post: {
        en: ", including all actions, comments, and governance data. This action cannot be undone.",
        ko: ". 모든 액션, 댓글, 거버넌스 데이터까지 함께 사라지며 되돌릴 수 없습니다.",
    },
    cancel: {
        en: "Cancel",
        ko: "취소",
    },
    confirm: {
        en: "Delete",
        ko: "삭제",
    },
}

/// Arena-styled confirmation modal for the irreversible "delete space"
/// action. Visual contract mirrors the team-delete arena modal in
/// `features/social/pages/setting/views/mod.rs` (`.ts-modal-*` there →
/// `.sga-modal-*` here, see `style.css`).
///
/// Renders its own overlay and panel — do NOT wrap in `popup.open()`.
/// The parent page owns the `open` state with a local signal and
/// conditionally renders this component so the arena token scope
/// (`.space-general-arena`) stays intact for the CSS variables.
#[component]
pub fn DeleteSpacePopup(
    space_title: String,
    pending: bool,
    on_confirm: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    let tr: DeleteSpacePopupTranslate = use_translate();

    rsx! {
        div {
            class: "sga-modal-overlay",
            onclick: move |_| on_cancel.call(()),
            div {
                class: "sga-modal",
                onclick: move |e: Event<MouseData>| e.stop_propagation(),
                div { class: "sga-modal__header",
                    span { class: "sga-modal__title", "{tr.title}" }
                    button {
                        class: "sga-modal__close",
                        r#type: "button",
                        aria_label: "Close",
                        "data-testid": "delete-space-close",
                        onclick: move |_| on_cancel.call(()),
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            "stroke-width": "2.5",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            line {
                                x1: "18",
                                y1: "6",
                                x2: "6",
                                y2: "18",
                            }
                            line {
                                x1: "6",
                                y1: "6",
                                x2: "18",
                                y2: "18",
                            }
                        }
                    }
                }
                div { class: "sga-modal__body",
                    p { class: "sga-modal__desc",
                        "{tr.confirm_pre}"
                        strong { "{space_title}" }
                        "{tr.confirm_post}"
                    }
                    div { class: "sga-modal__actions",
                        button {
                            r#type: "button",
                            class: "sga-modal__cancel",
                            "data-testid": "delete-space-cancel",
                            onclick: move |_| on_cancel.call(()),
                            "{tr.cancel}"
                        }
                        button {
                            r#type: "button",
                            class: "sga-modal__confirm-danger",
                            "data-testid": "delete-space-confirm",
                            disabled: pending,
                            onclick: move |_| on_confirm.call(()),
                            "{tr.confirm}"
                        }
                    }
                }
            }
        }
    }
}
