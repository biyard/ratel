use super::*;
use crate::features::spaces::pages::index::action_pages::quiz::*;
use crate::features::spaces::space_common::hooks::use_space;
use crate::features::spaces::space_common::hooks::use_space_role;
use crate::spaces::pages::dashboard::SpaceDashboardPage;

const DEFAULT_LOGO: &str = "https://metadata.ratel.foundation/logos/logo-symbol.png";

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ActivePanel {
    #[default]
    None,
    Overview,
    Leaderboard,
    Settings,
}

#[component]
pub fn SpaceIndexPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let space = use_space()();
    let role = use_space_role()();
    let mut active_panel = use_signal(|| ActivePanel::None);
    let action_overlay = use_context_provider(|| ActiveActionOverlaySignal(Signal::new(None)));
    let _completed_quiz = use_context_provider(|| CompletedQuizAction(Signal::new(None)));

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

    rsx! {
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

        div { class: "arena", "data-testid": "space-index-page",
            ArenaTopbar {
                logo: logo.clone(),
                title: space.title.clone(),
                status_text: status_text.clone(),
                active_panel,
            }

            if matches!(role, SpaceUserRole::Participant) {
                SuspenseBoundary {
                    ActionDashboard { space_id }
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
                space: space.clone(),
                participants: participants.clone(),
                remaining: remaining.clone(),
                rewards: rewards.clone(),
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
            None => rsx! {},
        }
        PopupZone {}
    }
}

#[component]
fn CandidateView(space_id: ReadSignal<SpacePartition>) -> Element {
    let actions = use_loader(move || async move {
        crate::features::spaces::pages::actions::controllers::list_actions(space_id()).await
    })?;
    let actions = actions();

    let prereqs: Vec<_> = actions.iter().filter(|a| a.prerequisite).cloned().collect();
    let all_done = prereqs.is_empty() || prereqs.iter().all(|a| a.user_participated);

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
