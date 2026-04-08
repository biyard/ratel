use super::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ParticipationTag {
    Warning,
    Success,
}

#[component]
pub fn ParticipationCard(
    space_id: ReadSignal<SpacePartition>,
    credential_path: Option<String>,
    on_login: EventHandler<()>,
) -> Element {
    let mut query = use_query_store();
    let ctx = crate::features::spaces::space_common::providers::use_space_context();
    let tr: ParticipationCardTranslate = use_translate();
    let mut layover = use_layover();
    let navigator = use_navigator();
    let mut space = ctx.space;
    let mut role = ctx.role;
    let mut current_role = ctx.current_role;
    let panel_requirements_query_key = vec![
        "Space".to_string(),
        space_id().to_string(),
        "PanelRequirements".to_string(),
    ];
    // Use use_server_future instead of use_query to avoid Suspense panics on
    // unmount when navigating away from a Space as a Viewer.
    let panel_requirements_resource = use_server_future({
        move || async move {
            crate::features::spaces::controllers::panel_requirements::get_panel_requirements(
                space_id(),
            )
            .await
        }
    })?;
    let panel_requirements = panel_requirements_resource
        .read()
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .cloned()
        .unwrap_or_default();
    let participate_credential_path = credential_path.clone();
    let credential_button_path = credential_path.clone();
    let login_for_participate = on_login.clone();
    let login_for_credentials = on_login;
    let layover_requirements = panel_requirements.clone();
    let layover_credential_path = credential_path.clone();
    let layover_login = login_for_participate.clone();

    // Closure invoked by the layover's "Join Space" button after the
    // user explicitly checks the consent checkbox in the
    // See your Difference step. Persists the consent flag onto the
    // SpaceParticipant record via `participate_space`.
    let on_join_after_consent = {
        let panel_requirements_query_key = panel_requirements_query_key.clone();
        move |_| {
            let panel_requirements_query_key = panel_requirements_query_key.clone();
            spawn(async move {
                let space_detail =
                    crate::features::spaces::space_common::types::space_key(&space_id());
                let panel_requirements_key = panel_requirements_query_key.clone();
                let req = crate::features::spaces::controllers::participate_space::ParticipateSpaceRequest {
                    informed_agreed: true,
                };
                if crate::features::spaces::controllers::participate_space::participate_space(
                    space_id(),
                    req,
                )
                .await
                .is_ok()
                {
                    let prerequisite_actions = if let Ok(actions) =
                        crate::features::spaces::pages::actions::controllers::list_actions(
                            space_id(),
                        )
                        .await
                    {
                        let filtered: Vec<
                            crate::features::spaces::pages::actions::types::SpaceActionSummary,
                        > = actions.into_iter().filter(|a| a.prerequisite).collect();
                        filtered
                    } else {
                        vec![]
                    };

                    let mut layover = layover;
                    // Close the join layover before swapping role / opening
                    // the prerequisite actions layover.
                    layover.close();

                    if !prerequisite_actions.is_empty() {
                        layover
                            .open(
                                "space-prerequisite-actions".to_string(),
                                String::new(),
                                rsx! {
                                    PrerequisiteActionsLayover {
                                        space_id: space_id(),
                                        actions: prerequisite_actions,
                                    }
                                },
                            )
                            .set_size(LayoverSize::Medium);
                    }

                    if let Ok(next_role) =
                        crate::features::spaces::space_common::controllers::get_user_role(
                            space_id(),
                        )
                        .await
                    {
                        current_role.set(next_role);
                    }

                    query.invalidate(&space_detail);
                    query.invalidate(&panel_requirements_key);
                    space.restart();
                    role.restart();
                }
            });
        }
    };

    let handle_participate = move |_| {
        if participate_credential_path.is_none() {
            login_for_participate.call(());
            return;
        }

        // Always open the join layover. The "See your Difference" step
        // either shows the consent checkbox + Join button (when all
        // requirements are satisfied) or routes the user through the
        // "Improve My Credential" verification flow.
        let mut refresh_panel_requirements = panel_requirements_resource;
        let panel_requirements_query_key_for_verified = panel_requirements_query_key.clone();
        let on_verified_refresh = move |_| {
            query.invalidate(&panel_requirements_query_key_for_verified);
            refresh_panel_requirements.restart();
            space.restart();
            role.restart();
        };
        let mut refresh_panel_requirements = panel_requirements_resource;
        let panel_requirements_query_key_for_completed = panel_requirements_query_key.clone();
        let on_completed = move |_| {
            query.invalidate(&panel_requirements_query_key_for_completed);
            refresh_panel_requirements.restart();
            space.restart();
            role.restart();
        };
        let on_join = on_join_after_consent.clone();
        layover
            .open(
                "space-participation-requirements".to_string(),
                String::new(),
                rsx! {
                    ParticipationRequirementsLayover {
                        space_id: space_id(),
                        requirements: layover_requirements.clone(),
                        on_verified_refresh,
                        on_completed,
                        on_join,
                    }
                },
            )
            .set_size(LayoverSize::Medium);
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
                        p { class: "w-full font-bold text-text-primary font-raleway text-[15px]/[18px] tracking-[-0.16px]",
                            {tr.title}
                        }
                        p { class: "w-full font-medium text-text-secondary font-raleway text-[13px]/[20px]",
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
            "text-text-primary",
        ),
        ParticipationTag::Success => (
            "border-green-500 bg-green-500/5",
            "border-green-500 text-green-500",
            "text-text-primary",
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
