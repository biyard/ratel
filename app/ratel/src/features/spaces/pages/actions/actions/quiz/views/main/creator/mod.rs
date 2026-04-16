use crate::features::spaces::pages::actions::actions::quiz::*;
use crate::features::spaces::pages::actions::components::ActionEditTopbar;

mod i18n;
pub use i18n::QuizCreatorTranslate;

mod content_card;
mod questions_card;
mod config_card;
use content_card::ContentCard;
use questions_card::QuestionsCard;
use config_card::ConfigCard;

#[component]
pub fn QuizCreatorPage(
    space_id: ReadSignal<SpacePartition>,
    quiz_id: ReadSignal<SpaceQuizEntityType>,
) -> Element {
    let tr: QuizCreatorTranslate = use_translate();
    let ctx = Context::init(space_id, quiz_id)?;
    let space = crate::features::spaces::space_common::hooks::use_space()();
    let nav = use_navigator();

    let initial_title = ctx.quiz.read().title.clone();
    let title = use_signal(|| initial_title);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "arena",
            ActionEditTopbar {
                space_name: space.title.clone(),
                action_type_label: tr.type_badge_label.to_string(),
                action_type_key: "quiz".to_string(),
                title,
                on_title_change: move |_v: String| {},
                editable_title: false,
                on_back: move |_| {
                    nav.go_back();
                },
                on_cancel: move |_| {
                    nav.go_back();
                },
            }
            main { class: "pager",
                ContentCard {}
                QuestionsCard {}
                ConfigCard {}
            }
        }
    }
}
