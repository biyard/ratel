use crate::features::spaces::pages::actions::actions::quiz::*;
use crate::features::spaces::pages::actions::{ActionCommonSettings, ActionDeleteButton};
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
    // `can_edit_quiz` is the long-standing guard that prevents quiz
    // questions/answers from being mutated once participants have
    // started responding (it would invalidate scores). Lifecycle lock
    // is no longer applied here — creators can keep tweaking even
    // after the action starts.
    let can_edit_quiz = ctx.quiz.read().user_response_count == 0;

    // The delete button is the only thing that respects the lifecycle
    // lock now: once the action has started we hide it.
    let space = crate::features::spaces::space_common::hooks::use_space()();
    let locked = crate::features::spaces::pages::actions::is_action_locked(
        space.status,
        ctx.quiz().space_action.started_at,
    );

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
                    OverviewTab { can_edit: true }
                }
                TabContent { index: 1usize, value: "upload-tab",
                    UploadTab { can_edit: true }
                }
                TabContent { index: 2usize, value: "quiz-tab",
                    QuizTab { can_edit: can_edit_quiz }
                }
                TabContent { index: 3usize, value: "setting-tab",
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
