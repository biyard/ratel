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
    ConsentParticipate = 4,
}

impl ParticipationLayoverStep {
    pub fn next(&self) -> Option<Self> {
        match self {
            ParticipationLayoverStep::SeeYourDifference => {
                Some(ParticipationLayoverStep::MatchRequiredAttributes)
            }
            ParticipationLayoverStep::MatchRequiredAttributes => {
                Some(ParticipationLayoverStep::CreateCredential)
            }
            ParticipationLayoverStep::CreateCredential => {
                Some(ParticipationLayoverStep::ConsentParticipate)
            }
            ParticipationLayoverStep::ConsentParticipate => None,
        }
    }

    pub fn back(&self) -> Option<Self> {
        match self {
            ParticipationLayoverStep::SeeYourDifference => None,
            ParticipationLayoverStep::MatchRequiredAttributes => {
                Some(ParticipationLayoverStep::SeeYourDifference)
            }
            ParticipationLayoverStep::CreateCredential => {
                Some(ParticipationLayoverStep::MatchRequiredAttributes)
            }
            ParticipationLayoverStep::ConsentParticipate => {
                Some(ParticipationLayoverStep::CreateCredential)
            }
        }
    }
}

#[component]
pub fn ParticipationStepBar(current_step: ParticipationLayoverStep) -> Element {
    let tr: ParticipationStepBarTranslate = use_translate();

    rsx! {
        div { class: "flex flex-col items-start w-full",
            div { class: "flex gap-5 justify-center items-center py-6 px-5 w-full border-y border-[#262626] max-mobile:flex-col max-mobile:items-start max-mobile:gap-3",
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
        "text-web-font-primary"
    } else {
        "text-font-secondary"
    };

    rsx! {
        div { class: "flex gap-2 items-center",
            div { class: "flex justify-center items-center font-semibold rounded-full size-6 shrink-0 text-[12px]/[16px] {circle_class}",
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
