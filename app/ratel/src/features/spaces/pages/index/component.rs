use super::*;
use crate::features::spaces::pages::index::action_pages::quiz::*;
use crate::features::spaces::space_common::hooks::use_space;
use crate::features::spaces::space_common::hooks::use_space_role;

const DEFAULT_LOGO: &str = "https://metadata.ratel.foundation/logos/logo-symbol.png";

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ActivePanel {
    #[default]
    None,
    Overview,
    Leaderboard,
    Settings,
    Notifications,
}

#[component]
pub fn SpaceIndexPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let space = use_space()();
    let role = use_space_role()();
    let ctx = crate::features::spaces::space_common::providers::use_space_context();
    let real_role = (ctx.role)();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let has_user = user_ctx().user.is_some();
    let mut active_panel = use_signal(|| ActivePanel::None);
    let action_overlay = use_context_provider(|| ActiveActionOverlaySignal(Signal::new(None)));
    let _completed_quiz = use_context_provider(|| CompletedActionCard(Signal::new(None)));

    let is_admin = real_role.is_admin();

    let logo = if space.logo.is_empty() {
        DEFAULT_LOGO.to_string()
    } else {
        space.logo.clone()
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
    let leaderboard_open = active_panel() == ActivePanel::Leaderboard;
    let settings_open = active_panel() == ActivePanel::Settings;
    let notifications_open = active_panel() == ActivePanel::Notifications;

    rsx! {
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "anonymous",
        }
        document::Link {
            rel: "preload",
            href: "https://fonts.googleapis.com/css2?family=Orbitron:wght@400;500;600;700;800;900&family=Outfit:wght@300;400;500;600;700&display=swap",
            r#as: "style",
        }
        document::Stylesheet { href: "https://fonts.googleapis.com/css2?family=Orbitron:wght@400;500;600;700;800;900&family=Outfit:wght@300;400;500;600;700&display=swap" }
        document::Stylesheet { href: asset!("./style.css") }
        document::Stylesheet { href: asset!("./style.css") }
        // Preload sub-component CSS to prevent flash of unstyled content
        document::Link {
            rel: "preload",
            href: asset!("./arena_topbar/style.css"),
            r#as: "style",
        }
        document::Link {
            rel: "preload",
            href: asset!("./arena_viewer/style.css"),
            r#as: "style",
        }
        document::Link {
            rel: "preload",
            href: asset!("./prerequisite_card/style.css"),
            r#as: "style",
        }
        document::Link {
            rel: "preload",
            href: asset!("./action_dashboard/style.css"),
            r#as: "style",
        }
        document::Link {
            rel: "preload",
            href: asset!("./action_pages/poll/style.css"),
            r#as: "style",
        }
        document::Link {
            rel: "preload",
            href: asset!("./action_pages/quiz/style.css"),
            r#as: "style",
        }
        document::Link {
            rel: "preload",
            href: asset!("./action_pages/discussion/style.css"),
            r#as: "style",
        }
        // Slide-in panels — preloaded here so the CSS is already cached
        // when the child components mount on SPA navigation. Without
        // these, the panel `<div>` renders in the DOM before its
        // stylesheet finishes fetching, so `position: absolute;
        // z-index: 25` isn't applied and the panel visually collapses
        // into normal flow underneath the HUD topbar. Full-page reloads
        // don't hit this race because the server-rendered HTML already
        // has every stylesheet in `<head>`.
        document::Link {
            rel: "preload",
            href: asset!("./overview_panel/style.css"),
            r#as: "style",
        }
        document::Link {
            rel: "preload",
            href: asset!("./leaderboard_panel/style.css"),
            r#as: "style",
        }
        document::Link {
            rel: "preload",
            href: asset!("./settings_panel/style.css"),
            r#as: "style",
        }

        div { class: "arena", "data-testid": "space-index-page",
            ArenaTopbar {
                logo: logo.clone(),
                title: space.title.clone(),
                status_text: status_text.clone(),
                active_panel,
            }

            if matches!(role, SpaceUserRole::Participant | SpaceUserRole::Creator) {
                SuspenseBoundary {
                    ActionDashboard { space_id, is_admin }
                }
            } else if matches!(role, SpaceUserRole::Candidate) {
                ArenaViewer {
                    space_id,
                    dimmed,
                    candidate_view: rsx! {
                        SuspenseBoundary {
                            CandidateView { space_id }
                        }
                    },
                }
            } else {
                ArenaViewer { space_id, dimmed }
            }

            // Panels (shared)
            OverviewPanel {
                open: overview_open,
                on_close: move |_| {
                    active_panel.set(ActivePanel::None);
                },
                remaining: remaining.clone(),
                rewards: rewards.clone(),
                is_admin,
                space_id,
            }

            LeaderboardPanel {
                space_id,
                open: leaderboard_open,
                on_close: move |_| {
                    active_panel.set(ActivePanel::None);
                },
            }

            SettingsPanel {
                open: settings_open,
                on_close: move |_| {
                    active_panel.set(ActivePanel::None);
                },
                is_admin,
                space_id,
            }

            if has_user {
                SuspenseBoundary {
                    crate::features::notifications::components::NotificationPanel {
                        open: notifications_open,
                        on_close: move |_| {
                            active_panel.set(ActivePanel::None);
                        },
                    }
                }
            }
        }
        match action_overlay.0() {
            Some(ActiveActionOverlay::Quiz(sid, qid)) => rsx! {
                div { class: "fixed inset-0 z-[100]", "data-testid": "quiz-arena-overlay",
                    SuspenseBoundary {
                        QuizArenaPage { space_id: sid.clone(), quiz_id: qid.clone() }
                    }
                }
            },
            Some(ActiveActionOverlay::Poll(sid, pid)) => rsx! {
                div { class: "fixed inset-0 z-[100]", "data-testid": "poll-arena-overlay",
                    SuspenseBoundary {
                        ActionPollViewer { space_id: sid.clone(), poll_id: pid.clone(), can_respond: true }
                    }
                }
            },
            Some(ActiveActionOverlay::Discussion(sid, did)) => rsx! {
                div { class: "fixed inset-0 z-[100]",
                    SuspenseBoundary {
                        DiscussionArenaPage { space_id: sid.clone(), discussion_id: did.clone() }
                    }
                }
            },
            None => rsx! {},
        }
    }
}

#[component]
fn CandidateView(space_id: ReadSignal<SpacePartition>) -> Element {
    let actions_loader = crate::features::spaces::space_common::hooks::use_actions();
    let actions = actions_loader();
    let space = use_space()();
    let mut ctx = crate::features::spaces::space_common::providers::use_space_context();

    let prereqs: Vec<_> = actions.iter().filter(|a| a.prerequisite).cloned().collect();
    let all_done = prereqs.is_empty() || prereqs.iter().all(|a| a.user_participated);

    // When all prerequisites are done and space is Ongoing (join_anytime),
    // restart the full context so `role` reloads from the server. The
    // server will return Participant instead of Candidate, causing the
    // parent to render ActionDashboard instead of CandidateView.
    let mut already_restarted = use_signal(|| false);
    if all_done && matches!(space.status, Some(SpaceStatus::Ongoing)) && !already_restarted() {
        already_restarted.set(true);
        ctx.restart();
    }

    if all_done {
        rsx! {
            WaitingCard { prereqs }
        }
    } else {
        rsx! {
            PrerequisiteCard { space_id }
        }
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
