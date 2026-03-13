use crate::features::spaces::pages::actions::actions::discussion::*;
use crate::features::spaces::pages::actions::ActionCommonSettings;
mod i18n;
mod overview_tab;
pub use i18n::DiscussionCreatorTranslate;
pub use overview_tab::*;

#[component]
pub fn CreatorMain(
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
) -> Element {
    let tr: DiscussionCreatorTranslate = use_translate();
    let ctx = use_discussion_context();

    rsx! {
        div { class: "flex flex-col flex-1 gap-4 w-full min-h-0",
            h3 { {tr.page_title} }
            Tabs { class: "flex-1 min-h-0", default_value: "overview-tab",
                TabList {
                    TabTrigger { index: 0usize, value: "overview-tab", {tr.tab_overview} }
                    TabTrigger { index: 1usize, value: "setting-tab", {tr.tab_setting} }
                }
                TabContent {
                    index: 0usize,
                    value: "overview-tab",
                    class: "flex flex-1 min-h-0",
                    OverviewTab {}
                }
                TabContent { index: 1usize, value: "setting-tab",
                    ActionCommonSettings {
                        space_id,
                        action_id: discussion_id().to_string(),
                        action_setting: ctx.discussion().space_action,
                        on_date_change: move |range: DateTimeRange| async move {
                            let space_id = space_id();
                            let discussion_id = discussion_id();
                            if let (Some(start_date), Some(end_date)) = (range.start_date, range.end_date) {
                                let started_at = date_time_to_millis(
                                    start_date,
                                    range.start_hour,
                                    range.start_minute,
                                );
                                let ended_at = date_time_to_millis(
                                    end_date,
                                    range.end_hour,
                                    range.end_minute,
                                );
                                let req = UpdateDiscussionRequest {
                                    title: None,
                                    html_contents: None,
                                    category_name: None,
                                    started_at: Some(started_at),
                                    ended_at: Some(ended_at),
                                };
                                let _ = update_discussion(space_id, discussion_id, req).await;
                            }
                        },
                    }
                }
            }
        }
    }
}

fn date_time_to_millis(date: time::Date, hour: u8, minute: u8) -> i64 {
    let datetime = date.with_hms(hour, minute, 0).expect("valid time");
    let offset_datetime = datetime.assume_utc();
    (offset_datetime.unix_timestamp()) * 1000
}
