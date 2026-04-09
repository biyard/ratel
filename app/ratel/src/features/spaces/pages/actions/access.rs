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

/// Phase-4 access check: determines whether a user may execute (submit)
/// an action given their role, the chapter that owns the action, DAG
/// dependency status, and the prior-chapter gate.
///
/// # Arguments
/// * `role` – the requesting user's role in the space.
/// * `chapter` – the `SpaceChapter` that owns this action.
/// * `action_deps_met` – `true` when every DAG parent action is `Cleared`.
/// * `prior_chapters_complete` – `true` when every chapter with a lower
///   `order` is fully cleared by this user.
/// * `status` – current `SpaceStatus`.
/// * `join_anytime` – whether late-joining is allowed.
pub fn can_execute_space_action(
    role: SpaceUserRole,
    chapter: &crate::features::spaces::pages::actions::gamification::models::SpaceChapter,
    action_deps_met: bool,
    prior_chapters_complete: bool,
    status: Option<SpaceStatus>,
    join_anytime: bool,
) -> bool {
    let role_ok = match role {
        SpaceUserRole::Creator => true,
        _ => {
            role_meets_chapter_requirement(role, chapter.actor_role)
                && action_deps_met
                && prior_chapters_complete
        }
    };

    let status_ok = match role {
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

    role_ok && status_ok
}

/// Returns a default `SpaceChapter` stub for legacy actions that have no
/// `chapter_id` yet.  The stub's `actor_role` is set to the caller's own
/// role so the role gate always passes for non-Viewer roles.
pub fn default_chapter_for_legacy_action(
    role: SpaceUserRole,
) -> crate::features::spaces::pages::actions::gamification::models::SpaceChapter {
    crate::features::spaces::pages::actions::gamification::models::SpaceChapter {
        actor_role: role,
        ..Default::default()
    }
}

/// Helper: check whether `role` meets or exceeds `required`.
fn role_meets_chapter_requirement(role: SpaceUserRole, required: SpaceUserRole) -> bool {
    match (role, required) {
        (SpaceUserRole::Creator, _) => true,
        (
            SpaceUserRole::Participant,
            SpaceUserRole::Participant | SpaceUserRole::Candidate | SpaceUserRole::Viewer,
        ) => true,
        (SpaceUserRole::Candidate, SpaceUserRole::Candidate | SpaceUserRole::Viewer) => true,
        (SpaceUserRole::Viewer, SpaceUserRole::Viewer) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::spaces::pages::actions::gamification::models::SpaceChapter;

    fn chapter(actor_role: SpaceUserRole) -> SpaceChapter {
        SpaceChapter {
            actor_role,
            ..Default::default()
        }
    }

    // ── Role-match tests ──────────────────────────────────────────────────────

    #[test]
    fn creator_always_allowed_if_space_not_finished() {
        let c = chapter(SpaceUserRole::Participant);
        assert!(can_execute_space_action(
            SpaceUserRole::Creator,
            &c,
            false, // deps NOT met — creator bypasses
            false, // prior chapters NOT complete — creator bypasses
            Some(SpaceStatus::Ongoing),
            false,
        ));
    }

    #[test]
    fn creator_blocked_when_finished() {
        let c = chapter(SpaceUserRole::Participant);
        assert!(!can_execute_space_action(
            SpaceUserRole::Creator,
            &c,
            true,
            true,
            Some(SpaceStatus::Finished),
            false,
        ));
    }

    #[test]
    fn participant_meets_participant_chapter() {
        let c = chapter(SpaceUserRole::Participant);
        assert!(can_execute_space_action(
            SpaceUserRole::Participant,
            &c,
            true,
            true,
            Some(SpaceStatus::Ongoing),
            false,
        ));
    }

    #[test]
    fn candidate_blocked_from_participant_chapter() {
        let c = chapter(SpaceUserRole::Participant);
        assert!(!can_execute_space_action(
            SpaceUserRole::Candidate,
            &c,
            true,
            true,
            Some(SpaceStatus::Ongoing),
            false,
        ));
    }

    // ── DAG gate ──────────────────────────────────────────────────────────────

    #[test]
    fn participant_blocked_when_deps_not_met() {
        let c = chapter(SpaceUserRole::Participant);
        assert!(!can_execute_space_action(
            SpaceUserRole::Participant,
            &c,
            false, // deps NOT met
            true,
            Some(SpaceStatus::Ongoing),
            false,
        ));
    }

    // ── Prior-chapter gate ────────────────────────────────────────────────────

    #[test]
    fn participant_blocked_when_prior_chapter_not_complete() {
        let c = chapter(SpaceUserRole::Participant);
        assert!(!can_execute_space_action(
            SpaceUserRole::Participant,
            &c,
            true,
            false, // prior chapters NOT complete
            Some(SpaceStatus::Ongoing),
            false,
        ));
    }

    // ── default_chapter_for_legacy_action ────────────────────────────────────

    #[test]
    fn default_chapter_passes_role_gate_for_same_role() {
        let c = default_chapter_for_legacy_action(SpaceUserRole::Participant);
        assert_eq!(c.actor_role, SpaceUserRole::Participant);
        // Participant + default chapter + Ongoing → allowed
        assert!(can_execute_space_action(
            SpaceUserRole::Participant,
            &c,
            true,
            true,
            Some(SpaceStatus::Ongoing),
            false,
        ));
    }

    #[test]
    fn default_chapter_passes_for_creator() {
        let c = default_chapter_for_legacy_action(SpaceUserRole::Creator);
        assert!(can_execute_space_action(
            SpaceUserRole::Creator,
            &c,
            true,
            true,
            Some(SpaceStatus::Ongoing),
            false,
        ));
    }
}
