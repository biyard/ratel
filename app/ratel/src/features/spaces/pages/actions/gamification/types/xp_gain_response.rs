use crate::common::*;

/// Response returned by every action-submission endpoint (poll / quiz /
/// discussion first-comment / follow first-target) after `award_xp`
/// finishes running. The client uses this payload to drive the
/// Completion Overlay animation sequence in Phase 6.
///
/// All fields are populated from a single call to `award_xp` against
/// the user's submission, so they are internally consistent: the combo
/// and streak here reflect the exact multipliers already applied to
/// `xp_earned`, and `new_level` is the level the user holds after the
/// award was committed.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct XpGainResponse {
    pub xp_earned: i64,
    pub base_points: i64,
    pub participants_snapshot: u32,
    pub combo_multiplier: f32,
    pub streak_multiplier: f32,

    pub old_level: u32,
    pub new_level: u32,

    /// Actions that became reachable because of this submission — i.e.
    /// DAG children whose other parents were already complete and which
    /// sat in the same chapter as the just-completed action.
    #[serde(default)]
    pub unlocked_actions: Vec<String>,

    /// True if completing this action also completed the chapter the
    /// action belonged to.
    #[serde(default)]
    pub chapter_completed: bool,

    /// `Some(new_role)` if the chapter completion triggered a role
    /// upgrade via `ChapterBenefit::RoleUpgradeTo` or
    /// `RoleUpgradeAndXp`; `None` otherwise.
    #[serde(default)]
    pub role_upgraded: Option<SpaceUserRole>,
}
