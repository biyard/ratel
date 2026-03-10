use super::*;

#[component]
pub fn ParticipationCredentialSection(
    requirements: Vec<
        crate::features::spaces::controllers::panel_requirements::PanelRequirementStatus,
    >,
) -> Element {
    let tr: ParticipationCredentialSectionTranslate = use_translate();
    let mut layover = use_layover();

    rsx! {
        div { class: "flex flex-1 flex-col bg-[#1A1A1A]",
            div { class: "flex flex-col items-center justify-center gap-5 bg-[#1A1A1A] px-[30px] pt-[30px] max-tablet:px-5 max-tablet:pt-5 max-mobile:px-4 max-mobile:pt-4",
                div { class: "flex w-full flex-row items-center justify-between gap-[10px]",
                    div { class: "flex flex-row items-center gap-[10px]",
                        div { class: "relative flex size-8 shrink-0 items-center justify-center rounded-full border-2 border-[#FCB300] bg-[rgba(252,179,0,0.15)]",
                            icons::validations::Check { class: "size-8 [&>path]:stroke-[#FCB300]" }
                        }
                        h3 { class: "font-bold font-raleway text-[24px]/[28px] tracking-[-0.24px] text-white",
                            {tr.verified_title}
                        }
                    }

                    div { class: "inline-flex items-center gap-[4px] rounded-full border border-[#FCB300] px-2 py-[3px]",
                        icons::security::ShieldGood {
                            width: "14",
                            height: "14",
                            class: "[&>path]:stroke-[#FCB300]",
                        }
                        span { class: "font-semibold font-raleway text-[12px]/[16px] text-[#FCB300]",
                            {tr.verified_badge}
                        }
                    }
                }

                p { class: "w-full font-medium font-raleway text-[15px]/[22px] text-[#D4D4D4]",
                    {tr.verified_description}
                }
            }

            div { class: "flex flex-1 flex-col gap-5 bg-[#1A1A1A] px-[30px] py-[30px] max-tablet:px-5 max-tablet:py-5 max-mobile:px-4 max-mobile:py-4",
                div { class: "flex w-full flex-col gap-[10px] rounded-[12px] border border-[#404040] bg-[#262626] px-4 py-5",
                    div { class: "flex w-full flex-col items-start gap-1",
                        p { class: "font-bold font-raleway text-[15px]/[18px] tracking-[-0.16px] text-white",
                            {tr.unlocked_attributes_title}
                        }
                        p { class: "font-medium font-raleway text-[13px]/[20px] text-[#D4D4D4]",
                            {tr.unlocked_attributes_description}
                        }
                    }

                    div { class: "flex w-full flex-col gap-[10px]",
                        for requirement in requirements.iter() {
                            CredentialAttributeRow { requirement: requirement.clone() }
                        }
                    }

                    div { class: "flex w-full flex-row items-start rounded-[12px] border border-[#404040] bg-[#262626] px-5 py-[15px]",
                        div { class: "flex w-full flex-row items-start gap-1",
                            icons::security::ShieldGood {
                                width: "18",
                                height: "18",
                                class: "mt-0.5 shrink-0 [&>path]:stroke-[#737373]",
                            }
                            div { class: "flex flex-col items-start gap-1",
                                p { class: "font-bold font-raleway text-[13px]/[16px] tracking-[-0.14px] text-[#D4D4D4]",
                                    {tr.notice_title}
                                }
                                p { class: "font-medium font-raleway text-[12px]/[16px] text-[#D4D4D4]",
                                    {tr.notice_description}
                                }
                            }
                        }
                    }
                }

                div { class: "mt-auto flex w-full justify-end pt-2",
                    Button {
                        class: "!rounded-[10px] !px-5 !py-3 max-mobile:!w-full",
                        style: ButtonStyle::Primary,
                        onclick: move |_| layover.close(),
                        span { class: "font-bold font-raleway text-[14px]/[16px] text-[#0A0A0A]",
                            {tr.enter_space}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CredentialAttributeRow(
    requirement: crate::features::spaces::controllers::panel_requirements::PanelRequirementStatus,
) -> Element {
    let tr: ParticipationCredentialSectionTranslate = use_translate();
    let label = credential_attribute_label(requirement.attribute, &tr);

    rsx! {
        div { class: "flex w-full flex-row items-center gap-[10px] max-mobile:flex-col max-mobile:items-start",
            div { class: "inline-flex h-[44px] min-w-[222px] items-center gap-[10px] rounded-full border-[0.5px] border-[#22C55E] bg-[rgba(34,197,94,0.1)] px-[15px] max-mobile:min-w-0",
                crate::common::lucide_dioxus::CircleCheck { size: 18, class: "shrink-0 text-[#22C55E]" }
                span { class: "font-bold font-raleway text-[15px]/[18px] tracking-[-0.16px] text-white",
                    {label}
                }
            }

            div { class: "flex h-[44px] flex-1 flex-row flex-wrap items-center gap-1 rounded-[8px] border-[0.5px] border-[#22C55E] bg-[rgba(34,197,94,0.1)] px-[10px] max-mobile:w-full max-mobile:h-fit max-mobile:py-[10px]",
                if let Some(current_value) = requirement.current_value.clone() {
                    span { class: "inline-flex items-center justify-center rounded-[6px] bg-[#FCB300] px-2 py-[3px] font-semibold font-raleway text-[14px]/[20px] tracking-[0.5px] text-[#0A0A0A]",
                        {current_value}
                    }
                } else {
                    for value in requirement.required_values.iter() {
                        span { class: "inline-flex items-center justify-center rounded-[6px] bg-[#FCB300] px-2 py-[3px] font-semibold font-raleway text-[14px]/[20px] tracking-[0.5px] text-[#0A0A0A]",
                            {value.clone()}
                        }
                    }
                }
            }
        }
    }
}

fn credential_attribute_label(
    attribute: crate::features::spaces::controllers::panel_requirements::PanelRequirementAttribute,
    tr: &ParticipationCredentialSectionTranslate,
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
    ParticipationCredentialSectionTranslate;

    verified_title: {
        en: "You're verified for this space!",
        ko: "이 스페이스 인증이 완료되었습니다!",
    },

    verified_badge: {
        en: "Verified",
        ko: "인증됨",
    },

    verified_description: {
        en: "Your document was verified successfully. Your identity attributes have been added to your credential.",
        ko: "문서 인증이 완료되었습니다. 신원 속성이 Credential에 추가되었습니다.",
    },

    unlocked_attributes_title: {
        en: "Unlocked Attributes",
        ko: "잠금 해제된 속성",
    },

    unlocked_attributes_description: {
        en: "These identity fields are now verified and applied to your profile.",
        ko: "이 신원 필드들이 인증되어 프로필에 적용되었습니다.",
    },

    notice_title: {
        en: "New Permissions Activated",
        ko: "새 권한이 활성화되었습니다",
    },

    notice_description: {
        en: "You can now access features and actions that require verified identity.",
        ko: "인증된 신원이 필요한 기능과 액션에 이제 접근할 수 있습니다.",
    },

    enter_space: {
        en: "Enter Space",
        ko: "스페이스 입장",
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
}
