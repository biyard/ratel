use super::*;

translate! {
    DeleteSpacePopupTranslate;

    title: {
        en: "Delete this space?",
        ko: "스페이스를 삭제하시겠습니까?",
    },
    description: {
        en: "Permanently removes the space, all actions, and comments. This cannot be undone.",
        ko: "스페이스, 모든 액션, 댓글이 영구적으로 삭제됩니다. 되돌릴 수 없습니다.",
    },
    cancel: {
        en: "Cancel",
        ko: "취소",
    },
    confirm: {
        en: "Delete space",
        ko: "스페이스 삭제",
    },
}

/// Confirmation popup for the irreversible "delete space" action.
/// Rendered via `popup.open(...)` from the General Settings page's
/// Danger Zone. `on_confirm` fires the mutation and closes the popup;
/// `on_cancel` just closes.
///
/// Intentionally reuses the default PopupZone chrome (title font,
/// neutral card background, built-in X close) to stay consistent with
/// the rest of the app's confirmation dialogs (see
/// `features/spaces/pages/actions/components/delete_action_popup`).
#[component]
pub fn DeleteSpacePopup(
    on_confirm: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    let tr: DeleteSpacePopupTranslate = use_translate();

    rsx! {
        div { class: "flex flex-col w-[480px] max-w-full gap-6 p-6",
            div { class: "flex flex-col gap-2",
                div { class: "text-lg font-bold text-text-primary text-center",
                    "{tr.title}"
                }
                div { class: "text-sm text-text-secondary leading-6 text-center",
                    "{tr.description}"
                }
            }

            div { class: "flex items-center justify-end gap-3",
                button {
                    r#type: "button",
                    class: "h-10 px-4 rounded-lg border border-neutral-300 text-text-primary transition-colors duration-150 hover:bg-neutral-100 disabled:opacity-50 disabled:cursor-not-allowed",
                    "data-testid": "delete-space-cancel",
                    onclick: move |_| on_cancel.call(()),
                    "{tr.cancel}"
                }
                button {
                    r#type: "button",
                    class: "h-10 px-4 rounded-lg bg-red-600 text-white font-semibold transition-opacity duration-150 hover:opacity-90 disabled:opacity-50 disabled:cursor-not-allowed",
                    "data-testid": "delete-space-confirm",
                    onclick: move |_| on_confirm.call(()),
                    "{tr.confirm}"
                }
            }
        }
    }
}
