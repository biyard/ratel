use crate::*;
use ratel_post::types::SpaceStatus;

#[component]
pub fn SpaceLabel(status: Option<SpaceStatus>) -> Element {
    let tr: SpaceLabelTranslate = use_translate();
    let label = match status {
        None => tr.draft,
        Some(SpaceStatus::Waiting) => tr.draft,
        Some(SpaceStatus::InProgress) => tr.in_progress,
        Some(SpaceStatus::Started) => tr.started,
        Some(SpaceStatus::Finished) => tr.finished,
    };

    rsx! {
        div { class: "box-border flex flex-row items-start px-[13px] py-[7px]
                bg-[rgba(34,197,94,0.2)] border border-[rgba(34,197,94,0.3)] rounded-full font-semibold text-sm leading-4 text-[#22C55E]",
            {label}
        }
    }
}

translate! {
    SpaceLabelTranslate;

    draft: {
        en: "Draft",
        ko: "초안",
    },
    in_progress: {
        en: "In Progress",
        ko: "진행 중",
    },
    started: {
        en: "Started",
        ko: "시작됨",
    },
    finished: {
        en: "Finished",
        ko: "종료",
    },
}
