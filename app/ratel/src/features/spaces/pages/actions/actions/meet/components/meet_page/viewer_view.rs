use crate::features::spaces::pages::actions::SpaceActionStatus;
use crate::features::spaces::pages::actions::actions::meet::components::meet_page::*;
use crate::*;

#[component]
pub fn MeetViewerView() -> Element {
    let tr: MeetActionTranslate = use_translate();
    let UseMeet { meet, .. } = use_context::<UseMeet>();
    let current = meet();
    let title = current.space_action.title.clone();
    let description = current.space_action.description.clone();
    let status = current
        .space_action
        .status
        .clone()
        .unwrap_or(SpaceActionStatus::Designing);
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let start_time = current.start_time;
    let duration = current.duration_min as i64;
    let is_live = matches!(status, SpaceActionStatus::Ongoing)
        && start_time <= now
        && now < start_time + duration * 60_000;
    let is_scheduled = matches!(status, SpaceActionStatus::Ongoing) && now < start_time;
    let is_ended = matches!(status, SpaceActionStatus::Finish);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        SeoMeta { title: "{title}" }
        div { class: "meet-viewer", "data-testid": "meet-viewer-view",
            h1 { class: "meet-viewer__title", "{title}" }
            p { class: "meet-viewer__desc", "{description}" }

            if is_scheduled {
                div { class: "meet-viewer__scheduled",
                    span { class: "meet-viewer__scheduled-label", "{tr.scheduled_starts_at}" }
                    span { class: "meet-viewer__scheduled-ts", "{start_time}" }
                }
            } else if is_live {
                div { class: "meet-viewer__live",
                    span { class: "meet-viewer__live-label", "{tr.live_label}" }
                    span { class: "meet-viewer__coming-soon", "{tr.coming_soon_badge}" }
                    Button { disabled: true, "data-testid": "meet-live-join", "{tr.live_cta}" }
                }
            } else if is_ended {
                div { class: "meet-viewer__ended",
                    span { class: "meet-viewer__ended-label", "{tr.ended_label}" }
                    span { class: "meet-viewer__coming-soon", "{tr.coming_soon_badge}" }
                    Button { disabled: true, "data-testid": "meet-ended-archive", "{tr.ended_cta}" }
                }
            }
        }
    }
}
