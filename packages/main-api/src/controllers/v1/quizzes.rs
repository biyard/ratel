use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Query, State},
        routing::get,
    },
};
use by_types::QueryResponse;
use dto::*;
use sqlx::postgres::PgRow;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct QuizPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct QuizController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl QuizController {
    async fn query(
        &self,
        _auth: Option<Authorization>,
        _param: QuizQuery,
    ) -> Result<QueryResponse<QuizSummary>> {
        let mut total_count = 0;
        let items: Vec<QuizSummary> = QuizSummary::query_builder()
            .order_by_random()
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;

                total_count = row.try_get("total_count").unwrap_or_default();
                row.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }
}

impl QuizController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::get_quiz))
            .with_state(self.clone()))
    }

    pub async fn get_quiz(
        State(ctrl): State<QuizController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<QuizParam>,
    ) -> Result<Json<QuizGetResponse>> {
        tracing::debug!("list_quiz {:?}", q);

        match q {
            QuizParam::Query(param) => {
                Ok(Json(QuizGetResponse::Query(ctrl.query(auth, param).await?)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{TestContext, setup};

    #[tokio::test]
    async fn test_list_all() {
        let TestContext {
            now,
            endpoint,
            pool,
            ..
        } = setup().await.unwrap();

        let quiz = Quiz::get_repository(pool.clone());
        let mut quizzes = vec![];

        for i in 0..10 {
            quizzes.push(format!("quiz{i}-{now}"));
        }
        let l = Quiz::query_builder()
            .query()
            .map(Quiz::from)
            .fetch_all(&pool)
            .await
            .unwrap()
            .len();

        let mut tx = pool.begin().await.unwrap();
        for q in quizzes {
            quiz.insert_with_tx(&mut *tx, q.clone()).await.unwrap();
        }
        tx.commit().await.unwrap();

        let cli = Quiz::get_client(&endpoint);

        let quizzes = cli.query(QuizQuery::new(5)).await.unwrap();

        assert_eq!(quizzes.items.len(), l + 10);
    }
}
