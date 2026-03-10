use super::*;

#[component]
pub fn ParticipationRequirementsLayover(
    space_id: SpacePartition,
    requirements: Vec<
        crate::features::spaces::controllers::panel_requirements::PanelRequirementStatus,
    >,
) -> Element {
    let mut current_step = use_signal(|| ParticipationLayoverStep::SeeYourDifference);
    let mut current_requirements = use_signal(|| requirements);

    let handle_continue = move |_| {
        current_step.set(ParticipationLayoverStep::MatchRequiredAttributes);
    };
    let handle_back = move |_| {
        current_step.set(ParticipationLayoverStep::SeeYourDifference);
    };
    let handle_verified = move |next_requirements| {
        current_requirements.set(next_requirements);
        current_step.set(ParticipationLayoverStep::CreateCredential);
    };

    rsx! {
        div { class: "flex h-full w-full flex-col bg-[#1A1A1A] text-web-font-primary",
            ParticipationLayoverHeader { title: "Join Space".to_string() }
            ParticipationStepBar { current_step: current_step() }

            match current_step() {
                ParticipationLayoverStep::SeeYourDifference => rsx! {
                    ParticipationAttributesSection {
                        requirements: current_requirements(),
                        current_step: current_step(),
                        on_continue: handle_continue,
                    }
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
                    ParticipationCredentialSection { requirements: current_requirements() }
                },
            }
        }
    }
}
