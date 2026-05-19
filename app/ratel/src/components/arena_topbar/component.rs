use super::*;
use crate::common::*;
use crate::features::auth::LoginModal;
use crate::features::posts::controllers::create_post::create_post_handler;
use crate::*;

/// Currently-active section indicator for the [`RatelArenaTopbar`].
///
/// The variant whose route matches the current page receives
/// `aria-current="page"`, which the global `.hud-btn[aria-current="page"]`
/// CSS rules in `main.css` use to highlight the active button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RatelArenaTopbarSection {
    Home,
    Character,
}

/// Lightweight reusable topbar shared between the Home Arena and
/// per-feature arena pages (currently the Character page).
///
/// The Home Arena currently keeps its own inline topbar (with the Teams
/// dropdown, NotificationBell, admin shield, and inline SettingsPanel
/// integration). This component is the minimum viable shared topbar for
/// pages that don't need that full surface area — it preserves the same
/// visual language (`arena-topbar`, `hud-btn`, `hud-btn__label`) so the
/// CSS in `main.css` styles both topbars identically.
#[component]
pub fn RatelArenaTopbar(active: Option<RatelArenaTopbarSection>) -> Element {
    let t: RatelArenaTopbarTranslate = use_translate();
    let nav = use_navigator();
    let mut popup = use_popup();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let username = user_ctx()
        .user
        .as_ref()
        .map(|u| u.username.clone())
        .unwrap_or_default();
    let has_user = user_ctx().user.is_some();

    let brand_logo = "https://metadata.ratel.foundation/logos/logo-symbol.png".to_string();

    let on_login_success: Callback<()> = use_callback(move |_| {});

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
        nav.push(Route::SocialDraft {
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
        nav.push(Route::SocialReward {
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

    let go_character = move |_: Event<MouseData>| {
        if !has_user {
            popup
                .open(rsx! {
                    LoginModal { on_success: on_login_success }
                })
                .with_title("Start building your Essence");
            return;
        }
        nav.push(Route::CharacterPage {});
    };

    let go_my_ai = move |_: Event<MouseData>| {
        if !has_user {
            popup
                .open(rsx! {
                    LoginModal { on_success: on_login_success }
                })
                .with_title("Start building your Essence");
            return;
        }
        nav.push(Route::MyAiPage {});
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

    let go_home_settings = move |_: Event<MouseData>| {
        // The settings panel is owned by the Home Arena view; this
        // shared topbar simply navigates back home where the panel is
        // available. A follow-up can lift SettingsPanel into a global
        // overlay so any arena page can open it in place.
        nav.push(Route::Index {});
    };

    let connections_username = username.clone();
    let go_connections = move |_: Event<MouseData>| {
        if !has_user {
            popup
                .open(rsx! {
                    LoginModal { on_success: on_login_success }
                })
                .with_title("Start building your Essence");
            return;
        }
        nav.push(Route::UserSettingsConnectionsPage {
            username: connections_username.clone(),
        });
    };

    let is_home = active == Some(RatelArenaTopbarSection::Home);
    let is_character = active == Some(RatelArenaTopbarSection::Character);

    rsx! {
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
                    aria_current: is_home.then_some("page"),
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
                // Character — XP & skills HUD. Same award-medal SVG used
                // in the design mockup at
                // `app/ratel/assets/design/character-xp-skills/character-page.html`.
                button {
                    class: "hud-btn",
                    aria_label: "{t.character}",
                    aria_current: is_character.then_some("page"),
                    "data-testid": "home-btn-character",
                    onclick: go_character,
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "1.6",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        circle { cx: "12", cy: "8", r: "6" }
                        path { d: "M15.477 12.89 17 22l-5-3-5 3 1.523-9.11" }
                    }
                    span { class: "hud-btn__label", "{t.character}" }
                }
                button {
                    class: "hud-btn hud-btn--ai",
                    aria_label: "{t.my_ai}",
                    "data-testid": "home-btn-my-ai",
                    onclick: go_my_ai,
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
                button {
                    class: "hud-btn",
                    aria_label: "{t.connections}",
                    "data-testid": "home-btn-connections",
                    onclick: go_connections,
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "1.6",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" }
                        path { d: "M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.72-1.71" }
                    }
                    span { class: "hud-btn__label", "{t.connections}" }
                }
                button {
                    class: "hud-btn",
                    aria_label: "{t.settings}",
                    "data-testid": "home-btn-settings",
                    onclick: go_home_settings,
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
    }
}
