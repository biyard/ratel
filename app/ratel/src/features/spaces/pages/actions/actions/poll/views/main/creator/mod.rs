use super::*;
mod question_tab;

use question_tab::*;

#[component]
pub fn PollCreatorPage(
    space_id: ReadSignal<SpacePartition>,
    poll_id: ReadSignal<SpacePollEntityType>,
) -> Element {
    let tr: CreatorTranslate = use_translate();
    Context::init(space_id, poll_id)?;

    rsx! {
        div { class: "flex flex-col gap-4 w-full",
            h3 { {tr.title} }
            Tabs { default_value: "question-tab",
                TabList {
                    TabTrigger { index: 0usize, value: "question-tab", {tr.tab_questions} }
                    TabTrigger { index: 1usize, value: "setting-tab", {tr.tab_setting} }
                }
                TabContent { index: 0usize, value: "question-tab", QuestionTab {} }
                TabContent { index: 1usize, value: "setting-tab", ActionCommonSettings {} }
            }
        }
    }
}

translate! {
    CreatorTranslate;

    title: {
        en: "Poll",
        ko: "투표",
    }

    tab_questions: {
        en: "Questions",
        ko: "질문",
    }

    tab_setting: {
        en: "Settings",
        ko: "설정",
    }
}
