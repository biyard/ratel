use crate::features::spaces::pages::actions::actions::quiz::components::*;
use crate::features::spaces::pages::actions::actions::quiz::controllers::*;
use crate::features::spaces::pages::actions::actions::quiz::*;
mod i18n;
mod overview_page;
mod quiz_page;
mod setting_page;
mod upload_page;
use crate::features::spaces::space_common::types::space_page_actions_quiz_key;
use i18n::QuizCreatorTranslate;
pub use overview_page::OverviewPage;
pub use quiz_page::*;
pub use setting_page::*;
pub use upload_page::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum QuizCreatorSection {
    Overview,
    Upload,
    Quiz,
    Setting,
}

#[component]
pub fn QuizCreatorPage(space_id: SpacePartition, quiz_id: SpaceQuizEntityType) -> Element {
    let tr: QuizCreatorTranslate = use_translate();
    let space_id = use_signal(|| space_id);
    let quiz_id = use_signal(|| quiz_id);
    let key = space_page_actions_quiz_key(&space_id(), &quiz_id());
    let quiz_loader = use_query(&key, { move || get_quiz(space_id(), quiz_id()) })?;
    let answer_key = {
        let mut k = key.clone();
        k.push("answers".into());
        k
    };
    let answer_loader = use_query(&answer_key, {
        move || get_quiz_answer(space_id(), quiz_id())
    })?;

    let quiz = quiz_loader.read().clone();
    let answer = answer_loader.read().clone();

    let mut editing = use_signal(|| false);
    let mut original_questions = use_signal(|| quiz.questions.clone());
    let mut original_answers = use_signal(|| align_answers(&quiz.questions, &answer.answers));
    let mut questions = use_signal(|| quiz.questions.clone());
    let mut answers = use_signal(|| align_answers(&quiz.questions, &answer.answers));
    let mut started_at = use_signal(|| quiz.started_at);
    let mut ended_at = use_signal(|| quiz.ended_at);
    let mut retry_count = use_signal(|| quiz.retry_count);
    let mut pass_score = use_signal(|| quiz.pass_score);
    let mut current_section = use_signal(|| QuizCreatorSection::Overview);

    let can_edit = quiz.user_response_count == 0;
    let show_editor = can_edit && editing();

    rsx! {
        div { class: "flex w-full flex-col gap-6",
            // The previous single-page creator flow is kept through the same handlers,
            // but rendered as section pages for the tab-based editor.
            match current_section() {
                QuizCreatorSection::Overview => rsx! {
                    OverviewPage {
                        space_id,
                        quiz_id,
                        initial_title: quiz.title.clone(),
                        initial_description: quiz.description.clone(),
                        can_edit,
                        current_section,
                    }
                },
                QuizCreatorSection::Upload => rsx! {
                    UploadPage { current_section }
                },
                QuizCreatorSection::Quiz => rsx! {
                    QuizPage {
                        space_id,
                        quiz_id,
                        can_edit,
                        editing,
                        questions,
                        answers,
                        original_questions,
                        original_answers,
                        current_section,
                    }
                },
                QuizCreatorSection::Setting => rsx! {
                    SettingPage {
                        space_id,
                        quiz_id,
                        can_edit,
                        started_at,
                        ended_at,
                        retry_count,
                        pass_score,
                        current_section,
                    }
                },
            }
        }
    }
}

pub(super) fn align_answers(
    questions: &[Question],
    answers: &[QuizCorrectAnswer],
) -> Vec<QuizCorrectAnswer> {
    let mut next = Vec::with_capacity(questions.len());
    for (idx, question) in questions.iter().enumerate() {
        let answer = answers
            .get(idx)
            .cloned()
            .unwrap_or_else(|| QuizCorrectAnswer::for_question(question));
        let aligned = match (question, answer) {
            (Question::MultipleChoice(_), QuizCorrectAnswer::Multiple { answers }) => {
                QuizCorrectAnswer::Multiple { answers }
            }
            (Question::SingleChoice(_), QuizCorrectAnswer::Single { answer }) => {
                QuizCorrectAnswer::Single { answer }
            }
            (Question::MultipleChoice(_), _) => QuizCorrectAnswer::Multiple { answers: vec![] },
            _ => QuizCorrectAnswer::Single { answer: None },
        };
        next.push(aligned);
    }
    next
}

pub(super) fn quiz_answer_to_viewer(
    question: &Question,
    answer: &QuizCorrectAnswer,
) -> Option<Answer> {
    match (question, answer) {
        (Question::SingleChoice(_), QuizCorrectAnswer::Single { answer }) => {
            Some(Answer::SingleChoice {
                answer: *answer,
                other: None,
            })
        }
        (Question::MultipleChoice(_), QuizCorrectAnswer::Multiple { answers }) => {
            Some(Answer::MultipleChoice {
                answer: Some(answers.clone()),
                other: None,
            })
        }
        _ => None,
    }
}
