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

                    // Encrypted Upload toggle
                    EncryptedUploadSetting { space_id, poll_id }
                }
            }
        }
    }
}

#[component]
fn EncryptedUploadSetting(
    space_id: ReadSignal<SpacePartition>,
    poll_id: ReadSignal<SpacePollEntityType>,
) -> Element {
    let tr: CreatorTranslate = use_translate();
    let mut toast = crate::common::use_toast();
    let ctx = use_space_poll_context();
    let mut enabled = use_signal(move || ctx.poll().encrypted_upload_enabled);

    rsx! {
        Card {
            direction: CardDirection::Row,
            main_axis_align: MainAxisAlign::Between,
            cross_axis_align: CrossAxisAlign::Center,
            div { class: "flex gap-1 items-center",
                p { class: "font-semibold font-raleway text-[15px]/[18px] tracking-[-0.16px] text-web-font-primary",
                    {tr.encrypted_upload_title}
                }
                Tooltip {
                    TooltipTrigger {
                        icons::help_support::Info {
                            width: "14",
                            height: "14",
                            class: "cursor-help text-web-font-neutral [&>path]:stroke-current [&>circle]:fill-current [&>path]:fill-none",
                        }
                    }
                    TooltipContent { {tr.encrypted_upload_tooltip} }
                }
            }
            Switch {
                active: enabled(),
                on_toggle: move |_| async move {
                    let new_val = !enabled();
                    match update_poll(
                            space_id(),
                            poll_id(),
                            UpdatePollRequest::CanisterUploadEnabled {
                                canister_upload_enabled: new_val,
                            },
                        )
                        .await
                    {
                        Ok(_) => {
                            enabled.set(new_val);
                            toast.info(tr.encrypted_upload_updated.to_string());
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

    encrypted_upload_title: {
        en: "Encrypted Upload",
        ko: "암호화 업로드",
    }

    encrypted_upload_tooltip: {
        en: "Encrypt vote results and store on-chain for transparency. Once enabled, responses cannot be edited after submission.",
        ko: "투표 결과를 암호화하여 온체인에 저장합니다. 활성화하면 제출 후 응답을 수정할 수 없습니다.",
    }

    encrypted_upload_updated: {
        en: "Encrypted upload setting updated.",
        ko: "암호화 업로드 설정이 업데이트되었습니다.",
    }
}
