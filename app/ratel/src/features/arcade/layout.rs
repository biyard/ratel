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
    let nav = use_navigator();
    let user_ctx = use_user_context();
    let is_admin = matches!(
        user_ctx().user.as_ref().map(|u| u.user_type),
        Some(crate::common::types::UserType::SystemAdmin),
    );

    let mut modal_open = use_signal(|| false);
    let mut menu_open = use_signal(|| false);
    let r_home = Route::ArcadeHomePage {};
    let r_leaderboard = Route::ArcadeLeaderboardPage {};

    // While the player is inside a live round, drop the mobile
    // hamburger from the header — they shouldn't be wandering off
    // mid-stage. The `data-game-room` attribute is set on the top
    // bar so the matching CSS rule (mobile media query) can hide
    // `.ff-arcade__menu-toggle` for game-room routes only.
    let in_game_room = matches!(route, Route::FactFoldGameRoomPage { .. });

    // Close the drawer whenever the route changes — covers browser
    // back/forward and any nav.push() that bypasses the in-drawer
    // onclick handlers (which already close on tap).
    use_effect(use_reactive((&route,), move |(_,)| {
        menu_open.set(false);
    }));

    let on_chip = move |_| modal_open.set(!modal_open());
    let on_chip_drawer = move |_| {
        menu_open.set(false);
        modal_open.set(!modal_open());
    };
    let on_close = move |_| modal_open.set(false);
    let toggle_menu = move |_| menu_open.set(!menu_open());
    let close_menu = move |_| menu_open.set(false);
    // Inline the admin route so the closure only captures `Copy` state
    // (`menu_open`, `nav`) and can be reused for the header CTA and the
    // drawer CTA without cloning the closure itself.
    let go_admin_new = move |_| {
        menu_open.set(false);
        nav.push(Route::FactFoldAdminNewSubjectPage {});
    };

    rsx! {
        div { class: "ff-arcade",
            header {
                class: "top-bar ff-arcade__top-bar",
                "data-game-room": in_game_room,
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
                        "data-testid": "ff-arcade-tab-home",
                        "aria-selected": route == r_home,
                        to: r_home.clone(),
                        span { class: "top-nav-btn-icon", "⌂" }
                        span { "{tr.tab_home}" }
                    }
                    Link {
                        class: "top-nav-btn",
                        "data-testid": "ff-arcade-tab-leaderboard",
                        "aria-selected": route == r_leaderboard,
                        to: r_leaderboard.clone(),
                        span { class: "top-nav-btn-icon", "♛" }
                        span { "{tr.tab_leaderboard}" }
                    }
                }
                div { class: "user-stats",
                    if is_admin {
                        button {
                            class: "ff-arcade__admin-cta",
                            "data-testid": "ff-arcade-admin-cta",
                            onclick: go_admin_new,
                            span { class: "top-nav-btn-icon", "⚐" }
                            span { class: "ff-arcade__admin-cta-label", "{tr.admin_create_round}" }
                        }
                    }
                    ChipBalance { on_click: on_chip }
                    ArcadeAvatar {}
                    // Hamburger toggle — modeled on `home-btn-menu`
                    // (`home-arena .hud-btn--primary`): 42px gold-glass
                    // square with a 3-line SVG icon and an Orbitron
                    // label that fades in on hover. The open state
                    // swaps the icon for a close X but keeps the same
                    // button shape so the position is stable.
                    button {
                        class: "ff-arcade__menu-toggle",
                        "data-testid": "ff-arcade-menu-toggle",
                        "aria-label": if menu_open() { tr.menu_close } else { tr.menu_open },
                        "aria-haspopup": "dialog",
                        "aria-expanded": menu_open(),
                        onclick: toggle_menu,
                        if menu_open() {
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "1.8",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                line {
                                    x1: "5",
                                    y1: "5",
                                    x2: "19",
                                    y2: "19",
                                }
                                line {
                                    x1: "19",
                                    y1: "5",
                                    x2: "5",
                                    y2: "19",
                                }
                            }
                        } else {
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "1.8",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                line {
                                    x1: "3",
                                    y1: "6",
                                    x2: "21",
                                    y2: "6",
                                }
                                line {
                                    x1: "3",
                                    y1: "12",
                                    x2: "21",
                                    y2: "12",
                                }
                                line {
                                    x1: "3",
                                    y1: "18",
                                    x2: "21",
                                    y2: "18",
                                }
                            }
                        }
                        span { class: "hud-btn__label",
                            if menu_open() {
                                "{tr.menu_close}"
                            } else {
                                "{tr.menu_open}"
                            }
                        }
                    }
                }
            }

            // Mobile drawer — always mounted so the slide-in/out
            // transition has both endpoints to animate between. CSS
            // toggles `visibility` + `pointer-events` based on
            // `data-open`, keeping the closed drawer out of the tab
            // order. Hidden on desktop via media query.
            div {
                class: "ff-arcade__menu-scrim",
                "data-open": menu_open(),
                "aria-hidden": "true",
                onclick: close_menu,
            }
            div {
                class: "ff-arcade__menu-drawer",
                "data-open": menu_open(),
                "aria-hidden": !menu_open(),
                nav {
                    class: "ff-arcade__menu-nav",
                    "aria-label": "{tr.menu_nav_label}",
                    role: "tablist",
                    Link {
                        class: "top-nav-btn",
                        "aria-selected": route == r_home,
                        to: r_home.clone(),
                        onclick: close_menu,
                        span { class: "top-nav-btn-icon", "⌂" }
                        span { "{tr.tab_home}" }
                    }
                    Link {
                        class: "top-nav-btn",
                        "aria-selected": route == r_leaderboard,
                        to: r_leaderboard.clone(),
                        onclick: close_menu,
                        span { class: "top-nav-btn-icon", "♛" }
                        span { "{tr.tab_leaderboard}" }
                    }
                    if is_admin {
                        button {
                            class: "ff-arcade__admin-cta",
                            onclick: go_admin_new,
                            span { class: "top-nav-btn-icon", "⚐" }
                            span { "{tr.admin_create_round}" }
                        }
                    }
                    ChipBalance {
                        on_click: on_chip_drawer,
                        testid: "ff-arcade-chip-drawer".to_string(),
                    }
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
