use crate::common::*;
use crate::features::spaces::pages::actions::types::SpaceActionStatus;

pub fn is_action_locked(
    space_status: Option<SpaceStatus>,
    action_status: Option<&SpaceActionStatus>,
) -> bool {
    if matches!(
        space_status,
        Some(SpaceStatus::Processing | SpaceStatus::Finished)
    ) {
        return true;
    }
    matches!(
        action_status,
        Some(SpaceActionStatus::Ongoing | SpaceActionStatus::Finish)
    )
}

pub fn can_execute_space_action(
    role: SpaceUserRole,
    prerequisite: bool,
    space_status: Option<SpaceStatus>,
    action_status: Option<&SpaceActionStatus>,
    dependencies_met: bool,
    join_anytime: bool,
) -> bool {
    let role_ok = match role {
        SpaceUserRole::Creator => true,
        SpaceUserRole::Candidate => prerequisite,
        SpaceUserRole::Participant => !prerequisite,
        SpaceUserRole::Viewer => false,
    };

    let status_ok = match role {
        SpaceUserRole::Creator => !matches!(
            space_status,
            Some(SpaceStatus::Processing | SpaceStatus::Finished)
        ),
        SpaceUserRole::Candidate => {
            let space_ok = matches!(space_status, Some(SpaceStatus::Open))
                || (join_anytime && matches!(space_status, Some(SpaceStatus::Ongoing)));
            // Candidates only reach prerequisite actions (enforced by role_ok),
            // and those must be runnable regardless of the action's Designing/
            // Ongoing state — completing them is what clears Candidate → Participant.
            space_ok && (prerequisite || matches!(action_status, Some(SpaceActionStatus::Ongoing)))
        }
        SpaceUserRole::Participant => {
            matches!(space_status, Some(SpaceStatus::Ongoing))
                && matches!(action_status, Some(SpaceActionStatus::Ongoing))
        }
        SpaceUserRole::Viewer => false,
    };

    role_ok && status_ok && dependencies_met
}
