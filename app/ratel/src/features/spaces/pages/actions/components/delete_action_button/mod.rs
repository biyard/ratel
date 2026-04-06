use super::*;

translate! {
    DeleteActionButtonTranslate;

    delete_action: {
        en: "Delete",
        ko: "삭제하기",
    },
    delete_success: {
        en: "Action deleted successfully.",
        ko: "액션이 삭제되었습니다.",
    },
}

#[component]
pub fn ActionDeleteButton(space_id: SpacePartition, action_id: String) -> Element {
    let tr: DeleteActionButtonTranslate = use_translate();
    let mut toast = crate::common::use_toast();
    let mut popup = use_popup();
    let nav = navigator();

    rsx! {
        div { class: "flex justify-end pt-5 w-full max-tablet:justify-stretch",
            Button {
                class: "border w-fit self-end max-tablet:w-full border-web-error/70 !bg-transparent !text-web-error transition-colors duration-150 hover:!bg-web-error/10 hover:!border-web-error hover:!text-web-error disabled:!bg-transparent disabled:border-web-error/40 disabled:!text-web-error/40",
                style: ButtonStyle::Text,
                onclick: move |_| {
                    let mut popup = popup;
                    let mut toast = toast;
                    let nav = nav.clone();
                    let space_id = space_id.clone();
                    let action_id = action_id.clone();
                    let on_cancel = move |_| popup.close();
                    let on_confirm = move |_| {
                        let mut popup = popup;
                        let mut toast = toast;
                        let nav = nav.clone();
                        let space_id = space_id.clone();
                        let action_id = action_id.clone();
                        spawn(async move {
                            match delete_space_action(space_id.clone(), action_id).await {
                                Ok(_) => {
                                    popup.close();
                                    toast.info(tr.delete_success.to_string());
                                    nav.push(Route::SpaceActionsPage { space_id });
                                }
                                Err(err) => {
                                    toast.error(err);
                                }
                            }
                        });
                    };
                    popup.open(rsx! {
                        DeleteActionPopup { on_confirm, on_cancel }
                    });
                },
                {tr.delete_action}
            }
        }
    }
}
