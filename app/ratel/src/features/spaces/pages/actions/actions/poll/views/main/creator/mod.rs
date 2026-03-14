use super::*;
mod question_tab;

use question_tab::*;

#[component]
pub fn PollCreatorPage(
    space_id: ReadSignal<SpacePartition>,
    poll_id: ReadSignal<SpacePollEntityType>,
) -> Element {
    let tr: CreatorTranslate = use_translate();
    let ctx = Context::init(space_id, poll_id)?;

    rsx! {
        div { class: "flex flex-col gap-4 w-full",
            h3 { {tr.title} }
            Tabs { default_value: "question-tab",
                TabList {
                    TabTrigger { index: 0usize, value: "question-tab", {tr.tab_questions} }
                    TabTrigger { index: 1usize, value: "setting-tab", {tr.tab_setting} }
                }
                TabContent { index: 0usize, value: "question-tab", QuestionTab {} }
                TabContent { index: 1usize, value: "setting-tab",
                    ActionCommonSettings {
                        space_id,
                        action_id: poll_id().to_string(),
                        action_setting: ctx.poll().space_action,
                        on_date_change: move |range: DateTimeRange| async move {
                            let space_id = space_id();
                            let poll_id = poll_id();
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
                                let _ = update_poll(
                                        space_id,
                                        poll_id,
                                        UpdatePollRequest::Time {
                                            started_at,
                                            ended_at,
                                        },
                                    )
                                    .await;
                            }
                        },
                    }
                }
            }
        }
    }
}

fn date_time_to_millis(date: time::Date, hour: u8, minute: u8) -> i64 {
    let datetime = date.with_hms(hour, minute, 0).expect("valid time");
    let offset_datetime = datetime.assume_utc();
    (offset_datetime.unix_timestamp()) * 1000
}

translate! {
    CreatorTranslate;

    title: {
        en: "Poll",
        ko: "투표",
    }

    tab_questions: {
        en: "Questions",
        ko: "질문",
    }

    tab_setting: {
        en: "Settings",
        ko: "설정",
    }
}
