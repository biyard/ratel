use super::*;

translate! {
    DeleteActionPopupTranslate;

    title: {
        en: "Delete action?",
        ko: "삭제하시겠습니까?",
    },
    description: {
        en: "Deleted actions cannot be restored.",
        ko: "삭제된 액션은 복구할 수 없습니다.",
    },
    cancel: {
        en: "Cancel",
        ko: "취소",
    },
    confirm: {
        en: "Confirm",
        ko: "확인",
    },
}

#[component]
pub fn DeleteActionPopup(
    on_confirm: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    let tr: DeleteActionPopupTranslate = use_translate();

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
                    onclick: move |_| on_cancel.call(()),
                    "{tr.cancel}"
                }
                button {
                    r#type: "button",
                    class: "h-10 px-4 rounded-lg bg-red-600 text-white font-semibold transition-opacity duration-150 hover:opacity-90 disabled:opacity-50 disabled:cursor-not-allowed",
                    onclick: move |_| on_confirm.call(()),
                    "{tr.confirm}"
                }
            }
        }
    }
}
