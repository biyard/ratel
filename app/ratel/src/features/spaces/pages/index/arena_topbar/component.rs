use crate::features::spaces::pages::index::*;
use crate::features::spaces::space_common::controllers::{self, update_space};
use crate::features::spaces::space_common::components::SpaceVisibilityModal;
use crate::features::spaces::space_common::providers::use_space_context;

#[component]
pub fn ArenaTopbar(
    logo: String,
    title: String,
    status_text: String,
    active_panel: Signal<ActivePanel>,
) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let mut ctx = use_space_context();
    let mut popup = use_popup();
    let real_role = ctx.role();
    let is_admin = real_role.is_admin();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let has_user = user_ctx().user.is_some();
    let is_published = ctx.space().publish_state == SpacePublishState::Published;
    let is_in_progress = ctx.space().status == Some(SpaceStatus::Open);
    let is_started = ctx.space().status == Some(SpaceStatus::Ongoing);
    let space_id = ctx.space().id;
    let overview_open = active_panel() == ActivePanel::Overview;
    let leaderboard_open = active_panel() == ActivePanel::Leaderboard;
    let settings_open = active_panel() == ActivePanel::Settings;

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "arena-topbar",
            div { class: "arena-topbar__brand",
                img {
                    class: "arena-topbar__logo",
                    src: "{logo}",
                    alt: "Space logo",
                }
                span { class: "arena-topbar__title", "{title}" }
                span { class: "arena-topbar__status", "{status_text}" }
                if is_admin {
                    span {
                        class: "arena-topbar__role",
                        "data-testid": "arena-topbar-admin-badge",
                        svg {
                            fill: "none",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            path { d: "M12 2l3 7h7l-5.5 4.5L18 21l-6-4-6 4 1.5-7.5L2 9h7z" }
                        }
                        "{tr.admin_badge}"
                    }
                }
            }
            div { class: "arena-topbar__actions",
                if has_user {
                    crate::features::notifications::components::NotificationBell {
                        class: "hud-btn",
                        onclick: move |_| {
                            if active_panel() == ActivePanel::Notifications {
                                active_panel.set(ActivePanel::None);
                            } else {
                                active_panel.set(ActivePanel::Notifications);
                            }
                        },
                    }
                }
                if is_admin {
                    if !is_published {
                        button {
                            aria_label: "{tr.publish}",
                            class: "hud-btn hud-btn--publish",
                            "data-testid": "btn-publish",
                            onclick: move |_| {
                                let initial = ctx.space().visibility;
                                popup.open(rsx! {
                                    SpaceVisibilityModal {
                                        initial,
                                        on_confirm: move |visibility| async move {
                                            let space_id = ctx.space().id;
                                            update_space(
                                                    space_id,
                                                    controllers::UpdateSpaceRequest::Publish {
                                                        publish: true,
                                                        visibility,
                                                    },
                                                )
                                                .await;
                                            ctx.space.restart();
                                        },
                                    }
                                });
                            },
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "1.5",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M4 12 L20 4 L16 12 L20 20 Z" }
                            }
                            span { class: "tooltip", "{tr.publish}" }
                        }
                    } else if is_in_progress {
                        button {
                            aria_label: "{tr.start}",
                            class: "hud-btn hud-btn--publish",
                            "data-testid": "btn-start",
                            onclick: move |_| {
                                let space_id = space_id.clone();
                                popup.open(rsx! {
                                    crate::features::spaces::space_common::components::SpaceStartModal {
                                        space_id,
                                        on_success: move |_| {
                                            ctx.space.restart();
                                        },
                                    }
                                });
                            },
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "1.5",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                polygon { points: "6 3 20 12 6 21 6 3" }
                            }
                            span { class: "tooltip", "{tr.start}" }
                        }
                    } else if is_started {
                        button {
                            aria_label: "{tr.finish}",
                            class: "hud-btn hud-btn--publish",
                            "data-testid": "btn-finish",
                            onclick: move |_| {
                                let space_id = space_id.clone();
                                popup.open(rsx! {
                                    crate::features::spaces::space_common::components::SpaceEndModal {
                                        space_id,
                                        on_success: move |_| {
                                            ctx.space.restart();
                                        },
                                    }
                                });
                            },
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "1.5",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                rect {
                                    x: "6",
                                    y: "6",
                                    width: "12",
                                    height: "12",
                                    rx: "2",
                                }
                            }
                            span { class: "tooltip", "{tr.finish}" }
                        }
                    }
                }
                button {
                    aria_label: "{tr.overview}",
                    aria_pressed: overview_open,
                    class: "hud-btn",
                    "data-testid": "btn-overview",
                    onclick: move |_| {
                        if active_panel() == ActivePanel::Overview {
                            active_panel.set(ActivePanel::None);
                        } else {
                            active_panel.set(ActivePanel::Overview);
                        }
                    },
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "1.5",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" }
                        polyline { points: "14 2 14 8 20 8" }
                        line {
                            x1: "16",
                            x2: "8",
                            y1: "13",
                            y2: "13",
                        }
                        line {
                            x1: "16",
                            x2: "8",
                            y1: "17",
                            y2: "17",
                        }
                        line {
                            x1: "10",
                            x2: "8",
                            y1: "9",
                            y2: "9",
                        }
                    }
                    span { class: "tooltip", "{tr.overview}" }
                }
                button {
                    aria_label: "{tr.leaderboard}",
                    aria_pressed: leaderboard_open,
                    class: "hud-btn",
                    "data-testid": "btn-leaderboard",
                    onclick: move |_| {
                        if active_panel() == ActivePanel::Leaderboard {
                            active_panel.set(ActivePanel::None);
                        } else {
                            active_panel.set(ActivePanel::Leaderboard);
                        }
                    },
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "1.5",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M7.21 15 2.66 7.14a2 2 0 0 1 .13-2.2L4.4 2.8A2 2 0 0 1 6 2h12a2 2 0 0 1 1.6.8l1.6 2.14a2 2 0 0 1 .14 2.2L16.79 15" }
                        path { d: "M11 12 5.12 2.2" }
                        path { d: "m13 12 5.88-9.8" }
                        path { d: "M8 7h8" }
                        circle { cx: "12", cy: "17", r: "5" }
                        path { d: "M12 18v-2h-.5" }
                    }
                    span { class: "tooltip", "{tr.leaderboard}" }
                }
                button {
                    aria_label: "{tr.settings}",
                    aria_pressed: settings_open,
                    class: "hud-btn",
                    "data-testid": "btn-settings",
                    onclick: move |_| {
                        if active_panel() == ActivePanel::Settings {
                            active_panel.set(ActivePanel::None);
                        } else {
                            active_panel.set(ActivePanel::Settings);
                        }
                    },
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "1.5",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" }
                        circle { cx: "12", cy: "12", r: "3" }
                    }
                    span { class: "tooltip", "{tr.settings}" }
                }
            }
        }
    }
}
