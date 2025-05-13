use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, Query, State},
        routing::{get, post},
    },
};
use by_types::QueryResponse;
use dto::*;
use sqlx::postgres::PgRow;

use crate::utils::users::extract_user_id;

#[derive(Clone, Debug)]
pub struct PromotionController {
    repo: PromotionRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct PromotionPath {
    pub id: i64,
}

impl PromotionController {
    
    async fn query(
        &self,
        _auth: Option<Authorization>,
        param: PromotionQuery,
        filter_: bool,
    ) -> Result<QueryResponse<PromotionSummary>> {
        let mut total_count = 0;
        let items: Vec<PromotionSummary> = PromotionSummary::query_builder()
            .limit(param.size())
            .page(param.page())
            .order_by_price_payed_desc() // Sort by price_payed in descending order
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;
                total_count = row.try_get("total_count").unwrap_or_default();

                row.into()
            })
            .fetch_all(&self.pool)       
            .await?;

        if filter_{
            let items: Vec<PromotionSummary> = items
                .clone()
                .into_iter()
                .filter(|item| !item.accepted)
                .collect();

            return Ok(QueryResponse { total_count, items });

        }

        Ok(QueryResponse { total_count, items })
    }
    
    
    async fn write_promotion(
        &self,
        auth: Option<Authorization>,
        PromotionWritePromotionRequest {
            title,
            html_contents,
            price_payed,
            ..
        }: PromotionWritePromotionRequest,
    ) -> Result<Promotion> {
       
        let user_id = extract_user_id(&self.pool, auth).await?;

        let res = self
            .repo
            .insert(
                title,
                html_contents,
                user_id,
                price_payed,
                false,
            )
            .await
            .map_err(|e| {
                tracing::error!("failed to insert post promotion: {:?}", e);
                Error::PromotionWritePostError
            })?;

        Ok(res)
    }

    async fn update(
        &self,
        id: i64,
        auth: Option<Authorization>,
        param: PromotionUpdateRequest,
    ) -> Result<Promotion> {
        // TODO: ONLY SERVICE OPERATOR CAN PERFROM
        if auth.is_none() {
            return Err(Error::Unauthorized);
        }

        let res = self.repo.update(id, param.into()).await?;

        Ok(res)
    }

    async fn delete(&self, id: i64, auth: Option<Authorization>) -> Result<Promotion> {
        // TODO: ONLY SERVICE OPERATOR CAN PERFROM
        if auth.is_none() {
            return Err(Error::Unauthorized);
        }

        let res = self.repo.delete(id).await?;

        Ok(res)
    }
}

impl PromotionController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Promotion::get_repository(pool.clone());
        Self { repo, pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/:id", get(Self::get_promotion_by_id).post(Self::act_promotion_by_id))
            .with_state(self.clone())
            .route("/", post(Self::act_promotion).get(Self::get_promotion))
            .with_state(self.clone())
            .route("/all", get(Self::get_promotion_all))
            .with_state(self.clone()))
    }

    pub async fn act_promotion(
        State(ctrl): State<PromotionController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<PromotionAction>,
    ) -> Result<Json<Promotion>> {
        tracing::debug!("act_promotion {:?}", body);
        let promotion = match body {
            PromotionAction::WritePromotion(param) => ctrl.write_promotion(auth, param).await?,
        };

        Ok(Json(promotion))
    }

    pub async fn get_promotion(
        State(ctrl): State<PromotionController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<PromotionParam>,
    ) -> Result<Json<PromotionGetResponse>> {
        tracing::debug!("list_promotion {:?}", q);

        match q {
            PromotionParam::Query(param) => {
                Ok(Json(PromotionGetResponse::Query(ctrl.query(auth, param, true).await?)))
            }
        }
    }

    pub async fn get_promotion_all(
        State(ctrl): State<PromotionController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<PromotionParam>,
    ) -> Result<Json<PromotionGetResponse>> {
        tracing::debug!("list_promotion {:?}", q);

        match q {
            PromotionParam::Query(param) => {
                Ok(Json(PromotionGetResponse::Query(ctrl.query(auth, param, false).await?)))
            }
        }
    }


    pub async fn get_promotion_by_id(
        State(ctrl): State<PromotionController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(PromotionPath { id }): Path<PromotionPath>,
    ) -> Result<Json<Promotion>> {
        tracing::debug!("get_promotion {:?}", id);

        Ok(Json(
            Promotion::query_builder()
                .id_equals(id)
                .query()
                .map(Promotion::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn act_promotion_by_id(
        State(ctrl): State<PromotionController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(PromotionPath { id }): Path<PromotionPath>,
        Json(body): Json<PromotionByIdAction>,
    ) -> Result<Json<Promotion>> {
        tracing::debug!("act_promotion_by_id {:?} {:?}", id, body);
        match body {
            PromotionByIdAction::Update(param) => {
                let res = ctrl.update(id, auth, param).await?;
                Ok(Json(res))
            }
            PromotionByIdAction::Delete(_) => {
                let res = ctrl.delete(id, auth).await?;
                Ok(Json(res))
            }
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{TestContext, setup};

    async fn test_setup() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();
        let html_contents = format!("<p>Test {now}</p>");
        let title = Some(format!("Test Title {now}"));

        let _post = Promotion::get_repository(pool.clone())
            .insert(
                title.unwrap(),
                html_contents.clone(),
                user.id,

                100,
                false,

            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_write_promotions() {
        test_setup().await;
        let TestContext {
            user,
            now,
            endpoint,
            ..
        } = setup().await.unwrap();
        let html_contents = format!("<p>Test {now}</p>");
        let title = Some(format!("Test Title {now}"));


        let res = Promotion::get_client(&endpoint)
            .write_promotion(title.clone().unwrap(), html_contents.clone(), 100, false)
            .await;

        assert!(res.is_ok());

        let promo = res.unwrap();
        assert_eq!(promo.html_contents, html_contents);
        assert_eq!(Some(promo.title), title);
        assert_eq!(promo.user_id, user.id);
        assert_eq!(promo.accepted, false);
        
    }

    #[tokio::test]
    async fn test_get_promotion_by_id() {
        // Set up test context with DB and API service
        let TestContext { pool, endpoint, user, .. } = setup().await.unwrap();

        // Insert a promotion directly via the repository
        let inserted = Promotion::get_repository(pool.clone())
            .insert(
                "Test Title".to_string(),
                "<p>Test HTML</p>".to_string(),
                user.id,
                250,
                false,
            )
            .await
            .unwrap();

        // Use the generated API client to fetch by ID
        let client = Promotion::get_client(&endpoint);
        let fetched = client.get(inserted.id).await.unwrap();

        // Assert the fetched record matches the inserted one
        assert_eq!(fetched.id, inserted.id);
        assert_eq!(fetched.title, inserted.title);
        assert_eq!(fetched.html_contents, inserted.html_contents);
        assert_eq!(fetched.user_id, inserted.user_id);
        assert_eq!(fetched.price_payed, inserted.price_payed);
        assert_eq!(fetched.accepted, inserted.accepted);
    }
}