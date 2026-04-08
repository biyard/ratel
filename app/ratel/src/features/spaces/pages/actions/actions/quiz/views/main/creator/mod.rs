use crate::features::spaces::pages::actions::actions::quiz::*;
use crate::features::spaces::pages::actions::{
    ActionCommonSettings, ActionDeleteButton, ActionLockedOverlay,
};
mod i18n;
mod overview_tab;
mod quiz_tab;
mod upload_tab;
pub use i18n::QuizCreatorTranslate;
pub use overview_tab::*;
pub use quiz_tab::*;
pub use upload_tab::*;

#[component]
pub fn QuizCreatorPage(
    space_id: ReadSignal<SpacePartition>,
    quiz_id: ReadSignal<SpaceQuizEntityType>,
) -> Element {
    let tr: QuizCreatorTranslate = use_translate();
    let ctx = Context::init(space_id, quiz_id)?;
    let can_edit_quiz = ctx.quiz.read().user_response_count == 0;

    // Lock check: once the action has started, all settings become
    // read-only (backend also rejects direct API calls).
    let space = crate::features::spaces::space_common::hooks::use_space()();
    let locked = crate::features::spaces::pages::actions::is_action_locked(
        space.status,
        ctx.quiz().space_action.started_at,
    );
    let editable_if_unlocked = !locked && can_edit_quiz;

    rsx! {
        div { class: "flex min-h-0 w-full flex-1 flex-col gap-4",
            h3 { {tr.page_title} }
            Tabs { class: "min-h-0 flex-1", default_value: "overview-tab",
                TabList {
                    TabTrigger { index: 0usize, value: "overview-tab", {tr.overview_title} }
                    TabTrigger { index: 1usize, value: "upload-tab", {tr.upload_title} }
                    TabTrigger { index: 2usize, value: "quiz-tab", {tr.quiz_section_title} }
                    TabTrigger { index: 3usize, value: "setting-tab", {tr.setting_section_title} }
                }
                TabContent {
                    index: 0usize,
                    value: "overview-tab",
                    class: "flex min-h-0 flex-1",
                    ActionLockedOverlay { locked,
                        OverviewTab { can_edit: !locked }
                    }
                }
                TabContent { index: 1usize, value: "upload-tab",
                    ActionLockedOverlay { locked,
                        UploadTab { can_edit: !locked }
                    }
                }
                TabContent { index: 2usize, value: "quiz-tab",
                    ActionLockedOverlay { locked,
                        QuizTab { can_edit: editable_if_unlocked }
                    }
                }
                TabContent { index: 3usize, value: "setting-tab",
                    ActionLockedOverlay { locked,
                        div { class: "flex flex-col gap-4 w-full",
                            ActionCommonSettings {
                                space_id,
                                action_id: quiz_id().to_string(),
                                action_setting: ctx.quiz().space_action,
                                on_date_change: move |_range: DateTimeRange| async move {},
                            }
                            // Delete button hidden once the action is locked.
                            if !locked {
                                ActionDeleteButton {
                                    space_id: space_id(),
                                    action_id: quiz_id().to_string(),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
