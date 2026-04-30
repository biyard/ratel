use super::*;
use dioxus_primitives::{ContentAlign, ContentSide};

mod i18n;
pub use i18n::PollCreatorTranslate;

mod config_card;
mod content_card;
use config_card::ConfigCard;
use content_card::ContentCard;

use crate::features::spaces::pages::actions::components::{
    ActionEditFooter, ActionEditSaveBus, ActionEditTopbar,
};

#[component]
pub fn PollCreatorPage(
    space_id: ReadSignal<SpacePartition>,
    poll_id: ReadSignal<SpacePollEntityType>,
) -> Element {
    let tr: PollCreatorTranslate = use_translate();
    let ctx = Context::init(space_id, poll_id)?;
    let space = crate::features::spaces::space_common::hooks::use_space()();
    let nav = use_navigator();

    let initial_title = ctx.poll.read().title.clone();
    let title = use_signal(|| initial_title);

    // Provide the save bus so cards (Content/Config) can flush pending
    // debounced autosaves when the footer's Save button is pressed.
    ActionEditSaveBus::provide();
    let current_page = use_signal(|| 0usize);

    rsx! {
        div { class: "arena",
            ActionEditTopbar {
                space_name: space.title.clone(),
                action_type_label: tr.type_badge_label.to_string(),
                action_type_key: "poll",
                title,
                on_title_change: move |_v: String| {},
                editable_title: false,
                on_back: move |_| {
                    nav.go_back();
                },
                on_cancel: move |_| {
                    nav.go_back();
                },
            }
            main { class: "pager",
                div {
                    class: "pager__track",
                    style: "transform: translateX(-{current_page() * 100}%);",
                    ContentCard {}
                    ConfigCard {}
                }
            }
            ActionEditFooter { current_page, total_pages: 2, action_type_key: "poll" }
        }
    }
}

#[component]
fn EncryptedUploadSetting(
    enabled: ReadSignal<bool>,
    on_toggle: EventHandler<MouseEvent>,
) -> Element {
    let tr: CreatorTranslate = use_translate();
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
                    TooltipContent { side: ContentSide::Bottom, align: ContentAlign::Start,
                        p { class: "w-72", {tr.encrypted_upload_tooltip} }
                    }
                }
            }
            Switch { active: enabled(), on_toggle }
        }
    }
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
