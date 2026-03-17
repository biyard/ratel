use super::*;

mod content;
mod i18n;
pub use content::PollContent;

#[component]
pub fn PollParticipantPage(
    space_id: ReadSignal<SpacePartition>,
    poll_id: ReadSignal<SpacePollEntityType>,
) -> Element {
    rsx! {
        PollContent { space_id, poll_id, can_respond: true }
    }
}
