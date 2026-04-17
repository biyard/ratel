use crate::features::spaces::space_common::*;

translate! {
    SpaceEndModalTranslate;

    warning: {
        en: "Are you sure you want to end this space?",
        ko: "정말로 이 스페이스를 종료하시겠습니까?",
    },
    cancel: {
        en: "Cancel",
        ko: "취소",
    },
    end: {
        en: "End Space",
        ko: "종료하기",
    },
    ending: {
        en: "Ending...",
        ko: "종료 중...",
    },
}

#[component]
pub fn SpaceEndModal(
    space_id: SpacePartition,
    on_success: EventHandler<()>,
    #[props(default)] on_close: Option<EventHandler<()>>,
) -> Element {
    let tr: SpaceEndModalTranslate = use_translate();
    let mut popup = use_popup();
    let mut toast = use_toast();
    let mut is_ending = use_signal(|| false);

    let on_cancel = move |_| {
        if let Some(handler) = on_close.as_ref() {
            handler.call(());
        }
        popup.close();
    };

    let on_end = move |_| {
        if is_ending() {
            return;
        }

        is_ending.set(true);

        let mut is_ending = is_ending;
        let mut popup = popup;
        let mut toast = toast;
        let space_id = space_id.clone();

        spawn(async move {
            let res = update_space(
                space_id,
                controllers::UpdateSpaceRequest::Finish { finished: true },
            )
            .await;

            match res {
                Ok(_) => {
                    on_success.call(());
                    popup.close();
                }
                Err(err) => {
                    toast.error(err);
                }
            }

            is_ending.set(false);
        });
    };

    rsx! {
        div { class: "flex flex-col mt-6 w-[420px] max-w-full",
            div { class: "mb-6 text-base font-medium text-center text-text-secondary",
                "{tr.warning}"
            }

            div { class: "flex flex-row gap-4 justify-end mt-4",
                button {
                    onclick: on_cancel,
                    class: "px-10 py-[14.5px] min-w-[120px] text-base font-bold rounded-[10px] transition-colors bg-btn-outline-bg border border-btn-outline-outline text-btn-outline-text hover:bg-btn-outline-hover-bg hover:border-btn-outline-hover-outline hover:text-btn-outline-hover-text",
                    "{tr.cancel}"
                }
                button {
                    "data-testid": "end-space-button",
                    onclick: on_end,
                    disabled: is_ending(),
                    class: if !is_ending() { "w-full py-[14.5px] font-bold text-base rounded-[10px] bg-primary text-black hover:bg-primary/80 transition-colors" } else { "w-full py-[14.5px] font-bold text-base rounded-[10px] bg-neutral-700 light:bg-neutral-300 text-neutral-500 cursor-not-allowed transition-colors" },
                    if is_ending() {
                        "{tr.ending}"
                    } else {
                        "{tr.end}"
                    }
                }
            }
        }
    }
}
