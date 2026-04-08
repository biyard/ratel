use crate::common::*;

/// Decides whether an action's configuration is locked (and therefore
/// creators should be routed to the participant view rather than the
/// creator page).
///
/// Rules:
/// - **Designing / Open / None** → not locked. The space hasn't been
///   launched yet, so creators keep seeing the configuration UI.
/// - **Ongoing** → locked once `action.started_at` is in the past.
///   Before the action window opens, creators can still reconfigure.
/// - **Processing / Finished** → locked. The entire space is beyond the
///   participation window; creators see the participant view read-only.
pub fn is_action_locked(space_status: Option<SpaceStatus>, action_started_at: i64) -> bool {
    let now = crate::common::utils::time::get_now_timestamp_millis();
    match space_status {
        Some(SpaceStatus::Ongoing | SpaceStatus::Open) => now >= action_started_at,
        Some(SpaceStatus::Processing | SpaceStatus::Finished) => true,
        Some(SpaceStatus::Designing) | None => false,
    }
}

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
        // Creators can interact at any phase except once the space
        // has ended — at that point they behave like viewers
        // (read-only) for polls, quizzes, discussions, follows, etc.
        SpaceUserRole::Creator => !matches!(
            status,
            Some(SpaceStatus::Processing | SpaceStatus::Finished)
        ),
        SpaceUserRole::Candidate => {
            matches!(status, Some(SpaceStatus::Open))
                || (join_anytime && matches!(status, Some(SpaceStatus::Ongoing)))
        }
        SpaceUserRole::Participant => matches!(status, Some(SpaceStatus::Ongoing)),
        SpaceUserRole::Viewer => false,
    };

    can_execute_role && can_execute_status
}
