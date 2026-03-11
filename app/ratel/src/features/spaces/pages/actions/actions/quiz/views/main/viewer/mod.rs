mod i18n;
use super::creator::{align_answers, OverviewPage};
use super::creator::{QuizCreatorSection, QuizPage, SettingPage, UploadPage};
use crate::features::spaces::pages::actions::actions::quiz::controllers::*;
use crate::features::spaces::pages::actions::actions::quiz::*;
use crate::features::spaces::space_common::types::space_page_actions_quiz_key;
use i18n::QuizViewerTranslate;

#[component]
pub fn QuizViewerPage(space_id: SpacePartition, quiz_id: SpaceQuizEntityType) -> Element {
    let tr: QuizViewerTranslate = use_translate();
    let nav = navigator();
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
    let questions = use_signal(|| quiz.questions.clone());
    let answers = use_signal(|| align_answers(&quiz.questions, &answer.answers));
    let original_questions = use_signal(|| quiz.questions.clone());
    let original_answers = use_signal(|| align_answers(&quiz.questions, &answer.answers));
    let editing = use_signal(|| false);
    let started_at = use_signal(|| quiz.started_at);
    let ended_at = use_signal(|| quiz.ended_at);
    let retry_count = use_signal(|| quiz.retry_count);
    let pass_score = use_signal(|| quiz.pass_score);
    let current_section = use_signal(|| QuizCreatorSection::Overview);

    rsx! {
        div { class: "flex w-full flex-col gap-4",
            Button {
                size: ButtonSize::Inline,
                style: ButtonStyle::Text,
                class: "flex items-center gap-1 text-sm text-neutral-400 hover:text-white transition-colors w-fit",
                onclick: move |_| nav.go_back(),
                "← {tr.btn_back}"
            }

            match current_section() {
                QuizCreatorSection::Overview => rsx! {
                    OverviewPage {
                        space_id,
                        quiz_id,
                        initial_title: quiz.title.clone(),
                        initial_description: quiz.description.clone(),
                        can_edit: false,
                        current_section: Some(current_section),
                        show_save: false,
                    }
                },
                QuizCreatorSection::Upload => rsx! {
                    UploadPage { current_section }
                },
                QuizCreatorSection::Quiz => rsx! {
                    QuizPage {
                        space_id,
                        quiz_id,
                        can_edit: false,
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
                        can_edit: false,
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
