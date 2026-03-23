use super::*;

#[component]
pub fn JoinAnytimeSetting() -> Element {
    let mut space = use_space();
    let tr: GeneralTranslate = use_translate();
    let mut toast = use_toast();
    let mut loading = use_signal(|| false);

    let enable_join_anytime = space().join_anytime;

    rsx! {
        Card {
            div { class: "flex justify-between items-center self-stretch py-4 px-5 border-b border-separator",
                p { class: "font-semibold text-center font-raleway text-[17px]/[20px] tracking-[-0.18px] text-web-font-primary",
                    {tr.join_anytime_setting}
                }
            }

            div { class: "flex flex-row justify-between items-center self-stretch p-5 gap-[10px] bg-card max-mobile:p-4",
                p { class: "font-normal leading-6 font-raleway text-[15px] tracking-[0.5px] text-card-meta",
                    {tr.join_anytime_description}
                }

                Switch {
                    active: enable_join_anytime,
                    on_toggle: move |_| async move {
                        if loading() {
                            return;
                        }
                        loading.set(true);
                        let space_id = space().id;
                        let next_join_anytime = !enable_join_anytime;
                        let result = update_space(
                                space_id,
                                UpdateSpaceRequest::JoinAnytime {
                                    join_anytime: next_join_anytime,
                                },
                            )
                            .await;
                        loading.set(false);
                        match result {
                            Ok(_) => {
                                space.with_mut(|s| s.join_anytime = next_join_anytime);
                                toast.info(tr.join_anytime_updated_successfully);
                            }
                            Err(err) => {
                                space.with_mut(|s| s.join_anytime = enable_join_anytime);
                                toast.error(err);
                            }
                        }
                    },
                }
            }
        }
    }
}
