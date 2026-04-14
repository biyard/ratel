use super::*;
use crate::features::spaces::space_common::hooks::use_space_role;

#[derive(Clone, Copy, DioxusController)]
pub struct Context {
    pub quiz: Loader<QuizResponse>,
    pub answer: Loader<QuizAnswerResponse>,
    pub space_id: ReadSignal<SpacePartition>,
    pub quiz_id: ReadSignal<SpaceQuizEntityType>,
    pub editing: Signal<bool>,
    pub original_questions: Signal<Vec<Question>>,
    pub original_answers: Signal<Vec<QuizCorrectAnswer>>,
    pub questions: Signal<Vec<Question>>,
    pub answers: Signal<Vec<QuizCorrectAnswer>>,
    pub retry_count: Signal<i64>,
    pub pass_score: Signal<i64>,
}

pub fn use_space_quiz_context() -> Context {
    use_context()
}

impl Context {
    pub fn init(
        space_id: ReadSignal<SpacePartition>,
        quiz_id: ReadSignal<SpaceQuizEntityType>,
    ) -> Result<Self, Loading> {
        let role = use_space_role()();
        let quiz = use_loader(move || get_quiz(space_id(), quiz_id()))?;
        let answer = use_loader(move || async move {
            if role == SpaceUserRole::Creator {
                get_quiz_answer(space_id(), quiz_id()).await
            } else {
                Ok(QuizAnswerResponse::default())
            }
        })?;

        let quiz_value = quiz.read().clone();
        let answer_value = answer.read().clone();
        let questions = quiz_value.questions.clone();
        let aligned_answers = align_answers(&questions, &answer_value.answers);

        let srv = Self {
            quiz,
            answer,
            space_id,
            quiz_id,
            editing: use_signal(|| false),
            original_questions: use_signal(|| questions.clone()),
            original_answers: use_signal(|| aligned_answers.clone()),
            questions: use_signal(|| questions.clone()),
            answers: use_signal(|| aligned_answers.clone()),
            retry_count: use_signal(|| quiz_value.retry_count),
            pass_score: use_signal(|| quiz_value.pass_score),
        };

        use_context_provider(move || srv);

        Ok(srv)
    }
}

fn align_answers(questions: &[Question], answers: &[QuizCorrectAnswer]) -> Vec<QuizCorrectAnswer> {
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
