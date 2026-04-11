use super::*;

mod content;
mod i18n;
pub use content::PollContent;

#[component]
pub fn PollParticipantPage(
    space_id: ReadSignal<SpacePartition>,
    poll_id: ReadSignal<SpacePollEntityType>,
) -> Element {
    // Participants see the new gamified viewer.
    rsx! {
        crate::features::spaces::pages::index::ActionPollViewer {
            space_id,
            poll_id,
            can_respond: true,
        }
    }
}
