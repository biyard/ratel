use super::*;

translate! {
    ParticipationRequirementsLayoverTranslate;

    join_space: {
        en: "Join Space",
        ko: "스페이스 참여",
    },
}

#[component]
pub fn ParticipationRequirementsLayover(
    space_id: SpacePartition,
    requirements: Vec<
        crate::features::spaces::controllers::panel_requirements::PanelRequirementStatus,
    >,
    on_verified_refresh: EventHandler<()>,
    /// Called when the user has acknowledged the consent checkbox in
    /// the See your Difference step and wants to actually join the
    /// space. The handler should run `participate_space` and close the
    /// layover.
    on_join: EventHandler<()>,
) -> Element {
    let tr: ParticipationRequirementsLayoverTranslate = use_translate();
    let mut current_requirements = use_signal(|| requirements);
    let all_satisfied = use_memo(move || {
        let requirements = current_requirements();

        let has_missing = requirements
            .iter()
            .any(|requirement| !requirement.satisfied);
        (!requirements.is_empty() && !has_missing) || requirements.is_empty()
    });

    let mut current_step = use_signal(move || {
        if all_satisfied() {
            ParticipationLayoverStep::ConsentParticipate
        } else {
            ParticipationLayoverStep::SeeYourDifference
        }
    });

    let mut handle_continue = move |_| {
        if let Some(next) = current_step().next() {
            current_step.set(next)
        }
    };

    let handle_back = move |_| {
        if let Some(back) = current_step().back() {
            current_step.set(back)
        }
    };
    let handle_verified = move |next_requirements| {
        current_requirements.set(next_requirements);
        current_step.set(ParticipationLayoverStep::CreateCredential);
        on_verified_refresh.call(());
    };

    rsx! {
        div { class: "flex flex-col w-full h-full bg-modal-card-bg text-web-font-primary",
            ParticipationLayoverHeader { title: tr.join_space }
            if current_step() != ParticipationLayoverStep::ConsentParticipate {
                ParticipationStepBar { current_step: current_step() }
            }

            match current_step() {
                ParticipationLayoverStep::SeeYourDifference => rsx! {
                    ParticipationAttributesSection { requirements: current_requirements(), on_continue: handle_continue }
                },
                ParticipationLayoverStep::MatchRequiredAttributes => rsx! {
                    ParticipationVerificationSection {
                        space_id: space_id.clone(),
                        requirements: current_requirements(),
                        on_back: handle_back,
                        on_verified: handle_verified,
                    }
                },
                ParticipationLayoverStep::CreateCredential => rsx! {
                    ParticipationCredentialSection { requirements: current_requirements(), on_completed: handle_continue }
                },
                ParticipationLayoverStep::ConsentParticipate => rsx! {
                    ParticipationConsentSection { requirements: current_requirements(), on_join }
                },
            }
        }
    }
}
