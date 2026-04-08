use crate::features::spaces::pages::actions::actions::discussion::*;
use crate::features::spaces::pages::actions::{
    ActionCommonSettings, ActionDeleteButton, ActionLockedOverlay,
};
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

    // Lock check: once the action has started, the config UI becomes
    // read-only via `ActionLockedOverlay`.
    let space = crate::features::spaces::space_common::hooks::use_space()();
    let locked = crate::features::spaces::pages::actions::is_action_locked(
        space.status,
        ctx.discussion().space_action.started_at,
    );

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
                    ActionLockedOverlay { locked,
                        OverviewTab {}
                    }
                }
                TabContent { index: 1usize, value: "setting-tab",
                    ActionLockedOverlay { locked,
                        div { class: "flex flex-col gap-5 w-full",
                            ActionCommonSettings {
                                space_id,
                                action_id: discussion_id().to_string(),
                                action_setting: ctx.discussion().space_action,
                                on_date_change: move |range: DateTimeRange| async move {
                                    let space_id = space_id();
                                    let discussion_id = discussion_id();
                                    if let (Some(start_date), Some(end_date)) = (range.start_date, range.end_date) {
                                        let started_at = range
                                            .timezone
                                            .local_to_utc_millis(start_date, range.start_hour, range.start_minute);
                                        let ended_at = range
                                            .timezone
                                            .local_to_utc_millis(end_date, range.end_hour, range.end_minute);
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
                            crate::features::ai_moderator::AiModeratorSetting {
                                space_id,
                                discussion_id: use_memo(move || SpaceDiscussionEntityType(discussion_id().to_string())),
                            }
                            // Delete button is hidden once the action is locked.
                            if !locked {
                                ActionDeleteButton {
                                    space_id: space_id(),
                                    action_id: discussion_id().to_string(),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

