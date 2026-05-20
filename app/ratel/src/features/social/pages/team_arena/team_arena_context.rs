use crate::common::*;
use crate::features::social::pages::team_arena::topbar::TeamArenaTab;
use crate::social::controllers::Wall;
use crate::social::hooks::UseWallContext;

/// Context exposed by `TeamArenaLayout` so child pages can highlight the correct
/// HUD tab and read resolved team data without re-fetching it.
#[derive(Clone, Copy, DioxusController)]
pub struct TeamArenaContext {
    pub active_tab: Signal<TeamArenaTab>,
    pub username: Signal<String>,
    pub display_name: Signal<String>,
    pub profile_url: Signal<String>,
    pub can_edit: Signal<bool>,
    pub is_admin: Signal<bool>,
    pub is_member: Signal<bool>,
    /// Bump this signal to force the layout to refetch the team profile
    /// (used by the settings page after Save Changes so the topbar reflects
    /// the new name/logo immediately).
    pub refresh_trigger: Signal<u32>,
    pub role: Signal<Option<crate::social::pages::member::dto::TeamRole>>,
    pub following: Signal<bool>,
    pub team_id: Signal<TeamPartition>,
    pub description: Signal<String>,
    pub created_at: Signal<i64>,
    /// Whether the team has flipped its sub-team activation switch ON.
    pub is_parent_eligible: Signal<bool>,
    pub min_sub_team_members: Signal<i32>,
    pub min_sub_team_age_days: Signal<i32>,
    /// `Some(applicant_username)` if the viewer is admin/owner of some
    /// team that has an in-flight application to this wall team —
    /// used by the topbar to route the sub-team HUD icon to the
    /// status page instead of the apply page.
    pub viewer_pending_applicant_username: Signal<Option<String>>,
    /// True iff the viewer admins at least one team that is still
    /// independent (no parent / no pending application). Drives the
    /// apply-icon visibility on unrelated parent-eligible teams.
    pub viewer_has_eligible_applicant_team: Signal<bool>,
}

impl From<UseWallContext> for TeamArenaContext {
    fn from(wall_ctx: UseWallContext) -> Self {
        match wall_ctx.data() {
            Wall::Team {
                id,
                username,
                display_name,
                profile_url,
                role,
                following,
                description,
                created_at,
                is_parent_eligible,
                min_sub_team_members,
                min_sub_team_age_days,
                viewer_pending_applicant_username,
                viewer_has_eligible_applicant_team,
                ..
            } => {
                let (can_edit, is_admin, is_member) = if let Some(role) = role {
                    (role.is_admin_or_owner(), role.is_admin_or_owner(), true)
                } else {
                    (false, false, false)
                };

                TeamArenaContext {
                    active_tab: Signal::new(TeamArenaTab::Home),
                    username: Signal::new(username),
                    display_name: Signal::new(display_name),
                    profile_url: Signal::new(profile_url),
                    can_edit: Signal::new(can_edit),
                    is_admin: Signal::new(is_admin),
                    is_member: Signal::new(is_member),
                    refresh_trigger: Signal::new(0),
                    role: Signal::new(role),
                    following: Signal::new(following),
                    team_id: Signal::new(id),
                    description: Signal::new(description),
                    created_at: Signal::new(created_at),
                    is_parent_eligible: Signal::new(is_parent_eligible),
                    min_sub_team_members: Signal::new(min_sub_team_members),
                    min_sub_team_age_days: Signal::new(min_sub_team_age_days),
                    viewer_pending_applicant_username: Signal::new(
                        viewer_pending_applicant_username,
                    ),
                    viewer_has_eligible_applicant_team: Signal::new(
                        viewer_has_eligible_applicant_team,
                    ),
                }
            }
            _ => panic!("Wall context is not a team"),
        }
    }
}

pub fn use_team_arena() -> TeamArenaContext {
    use_context::<TeamArenaContext>()
}
