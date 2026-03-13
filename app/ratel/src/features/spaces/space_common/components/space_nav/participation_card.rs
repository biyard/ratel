use super::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ParticipationTag {
    Warning,
    Success,
}

#[component]
pub fn ParticipationCard(
    space_id: SpacePartition,
    credential_path: Option<String>,
    on_login: EventHandler<()>,
) -> Element {
    let ctx = crate::features::spaces::space_common::providers::use_space_context();
    let tr: ParticipationCardTranslate = use_translate();
    let mut layover = use_layover();
    let navigator = use_navigator();
    let mut participate =
        use_action(crate::features::spaces::controllers::participate_space::participate_space);
    let mut space = ctx.space;
    let mut role = ctx.role;
    let mut current_role = ctx.current_role;
    let panel_requirements_query_key = vec![
        "Space".to_string(),
        space_id.to_string(),
        "PanelRequirements".to_string(),
    ];
    let panel_requirements_loader = use_query(&panel_requirements_query_key, {
        let space_id = space_id.clone();
        move || {
            crate::features::spaces::controllers::panel_requirements::get_panel_requirements(
                space_id.clone(),
            )
        }
    })?;
    let panel_requirements = panel_requirements_loader.read().clone();
    let all_requirements_satisfied = panel_requirements
        .iter()
        .all(|requirement| requirement.satisfied);
    let participate_credential_path = credential_path.clone();
    let credential_button_path = credential_path.clone();
    let login_for_participate = on_login.clone();
    let login_for_credentials = on_login;
    let layover_requirements = panel_requirements.clone();
    let layover_credential_path = credential_path.clone();
    let layover_login = login_for_participate.clone();

    let handle_participate = move |_| {
        let space_id = space_id.clone();

        if participate_credential_path.is_none() {
            login_for_participate.call(());
            return;
        }

        if !all_requirements_satisfied {
            layover
                .open(
                    "space-participation-requirements".to_string(),
                    String::new(),
                    rsx! {
                        ParticipationRequirementsLayover {
                            space_id: space_id.clone(),
                            requirements: layover_requirements.clone(),
                        }
                    },
                )
                .set_size(LayoverSize::Medium);
            return;
        }

        spawn(async move {
            let space_detail = crate::features::spaces::space_common::types::space_key(&space_id);
            let panel_requirements_key = vec![
                "Space".to_string(),
                space_id.to_string(),
                "PanelRequirements".to_string(),
            ];
            participate.call(space_id).await;
            current_role.set(SpaceUserRole::Participant);
            invalidate_query(&space_detail);
            invalidate_query(&panel_requirements_key);
            space.restart();
            role.restart();
        });
    };

    // let handle_open_credentials = move |_| {
    //     if let Some(path) = &credential_button_path {
    //         navigator.push(path.clone());
    //     } else {
    //         login_for_credentials.call(());
    //     }
    // };

    rsx! {
        div { class: "px-4 w-full",
            div { class: "flex flex-col gap-2.5 items-start py-4 px-3 w-full border rounded-[12px] border-primary bg-primary/5",
                div { class: "flex flex-col gap-2.5 items-start w-full",
                    div { class: "flex flex-col gap-1 items-start w-full",
                        p { class: "w-full font-bold text-white font-raleway text-[15px]/[18px] tracking-[-0.16px]",
                            {tr.title}
                        }
                        p { class: "w-full font-medium text-gray-300 font-raleway text-[13px]/[20px]",
                            {tr.description}
                        }
                    }

                    if !panel_requirements.is_empty() {
                        div { class: "flex flex-wrap gap-1 items-center",
                            for requirement in panel_requirements.iter() {
                                ParticipationRequirementTag {
                                    key: "{requirement.attribute:?}",
                                    kind: if requirement.satisfied { ParticipationTag::Success } else { ParticipationTag::Warning },
                                    label: panel_requirement_label(requirement.attribute, &tr),
                                }
                            }
                        }
                    }
                }

                div { class: "flex flex-col gap-2.5 items-start w-full",
                    Button {
                        class: "w-full",
                        style: ButtonStyle::Primary,
                        size: ButtonSize::Small,
                        onclick: handle_participate,
                        {tr.participate}
                    }
                                // Button {
                //     class: "w-full",
                //     style: ButtonStyle::Outline,
                //     size: ButtonSize::Small,
                //     onclick: handle_open_credentials,
                //     {tr.see_my_credential}
                // }
                }
            }
        }
    }
}

fn panel_requirement_label(
    attribute: crate::features::spaces::controllers::panel_requirements::PanelRequirementAttribute,
    tr: &ParticipationCardTranslate,
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

#[component]
fn ParticipationRequirementTag(kind: ParticipationTag, label: String) -> Element {
    let (container_class, icon_class, text_class) = match kind {
        ParticipationTag::Warning => (
            "border-red-500 bg-red-500/5",
            "border-red-500 text-red-500",
            "text-white",
        ),
        ParticipationTag::Success => (
            "border-green-500 bg-green-500/5",
            "border-green-500 text-green-500",
            "text-white",
        ),
    };

    rsx! {
        div { class: "flex justify-center items-center rounded-full border px-[8px] py-[5.5px] {container_class}",
            div { class: "flex gap-1 items-center",
                div { class: "flex justify-center items-center size-[14px] {icon_class}",
                    if kind == ParticipationTag::Warning {
                        crate::common::lucide_dioxus::CircleAlert { size: 15, class: "text-red-500" }
                    } else {
                        icons::validations::Check { class: "w-[15px] h-[15px] [&>path]:stroke-green-500" }
                    }
                }
                span { class: "font-bold font-raleway text-[12px]/[14px] tracking-[-0.12px] {text_class}",
                    {label}
                }
            }
        }
    }
}

translate! {
    ParticipationCardTranslate;

    title: {
        en: "Participation",
        ko: "참여",
    },

    description: {
        en: "You can read everything, but posting, voting and commenting require verification.",
        ko: "모든 내용을 읽을 수 있지만, 게시, 투표, 댓글은 검증이 필요합니다.",
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

    participate: {
        en: "Participate",
        ko: "참여하기",
    },

    see_my_credential: {
        en: "See My Credential",
        ko: "내 Credential 보기",
    },
}
