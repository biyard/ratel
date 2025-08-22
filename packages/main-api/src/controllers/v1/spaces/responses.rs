use crate::utils::users::extract_user_with_allowing_anonymous;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Query, State},
        routing::{get, post},
    },
};
use by_types::QueryResponse;
use dto::{by_axum::axum::extract::Path, *};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SurveyResponseController {
    repo: SurveyResponseRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl SurveyResponseController {
    pub async fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = SurveyResponse::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route(
                "/:id",
                get(Self::get_survey_response).post(Self::act_survey_response_by_id),
            )
            .with_state(self.clone())
            .route(
                "/",
                post(Self::act_survey_response).get(Self::list_survey_response),
            )
            .with_state(self.clone())
    }

    pub async fn act_survey_response_by_id(
        State(ctrl): State<SurveyResponseController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(SurveyResponsePath { space_id, id }): Path<SurveyResponsePath>,
        Json(body): Json<SurveyResponseByIdAction>,
    ) -> Result<Json<SurveyResponse>> {
        tracing::debug!("act_survey_response_by_id {} {:?} {:?}", space_id, id, body);

        let res = match body {
            SurveyResponseByIdAction::UpdateRespondAnswer(params) => {
                ctrl.update_respond_answer(id, auth, params).await?
            }
            SurveyResponseByIdAction::RemoveRespondAnswer(_) => {
                ctrl.remove_respond_answer(id, auth).await?
            }
        };

        Ok(res)
    }

    pub async fn get_survey_response(
        State(_ctrl): State<SurveyResponseController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(SurveyResponsePath { space_id, id }): Path<SurveyResponsePath>,
    ) -> Result<Json<SurveyResponse>> {
        //TODO: implement get_survey_response
        tracing::debug!("get_survey_response {} {}", space_id, id);
        Ok(Json(SurveyResponse::default()))
    }

    pub async fn list_survey_response(
        State(_ctrl): State<SurveyResponseController>,
        Path(SurveyResponseParentPath { space_id }): Path<SurveyResponseParentPath>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(q): Query<SurveyResponseParam>,
    ) -> Result<Json<SurveyResponseGetResponse>> {
        //TODO(api): implement list_survey_response
        tracing::debug!("list_survey_response {} {:?}", space_id, q);

        match q {
            SurveyResponseParam::Query(_q) => {
                Ok(Json(SurveyResponseGetResponse::Query(QueryResponse {
                    total_count: 0,
                    items: vec![],
                })))
            }
        }
    }

    pub async fn act_survey_response(
        State(ctrl): State<SurveyResponseController>,
        Path(SurveyResponseParentPath { space_id }): Path<SurveyResponseParentPath>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<SurveyResponseAction>,
    ) -> Result<Json<SurveyResponse>> {
        tracing::debug!("act_survey_response {} {:?}", space_id, body);

        match body {
            SurveyResponseAction::RespondAnswer(req) => {
                ctrl.respond_answer(space_id, auth, req).await
            }
        }
    }
}

impl SurveyResponseController {
    pub async fn remove_respond_answer(
        &self,
        response_id: i64,
        auth: Option<Authorization>,
    ) -> Result<Json<SurveyResponse>> {
        let _ = extract_user_with_allowing_anonymous(&self.pool, auth).await?;

        let respond = SurveyResponse::query_builder()
            .id_equals(response_id)
            .query()
            .map(SurveyResponse::from)
            .fetch_one(&self.pool)
            .await?;

        if respond.survey_type == SurveyType::Survey {
            return Err(Error::UpdateNotAllowed);
        }

        let res = self.repo.delete(response_id).await?;

        Ok(Json(res))
    }

    pub async fn update_respond_answer(
        &self,
        response_id: i64,
        auth: Option<Authorization>,
        SurveyResponseUpdateRespondAnswerRequest {
            answers,
        }: SurveyResponseUpdateRespondAnswerRequest,
    ) -> Result<Json<SurveyResponse>> {
        let _ = extract_user_with_allowing_anonymous(&self.pool, auth).await?;

        let respond = SurveyResponse::query_builder()
            .id_equals(response_id)
            .query()
            .map(SurveyResponse::from)
            .fetch_one(&self.pool)
            .await?;

        if respond.survey_type == SurveyType::Survey {
            return Err(Error::UpdateNotAllowed);
        }

        let res = self
            .repo
            .update(
                response_id,
                SurveyResponseRepositoryUpdateRequest {
                    answers: Some(answers),
                    ..Default::default()
                },
            )
            .await?;

        Ok(Json(res))
    }

    pub async fn respond_answer(
        &self,
        space_id: i64,
        auth: Option<Authorization>,
        SurveyResponseRespondAnswerRequest {
            answers,
            survey_type,
            survey_id_param,
        }: SurveyResponseRespondAnswerRequest,
    ) -> Result<Json<SurveyResponse>> {
        let user = extract_user_with_allowing_anonymous(&self.pool, auth).await?;
        let user_id = user.id;

        let res = self
            .repo
            .insert(
                space_id,
                user_id,
                answers,
                survey_id_param.unwrap_or_default(),
                survey_type,
            )
            .await?;
        Ok(Json(res))
    }
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct SurveyResponsePath {
    pub space_id: i64,
    pub id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct SurveyResponseParentPath {
    pub space_id: i64,
}
