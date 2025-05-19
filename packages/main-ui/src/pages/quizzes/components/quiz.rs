use bdk::prelude::*;
use dto::QuizSummary;

#[component]
pub fn QuizItem(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    quiz: QuizSummary,
) -> Element {
    rsx! {
        div { ..attributes,{quiz.policy} }
    }
}
