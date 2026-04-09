use crate::common::*;

/// What the user receives upon completing every action in a chapter.
///
/// Chapter completion benefits are **forward-only** at the type level: no
/// `RoleDowngrade` variant exists. Role progression in the Quest Map moves
/// from `Viewer` → `Candidate` → `Participant` and never back.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
#[serde(tag = "type", content = "value")]
pub enum ChapterBenefit {
    /// Upgrade the user to this role upon chapter completion. No XP award
    /// beyond what the individual quests already paid out.
    RoleUpgradeTo(SpaceUserRole),

    /// Upgrade the user to this role AND credit the chapter completion XP
    /// bonus on top of each quest reward.
    RoleUpgradeAndXp(SpaceUserRole),

    /// No role change; next chapter simply unlocks for the user.
    XpOnly,
}

impl Default for ChapterBenefit {
    fn default() -> Self {
        Self::XpOnly
    }
}
