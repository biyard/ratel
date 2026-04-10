use super::*;
use crate::features::auth::hooks::use_user_context;
use crate::features::spaces::space_common::hooks::use_space;
use crate::features::spaces::space_common::hooks::use_space_role;
use crate::features::spaces::space_common::providers::use_space_context;
use crate::spaces::pages::dashboard::SpaceDashboardPage;

const DEFAULT_LOGO: &str = "https://metadata.ratel.foundation/logos/logo-symbol.png";
const DEFAULT_PROFILE: &str = "https://metadata.ratel.foundation/ratel/default-profile.png";

#[derive(Clone, Copy, PartialEq, Default)]
pub(super) enum ActivePanel {
    #[default]
    None,
    Overview,
    Settings,
}

#[component]
pub fn SpaceIndexPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let space = use_space()();
    let role = use_space_role()();
    let user_ctx = use_user_context();
    let is_logged_in = user_ctx.read().user.is_some();
    let mut active_panel = use_signal(|| ActivePanel::None);

    if role.is_admin() {
        return rsx! {
            SpaceDashboardPage { space_id: space_id() }
        };
    }

    let logo = if space.logo.is_empty() {
        DEFAULT_LOGO.to_string()
    } else {
        space.logo.clone()
    };
    let author_profile = if space.author_profile_url.is_empty() {
        DEFAULT_PROFILE.to_string()
    } else {
        space.author_profile_url.clone()
    };
    let status_text = match space.status {
        Some(SpaceStatus::Open) => tr.status_open.to_string(),
        Some(SpaceStatus::Ongoing) => tr.status_ongoing.to_string(),
        Some(SpaceStatus::Finished) => tr.status_finished.to_string(),
        _ => tr.status_open.to_string(),
    };
    let participant_count = space.quota - space.remains;
    let participants = format_number(participant_count);
    let remaining = format_number(space.remains);
    let rewards = space
        .rewards
        .map(|r| format_number(r))
        .unwrap_or_else(|| "0".to_string());

    let dimmed = active_panel() != ActivePanel::None;
    let overview_open = active_panel() == ActivePanel::Overview;
    let settings_open = active_panel() == ActivePanel::Settings;

    let is_participant = matches!(role, SpaceUserRole::Participant | SpaceUserRole::Candidate);
    let show_participate =
        matches!(role, SpaceUserRole::Viewer) && !space.participated && space.can_participate;

    // Participants/candidates see the action dashboard
    if is_participant {
        return rsx! {
            SuspenseBoundary {
                ActionDashboard { space_id }
            }
        };
    }

    let mut ctx = use_space_context();
    let panel_requirements = ctx.panel_requirements();
    let has_unsatisfied = panel_requirements.iter().any(|r| !r.satisfied);
    let has_requirements = !panel_requirements.is_empty();
    let needs_verification = has_requirements && has_unsatisfied;

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        document::Script { src: asset!("./script.js") }

        div { class: "arena", "data-testid": "space-viewer-arena",
            div { class: "arena-ring" }
            div { class: "arena-ring arena-ring--mid" }
            div { class: "arena-ring arena-ring--inner" }

            div { class: "particle" }
            div { class: "particle particle--2" }
            div { class: "particle particle--3" }
            div { class: "particle particle--4" }
            div { class: "particle particle--5" }
            div { class: "particle particle--6" }
            div { class: "particle particle--7" }
            div { class: "particle particle--8" }

            // HUD buttons
            div { class: "hud-top-right",
                button {
                    aria_label: "{tr.overview}",
                    aria_pressed: overview_open,
                    aria_hidden: active_panel() != ActivePanel::None,
                    class: "hud-btn",
                    "data-testid": "btn-overview",
                    onclick: move |_| {
                        if active_panel() == ActivePanel::Overview {
                            active_panel.set(ActivePanel::None);
                        } else {
                            debug!("Opening overview panel");
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
                    aria_label: "{tr.settings}",
                    aria_pressed: settings_open,
                    aria_hidden: active_panel() != ActivePanel::None,
                    class: "hud-btn",
                    "data-testid": "btn-settings",
                    onclick: move |_| {
                        if active_panel() == ActivePanel::Settings {
                            active_panel.set(ActivePanel::None);
                        } else {
                            debug!("Opening settings panel");
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

            // Central portal
            div {
                class: "portal",
                "data-testid": "portal",
                "data-dimmed": dimmed,
                div { class: "portal-header",
                    img {
                        alt: "Space logo",
                        class: "portal-logo",
                        src: "{logo}",
                    }
                    h1 { class: "portal-title", "{space.title}" }
                }
                div { class: "portal-status", "{status_text}" }

                if is_logged_in && show_participate && needs_verification {
                    VerificationCard {
                        space_id,
                        requirements: panel_requirements.clone(),
                        on_verified: move |_next_requirements| {
                            ctx.space.restart();
                            ctx.role.restart();
                            ctx.panel_requirements.restart();
                        },
                    }
                } else if is_logged_in && show_participate {
                    ParticipateCard {
                        space_id,
                        participants: participants.clone(),
                        remaining: remaining.clone(),
                        rewards: rewards.clone(),
                        require_consent: has_requirements,
                        on_joined: move |_| {},
                    }
                } else if !is_logged_in {
                    SigninCard {
                        space_id,
                        participants: participants.clone(),
                        remaining: remaining.clone(),
                        rewards: rewards.clone(),
                    }
                }
            }

            // Author badge
            div { class: "portal-author", "data-dimmed": dimmed,
                img {
                    alt: "Author",
                    class: "portal-author__avatar",
                    src: "{author_profile}",
                }
                div {
                    div { class: "portal-author__name", "{space.author_display_name}" }
                    div { class: "portal-author__label", "{tr.space_creator}" }
                }
            }

            // Panels
            OverviewPanel {
                open: overview_open,
                on_close: move |_| {
                    active_panel.set(ActivePanel::None);
                },
                space: space.clone(),
                participants: participants.clone(),
                remaining: remaining.clone(),
                rewards: rewards.clone(),
            }

            SettingsPanel {
                open: settings_open,
                on_close: move |_| {
                    active_panel.set(ActivePanel::None);
                },
            }
        }
        PopupZone {}
    }
}

pub(super) fn format_number(n: i64) -> String {
    if n >= 1_000_000 {
        format!("{}M", n / 1_000_000)
    } else if n >= 1_000 {
        format!("{}K", n / 1_000)
    } else {
        n.to_string()
    }
}
