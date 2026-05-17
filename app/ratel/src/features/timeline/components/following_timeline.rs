use crate::features::timeline::components::TimelineRow;
use crate::features::timeline::controllers::list_timeline::list_timeline_handler;
use crate::features::timeline::*;

#[component]
pub fn FollowingTimeline() -> Element {
    let row =
        use_loader(
            move || async move { list_timeline_handler("following".to_string(), None).await },
        )?;

    rsx! {
        TimelineRow { row: row() }

    }
}
