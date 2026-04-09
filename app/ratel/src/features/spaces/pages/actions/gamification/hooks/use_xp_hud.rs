//! V1 stub — uses per-space score as a proxy for global XP until Phase 6 ships the real `UserGlobalXp` read endpoint.

use super::*;

use crate::features::activity::controllers::get_my_score_handler;
use crate::features::spaces::space_common::hooks::use_space;
use crate::features::spaces::space_common::types::space_my_score_key;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct XpHudState {
    pub level: u32,
    pub xp: i64,
    pub xp_to_next: i64,
    pub streak_days: u32,
    pub combo_multiplier: f32,
}

impl XpHudState {
    fn from_score(total_score: i64) -> Self {
        let level = ((total_score / 1000).max(0) as f64).sqrt().floor() as u32 + 1;
        let next_level_threshold = (level as i64).saturating_mul(level as i64) * 1000;
        let xp_to_next = (next_level_threshold - total_score).max(0);

        Self {
            level,
            xp: total_score,
            xp_to_next,
            streak_days: 0,
            combo_multiplier: 1.0,
        }
    }
}

#[allow(clippy::result_large_err)]
pub fn use_xp_hud() -> XpHudState {
    let space = use_space()();
    let space_id = space.id;

    let key = space_my_score_key(&space_id);
    let loader = use_query(&key, {
        let space_id = space_id.clone();
        move || {
            let space_id = space_id.clone();
            async move { get_my_score_handler(space_id).await }
        }
    });

    match loader {
        Ok(loader) => XpHudState::from_score(loader().total_score),
        Err(_) => XpHudState::default(),
    }
}
