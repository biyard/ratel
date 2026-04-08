use super::*;

#[component]
pub fn ParticipationAttributesSection(
    requirements: Vec<
        crate::features::spaces::controllers::panel_requirements::PanelRequirementStatus,
    >,
    current_step: ParticipationLayoverStep,
    on_continue: EventHandler<()>,
) -> Element {
    let tr: ParticipationAttributesSectionTranslate = use_translate();
    let has_missing = requirements
        .iter()
        .any(|requirement| !requirement.satisfied);
    let show_continue = current_step == ParticipationLayoverStep::SeeYourDifference;

    rsx! {
        div { class: "flex flex-1 flex-col gap-5 bg-[#1A1A1A] px-[30px] py-[30px] max-tablet:px-5 max-tablet:py-5 max-mobile:px-4 max-mobile:py-4",
            div { class: "flex flex-col items-start gap-[10px] w-full",
                h3 { class: "font-bold text-[24px]/[28px] tracking-[-0.24px] text-white",
                    {tr.partial_match_title}
                }

                if has_missing {
                    div { class: "inline-flex items-center gap-[10px] rounded-full bg-[rgba(249,115,22,0.1)] px-5 py-[10px]",
                        crate::common::lucide_dioxus::CircleAlert { size: 18, class: "text-[#F97316] shrink-0" }
                        span { class: "font-medium text-[13px]/[16px] tracking-[-0.14px] text-[#F97316]",
                            {tr.missing_notice}
                        }
                    }
                }
            }

            div { class: "flex w-full flex-col items-start gap-[10px] rounded-[12px] border border-[#404040] bg-[#262626] px-4 py-5",
                div { class: "flex w-full flex-col items-start gap-1",
                    p { class: "font-bold text-[15px]/[18px] tracking-[-0.16px] text-white",
                        {tr.requirements_to_unlock}
                    }
                    p { class: "font-medium text-[13px]/[20px] text-[#D4D4D4]",
                        {tr.requirements_description}
                    }
                }

                div { class: "flex w-full flex-col gap-[10px]",
                    for requirement in requirements.iter() {
                        AttributeRequirementRow { requirement: requirement.clone() }
                    }
                }
            }

            if show_continue {
                div { class: "flex flex-row w-full justify-end items-center mt-auto pt-5",
                    Button {
                        class: "!rounded-[10px] !px-5 !py-3 max-mobile:!w-full",
                        style: ButtonStyle::Primary,
                        onclick: move |_| on_continue.call(()),
                        span { class: "font-bold text-[14px]/[16px] text-[#0A0A0A]",
                            {tr.improve_my_credential}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AttributeRequirementRow(
    requirement: crate::features::spaces::controllers::panel_requirements::PanelRequirementStatus,
) -> Element {
    let tr: ParticipationAttributesSectionTranslate = use_translate();

    let (pill_class, value_box_class, icon) = if requirement.satisfied {
        (
            "bg-[rgba(34,197,94,0.1)] border-[0.5px] border-[#22C55E]",
            "bg-[rgba(34,197,94,0.1)] border-[0.5px] border-[#22C55E]",
            rsx! {
                icons::validations::Check { class: "size-[18px] [&>path]:stroke-[#22C55E] shrink-0" }
            },
        )
    } else {
        (
            "bg-[rgba(239,68,68,0.1)] border-[0.5px] border-[#EF4444]",
            "bg-[rgba(239,68,68,0.1)] border-[0.5px] border-[#EF4444]",
            rsx! {
                crate::common::lucide_dioxus::CircleAlert { size: 18, class: "text-[#EF4444] shrink-0" }
            },
        )
    };

    let label = requirement_label(requirement.attribute.clone(), &tr);
    let attribute = requirement.attribute.clone();

    // Collective attributes only need to confirm "category is verified"
    // — there's no point listing every possible value (e.g. all age
    // ranges or both genders). The label pill alone fills the row in
    // that case. Conditional rows still show the chip box with the
    // specific allowed values.
    if requirement.collective {
        return rsx! {
            div { class: "flex flex-row gap-[10px] items-center w-full",
                div { class: "h-[60px] inline-flex w-full items-center gap-[10px] rounded-full px-[15px] py-[13px] {pill_class}",
                    {icon}
                    span { class: "font-bold text-[15px]/[18px] tracking-[-0.16px] text-white",
                        {label}
                    }
                }
            }
        };
    }

    rsx! {
        div { class: "flex w-full flex-row items-center gap-[10px] max-mobile:flex-col max-mobile:items-start",
            div { class: "h-[60px] inline-flex min-w-[222px] items-center gap-[10px] rounded-full px-[15px] py-[13px] {pill_class} max-mobile:min-w-0",
                {icon}
                span { class: "font-bold text-[15px]/[18px] tracking-[-0.16px] text-white",
                    {label}
                }
            }

            div { class: "h-auto min-h-[60px] flex flex-1 flex-row flex-wrap items-center gap-1 rounded-[8px] px-[10px] py-[15px] {value_box_class} max-mobile:w-full",
                for value in requirement.required_values.iter() {
                    RequirementValueTag {
                        value: value.clone(),
                        is_mine: requirement.current_value.as_deref() == Some(value.as_str()),
                        attribute: attribute.clone(),
                    }
                }
            }
        }
    }
}

#[component]
fn RequirementValueTag(
    value: String,
    is_mine: bool,
    attribute: crate::features::spaces::controllers::panel_requirements::PanelRequirementAttribute,
) -> Element {
    let tr: ParticipationAttributesSectionTranslate = use_translate();
    let display = display_requirement_value(&attribute, &value, &tr);

    if is_mine {
        rsx! {
            span { class: "inline-flex items-center justify-center rounded-[6px] bg-[#FCB300] px-2 py-[3px] font-semibold text-[14px]/[20px] tracking-[0.5px] text-[#0A0A0A]",
                {display}
            }
        }
    } else {
        rsx! {
            span { class: "inline-flex items-center justify-center rounded-[6px] bg-white px-2 py-[3px] font-semibold text-[14px]/[20px] tracking-[0.5px] text-[#0A0A0A]",
                {display}
            }
        }
    }
}

/// Translates raw value strings produced by the backend (e.g. `"male"`,
/// `"female"`) into the user's current locale. Non-localizable values
/// like age ranges (`"0-17"`) and university names are returned as-is.
fn display_requirement_value(
    attribute: &crate::features::spaces::controllers::panel_requirements::PanelRequirementAttribute,
    value: &str,
    tr: &ParticipationAttributesSectionTranslate,
) -> String {
    use crate::features::spaces::controllers::panel_requirements::PanelRequirementAttribute;
    match attribute {
        PanelRequirementAttribute::Gender => match value.to_ascii_lowercase().as_str() {
            "male" => tr.gender_male.to_string(),
            "female" => tr.gender_female.to_string(),
            _ => value.to_string(),
        },
        PanelRequirementAttribute::Age | PanelRequirementAttribute::University => {
            value.to_string()
        }
    }
}

fn requirement_label(
    attribute: crate::features::spaces::controllers::panel_requirements::PanelRequirementAttribute,
    tr: &ParticipationAttributesSectionTranslate,
) -> String {
    match attribute {
        crate::features::spaces::controllers::panel_requirements::PanelRequirementAttribute::Age => {
            tr.age.to_string()
        }
        crate::features::spaces::controllers::panel_requirements::PanelRequirementAttribute::Gender => {
            tr.gender.to_string()
        }
        crate::features::spaces::controllers::panel_requirements::PanelRequirementAttribute::University => {
            tr.university.to_string()
        }
    }
}

translate! {
    ParticipationAttributesSectionTranslate;

    partial_match_title: {
        en: "Your Attributes is a Partial Match",
        ko: "속성이 일부 일치합니다",
    },

    missing_notice: {
        en: "Some required attributes are missing for this space",
        ko: "이 스페이스에 필요한 속성이 부족합니다",
    },

    requirements_to_unlock: {
        en: "Requirements to Unlock",
        ko: "잠금 해제 요건",
    },

    requirements_description: {
        en: "To join this space, certain attributes are required. Based on your current profile, some attributes do not match. You must meet the requirements below to unlock access.",
        ko: "이 스페이스에 참여하려면 특정 속성이 필요합니다. 현재 프로필 기준으로 일부 속성이 일치하지 않습니다. 아래 요건을 충족해야 접근이 가능합니다.",
    },

    improve_my_credential: {
        en: "Improve My Credential",
        ko: "내 Credential 개선하기",
    },

    age: {
        en: "Age",
        ko: "나이",
    },

    gender: {
        en: "Gender",
        ko: "성별",
    },

    university: {
        en: "University",
        ko: "대학교",
    },

    gender_male: {
        en: "Male",
        ko: "남성",
    },

    gender_female: {
        en: "Female",
        ko: "여성",
    },

    verification_required: {
        en: "Verification required",
        ko: "인증 필요",
    },
}
