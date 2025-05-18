use std::collections::HashMap;

use bdk::prelude::*;
use by_axum::{
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Query, State},
        routing::post,
    },
};
use dto::*;

use crate::utils::users::extract_user_with_allowing_anonymous;

#[derive(Clone, Debug)]
pub struct QuizResultController {
    repo: QuizResultRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl QuizResultController {
    async fn anwser(
        &self,
        auth: Option<Authorization>,
        QuizResultAnswerRequest { options }: QuizResultAnswerRequest,
    ) -> Result<QuizResult> {
        let user = extract_user_with_allowing_anonymous(&self.pool, auth).await?;
        let quizzes: HashMap<i64, Quiz> = Quiz::query_builder()
            .order_by_id_desc()
            .query()
            .map(Quiz::from)
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|q| (q.id, q))
            .collect();

        tracing::debug!("quizzes: {:?}", quizzes.len());

        let mut likes = vec![];
        let mut dislikes = vec![];

        for q in options.iter() {
            match q.answer {
                QuizOptions::Like => {
                    likes.push(q.quiz_id);
                }
                QuizOptions::Dislike => {
                    dislikes.push(q.quiz_id);
                }
            }
        }

        let mut results: HashMap<i64, SupportPolicy> = HashMap::new();

        for l in likes.iter() {
            tracing::debug!("like: {}", l);
            if let Some(q) = quizzes.get(l) {
                for p in q.election_pledges.iter() {
                    if let Some(support_policy) = results.get_mut(&p.presidential_candidate_id) {
                        support_policy.support += 1;
                    } else {
                        results.insert(
                            p.presidential_candidate_id,
                            SupportPolicy {
                                presidential_candidate_id: p.presidential_candidate_id,
                                support: 1,
                                against: 0,
                            },
                        );
                    }
                }
            } else {
                tracing::error!("Quiz not found: {}", l);
                return Err(Error::InvalidQuizId);
            }
        }

        for d in dislikes.iter() {
            tracing::debug!("dislike: {}", d);
            if let Some(q) = quizzes.get(d) {
                for p in q.election_pledges.iter() {
                    if let Some(support_policy) = results.get_mut(&p.presidential_candidate_id) {
                        support_policy.against += 1;
                    } else {
                        results.insert(
                            p.presidential_candidate_id,
                            SupportPolicy {
                                presidential_candidate_id: p.presidential_candidate_id,
                                support: 0,
                                against: 1,
                            },
                        );
                    }
                }
            } else {
                tracing::error!("Quiz not found: {}", d);
                return Err(Error::InvalidQuizId);
            }
        }
        tracing::debug!("length of results: {:?}", results.len());
        tracing::debug!("principal: {}", user.principal);
        let results = results.into_values().collect();
        tracing::debug!("results: {:?}", results);

        let result = self.repo.insert(user.principal, results).await?;

        Ok(result)
    }

    async fn get_result(
        &self,
        _auth: Option<Authorization>,
        QuizResultReadAction { principal, .. }: QuizResultReadAction,
    ) -> Result<QuizResult> {
        let result = QuizResult::query_builder()
            .principal_equals(principal.unwrap_or_default())
            .query()
            .map(QuizResult::from)
            .fetch_one(&self.pool)
            .await?;

        Ok(result)
    }
}

impl QuizResultController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = QuizResult::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_quiz_result).get(Self::get_quiz_result))
            .with_state(self.clone()))
    }

    pub async fn act_quiz_result(
        State(ctrl): State<QuizResultController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<QuizResultAction>,
    ) -> Result<Json<QuizResult>> {
        tracing::debug!("act_quiz_result {:?}", body);
        match body {
            QuizResultAction::Answer(param) => {
                let res = ctrl.anwser(auth, param).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn get_quiz_result(
        State(ctrl): State<QuizResultController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<QuizResultParam>,
    ) -> Result<Json<QuizResultGetResponse>> {
        tracing::debug!("list_quiz_result {:?}", q);

        match q {
            QuizResultParam::Read(param)
                if param.action == Some(QuizResultReadActionType::GetResult) =>
            {
                let res = ctrl.get_result(auth, param).await?;
                Ok(Json(QuizResultGetResponse::Read(res)))
            }
            _ => Err(Error::BadRequest),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{TestContext, setup};

    async fn setup_quiz(pool: &sqlx::Pool<sqlx::Postgres>) {
        let pc = PresidentialCandidate::get_repository(pool.clone());

        let mut tx = pool.begin().await.unwrap();

        let pc1 = pc
            .insert_with_tx(
                &mut *tx,
                "Candidate 1".to_string(),
                "https://".to_string(),
                CryptoStance::Supportive,
                Party::PeoplePowerParty,
            )
            .await
            .unwrap()
            .unwrap();
        let pc2 = pc
            .insert_with_tx(
                &mut *tx,
                "Candidate 2".to_string(),
                "https://".to_string(),
                CryptoStance::Supportive,
                Party::DemocraticParty,
            )
            .await
            .unwrap()
            .unwrap();

        let ep = ElectionPledge::get_repository(pool.clone());
        let ep1 = ep
            .insert_with_tx(&mut *tx, pc1.id, "Election 1".to_string())
            .await
            .unwrap()
            .unwrap();
        let ep2 = ep
            .insert_with_tx(&mut *tx, pc1.id, "Election 2".to_string())
            .await
            .unwrap()
            .unwrap();
        let ep3 = ep
            .insert_with_tx(&mut *tx, pc2.id, "Election 3".to_string())
            .await
            .unwrap()
            .unwrap();
        let ep4 = ep
            .insert_with_tx(&mut *tx, pc2.id, "Election 4".to_string())
            .await
            .unwrap()
            .unwrap();

        let quiz = Quiz::get_repository(pool.clone());
        let quiz1 = quiz
            .insert_with_tx(&mut *tx, "Quiz 1".to_string())
            .await
            .unwrap()
            .unwrap();
        let quiz2 = quiz
            .insert_with_tx(&mut *tx, "Quiz 2".to_string())
            .await
            .unwrap()
            .unwrap();
        let quiz3 = quiz
            .insert_with_tx(&mut *tx, "Quiz 3".to_string())
            .await
            .unwrap()
            .unwrap();

        let epq = ElectionPledgeQuiz::get_repository(pool.clone());
        epq.insert_with_tx(&mut *tx, ep1.id, quiz1.id)
            .await
            .unwrap()
            .unwrap();
        epq.insert_with_tx(&mut *tx, ep3.id, quiz2.id)
            .await
            .unwrap()
            .unwrap();
        epq.insert_with_tx(&mut *tx, ep2.id, quiz3.id)
            .await
            .unwrap()
            .unwrap();
        epq.insert_with_tx(&mut *tx, ep4.id, quiz3.id)
            .await
            .unwrap()
            .unwrap();

        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_quiz_results() {
        let TestContext {
            user,
            endpoint,
            pool,
            ..
        } = setup().await.unwrap();
        setup_quiz(&pool).await;

        let cli = QuizResult::get_client(&endpoint);
        let quiz_result = cli
            .answer(vec![
                QuizAnswer {
                    quiz_id: 1,
                    answer: QuizOptions::Like,
                },
                QuizAnswer {
                    quiz_id: 2,
                    answer: QuizOptions::Dislike,
                },
                QuizAnswer {
                    quiz_id: 3,
                    answer: QuizOptions::Like,
                },
            ])
            .await
            .unwrap();

        assert_eq!(quiz_result.principal, user.principal);
        assert_eq!(quiz_result.results.len(), 2);

        for r in quiz_result.results.iter() {
            if r.presidential_candidate_id == 1 {
                assert_eq!(r.support, 2);
                assert_eq!(r.against, 0);
            } else if r.presidential_candidate_id == 2 {
                assert_eq!(r.support, 1);
                assert_eq!(r.against, 1);
            } else {
                assert!(
                    false,
                    "Unexpected presidential candidate id: {}",
                    r.presidential_candidate_id
                );
            }
        }

        let quiz_result = cli.get_result(quiz_result.principal).await.unwrap();
        assert_eq!(quiz_result.principal, user.principal);
        assert_eq!(quiz_result.results.len(), 2);

        for r in quiz_result.results.iter() {
            if r.presidential_candidate_id == 1 {
                assert_eq!(r.support, 2);
                assert_eq!(r.against, 0);
            } else if r.presidential_candidate_id == 2 {
                assert_eq!(r.support, 1);
                assert_eq!(r.against, 1);
            } else {
                assert!(
                    false,
                    "Unexpected presidential candidate id: {}",
                    r.presidential_candidate_id
                );
            }
        }
    }
}
