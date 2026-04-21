use crate::common::*;
use crate::features::spaces::pages::actions::types::SpaceActionStatus;

/// Decides whether an action's configuration is locked so creators should see
/// the participant view instead of the edit view.
///
/// Rules:
/// - `None` (legacy) or `Some(Designing)` → **not locked**. Creator can still
///   configure.
/// - `Some(Ongoing)` or `Some(Finish)` → **locked**. The action has been
///   published; creators see it read-only (participant mode).
/// - Space beyond `Ongoing` (`Processing`/`Finished`) → always locked.
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

/// Decides whether a user can execute (respond / comment / follow) an action
/// right now.
///
/// - `Creator`: can always interact as long as the space hasn't moved past
///   the participation phase (`Processing`/`Finished`).
/// - `Participant`: requires `Space.status == Ongoing`, action
///   `status == Some(Ongoing)`, and all dependencies met.
/// - `Candidate`: only for prerequisite actions during `Open` (or any Ongoing
///   action when `join_anytime` is enabled), and dependencies must be met.
/// - `Viewer`: never.
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
            space_ok && matches!(action_status, Some(SpaceActionStatus::Ongoing))
        }
        SpaceUserRole::Participant => {
            matches!(space_status, Some(SpaceStatus::Ongoing))
                && matches!(action_status, Some(SpaceActionStatus::Ongoing))
        }
        SpaceUserRole::Viewer => false,
    };

    role_ok && status_ok && dependencies_met
}
