use super::*;
use dioxus_primitives::{ContentAlign, ContentSide};
mod question_tab;

use question_tab::*;

#[component]
pub fn PollCreatorPage(
    space_id: ReadSignal<SpacePartition>,
    poll_id: ReadSignal<SpacePollEntityType>,
) -> Element {
    let tr: CreatorTranslate = use_translate();
    let mut ctx = Context::init(space_id, poll_id)?;
    let mut enabled = use_signal(move || ctx.poll().encrypted_upload_enabled);
    let mut toast = crate::common::use_toast();

    let on_date_change = move |range: DateTimeRange| async move {
        let space_id = space_id();
        let poll_id = poll_id();
        if let (Some(start_date), Some(end_date)) = (range.start_date, range.end_date) {
            let started_at = date_time_to_millis(start_date, range.start_hour, range.start_minute);
            let ended_at = date_time_to_millis(end_date, range.end_hour, range.end_minute);
            let _ = update_poll(
                space_id,
                poll_id,
                UpdatePollRequest::Time {
                    started_at,
                    ended_at,
                },
            )
            .await;
            ctx.poll.restart();
        }
    };

    let on_response_editable_toggle = move |_| async move {
        let enabled = !ctx.poll().response_editable;
        let _ = update_poll(
            space_id(),
            poll_id(),
            UpdatePollRequest::ResponseEditable {
                response_editable: enabled,
            },
        )
        .await;
        ctx.poll.restart();
    };

    let on_encrypted_upload_toggle = move |_| async move {
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
                ctx.poll.restart();
            }
            Err(e) => {
                toast.error(e);
            }
        }
    };

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
                        on_date_change,
                    }

                    // Response Editable toggle
                    Card { class: "mt-4",
                        div { class: "flex justify-between items-center self-stretch border-b border-separator",
                            div { class: "flex gap-1 items-center",
                                p { class: "font-semibold font-raleway text-[15px]/[18px] tracking-[-0.16px] text-web-font-primary",
                                    {tr.response_editable_title}
                                }
                                Tooltip {
                                    TooltipTrigger {
                                        icons::help_support::Info {
                                            width: "14",
                                            height: "14",
                                            class: "cursor-help text-web-font-neutral [&>path]:stroke-current [&>circle]:fill-current [&>path]:fill-none",
                                        }
                                    }
                                    TooltipContent {
                                        side: ContentSide::Bottom,
                                        align: ContentAlign::Start,
                                        p { class: "w-72", {tr.response_editable_desc} }
                                    }
                                }
                            }

                            Switch {
                                active: ctx.poll().response_editable && !ctx.poll().encrypted_upload_enabled,
                                disabled: ctx.poll().encrypted_upload_enabled,
                                on_toggle: on_response_editable_toggle,
                            }
                        }
                    }

                    // Encrypted Upload toggle
                    EncryptedUploadSetting { enabled, on_toggle: on_encrypted_upload_toggle }
                }
            }
        }
    }
}

#[component]
fn EncryptedUploadSetting(
    enabled: ReadSignal<bool>,
    on_toggle: EventHandler<MouseEvent>,
) -> Element {
    let tr: CreatorTranslate = use_translate();
    let is_prod = crate::common::config::Environment::default()
        == crate::common::config::Environment::Production;

    if is_prod {
        return rsx! {};
    }

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
            Switch { active: enabled(), on_toggle }
        }
    }
}

fn date_time_to_millis(date: time::Date, hour: u8, minute: u8) -> i64 {
    crate::common::utils::time::kst_date_time_to_utc_millis(date, hour, minute)
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

    response_editable_title: {
        en: "Allow Response Editing",
        ko: "응답 수정 허용",
    }

    response_editable_desc: {
        en: "Participants can modify their submitted responses while the poll is in progress.",
        ko: "투표 진행 중 참여자가 제출한 응답을 수정할 수 있습니다.",
    }

    response_editable_tooltip: {
        en: "When enabled, participants can go back and change their answers after submitting. Disabled automatically when Encrypted Upload is on.",
        ko: "활성화하면 참여자가 제출 후에도 응답을 다시 수정할 수 있습니다. 암호화 업로드가 켜져 있으면 자동으로 비활성화됩니다.",
    }

    encrypted_upload_updated: {
        en: "Encrypted upload setting updated.",
        ko: "암호화 업로드 설정이 업데이트되었습니다.",
    }
}
