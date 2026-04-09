use super::*;

mod content;
mod i18n;
pub use content::PollContent;

use crate::features::spaces::pages::actions::gamification::components::quest_briefing::QuestBriefing;
use crate::features::spaces::pages::actions::gamification::hooks::use_quest_briefing;

#[component]
pub fn PollParticipantPage(
    space_id: ReadSignal<SpacePartition>,
    poll_id: ReadSignal<SpacePollEntityType>,
) -> Element {
    let (show_briefing, dismiss) = use_quest_briefing();
    let nav = navigator();

    if show_briefing {
        let node = QuestNodeView {
            id: poll_id().to_string(),
            action_type: SpaceActionType::Poll,
            title: String::new(),
            base_points: 0,
            projected_xp: 0,
            status: QuestNodeStatus::Active,
            depends_on: vec![],
            chapter_id: String::new(),
            started_at: None,
            ended_at: None,
            quiz_result: None,
        };
        rsx! {
            QuestBriefing {
                node,
                on_begin: move |_| dismiss.call(()),
                on_cancel: move |_| {
                    nav.go_back();
                },
            }
        }
    } else {
        rsx! {
            PollContent { space_id, poll_id, can_respond: true }
        }
    }
}
