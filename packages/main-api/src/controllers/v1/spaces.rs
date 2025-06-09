mod badges;

use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::State,
        routing::{get, post},
    },
};
use dto::{by_axum::axum::extract::Path, *};

use crate::by_axum::axum::extract::Query;
use crate::security::check_perm;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct SpacePath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct SpaceController {
    repo: SpaceRepository,
    space_member_repo: SpaceMemberRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl SpaceController {
    async fn get_space_by_id(&self, _auth: Option<Authorization>, id: i64) -> Result<Space> {
        tracing::debug!("get_space {:?}", id);

        Ok(Space::query_builder()
            .id_equals(id)
            .query()
            .map(Space::from)
            .fetch_one(&self.pool)
            .await?)
    }

    async fn create_space(
        &self,
        auth: Option<Authorization>,
        SpaceCreateSpaceRequest {
            space_type,
            feed_id,
            user_ids,
        }: SpaceCreateSpaceRequest,
    ) -> Result<Space> {
        let _ = space_type;

        let feed = Feed::query_builder()
            .id_equals(feed_id)
            .query()
            .map(Feed::from)
            .fetch_one(&self.pool.clone())
            .await
            .map_err(|e| {
                tracing::error!("failed to get a feed {feed_id}: {e}");
                Error::FeedInvalidQuoteId
            })?;

        let user = check_perm(
            &self.pool,
            auth,
            RatelResource::Post {
                team_id: feed.user_id,
            },
            GroupPermission::WritePosts,
        )
        .await?;

        let res = self
            .repo
            .insert(
                feed.title,
                feed.html_contents,
                space_type,
                user.id,
                feed.industry_id,
                feed_id,
                SpaceStatus::Draft,
                feed.files,
            )
            .await
            .map_err(|e| {
                tracing::error!("failed to insert post space: {:?}", e);
                Error::SpaceWritePostError
            })?;

        for id in user_ids {
            let _ = self
                .space_member_repo
                .insert(id, res.id)
                .await
                .map_err(|e| {
                    tracing::error!("failed to insert space with member error: {:?}", e);
                    Error::SpaceWritePostError
                })?;
        }

        Ok(res)
    }
}

impl SpaceController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Space::get_repository(pool.clone());
        let space_member_repo = SpaceMember::get_repository(pool.clone());

        Self {
            repo,
            pool,
            space_member_repo,
        }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_space).get(Self::get_space))
            .with_state(self.clone())
            .route("/:id", get(Self::get_by_id))
            .with_state(self.clone())
            .nest(
                "/:space-id/badges",
                badges::SpaceBadgeController::new(self.pool.clone()).route(),
            ))
    }

    pub async fn get_by_id(
        State(ctrl): State<SpaceController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(SpacePath { id }): Path<SpacePath>,
    ) -> Result<Json<Space>> {
        Ok(Json(ctrl.get_space_by_id(auth, id).await?))
    }

    pub async fn get_space(
        State(ctrl): State<SpaceController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<SpaceParam>,
    ) -> Result<Json<SpaceGetResponse>> {
        tracing::debug!("space {:?}", q);

        match q {
            SpaceParam::Read(param) if param.action == Some(SpaceReadActionType::FindById) => {
                let res = ctrl
                    .get_space_by_id(auth, param.id.unwrap_or_default())
                    .await?;
                Ok(Json(SpaceGetResponse::Read(res)))
            }
            _ => Err(Error::BadRequest),
        }
    }

    pub async fn act_space(
        State(ctrl): State<SpaceController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<SpaceAction>,
    ) -> Result<Json<Space>> {
        tracing::debug!("act_space {:?}", body);
        let feed = match body {
            SpaceAction::CreateSpace(param) => ctrl.create_space(auth, param).await?,
        };

        Ok(Json(feed))
    }
}
