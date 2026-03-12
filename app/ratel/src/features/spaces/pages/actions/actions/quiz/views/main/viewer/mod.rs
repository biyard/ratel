use super::creator::{OverviewTab, QuizCreatorTranslate, QuizTab, UploadTab};
use crate::features::spaces::pages::actions::actions::quiz::*;

#[component]
pub fn QuizViewerPage(
    space_id: ReadSignal<SpacePartition>,
    quiz_id: ReadSignal<SpaceQuizEntityType>,
) -> Element {
    let tr: QuizCreatorTranslate = use_translate();
    Context::init(space_id, quiz_id)?;

    rsx! {
        div { class: "flex flex-col gap-4 w-full",
            h3 { {tr.page_title} }
            Tabs { default_value: "overview-tab",
                TabList {
                    TabTrigger { index: 0usize, value: "overview-tab", {tr.overview_title} }
                    TabTrigger { index: 1usize, value: "upload-tab", {tr.upload_title} }
                    TabTrigger { index: 2usize, value: "quiz-tab", {tr.quiz_section_title} }
                }
                TabContent { index: 0usize, value: "overview-tab",
                    OverviewTab { can_edit: false }
                }
                TabContent { index: 1usize, value: "upload-tab",
                    UploadTab { can_edit: false }
                }
                TabContent { index: 2usize, value: "quiz-tab",
                    QuizTab { can_edit: false }
                }
            }
        }
    }
}
