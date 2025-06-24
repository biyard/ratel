mod badges;
mod comments;
mod discussions;
mod meeting;
mod redeem_codes;

use crate::security::check_perm;
use crate::{by_axum::axum::extract::Query, utils::users::extract_user_id};
use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{Extension, Json, extract::State, routing::post},
};
use dto::{by_axum::axum::extract::Path, *};

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
    discussion_repo: DiscussionRepository,
    elearning_repo: ElearningRepository,
    survey_repo: SurveyRepository,
    space_draft_repo: SpaceDraftRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl SpaceController {
    async fn get_space_by_id(&self, _auth: Option<Authorization>, id: i64) -> Result<Space> {
        // let user: std::result::Result<User, Error> =
        //     extract_user_with_allowing_anonymous(&self.pool, auth).await;
        // tracing::debug!("user: {:?}", user);

        let mut tx = self.pool.begin().await?;

        let space = Space::query_builder()
            .id_equals(id)
            .comments_builder(SpaceComment::query_builder())
            .feed_comments_builder(SpaceComment::query_builder())
            .query()
            .map(Space::from)
            .fetch_one(&mut *tx)
            .await?;
        // if let Ok(user) = user {
        //     let redeem_codes = RedeemCode::query_builder()
        //         .user_id_equals(user.id)
        //         .meta_id_equals(id)
        //         .query()
        //         .map(RedeemCode::from)
        //         .fetch_optional(&mut *tx)
        //         .await?;
        //     if redeem_codes.is_some() {
        //         space.codes = vec![redeem_codes.unwrap()];
        //     } else {
        //         let redeem_code_repo = RedeemCode::get_repository(self.pool.clone());
        //         let mut codes = vec![];
        //         for _ in 0..space.num_of_redeem_codes {
        //             let id = Uuid::new_v4().to_string();
        //             codes.push(id);
        //         }
        //         let res = redeem_code_repo
        //             .insert_with_tx(&mut *tx, user.id, id, codes, vec![])
        //             .await?;
        //         if res.is_none() {
        //             tracing::error!("failed to insert redeem codes for space {id}");
        //             return Err(Error::RedeemCodeCreationFailure);
        //         } else {
        //             space.codes = vec![res.unwrap()];
        //         }
        //     }
        // }
        tx.commit().await?;
        Ok(space)
    }

    async fn posting_space(&self, space_id: i64, auth: Option<Authorization>) -> Result<Space> {
        let user_id = extract_user_id(&self.pool, auth.clone())
            .await
            .unwrap_or_default();

        let space = Space::query_builder()
            .id_equals(space_id)
            .query()
            .map(Space::from)
            .fetch_one(&self.pool.clone())
            .await
            .map_err(|e| {
                tracing::error!("failed to get a space {space_id}: {e}");
                Error::FeedInvalidQuoteSpaceId
            })?;

        let feed = Feed::query_builder(user_id)
            .id_equals(space.feed_id)
            .query()
            .map(Feed::from)
            .fetch_one(&self.pool.clone())
            .await
            .map_err(|e| {
                tracing::error!("failed to get a feed {:?}: {e}", space.feed_id);
                Error::FeedInvalidQuoteId
            })?;

        let _ = check_perm(
            &self.pool,
            auth,
            RatelResource::Post {
                team_id: feed.user_id,
            },
            GroupPermission::WritePosts,
        )
        .await?;

        let mut tx = self.pool.begin().await?;

        match self
            .repo
            .update_with_tx(
                &mut *tx,
                space_id,
                SpaceRepositoryUpdateRequest {
                    status: Some(SpaceStatus::InProgress),
                    ..Default::default()
                },
            )
            .await
        {
            Ok(_) => {}
            Err(e) => {
                tx.rollback().await?;
                return Err(e);
            }
        };

        let surveys = Survey::query_builder()
            .space_id_equals(space_id)
            .query()
            .map(Survey::from)
            .fetch_all(&self.pool.clone())
            .await?;

        if !surveys.is_empty() {
            let survey = surveys[0].clone();

            match self
                .survey_repo
                .update_with_tx(
                    &mut *tx,
                    survey.id,
                    SurveyRepositoryUpdateRequest {
                        started_at: space.started_at,
                        ended_at: space.ended_at,
                        ..Default::default()
                    },
                )
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    tx.rollback().await?;
                    return Err(e);
                }
            }
        }

        tx.commit().await?;

        Ok(space)
    }

    async fn update_space(
        &self,
        space_id: i64,
        auth: Option<Authorization>,
        SpaceUpdateSpaceRequest {
            title,
            html_contents,
            files,
            discussions,
            elearnings,
            surveys,
            drafts,
            started_at,
            ended_at,
        }: SpaceUpdateSpaceRequest,
    ) -> Result<Space> {
        let user_id = extract_user_id(&self.pool, auth.clone())
            .await
            .unwrap_or_default();

        let space = Space::query_builder()
            .id_equals(space_id)
            .query()
            .map(Space::from)
            .fetch_one(&self.pool.clone())
            .await
            .map_err(|e| {
                tracing::error!("failed to get a space {space_id}: {e}");
                Error::FeedInvalidQuoteSpaceId
            })?;

        let feed = Feed::query_builder(user_id)
            .id_equals(space.feed_id)
            .query()
            .map(Feed::from)
            .fetch_one(&self.pool.clone())
            .await
            .map_err(|e| {
                tracing::error!("failed to get a feed {:?}: {e}", space.feed_id);
                Error::FeedInvalidQuoteId
            })?;

        let _ = check_perm(
            &self.pool,
            auth,
            RatelResource::Post {
                team_id: feed.user_id,
            },
            GroupPermission::WritePosts,
        )
        .await?;

        let mut tx = self.pool.begin().await?;

        let res = self
            .repo
            .update_with_tx(
                &mut *tx,
                space_id,
                SpaceRepositoryUpdateRequest {
                    title: title,
                    html_contents: Some(html_contents),
                    files: Some(files),
                    started_at,
                    ended_at,
                    ..Default::default()
                },
            )
            .await?;

        let discs = Discussion::query_builder()
            .space_id_equals(space_id)
            .query()
            .map(Discussion::from)
            .fetch_all(&self.pool.clone())
            .await?;

        for disc in discs {
            match self.discussion_repo.delete_with_tx(&mut *tx, disc.id).await {
                Ok(_) => {}
                Err(e) => {
                    tx.rollback().await?;
                    return Err(e);
                }
            }
        }

        for discussion in discussions {
            match self
                .discussion_repo
                .insert_with_tx(
                    &mut *tx,
                    space_id,
                    user_id,
                    discussion.started_at,
                    discussion.ended_at,
                    discussion.name,
                    discussion.description,
                    None,
                    "".to_string(),
                )
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    tx.rollback().await?;
                    return Err(e);
                }
            }
        }

        let es = Elearning::query_builder()
            .space_id_equals(space_id)
            .query()
            .map(Elearning::from)
            .fetch_all(&self.pool.clone())
            .await?;

        for e in es {
            match self.elearning_repo.delete_with_tx(&mut *tx, e.id).await {
                Ok(_) => {}
                Err(e) => {
                    tx.rollback().await?;
                    return Err(e);
                }
            }
        }

        for elearning in elearnings {
            match self
                .elearning_repo
                .insert_with_tx(&mut *tx, space_id, elearning.files)
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    tx.rollback().await?;
                    return Err(e);
                }
            }
        }

        if !surveys.is_empty() {
            let survey = surveys[0].clone();
            let s = Survey::query_builder()
                .space_id_equals(space_id)
                .query()
                .map(Survey::from)
                .fetch_all(&self.pool.clone())
                .await?;

            if s.is_empty() {
                match self
                    .survey_repo
                    .insert_with_tx(
                        &mut *tx,
                        space_id,
                        ProjectStatus::Ready,
                        survey.started_at,
                        survey.ended_at,
                        survey.questions,
                    )
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        tx.rollback().await?;
                        return Err(e);
                    }
                }
            } else {
                let s = s[0].clone();

                match self
                    .survey_repo
                    .update_with_tx(
                        &mut *tx,
                        s.id,
                        SurveyRepositoryUpdateRequest {
                            started_at: Some(survey.started_at),
                            ended_at: Some(survey.ended_at),
                            questions: Some(survey.questions),
                            ..Default::default()
                        },
                    )
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        tx.rollback().await?;
                        return Err(e);
                    }
                }
            }
        }

        if !drafts.is_empty() {
            let draft = drafts[0].clone();
            let s = SpaceDraft::query_builder()
                .space_id_equals(space_id)
                .query()
                .map(SpaceDraft::from)
                .fetch_all(&self.pool.clone())
                .await?;

            if s.is_empty() {
                match self
                    .space_draft_repo
                    .insert_with_tx(
                        &mut *tx,
                        space_id,
                        draft.title,
                        draft.html_contents,
                        draft.files,
                    )
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        tx.rollback().await?;
                        return Err(e);
                    }
                }
            } else {
                let s = s[0].clone();

                match self
                    .space_draft_repo
                    .update_with_tx(
                        &mut *tx,
                        s.id,
                        SpaceDraftRepositoryUpdateRequest {
                            title: Some(draft.title),
                            html_contents: Some(draft.html_contents),
                            files: Some(draft.files),
                            ..Default::default()
                        },
                    )
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        tx.rollback().await?;
                        return Err(e);
                    }
                }
            }
        }

        tx.commit().await?;

        Ok(res.unwrap())
    }

    async fn create_space(
        &self,
        auth: Option<Authorization>,
        SpaceCreateSpaceRequest {
            space_type,
            feed_id,
            user_ids,
            num_of_redeem_codes,
        }: SpaceCreateSpaceRequest,
    ) -> Result<Space> {
        let _ = space_type;
        let user_id = extract_user_id(&self.pool, auth.clone())
            .await
            .unwrap_or_default();

        let feed = Feed::query_builder(user_id)
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
        let mut tx = self.pool.begin().await?;

        let res = self
            .repo
            .insert_with_tx(
                &mut *tx,
                feed.title,
                feed.html_contents,
                space_type,
                user.id,
                feed.industry_id,
                None,
                None,
                feed_id,
                SpaceStatus::Draft,
                feed.files,
                num_of_redeem_codes,
            )
            .await
            .map_err(|e| {
                tracing::error!("failed to insert post space: {:?}", e);
                Error::SpaceWritePostError
            })?
            .ok_or(Error::SpaceWritePostError)?;

        let g = SpaceGroup::get_repository(self.pool.clone());
        let group = g
            .insert_with_tx(&mut *tx, res.id, "Admin".to_string())
            .await?
            .ok_or(Error::SpaceWritePostError)?;

        for id in user_ids {
            let _ = self
                .space_member_repo
                .insert_with_tx(&mut *tx, id, res.id, group.id)
                .await
                .map_err(|e| {
                    tracing::error!("failed to insert space with member error: {:?}", e);
                    Error::SpaceWritePostError
                })?;
        }
        tx.commit().await?;

        Ok(res)
    }
}

impl SpaceController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Space::get_repository(pool.clone());
        let space_member_repo = SpaceMember::get_repository(pool.clone());
        let space_draft_repo = SpaceDraft::get_repository(pool.clone());
        let discussion_repo = Discussion::get_repository(pool.clone());
        let elearning_repo = Elearning::get_repository(pool.clone());
        let survey_repo = Survey::get_repository(pool.clone());

        Self {
            repo,
            pool,
            discussion_repo,
            elearning_repo,
            survey_repo,
            space_member_repo,
            space_draft_repo,
        }
    }

    pub async fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_space).get(Self::get_space))
            .with_state(self.clone())
            .route("/:id", post(Self::act_space_by_id).get(Self::get_by_id))
            .with_state(self.clone())
            .nest(
                "/:space-id/comments",
                comments::SpaceCommentController::new(self.pool.clone()).route(),
            )
            .nest(
                "/:space-id/discussions",
                discussions::SpaceDiscussionController::new(self.pool.clone())
                    .await
                    .route(),
            )
            .nest(
                "/:space-id/meeting",
                meeting::SpaceMeetingController::new(self.pool.clone())
                    .await
                    .route(),
            )
            .nest(
                "/:space-id/badges",
                badges::SpaceBadgeController::new(self.pool.clone())
                    .await
                    .route(),
            )
            .nest(
                "/:space-id/redeem-codes",
                redeem_codes::SpaceRedeemCodeController::new(self.pool.clone()).route(),
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

    pub async fn act_space_by_id(
        State(ctrl): State<SpaceController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(SpacePath { id }): Path<SpacePath>,
        Json(body): Json<SpaceByIdAction>,
    ) -> Result<Json<Space>> {
        tracing::debug!("act_space_by_id {:?} {:?}", id, body);
        let feed = match body {
            SpaceByIdAction::UpdateSpace(param) => ctrl.update_space(id, auth, param).await?,
            SpaceByIdAction::PostingSpace(_) => ctrl.posting_space(id, auth).await?,
        };

        Ok(Json(feed))
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
