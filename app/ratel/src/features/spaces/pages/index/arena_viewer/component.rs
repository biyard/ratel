use crate::features::auth::hooks::use_user_context;
use crate::features::spaces::pages::index::*;
use crate::features::spaces::space_common::controllers::SpaceResponse;
use crate::features::spaces::space_common::hooks::use_space;
use crate::features::spaces::space_common::providers::use_space_context;

const DEFAULT_LOGO: &str = "https://metadata.ratel.foundation/logos/logo-symbol.png";
const DEFAULT_PROFILE: &str = "https://metadata.ratel.foundation/ratel/default-profile.png";

#[component]
pub fn ArenaViewer(
    space_id: ReadSignal<SpacePartition>,
    dimmed: bool,
    #[props(default)] candidate_view: Option<Element>,
) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let space = use_space()();
    let user_ctx = use_user_context();
    let is_logged_in = user_ctx.read().user.is_some();

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
    let participants = format_number(space.participants);
    let remaining = if space.quota == 0 {
        "-".to_string()
    } else {
        format_number(space.remains)
    };
    let rewards = space
        .rewards
        .map(|r| format_number(r))
        .unwrap_or_else(|| "0".to_string());

    let show_participate = !space.participated && space.can_participate;

    let mut ctx = use_space_context();
    let panel_requirements = ctx.panel_requirements();
    let has_unsatisfied = panel_requirements.iter().any(|r| !r.satisfied);
    let has_requirements = !panel_requirements.is_empty();
    let needs_verification = has_requirements && has_unsatisfied;

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div { class: "arena-viewer",
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

                if let Some(view) = candidate_view {
                    {view}
                } else if is_logged_in && show_participate && needs_verification {
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
        }
    }
}
