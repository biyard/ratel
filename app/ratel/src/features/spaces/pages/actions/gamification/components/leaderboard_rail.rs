use crate::common::*;
use crate::features::activity::controllers::{get_my_score_handler, get_ranking_handler};
use crate::features::spaces::pages::actions::gamification::i18n::GamificationTranslate;
use crate::features::spaces::space_common::types::{space_my_score_key, space_ranking_key};

/// Leaderboard rail for the dungeon-hero header.
///
/// Renders the top-3 users as gradient spheres (gold / teal / bronze) followed
/// by a clickable "YOU · #N" pill showing the current user's rank in the space.
///
/// The pill is a placeholder link — Phase 8 will wire it to the per-space
/// leaderboard page.
#[component]
pub fn LeaderboardRail(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: GamificationTranslate = use_translate();

    let ranking_key = space_ranking_key(&space_id());
    let ranking_loader = use_query(&ranking_key, move || async move {
        get_ranking_handler(space_id(), None).await
    })?;

    let my_score_key = space_my_score_key(&space_id());
    let my_score_loader = use_query(&my_score_key, move || async move {
        get_my_score_handler(space_id()).await
    })?;

    let ranking = ranking_loader();
    let my_score = my_score_loader();

    let top3: Vec<_> = ranking.items.iter().take(3).cloned().collect();

    if top3.is_empty() && my_score.rank == 0 {
        return rsx! {};
    }

    rsx! {
        Row {
            cross_axis_align: CrossAxisAlign::Center,
            class: "gap-3",
            "data-testid": "leaderboard-rail",

            for (idx , entry) in top3.iter().enumerate() {
                {
                    let shape = match idx {
                        0 => AvatarShape::Sphere,
                        1 => AvatarShape::SphereTeal,
                        _ => AvatarShape::SphereBronze,
                    };
                    rsx! {
                        Avatar {
                            key: "{entry.user_pk}",
                            size: AvatarImageSize::Small,
                            shape,
                            "data-testid": "leaderboard-rail-top-{idx + 1}",
                            if !entry.avatar.is_empty() {
                                AvatarImage { src: "{entry.avatar}", alt: "{entry.name}" }
                            }
                            AvatarFallback { "{entry.name.chars().next().unwrap_or('?')}" }
                        }
                    }
                }
            }

            // The "YOU · #N" pill is a placeholder — Phase 8 wires it to the
            // per-space leaderboard page. For now the onclick is a no-op.
            if my_score.rank > 0 {
                div {
                    class: "py-1 px-3 text-xs font-semibold rounded-full border cursor-pointer bg-primary/10 border-primary/30 text-primary",
                    "data-testid": "leaderboard-rail-you-pill",
                    onclick: move |_| {},
                    "{tr.you_label} · #{my_score.rank}"
                }
            }
        }
    }
}
