use super::super::views::{format_points, RewardsPageTranslate};
use super::super::*;
use crate::common::services::PointTransactionResponse;
use crate::common::utils::time::time_ago;

// TODO(character-xp-skills, Task 37): wire `RewardBreakdownChip` here
// once the upstream Biyard Points API surfaces `money_tree_bonus` and
// `money_tree_level` per transaction. The chip component is ready —
// `crate::features::character::components::RewardBreakdownChip` —
// and the matching CSS is already in `app/ratel/assets/main.css`. To
// integrate, render the chip inside the outer `div` (below the
// existing flex row) so it occupies the full width via its
// `grid-column: 1 / -1` rule from the mockup.

pub fn transaction_item(
    tr: &RewardsPageTranslate,
    transaction: &PointTransactionResponse,
    idx: usize,
) -> Element {
    let is_received = transaction.transaction_type.eq_ignore_ascii_case("award");
    let time_ago_label = time_ago(transaction.created_at);
    let description = transaction
        .description
        .clone()
        .unwrap_or_else(|| "Ratel".to_string());
    let amount_label = if is_received {
        format!("{} P", format_points(transaction.amount))
    } else {
        format!("-{} P", format_points(transaction.amount))
    };
    let status_label = if is_received { tr.received } else { tr.spent };
    let status_class = if is_received {
        "text-[15px] font-medium text-green-500"
    } else {
        "text-[15px] font-medium text-red-500"
    };

    rsx! {
        div {
            key: "{transaction.created_at}-{idx}",
            class: "p-5 rounded border border-card-border",
            div { class: "flex justify-between items-end w-full",
                div { class: "flex flex-col gap-0.5",
                    div { class: "flex gap-2.5 items-center",
                        span { class: "{status_class}", "{status_label}" }
                        div { class: "flex items-center",
                            div { class: "mr-1 w-5 h-5 rounded-full bg-primary" }
                            span { class: "font-medium text-[15px] text-text-primary",
                                "{amount_label}"
                            }
                        }
                    }
                    div { class: "flex gap-1 items-center",
                        span { class: "text-sm font-semibold text-foreground-muted tracking-[0.5px]",
                            "{tr.from}"
                        }
                        div { class: "w-5 h-5 rounded-full bg-foreground-muted" }
                        span { class: "text-sm font-semibold text-text-primary tracking-[0.5px]",
                            "{description}"
                        }
                    }
                }
                span { class: "text-sm font-medium text-foreground-muted tracking-[0.5px]",
                    "{time_ago_label}"
                }
            }
        }
    }
}
