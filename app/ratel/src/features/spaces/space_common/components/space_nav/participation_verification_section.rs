use super::*;
use std::collections::BTreeSet;

#[component]
pub fn ParticipationVerificationSection(
    space_id: SpacePartition,
    requirements: Vec<
        crate::features::spaces::controllers::panel_requirements::PanelRequirementStatus,
    >,
    on_back: EventHandler<()>,
    on_verified: EventHandler<
        Vec<crate::features::spaces::controllers::panel_requirements::PanelRequirementStatus>,
    >,
) -> Element {
    let mut query = use_query_store();
    let tr: ParticipationVerificationSectionTranslate = use_translate();
    let mut error_message = use_signal(|| Option::<String>::None);
    let mut toast = use_toast();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let imported_attributes = requirements
        .iter()
        .map(|requirement| requirement_label(requirement.attribute, &tr))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    rsx! {
        div { class: "flex flex-1 flex-col gap-6 bg-[#1A1A1A] px-[30px] py-[30px] max-tablet:px-5 max-tablet:py-5 max-mobile:px-4 max-mobile:py-4",
            div { class: "flex w-full flex-col gap-[10px]",
                h3 { class: "font-bold text-[24px]/[28px] tracking-[-0.24px] text-white",
                    {tr.title}
                }
            }

            if let Some(error_message) = error_message() {
                div { class: "inline-flex w-full items-center rounded-full bg-[rgba(239,68,68,0.1)] px-5 py-[10px]",
                    div { class: "flex items-center gap-[10px]",
                        crate::common::lucide_dioxus::CircleAlert { size: 18, class: "text-[#EF4444] shrink-0" }
                        span { class: "font-medium text-[13px]/[16px] tracking-[-0.14px] text-[#EF4444]",
                            {error_message}
                        }
                    }
                }
            }

            div { class: "grid w-full grid-cols-2 gap-4 max-tablet:grid-cols-1",
                div { class: "flex h-full flex-col rounded-[12px] border border-[#404040] bg-[#262626] px-4 py-5",
                    div { class: "flex flex-col gap-2",
                        h4 { class: "font-bold text-[15px]/[18px] tracking-[-0.16px] text-white",
                            {tr.method_title}
                        }
                        p { class: "font-medium text-[13px]/[20px] text-[#D4D4D4]",
                            {tr.method_description}
                        }
                    }

                    div { class: "mt-5 flex flex-1 flex-col rounded-[12px] border border-[#FCB300] bg-[rgba(252,179,0,0.05)] px-4 py-5",
                        div { class: "flex items-start justify-between gap-3",
                            div { class: "flex flex-col gap-2",
                                span { class: "font-bold text-[15px]/[18px] tracking-[-0.16px] text-white",
                                    {tr.portone_title}
                                }
                                p { class: "font-medium text-[13px]/[20px] text-[#D4D4D4]",
                                    {tr.portone_description}
                                }
                            }
                        }
                    }
                }

                div { class: "flex h-full flex-col rounded-[12px] border border-[#404040] bg-[#262626] px-4 py-5",
                    div { class: "flex flex-col gap-2",
                        h4 { class: "font-bold text-[15px]/[18px] tracking-[-0.16px] text-white",
                            {tr.import_title}
                        }
                        p { class: "font-medium text-[13px]/[20px] text-[#D4D4D4]",
                            {tr.import_description}
                        }
                    }

                    div { class: "mt-5 flex flex-1 flex-col gap-[10px] rounded-[12px] bg-[#262626]",
                        for label in imported_attributes.iter() {
                            div { class: "flex items-center gap-[10px]",
                                div { class: "flex size-5 items-center justify-center rounded-[4px] bg-[#FCB300]",
                                    icons::validations::Check { class: "size-5 [&>path]:stroke-[#0A0A0A]" }
                                }
                                span { class: "font-semibold text-[15px]/[18px] tracking-[-0.16px] text-white",
                                    {label.clone()}
                                }
                            }
                        }
                    }
                }
            }

            div { class: "mt-auto flex w-full justify-end gap-3 pt-5",
                Button {
                    style: ButtonStyle::Text,
                    class: "!rounded-[10px] !px-5 !py-3 !text-white hover:!bg-white/5 hover:!text-white",
                    onclick: move |_| on_back.call(()),
                    {tr.back}
                }

                Button {
                    style: ButtonStyle::Primary,
                    class: "!rounded-[10px] !px-5 !py-3",
                    onclick: move |_| {
                        #[cfg(not(feature = "server"))]
                        {
                            let conf = crate::features::social::pages::credentials::config::get();
                            let store_id = conf.portone.store_id.to_string();
                            let channel_key = conf.portone.inicis_channel_key.to_string();
                            let prefix = user_ctx().user_id().unwrap_or_default();
                            let mut error_message = error_message;
                            let mut toast = toast;
                            let space_id = space_id.clone();
                            let verification_failed_message = tr.verification_failed.to_string();

                            spawn(async move {
                                match crate::features::social::pages::credentials::interop::verify_identity(
                                        &store_id,
                                        &channel_key,
                                        &prefix,
                                    )
                                    .await
                                {
                                    Ok(_) => {
                                        let panel_requirements_key = vec![
                                            "Space".to_string(),
                                            space_id.to_string(),
                                            "PanelRequirements".to_string(),
                                        ];
                                        query.invalidate(&panel_requirements_key);

                                        match crate::features::spaces::controllers::panel_requirements::get_panel_requirements(
                                                space_id.clone(),
                                            )
                                            .await
                                        {
                                            Ok(next_requirements) => {
                                                let all_satisfied = next_requirements
                                                    .iter()
                                                    .all(|requirement| requirement.satisfied);
                                                if all_satisfied {
                                                    error_message.set(None);
                                                    on_verified.call(next_requirements);
                                                } else {
                                                    error_message.set(Some(verification_failed_message));
                                                }
                                            }
                                            Err(err) => {
                                                toast.error(err);
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        toast.error(err);
                                    }
                                }
                            });
                        }
                    },
                    span { class: "font-bold text-[14px]/[16px] text-[#0A0A0A]", {tr.continue_label} }
                }
            }
        }
    }
}

fn requirement_label(
    attribute: crate::features::spaces::controllers::panel_requirements::PanelRequirementAttribute,
    tr: &ParticipationVerificationSectionTranslate,
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
    ParticipationVerificationSectionTranslate;

    title: {
        en: "Match Required Attributes",
        ko: "필수 속성 일치",
    },

    method_title: {
        en: "Choose a verification method",
        ko: "인증 방법 선택",
    },

    method_description: {
        en: "Select how you want to verify the required attributes for this space.",
        ko: "이 스페이스에 필요한 속성을 어떤 방식으로 인증할지 선택하세요.",
    },

    portone_title: {
        en: "Identity Verification",
        ko: "본인 인증",
    },

    portone_description: {
        en: "Use identity verification to import the required attributes into your credential.",
        ko: "본인 인증으로 필요한 속성을 Credential에 가져옵니다.",
    },

    import_title: {
        en: "Attributes to import from the method",
        ko: "이 방법으로 가져올 속성",
    },

    import_description: {
        en: "These attributes will be read during verification and added to your credential with your consent.",
        ko: "이 속성들은 인증 과정에서 읽혀지고, 동의 후 Credential에 추가됩니다.",
    },

    notice: {
        en: "You can remove or edit these attributes later from your credential settings.",
        ko: "이 속성들은 나중에 Credential 설정에서 수정하거나 제거할 수 있습니다.",
    },

    back: {
        en: "Back",
        ko: "뒤로",
    },

    continue_label: {
        en: "Continue",
        ko: "계속하기",
    },

    verification_failed: {
        en: "Some required attributes are still missing. Your credential has not been updated enough for this space yet.",
        ko: "일부 필수 속성이 아직 부족합니다. 현재 Credential로는 이 스페이스 조건을 아직 충족하지 못했습니다.",
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
