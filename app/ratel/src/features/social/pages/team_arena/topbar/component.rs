use crate::common::*;
use crate::features::social::pages::team_arena::create_team_popup::ArenaTeamCreationPopup;
use crate::features::social::pages::team_arena::i18n::TeamArenaTranslate;
use crate::route::Route;

/// Which HUD button should be rendered as "active" in the topbar.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TeamArenaTab {
    Home,
    Drafts,
    Members,
    Rewards,
    Settings,
}

/// Arena-style topbar shared across every team page: back button, team switcher,
/// status chip, HUD buttons, and a Follow action.
#[component]
pub fn ArenaTopbar(
    /// The team's username/handle (used for routing).
    username: String,
    /// Display name shown inside the switcher title slot.
    display_name: String,
    /// Profile image URL; empty string falls back to an initial.
    profile_url: String,
    /// Which HUD tab is currently active.
    active: TeamArenaTab,
    /// Whether the viewer can access admin-only tabs (Drafts, Settings).
    #[props(default = false)]
    can_edit: bool,
    /// Whether the Follow action should show "Unfollow" instead.
    #[props(default = false)]
    is_following: bool,
    /// Whether the Follow button should be rendered at all (hidden for team members).
    #[props(default = true)]
    show_follow: bool,
    /// Optional click handler for the Follow button.
    #[props(default)]
    on_follow: EventHandler<()>,
    /// Click handler for the Settings HUD button — toggles the side panel.
    #[props(default)]
    on_open_settings: EventHandler<()>,
) -> Element {
    let tr: TeamArenaTranslate = use_translate();
    let nav = use_navigator();
    let team_ctx = crate::common::contexts::use_team_context();
    let mut popup = use_popup();

    let mut dd_open = use_signal(|| false);

    let initial = display_name
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "T".to_string());

    let handle = format!("@{username}");

    // Back button exits the team arena and returns to the app index.
    let on_back = move |_| {
        nav.push(Route::Index {});
    };

    let go_home_root = move |_| {
        nav.push(Route::Index {});
    };

    // Build the dropdown list. Always include the current team so the list never
    // toggles between empty/populated branches (which breaks Dioxus's element
    // reconciler and causes "cannot reclaim ElementId" errors).
    let dd_entries: Vec<DdEntry> = {
        let ctx_items = team_ctx.teams.read();
        let mut entries: Vec<DdEntry> = ctx_items
            .iter()
            .map(|t| DdEntry {
                username: t.username.clone(),
                display_name: if t.nickname.is_empty() {
                    t.username.clone()
                } else {
                    t.nickname.clone()
                },
                profile_url: t.profile_url.clone(),
            })
            .collect();
        if !entries.iter().any(|e| e.username == username) {
            entries.insert(
                0,
                DdEntry {
                    username: username.clone(),
                    display_name: display_name.clone(),
                    profile_url: profile_url.clone(),
                },
            );
        }
        entries
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div { class: "arena-topbar",
            div { class: "arena-topbar__brand",
                button {
                    class: "arena-topbar__back",
                    aria_label: "{tr.back}",
                    r#type: "button",
                    onclick: on_back,
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        line {
                            x1: "19",
                            y1: "12",
                            x2: "5",
                            y2: "12",
                        }
                        polyline { points: "12 19 5 12 12 5" }
                    }
                }
                button {
                    class: "arena-topbar__back",
                    aria_label: "{tr.home}",
                    r#type: "button",
                    onclick: go_home_root,
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M3 9l9-7 9 7v11a2 2 0 0 1-2 2h-4a2 2 0 0 1-2-2v-6h-2v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" }
                    }
                }
                span { class: "arena-topbar__divider" }

                div {
                    class: "arena-topbar__switcher",
                    role: "button",
                    tabindex: "0",
                    aria_expanded: dd_open(),
                    onclick: move |e: Event<MouseData>| {
                        e.stop_propagation();
                        dd_open.toggle();
                    },
                    if !profile_url.is_empty() {
                        img {
                            class: "arena-topbar__logo",
                            src: "{profile_url}",
                            alt: "{display_name}",
                        }
                    } else {
                        div { class: "arena-topbar__logo", "{initial}" }
                    }
                    div { class: "arena-topbar__switcher-body",
                        span { class: "arena-topbar__title", "{display_name}" }
                        span { class: "arena-topbar__handle", "{handle}" }
                    }
                    svg {
                        class: "arena-topbar__chevron",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2.5",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "6 9 12 15 18 9" }
                    }

                    div {
                        class: "team-dd",
                        role: "menu",
                        onclick: move |e: Event<MouseData>| e.stop_propagation(),
                        div { class: "team-dd__header", "Switch Team" }
                        for (idx , entry) in dd_entries.iter().cloned().enumerate() {
                            TeamDdItem {
                                key: "{entry.username}",
                                username: entry.username.clone(),
                                display_name: entry.display_name.clone(),
                                profile_url: entry.profile_url.clone(),
                                is_current: entry.username == username,
                                color_variant: (idx % 3) as u8,
                                on_pick: move |_| {
                                    dd_open.set(false);
                                },
                            }
                        }
                        div {
                            class: "team-dd__footer",
                            role: "button",
                            tabindex: "0",
                            onclick: move |_| {
                                dd_open.set(false);
                                popup
                                    .open(rsx! {
                                        ArenaTeamCreationPopup {}
                                    })
                                    .without_close()
                                    .with_backdrop_close();
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
                            "Create Team"
                        }
                    }
                }
                if dd_open() {
                    div {
                        style: "position:fixed;inset:0;z-index:25;",
                        onclick: move |_| {
                            dd_open.set(false);
                        },
                    }
                }

                span { class: "arena-topbar__status", "{tr.status_active}" }
            }

            div { class: "arena-topbar__actions",
                if can_edit {
                    HudButton {
                        label: tr.drafts.to_string(),
                        active: active == TeamArenaTab::Drafts,
                        to: Route::TeamDraft {
                            username: username.clone(),
                        },
                        icon: rsx! {
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                                polyline { points: "14 2 14 8 20 8" }
                                line {
                                    x1: "16",
                                    y1: "13",
                                    x2: "8",
                                    y2: "13",
                                }
                                line {
                                    x1: "16",
                                    y1: "17",
                                    x2: "8",
                                    y2: "17",
                                }
                                polyline { points: "10 9 9 9 8 9" }
                            }
                        },
                    }
                    span { class: "arena-topbar__divider", style: "margin:0 2px" }
                }

                HudButton {
                    label: tr.members.to_string(),
                    active: active == TeamArenaTab::Members,
                    to: Route::TeamMember {
                        username: username.clone(),
                    },
                    icon: rsx! {
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" }
                            circle { cx: "9", cy: "7", r: "4" }
                            path { d: "M23 21v-2a4 4 0 0 0-3-3.87" }
                            path { d: "M16 3.13a4 4 0 0 1 0 7.75" }
                        }
                    },
                }

                HudButton {
                    label: tr.rewards.to_string(),
                    active: active == TeamArenaTab::Rewards,
                    to: Route::TeamReward {
                        username: username.clone(),
                    },
                    icon: rsx! {
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            circle { cx: "12", cy: "8", r: "7" }
                            polyline { points: "8.21 13.89 7 23 12 20 17 23 15.79 13.88" }
                        }
                    },
                }

                button {
                    class: "arena-topbar__hud",
                    r#type: "button",
                    aria_label: "{tr.settings}",
                    onclick: move |_| on_open_settings.call(()),
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" }
                        circle { cx: "12", cy: "12", r: "3" }
                    }
                    span { class: "arena-topbar__hud-label", "{tr.settings}" }
                }

                if show_follow {
                    button {
                        class: "arena-topbar__follow",
                        r#type: "button",
                        onclick: move |_| on_follow.call(()),
                        if is_following {
                            "{tr.unfollow}"
                        } else {
                            "{tr.follow}"
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct DdEntry {
    username: String,
    display_name: String,
    profile_url: String,
}

#[component]
fn TeamDdItem(
    username: String,
    display_name: String,
    profile_url: String,
    #[props(default = false)] is_current: bool,
    #[props(default = 0)] color_variant: u8,
    on_pick: EventHandler<()>,
) -> Element {
    let nav = use_navigator();
    let initial = display_name
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "T".to_string());
    let handle = format!("@{username}");
    let avatar_class = match color_variant {
        1 => "team-dd__avatar team-dd__avatar--alt1",
        2 => "team-dd__avatar team-dd__avatar--alt2",
        _ => "team-dd__avatar",
    };

    rsx! {
        div {
            class: "team-dd__item",
            role: "button",
            tabindex: "0",
            onclick: move |_| {
                on_pick.call(());
                if !is_current {
                    nav.push(Route::TeamHome {
                        username: username.clone(),
                    });
                }
            },
            if !profile_url.is_empty() {
                img {
                    class: "{avatar_class}",
                    src: "{profile_url}",
                    alt: "{display_name}",
                }
            } else {
                div { class: "{avatar_class}", "{initial}" }
            }
            div { class: "team-dd__body",
                span { class: "team-dd__name", "{display_name}" }
                span { class: "team-dd__handle", "{handle}" }
            }
            if is_current {
                svg {
                    class: "team-dd__check",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "3",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    polyline { points: "20 6 9 17 4 12" }
                }
            }
        }
    }
}

#[component]
fn HudButton(label: String, active: bool, to: Route, icon: Element) -> Element {
    let class = if active {
        "arena-topbar__hud arena-topbar__hud--active"
    } else {
        "arena-topbar__hud"
    };

    rsx! {
        Link { to, class: "{class}", aria_label: "{label}",
            {icon}
            span { class: "arena-topbar__hud-label", "{label}" }
        }
    }
}
