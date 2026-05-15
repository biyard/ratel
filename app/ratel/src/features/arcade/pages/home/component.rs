//! `ArcadeHomePage` — `/arcade/home`. The lobby + leaderboard hub for
//! the entire arcade. The page is wrapped by `ArcadeLayout` so the
//! top-bar / chip widget / exchange modal aren't rendered here.

use crate::features::arcade::games::fact_or_fold::{
    BetSide, LeaderboardEntryResponse, LobbyResponse, RoundResponse, RoundStatus,
    UserStatsResponse,
};
use crate::features::arcade::pages::home::{use_arcade_home_provider, ArcadeHomeTranslate};
use crate::features::auth::hooks::use_user_context;
use crate::*;

#[component]
pub fn ArcadeHomePage() -> Element {
    let ctx = use_arcade_home_provider()?;
    let nav = use_navigator();

    let lobby = (ctx.lobby)();
    let my_stats = (ctx.my_stats)();
    let leaderboard = (ctx.leaderboard)();

    // Hash-routed tab toggle — top-bar leaderboard link drops a
    // #leaderboard fragment. Default to "lobby" otherwise.
    let active_tab = use_signal(|| {
        if read_hash().contains("leaderboard") {
            "leaderboard".to_string()
        } else {
            "lobby".to_string()
        }
    });

    // If the user is already inside a live round, jump straight to
    // the matching or game-room page. Driven off the loader so the
    // redirect fires on first render and any subsequent refresh.
    if let Some(round) = lobby.current_round.as_ref() {
        if lobby.already_joined {
            let route = match round.status {
                RoundStatus::Waiting => Route::FactFoldMatchingPage {},
                RoundStatus::Settled => return rsx! {
                    section { class: "ff-home",
                        div { class: "ff-home__placeholder", "Round just settled — refresh to see results." }
                    }
                },
                _ => Route::FactFoldGameRoomPage {
                    round_id: round.id.clone(),
                },
            };
            nav.push(route);
        }
    }

    rsx! {
        SeoMeta { title: "Ratel Arcade" }
        section { class: "ff-home",
            div {
                class: "ff-home__tab-section",
                "data-active": "{active_tab() == \"lobby\"}",
                LobbyView { lobby: lobby.clone(), my_stats }
            }
            div {
                class: "ff-home__tab-section",
                "data-active": "{active_tab() == \"leaderboard\"}",
                LeaderboardView { leaderboard }
            }
        }

        TabHashListener { active_tab }
    }
}

// ── Lobby tab ───────────────────────────────────────────────────────

#[component]
fn LobbyView(lobby: LobbyResponse, my_stats: UserStatsResponse) -> Element {
    rsx! {
        div { class: "lobby-grid",
            FeaturedCard { lobby: lobby.clone() }
            div { class: "lobby-side",
                StatsCard { stats: my_stats }
                HistoryCard {}
            }
        }
        CatalogSection { lobby }
    }
}

#[component]
fn FeaturedCard(lobby: LobbyResponse) -> Element {
    let tr: ArcadeHomeTranslate = use_translate();
    let mut ctx = use_arcade_home_provider()?;
    let nav = use_navigator();
    let mut submitting = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);

    let on_join = move |_| async move {
        if submitting() {
            return;
        }
        submitting.set(true);
        error_msg.set(None);
        match ctx.join().await {
            Ok(round) => {
                let route = match round.status {
                    RoundStatus::Waiting => Route::FactFoldMatchingPage {},
                    _ => Route::FactFoldGameRoomPage {
                        round_id: round.id.clone(),
                    },
                };
                nav.push(route);
            }
            Err(e) => {
                error_msg.set(Some(format!("{e}")));
            }
        }
        submitting.set(false);
    };

    let (tag_label, _tag_status) = featured_tag(&lobby, &tr);
    let stage_label = featured_stage_label(&lobby, &tr);
    let capacity_label = tr
        .meta_capacity_value
        .replace("{$count}", &lobby.round_capacity.to_string());
    let min_bet_label = tr
        .meta_min_bet_value
        .replace("{$rp}", &lobby.min_bet_rp.to_string());

    rsx! {
        div { class: "featured",
            div { class: "featured-tag",
                span { class: "featured-tag-dot" }
                span { "{tag_label}" }
            }
            h1 { class: "featured-title", "{tr.featured_title}" }
            p { class: "featured-tagline", "{tr.featured_tagline}" }

            div { class: "featured-meta",
                MetaCell { label: tr.meta_capacity.to_string(), value: capacity_label }
                MetaCell { label: tr.meta_min_bet.to_string(), value: min_bet_label }
                MetaCell { label: tr.meta_cycle.to_string(), value: tr.meta_cycle_value.to_string() }
                MetaCell { label: tr.meta_stage_now.to_string(), value: stage_label }
            }

            if let Some(err) = error_msg() {
                div { class: "reason-warn", style: "margin-bottom: 14px", "{err}" }
            }

            FeaturedCta {
                lobby,
                on_join,
                submitting: submitting(),
            }
        }
    }
}

#[component]
fn MetaCell(label: String, value: String) -> Element {
    rsx! {
        div { class: "meta-cell",
            div { class: "meta-cell-label", "{label}" }
            div { class: "meta-cell-value", "{value}" }
        }
    }
}

#[component]
fn FeaturedCta(
    lobby: LobbyResponse,
    on_join: EventHandler<MouseEvent>,
    submitting: bool,
) -> Element {
    let tr: ArcadeHomeTranslate = use_translate();
    let waiting_count = lobby
        .current_round
        .as_ref()
        .map(|r| r.participant_pks.len())
        .unwrap_or(0);

    rsx! {
        div { class: "featured-cta",
            if lobby.already_joined {
                // Should have already redirected — keep a fallback CTA
                // in case the page renders before the redirect lands.
                Link {
                    class: "btn btn-primary",
                    to: Route::FactFoldMatchingPage {},
                    "{tr.cta_resume}"
                }
                span { class: "featured-status",
                    span { class: "green", "●" }
                    " {tr.status_already_joined}"
                }
            } else if lobby.can_join {
                button {
                    class: "btn btn-primary",
                    disabled: submitting,
                    onclick: move |e| on_join.call(e),
                    "{tr.cta_join}"
                }
                span { class: "featured-status",
                    if let Some(r) = lobby.current_round.as_ref() {
                        {
                            let _ = r;
                            tr.status_can_join_existing.replace("{$count}", &waiting_count.to_string())
                        }
                    } else {
                        "{tr.status_can_join_new}"
                    }
                }
            } else if lobby.current_round.is_some() {
                button {
                    class: "btn btn-ghost",
                    disabled: true,
                    "{tr.cta_in_progress}"
                }
                span { class: "featured-status", "{tr.status_in_progress}" }
            } else {
                button {
                    class: "btn btn-ghost",
                    disabled: true,
                    "{tr.cta_no_headline}"
                }
                span { class: "featured-status", "{tr.status_no_headline}" }
            }
        }
    }
}

#[component]
fn StatsCard(stats: UserStatsResponse) -> Element {
    let tr: ArcadeHomeTranslate = use_translate();
    let accuracy = format!(
        "{:.1}%",
        (stats.accuracy_bps as f64 / 100.0).max(0.0)
    );
    let delta = stats.lifetime_delta_chips;
    let delta_label = if delta >= 0 {
        format!("+{delta}")
    } else {
        format!("{delta}")
    };
    let delta_class = if delta >= 0 { "num gold" } else { "num pink" };
    let played_label = if stats.last_played_at > 0 {
        format_relative_date(stats.last_played_at)
    } else {
        tr.stats_never.to_string()
    };

    rsx! {
        div { class: "my-stats-card",
            div { class: "section-head",
                h2 { "{tr.stats_card_title}" }
                span { class: "sub", "{tr.stats_card_sub}" }
            }
            div { class: "my-stats-row",
                span { "{tr.stats_rounds}" }
                span { class: "num", "{stats.total_rounds}" }
            }
            div { class: "my-stats-row",
                span { "{tr.stats_accuracy}" }
                span { class: "num teal", "{accuracy}" }
            }
            div { class: "my-stats-row",
                span { "{tr.stats_delta}" }
                span { class: "{delta_class}", "{delta_label}" }
            }
            div { class: "my-stats-row",
                span { "{tr.stats_last_played}" }
                span { class: "num", "{played_label}" }
            }
        }
    }
}

#[component]
fn HistoryCard() -> Element {
    let tr: ArcadeHomeTranslate = use_translate();
    rsx! {
        div { class: "history-card",
            div { class: "section-head",
                h2 { "{tr.history_card_title}" }
                span { class: "sub", "{tr.history_card_sub}" }
            }
            div { class: "ff-home__placeholder", "{tr.history_empty}" }
        }
    }
}

#[component]
fn CatalogSection(lobby: LobbyResponse) -> Element {
    let tr: ArcadeHomeTranslate = use_translate();
    let game_count = 1;
    let sub_label = tr
        .catalog_sub
        .replace("{$count}", &game_count.to_string());
    let playing = lobby.current_round.is_some() || lobby.already_joined;
    let pill_label = if playing {
        tr.catalog_tile_status_playing
    } else {
        tr.catalog_tile_status_idle
    };
    let pill_class = if playing { "pill gold" } else { "pill" };

    rsx! {
        div { class: "catalog-section",
            div { class: "section-head",
                h2 { "{tr.catalog_title}" }
                span { class: "sub", "{sub_label}" }
            }
            div { class: "catalog-grid",
                div { class: "game-tile",
                    div { class: "game-tile-icon gold", "◆" }
                    div { class: "game-tile-name", "Fact or Fold" }
                    div { class: "game-tile-desc",
                        "4 players judge a news headline. Bets and rationale decide the winners."
                    }
                    div { class: "game-tile-foot",
                        span { "{tr.catalog_tile_meta}" }
                        span { class: "{pill_class}", "{pill_label}" }
                    }
                }
            }
        }
    }
}

// ── Leaderboard tab ─────────────────────────────────────────────────

#[component]
fn LeaderboardView(leaderboard: ListResponse<LeaderboardEntryResponse>) -> Element {
    let tr: ArcadeHomeTranslate = use_translate();
    let user_ctx = use_user_context();
    let my_pk = user_ctx().user_pk().unwrap_or_default();

    rsx! {
        div { class: "lb-table",
            div { class: "section-head",
                h2 { "{tr.lb_section_title}" }
                span { class: "sub", "{tr.lb_section_sub}" }
            }

            div { class: "lb-row head",
                div { "{tr.lb_head_rank}" }
                div {}
                div { "{tr.lb_head_name}" }
                div { "{tr.lb_head_stats}" }
                div { "{tr.lb_head_accuracy}" }
                div { "{tr.lb_head_chips}" }
            }

            if leaderboard.items.is_empty() {
                div { class: "ff-home__placeholder", "{tr.lb_empty}" }
            }
            for (idx , entry) in leaderboard.items.iter().enumerate() {
                LeaderboardRow {
                    key: "{entry.user_pk}",
                    idx,
                    entry: entry.clone(),
                    is_me: entry.user_pk == my_pk,
                }
            }
        }
    }
}

#[component]
fn LeaderboardRow(idx: usize, entry: LeaderboardEntryResponse, is_me: bool) -> Element {
    let tr: ArcadeHomeTranslate = use_translate();
    let rank = idx + 1;
    let rank_class = match rank {
        1 => "lb-rank top1",
        2 => "lb-rank top2",
        3 => "lb-rank top3",
        _ => "lb-rank",
    };
    let avatar_variant = avatar_variant(idx);
    let avatar_class = if avatar_variant.is_empty() {
        "p-avatar".to_string()
    } else {
        format!("p-avatar {avatar_variant}")
    };
    let display = if entry.display_name.is_empty() {
        entry.username.clone()
    } else {
        entry.display_name.clone()
    };
    let initials = initials(&display, &entry.username);
    let accuracy = format!("{:.1}%", entry.accuracy_bps as f64 / 100.0);
    let chips_label = format!(
        "{}{}",
        if entry.lifetime_delta_chips >= 0 { "+" } else { "" },
        entry.lifetime_delta_chips
    );
    let stat_label = tr
        .lb_row_stat
        .replace("{$total}", &entry.total_rounds.to_string())
        .replace("{$correct}", &entry.correct_count.to_string());
    let row_class = if is_me { "lb-row me" } else { "lb-row" };

    rsx! {
        div { class: "{row_class}",
            div { class: "{rank_class}", "{rank}" }
            div { class: "{avatar_class}", "{initials}" }
            div { class: "lb-name-cell",
                span { class: "lb-name", "{display}" }
                if is_me {
                    span { class: "lb-you-badge", "YOU" }
                }
            }
            div { class: "lb-stat", "{stat_label}" }
            div { class: "lb-accuracy", "{accuracy}" }
            div { class: "lb-primary", "{chips_label}" }
        }
    }
}

// ── Hash-routed tab listener ────────────────────────────────────────

/// Sets up a window `hashchange` listener so when the user clicks the
/// top-bar Leaderboard link (which drops `#leaderboard`) the visible
/// tab toggles without a full route change. Runs only on the web
/// target; on mobile/server the future never spawns work.
#[component]
fn TabHashListener(active_tab: Signal<String>) -> Element {
    let _ = active_tab;
    #[cfg(feature = "web")]
    {
        let mut active_tab = active_tab;
        use_future(move || async move {
            let mut runner = dioxus::document::eval(
                r#"
                function publish() {
                  const hash = (window.location.hash || "").toLowerCase();
                  const tab = hash.includes("leaderboard") ? "leaderboard" : "lobby";
                  dioxus.send(tab);
                }
                publish();
                window.addEventListener("hashchange", publish);
                "#,
            );
            loop {
                match runner.recv::<String>().await {
                    Ok(tab) => active_tab.set(tab),
                    Err(_) => break,
                }
            }
        });
    }
    rsx! {}
}

// ── Helpers ─────────────────────────────────────────────────────────

fn featured_tag(lobby: &LobbyResponse, tr: &ArcadeHomeTranslate) -> (&'static str, &'static str) {
    if let Some(r) = lobby.current_round.as_ref() {
        match r.status {
            RoundStatus::Waiting => (tr.tag_waiting, "waiting"),
            RoundStatus::Settled => (tr.tag_settled, "settled"),
            _ => (tr.tag_live, "live"),
        }
    } else if lobby.headline_available {
        (tr.tag_open, "open")
    } else {
        (tr.tag_closed, "closed")
    }
}

fn featured_stage_label(lobby: &LobbyResponse, tr: &ArcadeHomeTranslate) -> String {
    let Some(r) = lobby.current_round.as_ref() else {
        return tr.meta_stage_idle.to_string();
    };
    match r.status {
        RoundStatus::Waiting => "1 / 6".to_string(),
        RoundStatus::NewsReveal => "1 / 6".to_string(),
        RoundStatus::Bet => "2 / 6".to_string(),
        RoundStatus::Rationale => "3 / 6".to_string(),
        RoundStatus::Reveal => "4 / 6".to_string(),
        RoundStatus::Debate => "5 / 6".to_string(),
        RoundStatus::Settlement | RoundStatus::Settled => "6 / 6".to_string(),
    }
}

fn avatar_variant(idx: usize) -> &'static str {
    match idx % 4 {
        0 => "",
        1 => "a2",
        2 => "a3",
        _ => "a4",
    }
}

fn initials(display: &str, username: &str) -> String {
    let src = if !display.is_empty() {
        display
    } else if !username.is_empty() {
        username
    } else {
        "?"
    };
    src.chars()
        .filter(|c| c.is_alphanumeric())
        .take(2)
        .collect::<String>()
        .to_uppercase()
}

fn format_relative_date(ts_ms: i64) -> String {
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let diff_ms = (now - ts_ms).max(0);
    let diff_days = diff_ms / (1000 * 60 * 60 * 24);
    if diff_days == 0 {
        "Today".to_string()
    } else if diff_days == 1 {
        "Yesterday".to_string()
    } else if diff_days < 7 {
        format!("{diff_days}d ago")
    } else {
        format!("{}w ago", diff_days / 7)
    }
}

#[cfg(feature = "web")]
fn read_hash() -> String {
    if let Some(window) = web_sys::window() {
        if let Ok(hash) = window.location().hash() {
            return hash;
        }
    }
    String::new()
}

#[cfg(not(feature = "web"))]
fn read_hash() -> String {
    String::new()
}

// Avoid suppressing unused-import warnings on RoundResponse / BetSide
// — they're exposed for completeness in the public DTO surface this
// page reads against, but consumed only via the match arms above.
#[allow(dead_code)]
fn _force_dto_imports_used() {
    let _ = std::any::type_name::<RoundResponse>();
    let _ = std::any::type_name::<BetSide>();
}
