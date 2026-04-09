use crate::common::*;
use crate::features::profile::i18n::ProfileTranslate;

/// Per-space standings list for the global player profile.
///
/// V1 renders a "Coming soon" placeholder. Future versions will show
/// rows for each space the user participates in, with mini progress
/// bars and XP totals.
#[component]
pub fn DungeonStandingsList() -> Element {
    let tr: ProfileTranslate = use_translate();

    rsx! {
        Card {
            variant: CardVariant::Outlined,
            direction: CardDirection::Col,
            class: "gap-3 p-4 w-full",
            "data-testid": "dungeon-standings",

            span { class: "text-sm font-semibold text-text-primary", "{tr.dungeon_standings}" }

            div { class: "flex justify-center items-center py-8",
                span { class: "text-sm italic text-foreground-muted", "{tr.coming_soon}" }
            }
        }
    }
}
