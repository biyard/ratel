use crate::features::spaces::space_common::*;
use crate::features::spaces::space_common::providers::use_space_context;

translate! {
    SpaceStartModalTranslate;

    warning: {
        en: "Once started, participants can begin their activities.",
        ko: "스페이스를 시작하면 참여자들이 활동을 시작할 수 있습니다.",
    },
    cancel: {
        en: "Cancel",
        ko: "취소",
    },
    start: {
        en: "Start Space",
        ko: "시작하기",
    },
    starting: {
        en: "Starting...",
        ko: "시작 중...",
    },
}

#[component]
pub fn SpaceStartModal(
    space_id: SpacePartition,
    #[props(default)] on_close: Option<EventHandler<()>>,
) -> Element {
    let tr: SpaceStartModalTranslate = use_translate();
    let mut popup = use_popup();
    let mut toast = use_toast();
    let mut is_starting = use_signal(|| false);
    let mut ctx = use_space_context();

    let on_cancel = move |_| {
        if let Some(handler) = on_close.as_ref() {
            handler.call(());
        }
        popup.close();
    };

    let on_start = move |_| {
        if is_starting() {
            return;
        }

        is_starting.set(true);

        let mut is_starting = is_starting;
        let mut popup = popup;
        let mut toast = toast;
        let space_id = space_id.clone();
        let mut ctx = ctx;

        spawn(async move {
            let res = update_space(
                space_id,
                controllers::UpdateSpaceRequest::Start { start: true },
            )
            .await;

            match res {
                Ok(_) => {
                    ctx.space.restart();
                    popup.close();
                }
                Err(err) => {
                    toast.error(err);
                }
            }

            is_starting.set(false);
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
                    "data-testid": "start-space-button",
                    onclick: on_start,
                    disabled: is_starting(),
                    class: if !is_starting() {
                        "w-full py-[14.5px] font-bold text-base rounded-[10px] bg-primary text-black hover:bg-primary/80 transition-colors"
                    } else {
                        "w-full py-[14.5px] font-bold text-base rounded-[10px] bg-neutral-700 light:bg-neutral-300 text-neutral-500 cursor-not-allowed transition-colors"
                    },
                    if is_starting() {
                        "{tr.starting}"
                    } else {
                        "{tr.start}"
                    }
                }
            }
        }
    }
}
