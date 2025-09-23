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

use crate::{
    security::check_perm,
    utils::users::{extract_user, extract_user_id},
};

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct FeedPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct FeedController {
    space_repo: SpaceRepository,
    repo: FeedRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl FeedController {
    pub async fn query(
        &self,
        auth: Option<Authorization>,
        param: FeedQuery,
    ) -> Result<QueryResponse<FeedSummary>> {
        let mut total_count = 0;
        let user = extract_user(&self.pool, auth.clone())
            .await
            .unwrap_or_default();
        let user_id = user.id;
        let teams = user.teams;

        let status = if let Some(status) = param.status {
            if status == FeedStatus::Draft {
                check_perm(
                    &self.pool,
                    auth,
                    RatelResource::Post { team_id: user_id },
                    GroupPermission::ReadPostDrafts,
                )
                .await?;
                FeedStatus::Draft
            } else {
                status
            }
        } else {
            FeedStatus::Published
        };

        let builder = FeedSummary::query_builder(user_id)
            .spaces_builder(Space::query_builder(user_id))
            .limit(param.size())
            .page(param.page())
            .status_equals(status)
            .order_by_created_at_desc();

        let builder = match param.feed_type {
            Some(feed_type) => builder.feed_type_equals(feed_type),
            None => builder.feed_type_between(FeedType::Artwork, FeedType::Post),
        };

        let feeds: Vec<FeedSummary> = builder
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;

                total_count = row.try_get("total_count").unwrap_or_default();
                row.into()
            })
            .fetch_all(&self.pool)
            .await?;
        tracing::debug!("query feed items: {:?}", feeds);

        let mut items = vec![];

        for f in feeds {
            let mut feed = f.clone();

            let space = Space::query_builder(user_id)
                .feed_id_equals(f.id)
                .query()
                .map(Space::from)
                .fetch_optional(&self.pool)
                .await?;

            if let Some(space) = space {
                if let Some(author) = space.author.last() {
                    tracing::debug!("space: {:?} {:?}", space, teams);
                    let should_filter = (space.status == SpaceStatus::Draft
                        || space.publishing_scope == PublishingScope::Private)
                        && author.id != user_id
                        && !teams.iter().any(|t| t.id == author.id);

                    if !should_filter {
                        feed.spaces = vec![space];
                    } else {
                        feed.spaces = vec![];
                    }
                } else {
                    feed.spaces = vec![];
                }
            } else {
                feed.spaces = vec![];
            }

            items.push(feed);
        }

        Ok(QueryResponse { total_count, items })
    }

    async fn posts_by_user_id(
        &self,
        auth: Option<Authorization>,
        param: FeedQuery,
    ) -> Result<QueryResponse<FeedSummary>> {
        let user_id = extract_user_id(&self.pool, auth.clone())
            .await
            .unwrap_or_default();
        let mut total_count = 0;
        let status = if let Some(status) = param.status {
            if status == FeedStatus::Draft {
                check_perm(
                    &self.pool,
                    auth,
                    RatelResource::Post { team_id: user_id },
                    GroupPermission::ReadPostDrafts,
                )
                .await?;
                FeedStatus::Draft
            } else {
                status
            }
        } else {
            FeedStatus::Published
        };
        let items: Vec<FeedSummary> = FeedSummary::query_builder(user_id)
            .limit(param.size())
            .page(param.page())
            .status_equals(status)
            .user_id_equals(param.user_id.unwrap_or_default())
            .order_by_created_at_desc()
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
    pub async fn create_draft(
        &self,
        auth: Option<Authorization>,
        param: FeedCreateDraftRequest,
    ) -> Result<Feed> {
        let user_id = param.user_id;
        check_perm(
            &self.pool,
            auth,
            RatelResource::Post { team_id: user_id },
            GroupPermission::WritePosts,
        )
        .await?;

        let res = self
            .repo
            .insert(
                FeedType::default(),
                user_id,
                1,
                None,
                None,
                None,
                String::default(),
                None,
                UrlType::default(),
                vec![],
                0,
                FeedStatus::Draft,
            )
            .await
            .map_err(|e| {
                tracing::error!("failed to insert post feed: {:?}", e);
                Error::FeedWritePostError
            })?;

        Ok(res)
    }

    pub async fn comment(
        &self,
        auth: Option<Authorization>,
        FeedCommentRequest {
            html_contents,
            parent_id,
        }: FeedCommentRequest,
    ) -> Result<Feed> {
        let parent_id = parent_id.ok_or_else(|| {
            tracing::error!("parent id is missing");
            Error::FeedInvalidParentId
        })?;

        let feed = Feed::query_builder(0)
            .id_equals(parent_id)
            .status_not_equals(FeedStatus::Draft)
            .query()
            .map(Feed::from)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("failed to get a feed {parent_id}: {e}");
                Error::FeedInvalidParentId
            })?;

        let user_id = extract_user_id(&self.pool, auth.clone()).await?;
        check_perm(
            &self.pool,
            auth,
            RatelResource::Post { team_id: user_id },
            GroupPermission::WriteReplies,
        )
        .await?;
        let res = self
            .repo
            .insert(
                FeedType::Reply,
                user_id,
                feed.industry_id,
                Some(parent_id),
                None,
                None,
                html_contents,
                None,
                UrlType::None,
                vec![],
                0,
                FeedStatus::Published,
            )
            .await
            .map_err(|e| {
                tracing::error!("failed to insert comment feed: {:?}", e);
                Error::FeedWriteCommentError
            })?;

        Ok(res)
    }

    // async fn review_doc(
    //     &self,
    //     auth: Option<Authorization>,
    //     FeedReviewDocRequest {
    //         html_contents,
    //         parent_id,
    //         part_id: _,
    //     }: FeedReviewDocRequest,
    // ) -> Result<Feed> {
    //     let user_id = extract_user_id(&self.pool, auth.clone())
    //         .await?;
    //     check_perm(
    //         &self.pool,
    //         auth,
    //         RatelResource::Post { team_id: user_id },
    //         GroupPermission::WriteReplies,
    //     )
    //     .await?;
    //     let parent_id = parent_id.ok_or_else(|| {
    //         tracing::error!("parent id is missing: {user_id}");
    //         Error::FeedInvalidParentId
    //     })?;

    //     let feed = Feed::query_builder(user_id)
    //         .id_equals(parent_id)
    //         .query()
    //         .map(Feed::from)
    //         .fetch_one(&self.pool)
    //         .await
    //         .map_err(|e| {
    //             tracing::error!("failed to get a feed {parent_id}: {e}");
    //             Error::FeedInvalidParentId
    //         })?;

    //     let res = self
    //         .repo
    //         .insert(
    //             html_contents,
    //             FeedType::DocReview,
    //             user_id,
    //             feed.industry_id,
    //             Some(parent_id),
    //             None,
    //             None,
    //             None,
    //             feed.files,
    //             0,
    //             0,
    //             FeedStatus::Draft,
    //             None,
    //             UrlType::None,
    //         )
    //         .await
    //         .map_err(|e| {
    //             tracing::error!("failed to insert comment feed: {:?}", e);
    //             Error::FeedWriteCommentError
    //         })?;

    //     Ok(res)
    // }

    async fn repost(
        &self,
        auth: Option<Authorization>,
        FeedRepostRequest {
            html_contents,
            quote_feed_id,
            parent_id,
            user_id,
        }: FeedRepostRequest,
    ) -> Result<Feed> {
        check_perm(
            &self.pool,
            auth,
            RatelResource::Post { team_id: user_id },
            GroupPermission::WriteReplies,
        )
        .await?;
        let parent_id = parent_id.ok_or_else(|| {
            tracing::error!("parent id is missing");
            Error::FeedInvalidParentId
        })?;

        let feed = Feed::query_builder(0)
            .id_equals(parent_id)
            .status_not_equals(FeedStatus::Draft)
            .query()
            .map(Feed::from)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("failed to get a feed {parent_id}: {e}");
                Error::FeedInvalidParentId
            })?;

        if let Some(quote_feed_id) = quote_feed_id {
            Feed::query_builder(user_id)
                .id_equals(quote_feed_id)
                .status_not_equals(FeedStatus::Draft)
                .query()
                .map(Feed::from)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| {
                    tracing::error!("failed to get a feed {quote_feed_id}: {e}");
                    Error::FeedInvalidQuoteId
                })?;
        }

        let mut tx = self.pool.begin().await?;

        let res = self
            .repo
            .insert_with_tx(
                tx.as_mut(),
                FeedType::Repost,
                user_id,
                feed.industry_id,
                Some(parent_id),
                quote_feed_id,
                None,
                html_contents,
                None,
                UrlType::None,
                vec![],
                0,
                FeedStatus::Published,
            )
            .await
            .map_err(|e| {
                tracing::error!("failed to insert comment feed: {:?}", e);
                Error::FeedWriteCommentError
            })?
            .ok_or_else(|| {
                tracing::error!("Insert operation returned None");
                Error::DatabaseException("Insert operation failed".to_string())
            })?;

        FeedShare::get_repository(self.pool.clone())
            .insert_with_tx(tx.as_mut(), parent_id, user_id)
            .await
            .map_err(|e| {
                tracing::error!("failed to insert feed share: {:?}", e);
                Error::DatabaseException(e.to_string())
            })?;

        tx.commit().await?;

        Ok(res)
    }

    pub async fn update(
        &self,
        id: i64,
        auth: Option<Authorization>,
        param: FeedUpdateRequest,
    ) -> Result<Feed> {
        let feed = Feed::query_builder(0)
            .id_equals(id)
            .status_equals(FeedStatus::Draft)
            .query()
            .map(Feed::from)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("failed to get a feed {id}: {e}");
                Error::NotFound
            })?;
        check_perm(
            &self.pool,
            auth,
            match feed.feed_type {
                FeedType::Post => RatelResource::Post {
                    team_id: feed.user_id,
                },
                _ => RatelResource::Reply {
                    team_id: feed.user_id,
                },
            },
            match feed.feed_type {
                FeedType::Post => GroupPermission::WritePendingPosts,
                _ => GroupPermission::WriteReplies,
            },
        )
        .await?;

        let mut tx = self.pool.begin().await?;

        let res = self
            .repo
            .update_with_tx(&mut *tx, id, param.clone().into())
            .await?
            .unwrap_or_default();

        let spaces = res.spaces.clone();

        if !spaces.is_empty() {
            let space = spaces[0].clone();

            let _ = self
                .space_repo
                .update_with_tx(
                    &mut *tx,
                    space.id,
                    SpaceRepositoryUpdateRequest {
                        title: param.title,
                        html_contents: Some(param.html_contents.clone()),
                        ..Default::default()
                    },
                )
                .await?;
        }

        tx.commit().await?;
        Ok(res)
    }

    pub async fn edit(
        &self,
        id: i64,
        auth: Option<Authorization>,
        param: FeedEditRequest,
    ) -> Result<Feed> {
        let feed = Feed::query_builder(0)
            .id_equals(id)
            .query()
            .map(Feed::from)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("failed to get a feed {id}: {e}");
                Error::NotFound
            })?;
        check_perm(
            &self.pool,
            auth,
            match feed.feed_type {
                FeedType::Post => RatelResource::Post {
                    team_id: feed.user_id,
                },
                _ => RatelResource::Reply {
                    team_id: feed.user_id,
                },
            },
            match feed.feed_type {
                FeedType::Post => GroupPermission::EditPosts,
                _ => GroupPermission::WriteReplies,
            },
        )
        .await?;

        let res = self.repo.update(id, param.into()).await?;

        Ok(res)
    }

    async fn delete(&self, id: i64, auth: Option<Authorization>) -> Result<Feed> {
        if auth.is_none() {
            return Err(Error::Unauthorized);
        }
        let mut tx = self.pool.begin().await?;

        let res = self
            .repo
            .delete_with_tx(tx.as_mut(), id)
            .await
            .map_err(|e| {
                tracing::error!("failed to delete feed: {:?}", e);
                Error::DatabaseException("Delete operation failed".to_string())
            })?
            .ok_or_else(|| {
                tracing::error!("Delete operation returned None");
                Error::DatabaseException("Delete operation failed".to_string())
            })?;

        tracing::debug!("deleted feed: {:?}", res.id);

        if res.parent_id.is_some() && res.feed_type == FeedType::Repost {
            let share = FeedShare::query_builder()
                .feed_id_equals(res.parent_id.unwrap())
                .user_id_equals(res.user_id)
                .query()
                .map(FeedShare::from)
                .fetch_optional(&self.pool)
                .await?
                .ok_or_else(|| {
                    tracing::error!("Delete operation returned None");
                    Error::DatabaseException("Delete operation failed".to_string())
                })?;

            FeedShare::get_repository(self.pool.clone())
                .delete_with_tx(tx.as_mut(), share.id)
                .await
                .map_err(|e| {
                    tracing::error!("failed to delete feed share: {:?}", e);
                    Error::DatabaseException(e.to_string())
                })?;
        }
        tx.commit().await?;

        Ok(res)
    }

    async fn unrepost(&self, id: i64, auth: Option<Authorization>) -> Result<Feed> {
        if auth.is_none() {
            return Err(Error::Unauthorized);
        }

        let user_id = extract_user_id(&self.pool, auth).await?;
        let mut tx = self.pool.begin().await?;

        let share = FeedShare::query_builder()
            .feed_id_equals(id)
            .user_id_equals(user_id)
            .query()
            .map(FeedShare::from)
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| {
                tracing::error!("Delete operation returned None");
                Error::DatabaseException("Delete operation failed".to_string())
            })?;

        FeedShare::get_repository(self.pool.clone())
            .delete_with_tx(tx.as_mut(), share.id)
            .await
            .map_err(|e| {
                tracing::error!("failed to delete feed share: {:?}", e);
                Error::DatabaseException(e.to_string())
            })?;

        let feed = Feed::query_builder(user_id)
            .parent_id_equals(id)
            .status_not_equals(FeedStatus::Draft)
            .feed_type_equals(FeedType::Repost)
            .user_id_equals(user_id)
            .query()
            .map(Feed::from)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("failed to get a feed {id}: {e}");
                Error::FeedInvalidParentId
            })?;
        let res = self
            .repo
            .delete_with_tx(tx.as_mut(), feed.id)
            .await
            .map_err(|e| {
                tracing::error!("failed to delete feed: {:?}", e);
                Error::DatabaseException("Delete operation failed".to_string())
            })?
            .ok_or_else(|| {
                tracing::error!("Delete operation returned None");
                Error::DatabaseException("Delete operation failed".to_string())
            })?;

        tx.commit().await?;

        Ok(res)
    }

    pub async fn like(&self, id: i64, auth: Option<Authorization>, value: bool) -> Result<Feed> {
        let user_id = extract_user_id(&self.pool, auth).await?;
        let repo = FeedUser::get_repository(self.pool.clone());
        if !value {
            let feed_user = FeedUser::query_builder()
                .feed_id_equals(id)
                .user_id_equals(user_id)
                .query()
                .map(FeedUser::from)
                .fetch_optional(&self.pool)
                .await?;
            if let Some(feed_user) = feed_user {
                repo.delete(feed_user.id).await?;
            }
        } else {
            repo.insert(id, user_id).await?;
        }

        Ok(Feed::default())
    }

    pub async fn publish_draft(&self, id: i64, auth: Option<Authorization>) -> Result<Feed> {
        let feed = Feed::query_builder(0)
            .id_equals(id)
            .status_equals(FeedStatus::Draft)
            .query()
            .map(Feed::from)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("failed to get a feed {id}: {e}");
                Error::FeedPublishError
            })?;

        check_perm(
            &self.pool,
            auth,
            RatelResource::Post {
                team_id: feed.user_id,
            },
            GroupPermission::WritePendingPosts,
        )
        .await?;
        let res = self
            .repo
            .update(
                id,
                FeedRepositoryUpdateRequest {
                    status: Some(FeedStatus::Published),
                    ..Default::default()
                },
            )
            .await?;

        Ok(res)
    }
    // async fn run_read_action(
    //     &self,
    //     _auth: Option<Authorization>,
    //     FeedReadAction { action, .. }: FeedReadAction,
    // ) -> Result<Feed> {
    //     todo!()
    // }

    pub async fn get(&self, id: i64, auth: Option<Authorization>) -> Result<Feed> {
        let user = extract_user(&self.pool, auth.clone())
            .await
            .unwrap_or_default();
        let user_id = user.id;
        let teams = user.teams;

        let mut feed = Feed::query_builder(user_id)
            .comment_list_builder(
                Comment::query_builder(user_id).replies_builder(Reply::query_builder()),
            )
            .id_equals(id)
            .query()
            .map(Feed::from)
            .fetch_one(&self.pool)
            .await?;

        if !feed.author.is_empty() {
            let author = feed.author[0].clone();
            if !feed.spaces.is_empty() {
                let space = feed.spaces[0].clone();
                let should_filter = space.status == SpaceStatus::Draft
                    && author.id != user_id
                    && !teams.iter().any(|t| t.id == author.id);

                if !should_filter {
                    feed.spaces = vec![space];
                } else {
                    feed.spaces = vec![];
                }
            }
        }

        if feed.status == FeedStatus::Draft {
            check_perm(
                &self.pool,
                auth,
                RatelResource::Post { team_id: user_id },
                GroupPermission::ReadPostDrafts,
            )
            .await?;
        }

        Ok(feed)
    }
}

impl FeedController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Feed::get_repository(pool.clone());
        let space_repo = Space::get_repository(pool.clone());

        Self {
            repo,
            space_repo,
            pool,
        }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/:id", get(Self::get_feed_by_id).post(Self::act_feed_by_id))
            .route("/", post(Self::act_feed).get(Self::get_feed))
            .with_state(self.clone()))
    }

    pub async fn act_feed(
        State(ctrl): State<FeedController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<FeedAction>,
    ) -> Result<Json<Feed>> {
        tracing::debug!("act_feed {:?}", body);
        let feed = match body {
            FeedAction::CreateDraft(param) => ctrl.create_draft(auth, param).await?,
            FeedAction::Comment(param) => ctrl.comment(auth, param).await?,
            FeedAction::Repost(param) => ctrl.repost(auth, param).await?,
            // FeedAction::WritePost(param) => ctrl.write_post(auth, param).await?,

            // FeedAction::ReviewDoc(param) => ctrl.review_doc(auth, param).await?,
        };

        Ok(Json(feed))
    }

    pub async fn act_feed_by_id(
        State(ctrl): State<FeedController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(FeedPath { id }): Path<FeedPath>,
        Json(body): Json<FeedByIdAction>,
    ) -> Result<Json<Feed>> {
        tracing::debug!("act_feed_by_id {:?} {:?}", id, body);
        match body {
            FeedByIdAction::Update(param) => {
                let res = ctrl.update(id, auth, param).await?;
                Ok(Json(res))
            }
            FeedByIdAction::Edit(param) => {
                let res = ctrl.edit(id, auth, param).await?;
                Ok(Json(res))
            }
            FeedByIdAction::Delete(_) => {
                let res = ctrl.delete(id, auth).await?;
                Ok(Json(res))
            }
            FeedByIdAction::Like(FeedLikeRequest { value }) => {
                let res = ctrl.like(id, auth, value).await?;
                Ok(Json(res))
            }
            FeedByIdAction::Publish(_) => {
                let res = ctrl.publish_draft(id, auth).await?;
                Ok(Json(res))
            }
            FeedByIdAction::Unrepost(_) => {
                let res = ctrl.unrepost(id, auth).await?;
                Ok(Json(res))
            }
        }
    }
    pub async fn get_feed_by_id(
        State(ctrl): State<FeedController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(FeedPath { id }): Path<FeedPath>,
    ) -> Result<Json<Feed>> {
        tracing::debug!("get_feed {:?}", id);

        let feed = ctrl.get(id, auth).await?;
        Ok(Json(feed))
    }

    pub async fn get_feed(
        State(ctrl): State<FeedController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<FeedParam>,
    ) -> Result<Json<FeedGetResponse>> {
        tracing::debug!("list_feed {:?}", q);

        match q {
            FeedParam::Query(param) => match param.action {
                Some(FeedQueryActionType::PostsByUserId) => Ok(Json(FeedGetResponse::Query(
                    ctrl.posts_by_user_id(auth, param).await?,
                ))),

                None => Ok(Json(FeedGetResponse::Query(ctrl.query(auth, param).await?))),
            },
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
        // predefined industry: Crypto
        let industry_id = 1;

        let post = Feed::get_repository(pool.clone())
            .insert(
                FeedType::Post,
                user.id,
                industry_id,
                None,
                None,
                title,
                html_contents.clone(),
                None,
                UrlType::None,
                vec![],
                0,
                FeedStatus::Draft,
            )
            .await
            .unwrap();

        let _ = Feed::get_repository(pool.clone())
            .insert(
                FeedType::Reply,
                user.id,
                industry_id,
                Some(post.id),
                None,
                None,
                html_contents,
                None,
                UrlType::None,
                vec![],
                0,
                FeedStatus::Published,
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_write_post() {
        test_setup().await;
        let TestContext {
            user,
            now,
            endpoint,
            ..
        } = setup().await.unwrap();
        let html_contents = format!("<p>Test {now}</p>");
        let title = Some(format!("Test Title {now}"));
        // predefined industry: Crypto
        let industry_id = 1;
        let res = Feed::get_client(&endpoint)
            .create_draft(FeedType::Post, user.id)
            .await;
        assert!(res.is_ok());
        let res = res.unwrap();
        let res = Feed::get_client(&endpoint)
            .update(
                res.id,
                industry_id,
                None,
                None,
                title.clone(),
                html_contents.clone(),
                None,
                UrlType::None,
                vec![],
            )
            .await;

        assert!(res.is_ok());

        let feed = res.unwrap();
        assert_eq!(feed.html_contents, html_contents);
        assert_eq!(feed.industry_id, industry_id);
        assert_eq!(feed.title, title);
        assert_eq!(feed.feed_type, FeedType::Post);
        assert_eq!(feed.user_id, user.id);
        assert_eq!(feed.parent_id, None);
        assert_eq!(feed.quote_feed_id, None);
    }

    #[tokio::test]
    async fn test_write_post_with_quote() {
        test_setup().await;
        let TestContext {
            user,
            now,
            endpoint,
            pool,
            ..
        } = setup().await.unwrap();

        let html_contents = format!("<p>Test {now}</p>");
        let title = Some(format!("Test Title {now}"));
        // predefined industry: Crypto
        let industry_id = 1;

        let quote = Feed::query_builder(user.id)
            .feed_type_equals(FeedType::Reply)
            .order_by_created_at_asc()
            .limit(1)
            .query()
            .map(Feed::from)
            .fetch_one(&pool)
            .await
            .unwrap();

        let feed = Feed::get_client(&endpoint)
            .create_draft(FeedType::Post, user.id)
            .await;
        assert!(feed.is_ok());
        let feed = feed.unwrap();
        let res = Feed::get_client(&endpoint)
            .update(
                feed.id,
                industry_id,
                None,
                Some(quote.id),
                title.clone(),
                html_contents.clone(),
                None,
                UrlType::None,
                vec![],
            )
            .await;
        // let res = Feed::get_client(&endpoint)
        //     .write_post(
        //         html_contents.clone(),
        //         user.id,
        //         industry_id,
        //         title.clone(),
        //         Some(quote.id),
        //         vec![],
        //         None,
        //         UrlType::None,
        //     )
        //     .await;

        assert!(res.is_ok());

        let feed = res.unwrap();
        assert_eq!(feed.html_contents, html_contents);
        assert_eq!(feed.industry_id, industry_id);
        assert_eq!(feed.title, title);
        assert_eq!(feed.feed_type, FeedType::Post);
        assert_eq!(feed.user_id, user.id);
        assert_eq!(feed.parent_id, None);
        assert_eq!(feed.quote_feed_id, Some(quote.id));
    }

    #[tokio::test]
    async fn test_write_comment() {
        test_setup().await;
        let TestContext {
            user,
            now,
            endpoint,
            pool,
            ..
        } = setup().await.unwrap();

        let html_contents = format!("<p>Test {now}</p>");
        let title = Some(format!("Test Title {now}"));
        // predefined industry: Crypto
        let industry_id = 1;

        let _ = Feed::get_repository(pool.clone())
            .insert(
                FeedType::Post,
                user.id,
                industry_id,
                None,
                None,
                title,
                html_contents.clone(),
                None,
                UrlType::None,
                vec![],
                0,
                FeedStatus::Published,
            )
            .await
            .unwrap();

        let post = Feed::query_builder(user.id)
            .feed_type_equals(FeedType::Post)
            .status_not_equals(FeedStatus::Draft)
            .order_by_created_at_asc()
            .limit(1)
            .query()
            .map(Feed::from)
            .fetch_one(&pool)
            .await
            .unwrap();

        let html_contents = format!("<p>Comment {now}</p>");

        let res = Feed::get_client(&endpoint)
            .comment(Some(post.id), html_contents.clone())
            .await;

        assert!(res.is_ok(), "res: {:?}", res);

        let feed = res.unwrap();
        assert_eq!(feed.html_contents, html_contents);
        assert_eq!(feed.industry_id, post.industry_id);
        assert_eq!(feed.title, None);
        assert_eq!(feed.feed_type, FeedType::Reply);
        assert_eq!(feed.parent_id, Some(post.id));
        assert_eq!(feed.user_id, user.id);
    }

    #[tokio::test]
    async fn test_write_repost_as_comment() {
        test_setup().await;
        let TestContext {
            user,
            now,
            endpoint,
            pool,
            ..
        } = setup().await.unwrap();

        let html_contents = format!("<p>Test {now}</p>");
        let title = Some(format!("Test Title {now}"));
        // predefined industry: Crypto
        let industry_id = 1;

        let post = Feed::get_repository(pool.clone())
            .insert(
                FeedType::Post,
                user.id,
                industry_id,
                None,
                None,
                title,
                html_contents.clone(),
                None,
                UrlType::None,
                vec![],
                0,
                FeedStatus::Published,
            )
            .await
            .unwrap();

        let _ = Feed::get_repository(pool.clone())
            .insert(
                FeedType::Reply,
                user.id,
                industry_id,
                Some(post.id),
                None,
                None,
                html_contents,
                None,
                UrlType::None,
                vec![],
                0,
                FeedStatus::Published,
            )
            .await
            .unwrap();

        let post = Feed::query_builder(user.id)
            .feed_type_equals(FeedType::Post)
            .status_not_equals(FeedStatus::Draft)
            .order_by_created_at_asc()
            .limit(1)
            .query()
            .map(Feed::from)
            .fetch_one(&pool)
            .await
            .unwrap();

        let quote_feed = Feed::query_builder(user.id)
            .feed_type_equals(FeedType::Reply)
            .status_not_equals(FeedStatus::Draft)
            .order_by_created_at_asc()
            .limit(1)
            .query()
            .map(Feed::from)
            .fetch_one(&pool)
            .await
            .unwrap();

        let html_contents = format!(
            "<quote-feed>{}</quote-feed><p>Repost {}</p>",
            quote_feed.id, now
        );

        let res = Feed::get_client(&endpoint)
            .repost(
                user.id,
                Some(post.id),
                Some(quote_feed.id),
                html_contents.clone(),
            )
            .await;

        assert!(res.is_ok(), "res: {:?}", res);

        let feed = res.unwrap();
        assert_eq!(feed.html_contents, html_contents);
        assert_eq!(feed.industry_id, post.industry_id);
        assert_eq!(feed.title, None);
        assert_eq!(feed.feed_type, FeedType::Repost);
        assert_eq!(feed.parent_id, Some(post.id));
        assert_eq!(feed.quote_feed_id, Some(quote_feed.id));
        assert_eq!(feed.user_id, user.id);
    }

    #[tokio::test]
    async fn test_write_simple_repost() {
        test_setup().await;
        let TestContext {
            user,
            now,
            endpoint,
            pool,
            ..
        } = setup().await.unwrap();

        let html_contents = format!("<p>Test {now}</p>");
        let title = Some(format!("Test Title {now}"));
        // predefined industry: Crypto
        let industry_id = 1;

        // Create a new post to act as the parent
        let parent_post = Feed::get_repository(pool.clone())
            .insert(
                FeedType::Post,
                user.id,
                industry_id,
                None,
                None,
                title,
                html_contents.clone(),
                None,
                UrlType::None,
                vec![],
                0,
                FeedStatus::Published,
            )
            .await
            .unwrap();

        let repost_html_contents = format!("<p>Simple repost {}</p>", now);

        let res = Feed::get_client(&endpoint)
            .repost(
                user.id,
                Some(parent_post.id),
                None, // No quote_feed_id for simple repost
                repost_html_contents.clone(),
            )
            .await;

        assert!(res.is_ok(), "res: {:?}", res);

        let feed = res.unwrap();
        assert_eq!(feed.html_contents, repost_html_contents);
        assert_eq!(feed.industry_id, parent_post.industry_id);
        assert_eq!(feed.title, None);
        assert_eq!(feed.feed_type, FeedType::Repost);
        assert_eq!(feed.parent_id, Some(parent_post.id));
        assert_eq!(feed.quote_feed_id, None);
        assert_eq!(feed.user_id, user.id);

        // Check that the parent post's shares count is incremented
        let updated_parent = Feed::query_builder(user.id)
            .id_equals(parent_post.id)
            .query()
            .map(Feed::from)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(updated_parent.shares, 1);
    }

    #[tokio::test]
    async fn test_comment_with_invalid_parent_id() {
        let TestContext { now, endpoint, .. } = setup().await.unwrap();

        let html_contents = format!("<p>Comment {now}</p>");

        let res = Feed::get_client(&endpoint)
            .comment(Some(0), html_contents.clone())
            .await;

        assert!(res.is_err(), "res: {:?}", res);

        assert_eq!(res, Err(Error::FeedInvalidParentId));
    }

    #[tokio::test]
    async fn test_comment_with_none() {
        let TestContext { now, endpoint, .. } = setup().await.unwrap();

        let html_contents = format!("<p>Comment {now}</p>");

        let res = Feed::get_client(&endpoint)
            .comment(None, html_contents.clone())
            .await;

        assert!(res.is_err(), "res: {:?}", res);

        assert_eq!(res, Err(Error::FeedInvalidParentId));
    }

    // #[tokio::test]
    // async fn test_review_with_invalid_parent_id() {
    //     let TestContext {
    //         user,
    //         now,
    //         endpoint,
    //         ..
    //     } = setup().await.unwrap();

    //     let html_contents = format!("<p>Review {now}</p>");

    //     let res = Feed::get_client(&endpoint)
    //         .review_doc(html_contents, user.id, Some(0), Some(1))
    //         .await;

    //     assert!(res.is_err(), "res: {:?}", res);

    //     assert_eq!(res, Err(Error::FeedInvalidParentId));
    // }

    #[tokio::test]
    async fn test_repost_with_invalid_parent_id() {
        test_setup().await;
        let TestContext {
            pool,
            now,
            endpoint,
            user,
            ..
        } = setup().await.unwrap();

        let html_contents = format!("<p>Review {now}</p>");

        let quote = Feed::query_builder(0)
            .feed_type_equals(FeedType::Reply)
            .order_by_created_at_asc()
            .limit(1)
            .query()
            .map(Feed::from)
            .fetch_one(&pool)
            .await
            .unwrap();

        let res = Feed::get_client(&endpoint)
            .repost(user.id, Some(0), Some(quote.id), html_contents)
            .await;

        assert!(res.is_err(), "res: {:?}", res);

        assert_eq!(res, Err(Error::FeedInvalidParentId));
    }

    #[tokio::test]
    async fn test_repost_with_invalid_quote_id() {
        let TestContext {
            pool,
            now,
            endpoint,
            user,
            ..
        } = setup().await.unwrap();

        let html_contents = format!("<p>Test {now}</p>");
        let title = Some(format!("Test Title {now}"));
        // predefined industry: Crypto
        let industry_id = 1;

        let _ = Feed::get_repository(pool.clone())
            .insert(
                FeedType::Post,
                user.id,
                industry_id,
                None,
                None,
                title,
                html_contents.clone(),
                None,
                UrlType::None,
                vec![],
                0,
                FeedStatus::Published,
            )
            .await
            .unwrap();

        let html_contents = format!("<p>Review {now}</p>");

        let feed = Feed::query_builder(0)
            .feed_type_equals(FeedType::Post)
            .status_not_equals(FeedStatus::Draft)
            .order_by_created_at_asc()
            .limit(1)
            .query()
            .map(Feed::from)
            .fetch_one(&pool)
            .await
            .unwrap();

        let res = Feed::get_client(&endpoint)
            .repost(user.id, Some(feed.id), Some(0), html_contents)
            .await;

        assert!(res.is_err(), "res: {:?}", res);

        assert_eq!(res, Err(Error::FeedInvalidQuoteId));
    }

    #[tokio::test]
    async fn test_delete_repost_decrements_shares() {
        test_setup().await;
        let TestContext {
            user,
            now,
            endpoint,
            pool,
            ..
        } = setup().await.unwrap();

        let html_contents = format!("<p>Test {now}</p>");
        let title = Some(format!("Test Title {now}"));
        // predefined industry: Crypto
        let industry_id = 1;

        // Create a new post to act as the parent
        let parent_post = Feed::get_repository(pool.clone())
            .insert(
                FeedType::Post,
                user.id,
                industry_id,
                None,
                None,
                title,
                html_contents.clone(),
                None,
                UrlType::None,
                vec![],
                0,
                FeedStatus::Published,
            )
            .await
            .unwrap();

        let repost_html_contents = format!("<p>Simple repost {}</p>", now);

        // Create a repost
        let repost = Feed::get_client(&endpoint)
            .repost(
                user.id,
                Some(parent_post.id),
                None,
                repost_html_contents.clone(),
            )
            .await
            .unwrap();

        // Verify shares count is incremented
        let updated_parent = Feed::query_builder(user.id)
            .id_equals(parent_post.id)
            .query()
            .map(Feed::from)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(updated_parent.shares, 1);

        // Delete the repost
        let res = Feed::get_client(&endpoint).delete(repost.id).await;

        assert!(res.is_ok(), "res: {:?}", res);

        // Check that the parent post's shares count is decremented
        let updated_parent = Feed::query_builder(user.id)
            .id_equals(parent_post.id)
            .query()
            .map(Feed::from)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(updated_parent.shares, 0);
    }

    #[tokio::test]
    async fn test_unrepost_endpoint() {
        test_setup().await;
        let TestContext {
            user,
            now,
            endpoint,
            pool,
            ..
        } = setup().await.unwrap();

        let html_contents = format!("<p>Test {now}</p>");
        let title = Some(format!("Test Title {now}"));
        // predefined industry: Crypto
        let industry_id = 1;

        // Create a new post to act as the parent
        let parent_post = Feed::get_repository(pool.clone())
            .insert(
                FeedType::Post,
                user.id,
                industry_id,
                None,
                None,
                title,
                html_contents.clone(),
                None,
                UrlType::None,
                vec![],
                0,
                FeedStatus::Published,
            )
            .await
            .unwrap();

        let repost_html_contents = format!("<p>Simple repost {}</p>", now);

        // Create a repost
        let repost = Feed::get_client(&endpoint)
            .repost(
                user.id,
                Some(parent_post.id),
                None,
                repost_html_contents.clone(),
            )
            .await
            .unwrap();

        // Verify shares count is incremented
        let updated_parent = Feed::query_builder(user.id)
            .id_equals(parent_post.id)
            .query()
            .map(Feed::from)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(updated_parent.shares, 1);

        // Unrepost using the unrepost endpoint
        let res = Feed::get_client(&endpoint).unrepost(parent_post.id).await;

        assert!(res.is_ok(), "res: {:?}", res);

        // Check that the parent post's shares count is decremented
        let updated_parent = Feed::query_builder(user.id)
            .id_equals(parent_post.id)
            .query()
            .map(Feed::from)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(updated_parent.shares, 0);

        // Verify the repost is deleted
        let repost_result = Feed::query_builder(user.id)
            .id_equals(repost.id)
            .query()
            .map(Feed::from)
            .fetch_optional(&pool)
            .await
            .unwrap();

        assert!(repost_result.is_none(), "Repost should be deleted");
    }

    #[tokio::test]
    async fn test_delete_quote_repost_decrements_shares() {
        test_setup().await;
        let TestContext {
            user,
            now,
            endpoint,
            pool,
            ..
        } = setup().await.unwrap();

        let html_contents = format!("<p>Test {now}</p>");
        let title = Some(format!("Test Title {now}"));
        // predefined industry: Crypto
        let industry_id = 1;

        // Create parent post
        let parent_post = Feed::get_repository(pool.clone())
            .insert(
                FeedType::Post,
                user.id,
                industry_id,
                None,
                None,
                title.clone(),
                html_contents.clone(),
                None,
                UrlType::None,
                vec![],
                0,
                FeedStatus::Published,
            )
            .await
            .unwrap();

        // Create quote feed (reply)
        let quote_feed = Feed::get_repository(pool.clone())
            .insert(
                FeedType::Reply,
                user.id,
                industry_id,
                Some(parent_post.id),
                None,
                None,
                html_contents,
                None,
                UrlType::None,
                vec![],
                0,
                FeedStatus::Published,
            )
            .await
            .unwrap();

        let repost_html_contents = format!(
            "<quote-feed>{}</quote-feed><p>Quote repost {}</p>",
            quote_feed.id, now
        );

        // Create a quote repost
        let repost = Feed::get_client(&endpoint)
            .repost(
                user.id,
                Some(parent_post.id),
                Some(quote_feed.id),
                repost_html_contents.clone(),
            )
            .await
            .unwrap();

        // Verify shares count is incremented
        let updated_parent = Feed::query_builder(user.id)
            .id_equals(parent_post.id)
            .query()
            .map(Feed::from)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(updated_parent.shares, 1);

        // Delete the repost
        let res = Feed::get_client(&endpoint).delete(repost.id).await;

        assert!(res.is_ok(), "res: {:?}", res);

        // Check that the parent post's shares count is decremented
        let updated_parent = Feed::query_builder(user.id)
            .id_equals(parent_post.id)
            .query()
            .map(Feed::from)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(updated_parent.shares, 0);
    }
}
