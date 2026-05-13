use crate::common::*;
use crate::features::character::leveling;
use crate::features::character::models::CharacterXp;
use crate::features::character::types::SkillId;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct CharacterResponse {
    pub total_xp: i64,
    pub level: i32,
    /// Cumulative XP threshold of the next level.
    pub xp_to_next_level: i64,
    /// `total_xp - cumulative_xp_at_level(level)`.
    pub xp_progress_in_level: i64,
    /// `xp_to_next_level - cumulative_xp_at_level(level)`.
    pub xp_span_of_level: i64,
    pub unspent_sp: i32,
    pub total_sp_granted: i32,
    pub total_sp_spent: i32,
    pub skills: Vec<CharacterSkillResponse>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct CharacterSkillResponse {
    pub skill_id: SkillId,
    pub level: i32,
    pub max_level: i32,
    /// `None` when at max level.
    pub next_level_cost: Option<i32>,
    /// 1000 = 1.0×, 1500 = 1.5×.
    pub multiplier_permille: i32,
    /// Whether this skill is part of the MVP set; non-MVP skills appear in
    /// the response with level=0 and `next_level_cost=None`.
    pub is_released: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct PublicCharacterResponse {
    /// Only level is exposed publicly (per spec Q5).
    pub level: i32,
}

impl CharacterResponse {
    pub fn from_parts(xp: &CharacterXp, skills: Vec<(SkillId, i32)>) -> Self {
        let cur_threshold = leveling::cumulative_xp_at_level(xp.level);
        let next_threshold = leveling::cumulative_xp_at_level(xp.level + 1);

        let mvp = [SkillId::MoneyTree, SkillId::Ranker];
        let v2 = [SkillId::Influencer, SkillId::Sweeper];

        let level_for = |id: SkillId| -> i32 {
            skills
                .iter()
                .find(|(s, _)| *s == id)
                .map(|(_, l)| *l)
                .unwrap_or(0)
        };

        let mut response_skills = Vec::with_capacity(4);
        for id in mvp.iter().copied() {
            let lv = level_for(id);
            response_skills.push(CharacterSkillResponse {
                skill_id: id,
                level: lv,
                max_level: leveling::MAX_SKILL_LEVEL,
                next_level_cost: leveling::skill_cost_next(lv),
                multiplier_permille: leveling::multiplier_permille(lv),
                is_released: true,
            });
        }
        for id in v2.iter().copied() {
            response_skills.push(CharacterSkillResponse {
                skill_id: id,
                level: 0,
                max_level: leveling::MAX_SKILL_LEVEL,
                next_level_cost: None,
                multiplier_permille: 1000,
                is_released: false,
            });
        }

        Self {
            total_xp: xp.total_xp,
            level: xp.level,
            xp_to_next_level: next_threshold,
            xp_progress_in_level: xp.total_xp - cur_threshold,
            xp_span_of_level: next_threshold - cur_threshold,
            unspent_sp: xp.unspent_sp(),
            total_sp_granted: xp.total_sp_granted,
            total_sp_spent: xp.total_sp_spent,
            skills: response_skills,
        }
    }
}
