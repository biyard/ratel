use super::space_card::*;
use super::*;
use crate::common::components::{Robots, SeoMeta};
use crate::common::types::ListResponse;
use crate::common::use_loader;
use crate::features::auth::LoginModal;
use crate::features::posts::controllers::create_post::create_post_handler;
use crate::features::spaces::pages::index::SettingsPanel;
use crate::features::spaces::space_common::controllers::{
    HotSpaceHeat, HotSpaceResponse, list_hot_spaces_handler, list_my_home_spaces_handler,
};
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
    let mut settings_open = use_signal(|| false);

    let keywords = vec![
        "ratel".to_string(),
        "knowledge platform".to_string(),
        "ai knowledge base".to_string(),
        "hot spaces".to_string(),
        "participatory platform".to_string(),
        "survey rewards".to_string(),
        "poll rewards".to_string(),
        "web3 knowledge economy".to_string(),
        "collective intelligence".to_string(),
    ];

    let brand_logo = "https://metadata.ratel.foundation/logos/logo-symbol.png".to_string();

    let hot_spaces = use_loader(move || async move {
        list_hot_spaces_handler(None).await
    })?;
    let my_spaces = use_loader(move || async move {
        if has_user {
            list_my_home_spaces_handler(None).await
        } else {
            Ok(ListResponse::default())
        }
    })?;

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
                    LoginModal {}
                })
                .with_title("Join the movement");
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
                    LoginModal {}
                })
                .with_title("Join the movement");
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
                    LoginModal {}
                })
                .with_title("Join the movement");
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
                    LoginModal {}
                })
                .with_title("Join the movement");
            return;
        }
        nav.push(Route::CredentialsHome {});
    };

    let open_settings = move |_: Event<MouseData>| {
        settings_open.set(true);
    };

    let go_browse_all = move |_: Event<MouseData>| {
        nav.push(Route::PostIndex {});
    };

    rsx! {
        SeoMeta {
            title: "Ratel – Hot Spaces Arena",
            description: "Enter the arena. Vote in polls, join discussions, complete quests, and earn rewards across the hottest decentralized communities on Ratel.",
            image: "https://metadata.ratel.foundation/logos/logo-symbol.png",
            url: "https://ratel.foundation",
            robots: Robots::IndexNofollow,
            keywords,
        }

        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "anonymous",
        }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Orbitron:wght@400;500;600;700;800;900&family=Outfit:wght@300;400;500;600;700&display=swap",
        }
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
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
                    if !has_user {
                        button {
                            class: "hud-btn hud-btn--signin",
                            aria_label: "{t.sign_in}",
                            "data-testid": "home-btn-signin",
                            onclick: move |_| {
                                popup.open(rsx! {
                                    LoginModal {}
                                }).with_title("Join the movement");
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
                button {
                    class: "browse-btn",
                    "data-testid": "home-btn-browse",
                    onclick: go_browse_all,
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        circle { cx: "11", cy: "11", r: "8" }
                        line {
                            x1: "21",
                            y1: "21",
                            x2: "16.65",
                            y2: "16.65",
                        }
                    }
                    "{t.browse_all}"
                }
            }

            // SETTINGS PANEL — same component as Space Arena
            SettingsPanel {
                open: settings_open(),
                on_close: move |_| settings_open.set(false),
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
    if has_user { "—".to_string() } else { "0".to_string() }
}
