use crate::features::spaces::pages::actions::actions::quiz::components::*;
use crate::features::spaces::pages::actions::actions::quiz::*;
use crate::features::spaces::pages::actions::ActionCommonSettings;
mod i18n;
mod overview_tab;
mod quiz_tab;
mod setting_tab;
mod upload_tab;
pub use i18n::QuizCreatorTranslate;
pub use overview_tab::*;
pub use quiz_tab::*;
pub use setting_tab::*;
pub use upload_tab::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum QuizCreatorSection {
    Overview,
    Upload,
    Quiz,
    Setting,
}

#[component]
pub fn QuizCreatorPage(
    space_id: ReadSignal<SpacePartition>,
    quiz_id: ReadSignal<SpaceQuizEntityType>,
) -> Element {
    let tr: QuizCreatorTranslate = use_translate();
    let ctx = Context::init(space_id, quiz_id)?;
    let can_edit = ctx.quiz.read().user_response_count == 0;

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
                    OverviewTab { can_edit }
                }
                TabContent { index: 1usize, value: "upload-tab",
                    UploadTab { can_edit }
                }
                TabContent { index: 2usize, value: "quiz-tab",
                    QuizTab { can_edit }
                }
                TabContent { index: 3usize, value: "setting-tab", ActionCommonSettings {} }
            }
        }
    }
}
