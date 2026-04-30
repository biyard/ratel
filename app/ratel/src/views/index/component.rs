use super::space_card::*;
use super::*;
use crate::common::components::{Robots, SeoMeta};
use crate::common::contexts::TeamItem;
use crate::common::hooks::use_infinite_query;
use crate::common::types::ListResponse;
use crate::common::use_loader;
use crate::features::auth::LoginModal;
use crate::features::posts::controllers::create_post::create_post_handler;
use crate::features::social::pages::team_arena::ArenaTeamCreationPopup;
use crate::features::spaces::pages::index::SettingsPanel;
use crate::features::spaces::space_common::controllers::{
    list_hot_spaces_handler, list_my_home_spaces_handler, HotSpaceResponse,
};
use crate::features::spaces::space_common::models::HotSpaceHeat;
use crate::me::use_my_spaces;
use crate::*;

#[derive(Clone, Copy, PartialEq)]
enum HomeTab {
    Hot,
    Mine,
}

#[component]
pub fn Index() -> Element {
    let t: HomeArenaTranslate = use_translate();
    let nav = use_navigator();
    let mut popup = use_popup();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let username = user_ctx()
        .user
        .as_ref()
        .map(|u| u.username.clone())
        .unwrap_or_default();
    let has_user = user_ctx().user.is_some();
    // Admin users get a shield button next to Settings in the topbar
    // HUD that jumps straight to `/admin/`. Non-admins see nothing —
    // mirrors the team_arena topbar so admin tooling is one click away
    // from the home arena instead of a manually-typed URL.
    let is_admin = user_ctx()
        .user
        .as_ref()
        .map(|u| matches!(u.user_type, UserType::Admin))
        .unwrap_or(false);
    let mut settings_open = use_signal(|| false);
    let mut teams_open = use_signal(|| false);
    let mut notifications_open = use_signal(|| false);

    // Read user_ctx inside the async so the fetch reflects the current
    // login state at invocation time. Login from the home page triggers
    // `teams_query.restart()` via the LoginModal's `on_success` callback
    // (wired into each HUD button below) instead of a use_effect.
    let mut teams_query = use_infinite_query(move |bookmark| async move {
        let logged_in = user_ctx().user.is_some();
        if logged_in {
            crate::features::social::controllers::get_user_teams_handler(bookmark).await
        } else {
            Ok(ListResponse::<TeamItem>::default())
        }
    })?;
    let teams: Vec<TeamItem> = teams_query.items();
    let teams_more = teams_query.more_element();

    // Shared restart callback passed into every LoginModal we open from
    // this page — fires right after a successful login so the Teams
    // dropdown reloads without a full page reload.
    let on_login_success: Callback<()> = use_callback(move |_| {
        teams_query.restart();
    });

    let keywords = vec![
        "ratel".to_string(),
        "human essence platform".to_string(),
        "essence house".to_string(),
        "personal ai agent".to_string(),
        "mcp subscription".to_string(),
        "passive income ai".to_string(),
        "rag knowledge base".to_string(),
        "notion to ai".to_string(),
        "creator monetization".to_string(),
        "collective intelligence".to_string(),
    ];

    let brand_logo = "https://metadata.ratel.foundation/logos/logo-symbol.png".to_string();

    let hot_spaces = use_loader(move || async move { list_hot_spaces_handler(None).await })?;
    let my_spaces = use_my_spaces()?.my_spaces;

    let hot_cards = hot_spaces().items;
    let mine_cards = my_spaces().items;

    let default_tab = if has_user && !mine_cards.is_empty() {
        HomeTab::Mine
    } else {
        HomeTab::Hot
    };
    let mut active_tab = use_signal(|| default_tab);
    let current_tab = active_tab();

    let cards = match current_tab {
        HomeTab::Hot => hot_cards.clone(),
        HomeTab::Mine => mine_cards.clone(),
    };

    let active_spaces = hot_cards.len() as i64;

    let go_create_post = move |_: Event<MouseData>| {
        if !has_user {
            popup
                .open(rsx! {
                    LoginModal { on_success: on_login_success }
                })
                .with_title("Start building your Essence");
            return;
        }
        spawn(async move {
            match create_post_handler(None).await {
                Ok(resp) => {
                    nav.push(Route::PostEdit {
                        post_id: resp.post_pk.into(),
                    });
                }
                Err(e) => {
                    dioxus::logger::tracing::error!("Failed to create post: {:?}", e);
                }
            }
        });
    };

    let drafts_username = username.clone();
    let go_drafts = move |_: Event<MouseData>| {
        if !has_user {
            popup
                .open(rsx! {
                    LoginModal { on_success: on_login_success }
                })
                .with_title("Start building your Essence");
            return;
        }
        nav.push(Route::UserDrafts {
            username: drafts_username.clone(),
        });
    };

    let rewards_username = username.clone();
    let go_rewards = move |_: Event<MouseData>| {
        if !has_user {
            popup
                .open(rsx! {
                    LoginModal { on_success: on_login_success }
                })
                .with_title("Start building your Essence");
            return;
        }
        nav.push(Route::UserRewards {
            username: rewards_username.clone(),
        });
    };

    let go_credentials = move |_: Event<MouseData>| {
        if !has_user {
            popup
                .open(rsx! {
                    LoginModal { on_success: on_login_success }
                })
                .with_title("Start building your Essence");
            return;
        }
        nav.push(Route::CredentialsHome {});
    };

    let go_essence = move |_: Event<MouseData>| {
        if !has_user {
            popup
                .open(rsx! {
                    LoginModal { on_success: on_login_success }
                })
                .with_title("Start building your Essence");
            return;
        }
        nav.push(Route::EssenceSourcesPage {});
    };

    let open_settings = move |_: Event<MouseData>| {
        settings_open.set(true);
    };

    // let go_browse_all = move |_: Event<MouseData>| {
    //     nav.push(Route::PostIndex {});
    // };

    rsx! {
        SeoMeta {
            title: "Ratel – Human Essence Platform",
            description: "Turn your thoughts into your Essence. Post, discuss, vote — then plug your Essence House into ChatGPT or Claude via a single MCP endpoint, deploy agents, and earn passive income.",
            image: "https://metadata.ratel.foundation/logos/logo-symbol.png",
            url: "https://ratel.foundation",
            robots: Robots::IndexNofollow,
            keywords,
        }

        document::Script { defer: true, src: asset!("./script.js") }

        div { class: "home-arena",
            // TOP BAR
            div { class: "arena-topbar",
                div { class: "arena-topbar__brand",
                    img {
                        class: "arena-topbar__logo",
                        src: "{brand_logo}",
                        alt: "Ratel logo",
                    }
                    span { class: "arena-topbar__title", "{t.brand_title}" }
                    span { class: "arena-topbar__status", "{t.live_status}" }
                }
                div { class: "arena-topbar__actions",
                    if has_user {
                        crate::features::notifications::components::NotificationBell {
                            class: "hud-btn",
                            onclick: move |_| notifications_open.toggle(),
                        }
                    }
                    button {
                        class: "hud-btn hud-btn--primary",
                        aria_label: "{t.create}",
                        "data-testid": "home-btn-create",
                        onclick: go_create_post,
                        svg {
                            fill: "none",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "1.6",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            path { d: "M12 20h9" }
                            path { d: "M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z" }
                        }
                        span { class: "hud-btn__label", "{t.create}" }
                    }
                    button {
                        class: "hud-btn",
                        aria_label: "{t.drafts}",
                        "data-testid": "home-btn-drafts",
                        onclick: go_drafts,
                        svg {
                            fill: "none",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "1.6",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                            polyline { points: "14 2 14 8 20 8" }
                            line {
                                x1: "9",
                                y1: "13",
                                x2: "15",
                                y2: "13",
                            }
                            line {
                                x1: "9",
                                y1: "17",
                                x2: "13",
                                y2: "17",
                            }
                        }
                        span { class: "hud-btn__label", "{t.drafts}" }
                    }
                    button {
                        class: "hud-btn",
                        aria_label: "{t.rewards}",
                        "data-testid": "home-btn-rewards",
                        onclick: go_rewards,
                        svg {
                            fill: "none",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "1.6",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            polyline { points: "20 12 20 22 4 22 4 12" }
                            rect {
                                x: "2",
                                y: "7",
                                width: "20",
                                height: "5",
                            }
                            line {
                                x1: "12",
                                y1: "22",
                                x2: "12",
                                y2: "7",
                            }
                            path { d: "M12 7H7.5a2.5 2.5 0 0 1 0-5C11 2 12 7 12 7z" }
                            path { d: "M12 7h4.5a2.5 2.5 0 0 0 0-5C13 2 12 7 12 7z" }
                        }
                        span { class: "hud-btn__label", "{t.rewards}" }
                    }

                    button {
                        class: "hud-btn",
                        aria_label: "{t.credentials}",
                        "data-testid": "home-btn-credentials",
                        onclick: go_credentials,
                        svg {
                            fill: "none",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "1.6",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            path { d: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" }
                            path { d: "m9 12 2 2 4-4" }
                        }
                        span { class: "hud-btn__label", "{t.credentials}" }
                    }
                    // My AI — opens the personal MCP endpoint page so an
                    // AI agent (Claude Code, Cursor, etc.) can be wired up
                    // to act on the user's behalf. Cyan accent so it reads
                    // as an external integration affordance distinct from
                    // the gold action buttons.
                    button {
                        class: "hud-btn hud-btn--ai",
                        aria_label: "{t.my_ai}",
                        "data-testid": "home-btn-my-ai",
                        onclick: move |_| {
                            if !has_user {
                                popup
                                    .open(rsx! {
                                    LoginModal { on_success: on_login_success }
                                })
                                .with_title("Start building your Essence");
                                return;
                            }
                            nav.push(Route::MyAiPage {});
                        },
                        svg {
                            fill: "none",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "1.6",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            path { d: "M12 3 13.6 9.4 20 11l-6.4 1.6L12 19l-1.6-6.4L4 11l6.4-1.6L12 3Z" }
                            path {
                                d: "M19 4 19.7 6.3 22 7l-2.3 0.7L19 10l-0.7-2.3L16 7l2.3-0.7L19 4Z",
                                opacity: "0.7",
                                stroke_width: "1.2",
                            }
                        }
                        span { class: "hud-btn__label", "{t.my_ai}" }
                    }
                    button {
                        class: "hud-btn",
                        aria_label: "{t.essence}",
                        "data-testid": "home-btn-essence",
                        onclick: go_essence,
                        svg {
                            fill: "none",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "1.6",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            path { d: "M4 19.5A2.5 2.5 0 0 1 6.5 17H20" }
                            path { d: "M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z" }
                        }
                        span { class: "hud-btn__label", "{t.essence}" }
                    }
                    if has_user {
                        div { class: "hud-teams", "aria-expanded": teams_open(),
                            button {
                                class: "hud-btn",
                                aria_label: "{t.teams}",
                                "data-testid": "home-btn-teams",
                                onclick: move |e: Event<MouseData>| {
                                    e.stop_propagation();
                                    teams_open.toggle();
                                },
                                svg {
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "1.6",
                                    view_box: "0 0 24 24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    path { d: "M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" }
                                    circle { cx: "9", cy: "7", r: "4" }
                                    path { d: "M23 21v-2a4 4 0 0 0-3-3.87" }
                                    path { d: "M16 3.13a4 4 0 0 1 0 7.75" }
                                }
                                span { class: "hud-btn__label", "{t.teams}" }
                            }
                            // Always rendered; CSS uses [aria-expanded="true"]
                            // on the parent to toggle visibility. Matches the
                            // team_arena topbar pattern exactly — button owns
                            // stop_propagation + toggle, dropdown owns its own
                            // stop_propagation so clicks inside don't bubble
                            // to the outer backdrop.
                            div {
                                class: "team-dd",
                                role: "menu",
                                "data-testid": "home-teams-dd",
                                onclick: move |e: Event<MouseData>| e.stop_propagation(),
                                div { class: "team-dd__header", "{t.teams_header}" }
                                div {
                                    class: "team-dd__list",
                                    id: "home-teams-dd-list",
                                    // The shared `use_infinite_query` sentinel uses
                                    // IntersectionObserver against the viewport, which
                                    // doesn't fire for internal scrolling inside this
                                    // bounded dropdown. Detect near-bottom directly via
                                    // onscroll + JS so pagination triggers reliably.
                                    onscroll: move |_| {
                                        let mut ctrl = teams_query;
                                        spawn(async move {
                                            let mut eval = document::eval(include_str!("./web/team-list.js"));
                                            if let Ok(near_bottom) = eval.recv::<bool>().await {
                                                if near_bottom && ctrl.has_more() && !ctrl.is_loading() {
                                                    ctrl.next();
                                                }
                                            }
                                        });
                                    },
                                    if teams.is_empty() {
                                        div { class: "team-dd__empty", "{t.teams_empty}" }
                                    } else {
                                        for team in teams.iter().cloned() {
                                            HomeTeamDdItem {
                                                key: "{team.username}",
                                                username: team.username.clone(),
                                                display_name: if team.nickname.is_empty() { team.username.clone() } else { team.nickname.clone() },
                                                profile_url: team.profile_url.clone(),
                                                on_pick: move |_| {
                                                    teams_open.set(false);
                                                },
                                            }
                                        }
                                    }
                                    {teams_more}
                                }
                                div {
                                    class: "team-dd__footer",
                                    role: "button",
                                    tabindex: "0",
                                    "data-testid": "home-btn-create-team",
                                    onclick: move |_| {
                                        teams_open.set(false);
                                        popup.open(rsx! {
                                            ArenaTeamCreationPopup {}
                                        }).without_close().with_backdrop_close();
                                    },
                                    svg {
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2.5",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        line {
                                            x1: "12",
                                            y1: "5",
                                            x2: "12",
                                            y2: "19",
                                        }
                                        line {
                                            x1: "5",
                                            y1: "12",
                                            x2: "19",
                                            y2: "12",
                                        }
                                    }
                                    "{t.create_team}"
                                }
                            }
                        }
                        if teams_open() {
                            div {
                                style: "position:fixed;inset:0;z-index:25;",
                                onclick: move |_| teams_open.set(false),
                            }
                        }
                    }
                    if !has_user {
                        button {
                            class: "hud-btn hud-btn--signin",
                            aria_label: "{t.sign_in}",
                            "data-testid": "home-btn-signin",
                            onclick: move |_| {
                                popup.open(rsx! {
                                    LoginModal { on_success: on_login_success }
                                }).with_title("Start building your Essence");
                            },
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "1.6",
                                view_box: "0 0 25 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M15.5 16.5V19C15.5 20.1046 14.6046 21 13.5 21H6.5C5.39543 21 4.5 20.1046 4.5 19V5C4.5 3.89543 5.39543 3 6.5 3H13.5C14.6046 3 15.5 3.89543 15.5 5V8.0625M20.5 12L9.5 12M9.5 12L12 14.5M9.5 12L12 9.5" }
                            }
                            span { class: "hud-btn__label", "{t.sign_in}" }
                        }
                    }
                    if is_admin {
                        button {
                            class: "hud-btn",
                            aria_label: "Admin",
                            "data-testid": "home-btn-admin",
                            onclick: move |_| {
                                nav.push(Route::AdminMainPage {});
                            },
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "1.6",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                // Plain shield — read at a glance as "elevated
                                // permissions" without the checkmark used by
                                // the credentials button next to it.
                                path { d: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" }
                            }
                            span { class: "hud-btn__label", "Admin" }
                        }
                    }
                    button {
                        class: "hud-btn",
                        aria_label: "{t.settings}",
                        "data-testid": "home-btn-settings",
                        onclick: open_settings,
                        svg {
                            fill: "none",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "1.6",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            path { d: "M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" }
                            circle { cx: "12", cy: "12", r: "3" }
                        }
                        span { class: "hud-btn__label", "{t.settings}" }
                    }
                }
            }

            // SECTION LABEL
            div { class: "section-label",
                span { class: "section-label__dash" }
                span { class: "section-label__title",
                    strong { "{t.section_hot}" }
                    " {t.section_spaces}"
                }
                span { class: "section-label__dash" }
            }

            // TABS (only visible when logged in)
            if has_user {
                div { class: "section-tabs",
                    button {
                        class: "section-tab",
                        aria_selected: current_tab == HomeTab::Hot,
                        "data-testid": "home-tab-hot",
                        onclick: move |_| active_tab.set(HomeTab::Hot),
                        "{t.tab_hot}"
                    }
                    button {
                        class: "section-tab",
                        aria_selected: current_tab == HomeTab::Mine,
                        "data-testid": "home-tab-mine",
                        onclick: move |_| active_tab.set(HomeTab::Mine),
                        "{t.tab_mine}"
                    }
                }
            }

            // CAROUSEL
            if cards.is_empty() {
                div { class: "home-arena__empty",
                    if current_tab == HomeTab::Mine {
                        "{t.empty_mine}"
                    } else {
                        "{t.empty_hot}"
                    }
                }
            } else {
                div { class: "carousel-wrapper",
                    div { class: "carousel-track", id: "home-carousel-track",
                        for card in cards.iter() {
                            ArenaSpaceCard {
                                key: "{card.space_id.clone().to_string()}",
                                heat: heat_from_response(card.heat),
                                rank: card.rank as u32,
                                logo: card_logo(card, &brand_logo),
                                category: card_category(card),
                                title: card_title(card),
                                description: card.description.clone(),
                                members: format_count(card.participants),
                                quests: card.total_actions.to_string(),
                                heat_delta: heat_label(card.total_actions),
                                chips: chips_for(card),
                                reward_amount: format_thousands(card.rewards),
                                onenter: {
                                    let space_id = card.space_id.clone();
                                    EventHandler::new(move |_| {
                                        nav.push(Route::SpaceIndexPage {
                                            space_id: space_id.clone(),
                                        });
                                    })
                                },
                            }
                        }
                    }
                }

                div { class: "carousel-dots", id: "home-carousel-dots",
                    for card in cards.iter() {
                        button {
                            key: "{card.space_id.clone().to_string()}",
                            class: "carousel-dot",
                            "data-heat": heat_css_name(card.heat),
                        }
                    }
                }
            }

            // BOTTOM HUD
            div { class: "bottom-bar",
                div { class: "hud-stat",
                    div { class: "hud-stat__icon",
                        svg {
                            fill: "none",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            circle { cx: "12", cy: "12", r: "10" }
                            path { d: "M12 6v12" }
                            path { d: "M16 10H8" }
                        }
                    }
                    div { class: "hud-stat__body",
                        span { class: "hud-stat__label", "{t.hud_your_balance}" }
                        span { class: "hud-stat__value",
                            strong { {balance_text(has_user)} }
                            " "
                            small { "CR" }
                        }
                    }
                }
                div { class: "hud-stat hud-stat--rising",
                    div { class: "hud-stat__icon",
                        svg {
                            fill: "none",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            polyline { points: "23 6 13.5 15.5 8.5 10.5 1 18" }
                            polyline { points: "17 6 23 6 23 12" }
                        }
                    }
                    div { class: "hud-stat__body",
                        span { class: "hud-stat__label", "{t.hud_hot_right_now}" }
                        span { class: "hud-stat__value",
                            strong { "{active_spaces}" }
                            " "
                            small { "{t.hud_active_spaces}" }
                        }
                    }
                }
                // button {
                //     class: "browse-btn",
                //     "data-testid": "home-btn-browse",
                //     onclick: go_browse_all,
                //     svg {
                //         fill: "none",
                //         stroke: "currentColor",
                //         stroke_linecap: "round",
                //         stroke_linejoin: "round",
                //         stroke_width: "2",
                //         view_box: "0 0 24 24",
                //         xmlns: "http://www.w3.org/2000/svg",
                //         circle { cx: "11", cy: "11", r: "8" }
                //         line {
                //             x1: "21",
                //             y1: "21",
                //             x2: "16.65",
                //             y2: "16.65",
                //         }
                //     }
                //     "{t.browse_all}"
                // }
            }

            // SETTINGS PANEL — same component as Space Arena
            SettingsPanel {
                open: settings_open(),
                on_close: move |_| settings_open.set(false),
            }
        }

        if has_user {
            SuspenseBoundary {
                crate::features::notifications::components::NotificationPanel {
                    open: notifications_open(),
                    on_close: move |_| notifications_open.set(false),
                }
            }
        }
    }
}

fn heat_from_response(h: HotSpaceHeat) -> HeatLevel {
    match h {
        HotSpaceHeat::Blazing => HeatLevel::Blazing,
        HotSpaceHeat::Trending => HeatLevel::Trending,
        HotSpaceHeat::Rising => HeatLevel::Rising,
    }
}

fn heat_css_name(h: HotSpaceHeat) -> &'static str {
    match h {
        HotSpaceHeat::Blazing => "blazing",
        HotSpaceHeat::Trending => "trending",
        HotSpaceHeat::Rising => "rising",
    }
}

fn card_title(card: &HotSpaceResponse) -> String {
    if card.title.is_empty() {
        "Untitled Space".to_string()
    } else {
        card.title.clone()
    }
}

fn card_logo(card: &HotSpaceResponse, fallback: &str) -> String {
    if card.logo.is_empty() {
        fallback.to_string()
    } else {
        card.logo.clone()
    }
}

fn card_category(card: &HotSpaceResponse) -> String {
    if card.author_display_name.is_empty() {
        "Space".to_string()
    } else {
        card.author_display_name.clone()
    }
}

fn chips_for(card: &HotSpaceResponse) -> Vec<ActionChip> {
    let mut chips: Vec<ActionChip> = Vec::new();
    if card.poll_count > 0 {
        chips.push(ActionChip {
            kind: ChipKind::Poll,
            label: plural(card.poll_count, "Poll", "Polls"),
        });
    }
    if card.discussion_count > 0 {
        chips.push(ActionChip {
            kind: ChipKind::Discuss,
            label: plural(card.discussion_count, "Discussion", "Discussions"),
        });
    }
    if card.quiz_count > 0 {
        chips.push(ActionChip {
            kind: ChipKind::Quiz,
            label: plural(card.quiz_count, "Quiz", "Quizzes"),
        });
    }
    if card.follow_count > 0 {
        chips.push(ActionChip {
            kind: ChipKind::Follow,
            label: plural(card.follow_count, "Follow Quest", "Follow Quests"),
        });
    }
    chips
}

fn plural(n: i64, singular: &str, plural: &str) -> String {
    if n == 1 {
        format!("{} {}", n, singular)
    } else {
        format!("{} {}", n, plural)
    }
}

fn format_count(n: i64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}k", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

fn format_thousands(n: i64) -> String {
    let s = n.abs().to_string();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    let mut reversed: String = out.chars().rev().collect();
    if n < 0 {
        reversed.insert(0, '-');
    }
    reversed
}

fn heat_label(total_actions: i64) -> String {
    if total_actions == 0 {
        "—".to_string()
    } else {
        format!("{} quests", total_actions)
    }
}

fn balance_text(has_user: bool) -> String {
    if has_user {
        "—".to_string()
    } else {
        "0".to_string()
    }
}

#[component]
fn HomeTeamDdItem(
    username: String,
    display_name: String,
    profile_url: String,
    on_pick: EventHandler<()>,
) -> Element {
    let nav = use_navigator();
    let initial = display_name
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "T".to_string());
    let handle = format!("@{username}");

    rsx! {
        div {
            class: "team-dd__item",
            role: "button",
            tabindex: "0",
            "data-testid": "home-team-dd-item-{username}",
            onclick: move |_| {
                on_pick.call(());
                nav.push(Route::TeamHome {
                    username: username.clone(),
                });
            },
            if !profile_url.is_empty() {
                img {
                    class: "team-dd__avatar",
                    src: "{profile_url}",
                    alt: "{display_name}",
                }
            } else {
                div { class: "team-dd__avatar", "{initial}" }
            }
            div { class: "team-dd__body",
                span { class: "team-dd__name", "{display_name}" }
                span { class: "team-dd__handle", "{handle}" }
            }
        }
    }
}
