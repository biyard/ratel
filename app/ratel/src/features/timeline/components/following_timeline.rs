use crate::features::timeline::components::TimelineRow;
use crate::features::timeline::controllers::list_timeline::list_timeline_handler;
use crate::features::timeline::*;

#[component]
pub fn FollowingTimeline() -> Element {
    let feed = use_server_future(move || async move {
        list_timeline_handler("following".to_string(), None).await
    })?;

    let val = feed.read();
    let res = val.as_ref().unwrap();

    match res {
        Ok(row) if !row.items.is_empty() => {
            rsx! { TimelineRow { row: row.clone() } }
        }
        _ => rsx! {},
    }
}
