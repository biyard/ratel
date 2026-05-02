//! Character XP / Level / Skill-Point math. Single source of truth.
//!
//! All formulas locked by the spec at `roadmap/character-xp-skills.md` and
//! the design doc at `docs/superpowers/specs/2026-05-01-character-xp-skills-design.md`.

/// XP curve scale. `xp_required(L→L+1) = round(C · L²)`.
pub const C: i64 = 220;

/// Skill points granted per character level.
pub const SP_PER_LEVEL: i32 = 5;

/// Maximum level any one skill can reach.
pub const MAX_SKILL_LEVEL: i32 = 10;

/// Skill cost from level n to n+1 is `5 + 4·n`.
pub const SKILL_COST_BASE: i32 = 5;
pub const SKILL_COST_SLOPE: i32 = 4;

/// Effect multiplier per skill level, expressed as a per-mille step
/// (5% = 50‰). Cap at level 10 is 500‰ → multiplier 1500‰ (1.5×).
pub const MULTIPLIER_PER_LEVEL_PERMILLE: i32 = 50;

/// Cumulative XP required to reach character level `L` from level 1.
/// Closed form: `C · (L−1) · L · (2L−1) / 6`.
pub fn cumulative_xp_at_level(level: i32) -> i64 {
    if level <= 1 {
        return 0;
    }
    let l = level as i64;
    C * (l - 1) * l * (2 * l - 1) / 6
}

/// Derive character level from cumulative XP. Levels start at 1.
/// Linear search is fine — character level is bounded in practice (<200).
pub fn level_from_xp(total_xp: i64) -> i32 {
    let mut l: i32 = 1;
    while cumulative_xp_at_level(l + 1) <= total_xp {
        l += 1;
        if l > 1_000 {
            // Safety bound; should never fire under realistic XP.
            break;
        }
    }
    l
}

/// Total skill points granted at character level `L`.
pub fn total_sp_granted(level: i32) -> i32 {
    SP_PER_LEVEL * level
}

/// Cost to advance a skill from `current_level` (0..MAX) to `current_level + 1`.
/// Returns `None` if already at max.
pub fn skill_cost_next(current_level: i32) -> Option<i32> {
    if current_level >= MAX_SKILL_LEVEL {
        None
    } else {
        Some(SKILL_COST_BASE + SKILL_COST_SLOPE * current_level)
    }
}

/// Effect multiplier as a per-mille integer (1000 = 1.0×, 1500 = 1.5×).
/// Use as `apply_permille(amount, multiplier_permille(skill_level))`.
pub fn multiplier_permille(skill_level: i32) -> i32 {
    1000 + MULTIPLIER_PER_LEVEL_PERMILLE * skill_level
}

/// Apply a per-mille multiplier to an `i64` amount, rounding to nearest.
pub fn apply_permille(amount: i64, permille: i32) -> i64 {
    (amount * permille as i64 + 500) / 1000
}
