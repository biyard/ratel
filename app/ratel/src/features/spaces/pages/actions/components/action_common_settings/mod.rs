mod prerequisite_setting;
mod reward_setting;
pub use prerequisite_setting::*;
pub use reward_setting::*;

use super::*;

#[component]
pub fn ActionCommonSettings(
    space_id: ReadSignal<SpacePartition>,
    action_id: ReadSignal<String>,
    action_setting: ReadSignal<SpaceAction>,
    #[props(default)] on_date_change: EventHandler<DateTimeRange>,
    #[props(default)] on_credit_change: EventHandler<u64>,
    #[props(default)] on_prerequisite_change: EventHandler<bool>,
) -> Element {
    let tr: ActionCommonSettingsTranslate = use_translate();
    let mut toast = crate::common::use_toast();

    rsx! {
        div { class: "flex flex-col gap-5 w-full",
            div { class: "flex flex-col gap-2.5",
                p { {tr.date} }
                DateAndTimePicker {
                    on_change: move |range: DateTimeRange| async move {
                        if let (Some(start_date), Some(end_date)) = (range.start_date, range.end_date) {
                            let started_at = date_time_to_millis(
                                start_date,
                                range.start_hour,
                                range.start_minute,
                            );
                            let ended_at = date_time_to_millis(
                                end_date,
                                range.end_hour,
                                range.end_minute,
                            );
                            let req = UpdateSpaceActionRequest::Time {
                                started_at,
                                ended_at,
                            };
                            match update_space_action(space_id(), action_id(), req).await {
                                Ok(_) => {
                                    toast.info(tr.date_updated.to_string());
                                    on_date_change.call(range);
                                }
                                Err(e) => {
                                    toast.error(e);
                                }
                            }
                        }
                    },
                }
            }

            PrerequisiteSetting {
                space_id,
                action_id,
                action_setting,
                on_change: on_prerequisite_change,
            }

            RewardSetting {
                space_action: action_setting,
                on_change: move |credits: u64| async move {
                    let req = UpdateSpaceActionRequest::Credits {
                        credits,
                    };
                    match update_space_action(space_id(), action_id(), req).await {
                        Ok(_) => {
                            toast.info(tr.reward_updated.to_string());
                            on_credit_change.call(credits);
                        }
                        Err(e) => {
                            toast.error(e);
                        }
                    }
                },
            }
        }
    }
}

translate! {
    ActionCommonSettingsTranslate;

    date: {
        en: "Date",
        ko: "참여기간",
    },
    reward_updated: {
        en: "Reward credits updated.",
        ko: "보상 크레딧이 업데이트되었습니다.",
    },
    date_updated: {
        en: "Date range updated.",
        ko: "참여기간이 업데이트되었습니다.",
    },
}

fn date_time_to_millis(date: time::Date, hour: u8, minute: u8) -> i64 {
    let datetime = date.with_hms(hour, minute, 0).expect("valid time");
    let offset_datetime = datetime.assume_utc();
    offset_datetime.unix_timestamp() * 1000
}
