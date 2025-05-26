use bdk::prelude::*;
use dto::*;

use crate::route::Route;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,

    pub started: Signal<bool>,
    pub finished: Signal<bool>,
    pub quizzes: Resource<Vec<QuizSummary>>,
    pub current_step: Signal<usize>,
    pub answer: Signal<Vec<QuizAnswer>>,
    pub nav: Navigator,
    pub already_done: Signal<bool>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let quizzes = use_server_future(move || async move {
            Quiz::get_client(crate::config::get().main_api_endpoint)
                .query(QuizQuery::new(100))
                .await
                .unwrap_or_default()
                .items
        })?;
        #[allow(unused_mut)]
        let mut already_done = use_signal(|| false);
        #[cfg(feature = "web")]
        let anonymouse_service: crate::services::anonymouse_service::AnonymouseService =
            use_context();
        #[cfg(feature = "web")]
        let user_service: crate::services::user_service::UserService = use_context();

        #[cfg(feature = "web")]
        use_future(move || async move {
            let user_info = user_service.user_info();

            let principal = if user_info.principal != "" {
                user_info.principal
            } else {
                anonymouse_service.get_principal()
            };

            if QuizResult::get_client(crate::config::get().main_api_endpoint)
                .get_result(principal)
                .await
                .is_ok()
            {
                already_done.set(true);
            };
        });

        let ctrl = Self {
            lang,
            started: use_signal(|| false),
            finished: use_signal(|| false),
            current_step: use_signal(|| 0),
            quizzes,
            answer: use_signal(|| vec![]),
            nav: use_navigator(),
            already_done,
        };

        Ok(ctrl)
    }

    pub fn progress(&self) -> f32 {
        let quizzes = self.quizzes().unwrap_or_default();
        if quizzes.is_empty() || self.current_step() == 0 {
            return 0.0;
        }
        let step = self.current_step() - 1;
        let total = quizzes.len();
        (step as f32 / total as f32) * 100.0
    }

    pub fn start(&mut self) {
        self.started.set(true);
        self.current_step.set(1);
    }

    pub fn left(&self, i: usize) -> i32 {
        let base = (i + 1) as i32 - self.current_step() as i32;
        base * 100
    }

    pub async fn finish(&mut self) {
        match QuizResult::get_client(crate::config::get().main_api_endpoint)
            .answer(self.answer())
            .await
        {
            Ok(q) => {
                tracing::debug!("Quiz result sent");
                self.nav.push(Route::ResultsPage { id: q.principal });
            }
            Err(e) => {
                btracing::e!(e);
            }
        };

        self.finished.set(true);
    }

    pub async fn next(&mut self) {
        let mut step = self.current_step();
        step += 1;
        if step > self.quizzes().unwrap_or_default().len() {
            self.finish().await;
        }
        self.current_step.set(step);
    }

    pub async fn like(&mut self, i: usize) {
        let mut answers = self.answer().clone();
        if answers.len() <= i {
            answers.push(QuizAnswer {
                quiz_id: self.quizzes().unwrap_or_default()[i].id,
                answer: QuizOptions::Like,
            });
        } else {
            answers[i].answer = QuizOptions::Like;
        }
        self.answer.set(answers);
        self.next().await;
    }

    pub async fn dislike(&mut self, i: usize) {
        let mut answers = self.answer().clone();
        if answers.len() <= i {
            answers.push(QuizAnswer {
                quiz_id: self.quizzes().unwrap_or_default()[i].id,
                answer: QuizOptions::Dislike,
            });
        } else {
            answers[i].answer = QuizOptions::Dislike;
        }
        self.answer.set(answers);
        self.next().await;
    }

    pub fn go_to_result(&mut self) {
        self.nav.push(Route::ResultsPage {
            id: self.principal(),
        });
    }
}
