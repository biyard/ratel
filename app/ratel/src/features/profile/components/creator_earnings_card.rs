use crate::common::*;
use crate::features::profile::i18n::ProfileTranslate;

/// Card showing the user's cumulative XP earned as a space creator.
/// Only renders when `earnings_xp > 0`.
#[component]
pub fn CreatorEarningsCard(earnings_xp: i64) -> Element {
    let tr: ProfileTranslate = use_translate();

    if earnings_xp <= 0 {
        return rsx! {};
    }

    rsx! {
        Card {
            variant: CardVariant::Outlined,
            direction: CardDirection::Row,
            cross_axis_align: CrossAxisAlign::Center,
            class: "gap-4 p-4 w-full",
            "data-testid": "creator-earnings-card",

            div { class: "flex justify-center items-center w-10 h-10 rounded-full shrink-0 bg-primary/10",
                lucide_dioxus::Crown {
                    size: 20,
                    class: "[&>path]:stroke-primary [&>polygon]:stroke-primary",
                }
            }

            Col { class: "flex-1 gap-0.5 min-w-0",
                span { class: "text-xs text-foreground-muted", "{tr.creator_earnings}" }
                Row { cross_axis_align: CrossAxisAlign::End, class: "gap-1",
                    span { class: "text-lg font-bold tabular-nums text-primary", "{earnings_xp}" }
                    span { class: "pb-0.5 text-sm text-foreground-muted", "{tr.xp_suffix}" }
                }
            }
        }
    }
}
