use super::RewardBreakdownChipTranslate;
use crate::*;

/// Money Tree reward breakdown chip — Variant A (inline split-row).
///
/// Renders a small breakdown row showing `base + Money Tree boost = total`
/// for a reward transaction. Class names mirror
/// `app/ratel/assets/design/character-xp-skills/reward-breakdown.html`
/// (`.reward-tx__breakdown`, `.breakdown__base`, `.mt-chip`,
/// `.breakdown__total`) so the rules in `app/ratel/assets/main.css` style
/// it identically to the mockup.
///
/// The component renders nothing when `level == 0` or `bonus == 0`. This
/// makes it safe to drop into any reward row regardless of whether the
/// underlying transaction was actually boosted.
///
/// NOTE (2026-05-02): Currently this component is intentionally NOT wired
/// into the user_reward views. The transaction DTO
/// (`crate::common::services::PointTransactionResponse`) is shaped by the
/// upstream Biyard Points API, which does not yet surface
/// `money_tree_bonus` / `money_tree_level` per-transaction. Wiring this in
/// requires a coordinated upstream API change — see
/// `app/ratel/src/features/social/pages/user_reward/components/transaction_item.rs`
/// for the integration site (a TODO comment is left there).
#[component]
pub fn RewardBreakdownChip(
    /// Money Tree skill level the user had at the time of the award (0–10).
    level: i32,
    /// The Money Tree bonus credited on top of the base amount, in points.
    bonus: i64,
    /// The full credited amount (base + bonus). Used to display the total
    /// on the right-hand side of the chip.
    total_amount: i64,
) -> Element {
    if level <= 0 || bonus <= 0 {
        return rsx! {};
    }

    let tr: RewardBreakdownChipTranslate = use_translate();

    // base = total - bonus, never negative.
    let base = (total_amount - bonus).max(0);

    // +X% boost — Money Tree gives +5% per level, capped at +50% at L10.
    let pct = (level * 5).clamp(0, 50);

    // Localized "Money Tree LX +Y%" / at L10, show "MAX".
    let chip_label = if level >= 10 {
        format!("{} {}", tr.money_tree_short, tr.money_tree_max)
    } else {
        format!("{} L{} +{}%", tr.money_tree_short, level, pct)
    };
    let chip_title = format!("{} L{} +{}%", tr.money_tree_short, level, pct);

    rsx! {
        div {
            class: "reward-tx__breakdown",
            role: "note",
            "aria-label": "Reward breakdown",
            span { class: "breakdown__base",
                "{tr.base_label} "
                em { "{base}" }
            }
            span { class: "breakdown__plus", "+" }
            span { class: "mt-chip", title: "{chip_title}",
                svg {
                    "viewBox": "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    "stroke-width": "2.2",
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                    circle { cx: "12", cy: "8", r: "5" }
                    path { d: "M12 13v8" }
                    path { d: "M9 18h6" }
                }
                "{chip_label}"
            }
            span { class: "breakdown__equals", "=" }
            span { class: "breakdown__total", "{total_amount}" }
        }
    }
}
