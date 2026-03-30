use crate::common::*;

pub fn can_execute_space_action(
    role: SpaceUserRole,
    prerequisite: bool,
    status: Option<SpaceStatus>,
    join_anytime: bool,
) -> bool {
    let can_execute_role = match role {
        SpaceUserRole::Creator => true,
        SpaceUserRole::Candidate => prerequisite,
        SpaceUserRole::Participant => !prerequisite,
        SpaceUserRole::Viewer => false,
    };

    let can_execute_status = match role {
        SpaceUserRole::Creator => true,
        SpaceUserRole::Candidate => {
            matches!(status, Some(SpaceStatus::InProgress))
                || (join_anytime && matches!(status, Some(SpaceStatus::Started)))
        }
        SpaceUserRole::Participant => matches!(status, Some(SpaceStatus::Started)),
        SpaceUserRole::Viewer => false,
    };

    can_execute_role && can_execute_status
}
