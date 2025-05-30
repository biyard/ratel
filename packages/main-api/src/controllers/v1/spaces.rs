use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, State},
        routing::{get, post},
    },
};
use dto::*;

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
    async fn create_space(
        &self,
        auth: Option<Authorization>,
        SpaceCreateSpaceRequest {
            space_type,
            space_form,
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
                space_form,
                user.id,
                feed.industry_id,
                feed_id,
                Some(user.profile_url),
                Some(user.nickname),
                ContentType::Crypto,
                SpaceStatus::Draft,
                feed.files,
                0,
                0,
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
            .route("/:id", get(Self::get_space_by_id))
            .with_state(self.clone())
            .route("/", post(Self::act_space))
            .with_state(self.clone()))
    }

    pub async fn get_space_by_id(
        State(ctrl): State<SpaceController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(SpacePath { id }): Path<SpacePath>,
    ) -> Result<Json<Space>> {
        tracing::debug!("get_space {:?}", id);

        Ok(Json(
            Space::query_builder()
                .id_equals(id)
                .query()
                .map(Space::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
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
