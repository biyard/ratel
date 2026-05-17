//! `ArcadeLayout` — top-bar + wallet provider that wraps every
//! arcade page (home, matching, game room). The chip-balance widget
//! lives in the header and toggles the exchange modal. The wallet
//! hook is provided once here so every descendant page reads from
//! one cached `UseArcadeWallet`.

use crate::features::arcade::components::{ArcadeExchangeModal, ChipBalance};
use crate::features::arcade::hooks::use_arcade_wallet_provider;
use crate::features::arcade::i18n::ArcadeLayoutTranslate;
use crate::features::auth::hooks::use_user_context;
use crate::route::Route;
use crate::*;

#[component]
pub fn ArcadeLayout() -> Element {
    let tr: ArcadeLayoutTranslate = use_translate();
    let _wallet = use_arcade_wallet_provider()?;
    let route: Route = use_route();
    let user_ctx = use_user_context();
    let is_admin = matches!(
        user_ctx().user.as_ref().map(|u| u.user_type),
        Some(crate::common::types::UserType::Admin),
    );

    let mut modal_open = use_signal(|| false);
    let r_home = Route::ArcadeHomePage {};
    let r_leaderboard = Route::ArcadeLeaderboardPage {};
    let r_admin_new = Route::FactFoldAdminNewHeadlinePage {};

    let on_chip = move |_| modal_open.set(!modal_open());
    let on_close = move |_| modal_open.set(false);

    rsx! {
        div { class: "ff-arcade",
            header { class: "top-bar ff-arcade__top-bar",
                div { class: "brand",
                    div { class: "brand-logo", "R" }
                    div { class: "brand-text",
                        div { class: "brand-name", "{tr.brand}" }
                        div { class: "brand-sub", "{tr.brand_sub}" }
                    }
                }
                nav { class: "top-nav", role: "tablist",
                    Link {
                        class: "top-nav-btn",
                        "aria-selected": route == r_home,
                        to: r_home.clone(),
                        span { class: "top-nav-btn-icon", "⌂" }
                        span { "{tr.tab_home}" }
                    }
                    Link {
                        class: "top-nav-btn",
                        "aria-selected": route == r_leaderboard,
                        to: r_leaderboard.clone(),
                        span { class: "top-nav-btn-icon", "♛" }
                        span { "{tr.tab_leaderboard}" }
                    }
                }
                div { class: "user-stats",
                    if is_admin {
                        Link {
                            class: "top-nav-btn ff-arcade__admin-cta",
                            to: r_admin_new,
                            span { class: "top-nav-btn-icon", "⚐" }
                            span { "{tr.admin_create_round}" }
                        }
                    }
                    ChipBalance { on_click: on_chip }
                    ArcadeAvatar {}
                }
            }
            main { class: "ff-arcade__main", Outlet::<Route> {} }

            ArcadeExchangeModal { open: modal_open(), on_close }
        }
    }
}

/// Small inline avatar — pulls initials from the session user. Falls
/// back to `?` when there's no session (route guards should keep
/// that branch unreachable in production).
#[component]
fn ArcadeAvatar() -> Element {
    let user_ctx = use_user_context();
    let snapshot = user_ctx();
    let initials = snapshot
        .user
        .as_ref()
        .map(|u| {
            let src = if !u.display_name.is_empty() {
                u.display_name.as_str()
            } else if !u.username.is_empty() {
                u.username.as_str()
            } else {
                "?"
            };
            src.chars()
                .filter(|c| c.is_alphanumeric())
                .take(2)
                .collect::<String>()
                .to_uppercase()
        })
        .unwrap_or_else(|| "?".to_string());
    rsx! {
        div { class: "avatar", "{initials}" }
    }
}
