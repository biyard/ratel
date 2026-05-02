use crate::features::character::leveling::*;

#[test]
fn cumulative_xp_known_values() {
    assert_eq!(cumulative_xp_at_level(1), 0);
    // L2: 220 · 1·2·3 / 6 = 220
    assert_eq!(cumulative_xp_at_level(2), 220);
    // L5: 220 · 4·5·9 / 6 = 6_600
    assert_eq!(cumulative_xp_at_level(5), 6_600);
    // L10: 220 · 9·10·19 / 6 = 62_700
    assert_eq!(cumulative_xp_at_level(10), 62_700);
    // L46: 220 · 45·46·91 / 6 = 6_906_900
    assert_eq!(cumulative_xp_at_level(46), 6_906_900);
}

#[test]
fn level_from_xp_boundaries() {
    assert_eq!(level_from_xp(0), 1);
    assert_eq!(level_from_xp(219), 1);
    assert_eq!(level_from_xp(220), 2);
    assert_eq!(level_from_xp(6_599), 4);
    assert_eq!(level_from_xp(6_600), 5);
    assert_eq!(level_from_xp(6_906_900), 46);
}

#[test]
fn sp_granted_linear() {
    assert_eq!(total_sp_granted(1), 5);
    assert_eq!(total_sp_granted(10), 50);
    assert_eq!(total_sp_granted(46), 230);
}

#[test]
fn skill_cost_curve() {
    assert_eq!(skill_cost_next(0), Some(5));
    assert_eq!(skill_cost_next(1), Some(9));
    assert_eq!(skill_cost_next(2), Some(13));
    assert_eq!(skill_cost_next(3), Some(17));
    assert_eq!(skill_cost_next(4), Some(21));
    assert_eq!(skill_cost_next(5), Some(25));
    assert_eq!(skill_cost_next(6), Some(29));
    assert_eq!(skill_cost_next(7), Some(33));
    assert_eq!(skill_cost_next(8), Some(37));
    assert_eq!(skill_cost_next(9), Some(41));
    assert_eq!(skill_cost_next(10), None);

    // Total cost to max: 5+9+13+17+21+25+29+33+37+41 = 230
    let total: i32 = (0..MAX_SKILL_LEVEL).map(|n| skill_cost_next(n).unwrap()).sum();
    assert_eq!(total, 230);
}

#[test]
fn multiplier_curve() {
    assert_eq!(multiplier_permille(0), 1000);
    assert_eq!(multiplier_permille(1), 1050);
    assert_eq!(multiplier_permille(5), 1250);
    assert_eq!(multiplier_permille(10), 1500); // +50% at max = 1.5×
}

#[test]
fn apply_permille_rounding() {
    // 10_000 × 1.20 = 12_000
    assert_eq!(apply_permille(10_000, 1200), 12_000);
    // 10_000 × 1.05 = 10_500
    assert_eq!(apply_permille(10_000, 1050), 10_500);
    // 7 × 1.05 = 7.35 → rounds to 7
    assert_eq!(apply_permille(7, 1050), 7);
    // 9 × 1.05 = 9.45 → rounds to 9
    assert_eq!(apply_permille(9, 1050), 9);
}
