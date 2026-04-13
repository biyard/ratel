use super::*;

#[component]
pub fn StartTimeSetting() -> Element {
    let mut space = use_space();
    let tr: GeneralTranslate = use_translate();
    let mut toast = use_toast();

    rsx! {
        Card {
            div { class: "flex justify-between items-center self-stretch py-4 px-5 border-b border-separator",
                p { class: "font-semibold text-center font-raleway text-[17px]/[20px] tracking-[-0.18px] text-web-font-primary",
                    {tr.start_time_setting}
                }
            }

            div { class: "flex flex-col items-start self-stretch gap-2.5 p-5 bg-card max-mobile:p-4",
                p { class: "font-normal leading-6 font-raleway text-[15px] tracking-[0.5px] text-card-meta",
                    {tr.start_time_description}
                }

                DateAndTimePicker {
                    initial_started_at: space().started_at,
                    on_change: move |range: DateTimeRange| async move {
                        if let Some(start_date) = range.start_date {
                            let started_at = range
                                .timezone
                                .local_to_utc_millis(start_date, range.start_hour, range.start_minute);
                            let space_id = space().id;
                            let result = update_space(
                                    space_id,
                                    UpdateSpaceRequest::StartTime {
                                        started_at: Some(started_at),
                                    },
                                )
                                .await;
                            match result {
                                Ok(_) => {
                                    space.with_mut(|s| s.started_at = Some(started_at));
                                    toast.info(tr.start_time_updated_successfully);
                                }
                                Err(err) => {
                                    toast.error(err);
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}
