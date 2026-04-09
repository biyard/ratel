use crate::features::activity::controllers::{get_ranking_handler, RankingEntryResponse};
use crate::features::auth::hooks::use_user_context;
use crate::features::spaces::pages::leaderboard::components::*;
use crate::features::spaces::pages::leaderboard::i18n::LeaderboardTranslate;
use crate::features::spaces::space_common::types::space_ranking_key;

use super::*;

/// Per-space leaderboard page.
///
/// Loads ranking data for the space and composes filter chips, a podium for
/// the top 3 entries, and a scrollable list for all remaining entries.
#[component]
pub fn SpaceLeaderboardPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: LeaderboardTranslate = use_translate();
    let user_ctx = use_user_context();
    let current_user_pk = user_ctx.read().user.as_ref().map(|u| u.pk.to_string());

    let ranking_key = space_ranking_key(&space_id());
    let ranking_loader = use_query(&ranking_key, move || async move {
        get_ranking_handler(space_id(), None).await
    })?;

    let ranking = ranking_loader();
    let all_entries: Vec<RankingEntryResponse> = ranking.items.clone();

    let top3: Vec<RankingEntryResponse> = all_entries.iter().take(3).cloned().collect();

    rsx! {
        SeoMeta { title: "{tr.leaderboard_title}" }

        Col { class: "gap-6 w-full", "data-testid": "space-leaderboard-page",

            // Page header
            Row {
                main_axis_align: MainAxisAlign::Between,
                cross_axis_align: CrossAxisAlign::Center,
                class: "w-full",

                h1 { class: "text-xl font-bold text-text-primary", "{tr.leaderboard_title}" }
            }

            // Filter chips (visual-only in V1)
            FilterChips {}

            // Podium (top 3)
            Podium { entries: top3 }

            // Full ranked list
            Card { variant: CardVariant::Outlined, class: "w-full",

                LeaderboardList { entries: all_entries, current_user_pk }
            }
        }
    }
}
