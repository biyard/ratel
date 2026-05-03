use serde::{Deserialize, Serialize};

/// Stable string-keyed skill identifier. The serialization (`snake_case`) is
/// what gets stored in `EntityType::CharacterSkill(...)`, sent over the wire,
/// and used as the path parameter on `/api/me/skills/:skill_id/level-up`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, Default)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum SkillId {
    #[default]
    MoneyTree,
    Ranker,
    /// v2 — declared so the data model can store them, but the level-up
    /// endpoint rejects any non-MVP id until the v2 spec ships.
    Influencer,
    Sweeper,
}

impl SkillId {
    pub fn as_str(&self) -> &'static str {
        match self {
            SkillId::MoneyTree => "money_tree",
            SkillId::Ranker => "ranker",
            SkillId::Influencer => "influencer",
            SkillId::Sweeper => "sweeper",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "money_tree" => Some(Self::MoneyTree),
            "ranker" => Some(Self::Ranker),
            "influencer" => Some(Self::Influencer),
            "sweeper" => Some(Self::Sweeper),
            _ => None,
        }
    }

    /// MVP skills the level-up endpoint accepts.
    pub fn is_mvp(&self) -> bool {
        matches!(self, SkillId::MoneyTree | SkillId::Ranker)
    }
}
