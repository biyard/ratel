use super::*;

translate! {
    ParticipationStepBarTranslate;

    see_your_difference: {
        en: "See your Difference",
        ko: "차이 확인하기",
    },
    match_required_attributes: {
        en: "Match Required Attributes",
        ko: "필수 속성 맞추기",
    },
    create_credential: {
        en: "Create Credential",
        ko: "크리덴셜 생성",
    },
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParticipationLayoverStep {
    SeeYourDifference = 1,
    MatchRequiredAttributes = 2,
    CreateCredential = 3,
}

#[component]
pub fn ParticipationStepBar(current_step: ParticipationLayoverStep) -> Element {
    let tr: ParticipationStepBarTranslate = use_translate();

    rsx! {
        div { class: "flex w-full flex-col items-start bg-[#1A1A1A]",
            div { class: "flex w-full items-center justify-center gap-5 border-y border-[#262626] px-5 py-6 max-mobile:flex-col max-mobile:items-start max-mobile:gap-3",
                StepBarItem {
                    step: 1,
                    label: tr.see_your_difference.to_string(),
                    active: current_step >= ParticipationLayoverStep::SeeYourDifference,
                    show_line: true,
                }
                StepBarItem {
                    step: 2,
                    label: tr.match_required_attributes.to_string(),
                    active: current_step >= ParticipationLayoverStep::MatchRequiredAttributes,
                    show_line: true,
                }
                StepBarItem {
                    step: 3,
                    label: tr.create_credential.to_string(),
                    active: current_step >= ParticipationLayoverStep::CreateCredential,
                    show_line: false,
                }
            }
        }
    }
}

#[component]
fn StepBarItem(step: u8, label: String, active: bool, show_line: bool) -> Element {
    let circle_class = if active {
        "bg-primary text-[#0A0A0A]"
    } else {
        "bg-[#737373] text-[#0A0A0A]"
    };
    let text_class = if active {
        "text-white"
    } else {
        "text-[#8C8C8C]"
    };

    rsx! {
        div { class: "flex items-center gap-2",
            div { class: "flex size-6 shrink-0 items-center justify-center rounded-full font-semibold text-[12px]/[16px] {circle_class}",
                {step.to_string()}
            }
            span { class: "font-semibold font-raleway text-[15px]/[18px] tracking-[-0.16px] {text_class}",
                {label}
            }
            if show_line {
                div { class: "h-px w-[58px] shrink-0 bg-[#737373] max-mobile:hidden" }
            }
        }
    }
}
