use super::*;

#[component]
pub fn ParticipationCredentialSection(
    requirements: Vec<
        crate::features::spaces::controllers::panel_requirements::PanelRequirementStatus,
    >,
    on_completed: EventHandler<()>,
) -> Element {
    let tr: ParticipationCredentialSectionTranslate = use_translate();
    let mut layover = use_layover();
    let mut ctx = crate::features::spaces::space_common::providers::use_space_context();

    rsx! {
        div { class: "flex flex-col flex-1 bg-card-bg text-text-primary",
            div { class: "flex flex-col gap-5 justify-center items-center px-[30px] pt-[30px] max-tablet:px-5 max-tablet:pt-5 max-mobile:px-4 max-mobile:pt-4",
                div { class: "flex flex-row justify-between items-center w-full gap-[10px]",
                    div { class: "flex flex-row items-center gap-[10px]",
                        div { class: "flex relative justify-center items-center rounded-full border-2 size-8 shrink-0 border-primary bg-primary/15",
                            icons::validations::Check { class: "size-8 [&>path]:fill-none [&>path]:stroke-primary" }
                        }
                        h3 { class: "font-bold font-raleway text-[24px]/[28px] tracking-[-0.24px]",
                            {tr.verified_title}
                        }
                    }

                    div { class: "inline-flex items-center px-2 rounded-full border gap-[4px] border-primary py-[3px]",
                        icons::security::ShieldGood {
                            width: "14",
                            height: "14",
                            class: "[&>path]:fill-none [&>path]:stroke-primary",
                        }
                        span { class: "font-semibold font-raleway text-[12px]/[16px] text-primary",
                            {tr.verified_badge}
                        }
                    }
                }

                p { class: "w-full font-medium font-raleway text-[15px]/[22px] text-foreground-muted",
                    {tr.verified_description}
                }
            }

            div { class: "flex flex-col flex-1 gap-5 px-[30px] py-[30px] max-tablet:px-5 max-tablet:py-5 max-mobile:px-4 max-mobile:py-4",
                div { class: "flex flex-col py-5 px-4 w-full border gap-[10px] rounded-[12px] border-border bg-card-bg-3",
                    div { class: "flex flex-col gap-1 items-start w-full",
                        p { class: "font-bold font-raleway text-[15px]/[18px] tracking-[-0.16px]",
                            {tr.unlocked_attributes_title}
                        }
                        p { class: "font-medium font-raleway text-[13px]/[20px] text-foreground-muted",
                            {tr.unlocked_attributes_description}
                        }
                    }

                    div { class: "flex flex-col w-full gap-[10px]",
                        for requirement in requirements.iter() {
                            CredentialAttributeRow { requirement: requirement.clone() }
                        }
                    }

                    div { class: "flex flex-row items-start px-5 w-full border rounded-[12px] border-border bg-card-bg-3 py-[15px]",
                        div { class: "flex flex-row gap-1 items-start w-full",
                            icons::security::ShieldGood {
                                width: "18",
                                height: "18",
                                class: "mt-0.5 shrink-0 [&>path]:fill-none [&>path]:stroke-text-primary",
                            }
                            div { class: "flex flex-col gap-1 items-start",
                                p { class: "font-bold font-raleway text-[13px]/[16px] tracking-[-0.14px] text-foreground-muted",
                                    {tr.notice_title}
                                }
                                p { class: "font-medium font-raleway text-[12px]/[16px] text-foreground-muted",
                                    {tr.notice_description}
                                }
                            }
                        }
                    }
                }

                div { class: "flex justify-end pt-2 mt-auto w-full",
                    Button {
                        class: "!rounded-[10px] !px-5 !py-3 max-mobile:!w-full",
                        style: ButtonStyle::Primary,
                        onclick: move |_| {
                            ctx.panel_requirements.restart();
                            ctx.space.restart();
                            ctx.role.restart();
                            on_completed.call(());
                        },
                        span { class: "font-bold font-raleway text-[14px]/[16px] text-btn-primary-text",
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
        div { class: "flex flex-row items-center w-full gap-[10px] max-mobile:flex-col max-mobile:items-start",
            div { class: "inline-flex items-center rounded-full h-[44px] min-w-[222px] gap-[10px] border-[0.5px] border-[#22C55E] bg-[rgba(34,197,94,0.1)] px-[15px] max-mobile:min-w-0",
                crate::common::lucide_dioxus::CircleCheck { size: 18, class: "shrink-0 text-[#22C55E]" }
                span { class: "font-bold font-raleway text-[15px]/[18px] tracking-[-0.16px]",
                    {label}
                }
            }

            div { class: "flex flex-row flex-wrap flex-1 gap-1 items-center h-[44px] rounded-[8px] border-[0.5px] border-[#22C55E] bg-[rgba(34,197,94,0.1)] px-[10px] max-mobile:w-full max-mobile:h-fit max-mobile:py-[10px]",
                if let Some(current_value) = requirement.current_value.clone() {
                    span { class: "inline-flex justify-center items-center px-2 font-semibold rounded-[6px] bg-[#FCB300] py-[3px] font-raleway text-[14px]/[20px] tracking-[0.5px] text-[#0A0A0A]",
                        {current_value}
                    }
                } else {
                    for value in requirement.required_values.iter() {
                        span { class: "inline-flex justify-center items-center px-2 font-semibold rounded-[6px] bg-[#FCB300] py-[3px] font-raleway text-[14px]/[20px] tracking-[0.5px] text-[#0A0A0A]",
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
