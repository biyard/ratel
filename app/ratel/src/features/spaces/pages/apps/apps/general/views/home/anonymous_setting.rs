use super::*;

#[component]
pub fn AnonymousSetting() -> Element {
    let space = use_space();
    let tr: GeneralTranslate = use_translate();
    let mut toast = use_toast();
    let mut loading = use_signal(|| false);

    let enable_anonymous = space().anonymous_participation;

    rsx! {
        Card {
            div { class: "flex justify-between items-center self-stretch py-4 px-5 border-b border-separator",
                p { class: "font-semibold text-center font-raleway text-[17px]/[20px] tracking-[-0.18px] text-web-font-primary",
                    {tr.anonymous_setting}
                }
            }

            div { class: "flex flex-row justify-between items-center self-stretch p-5 gap-[10px] bg-card max-mobile:p-4",
                p { class: "font-normal leading-6 font-raleway text-[15px] tracking-[0.5px] text-card-meta",
                    {tr.anonymous_setting_description}
                }

                Switch {
                    active: enable_anonymous,
                    on_toggle: move |_| async move {
                        if loading() {
                            return;
                        }
                        loading.set(true);
                        let space_id = space().id;
                        let result = update_space(
                                space_id,
                                UpdateSpaceRequest::Anonymous {
                                    anonymous_participation: !enable_anonymous,
                                },
                            )
                            .await;
                        loading.set(false);
                        match result {
                            Ok(_) => {
                                toast.info(tr.anonymous_updated_successfully);
                            }
                            Err(err) => {
                                toast.error(err);
                            }
                        }
                    },
                }
            }
        }
    }
}
