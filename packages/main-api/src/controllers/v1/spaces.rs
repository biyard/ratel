mod badges;
mod comments;
mod discussions;
mod meeting;
mod notice_quiz_answers;
mod notice_quiz_attempts;
mod redeem_codes;
mod responses;
mod sprint_leagues;

use std::collections::{HashMap, HashSet};

use crate::security::check_perm;
use crate::utils::aws_media_convert::merge_recording_chunks;
use crate::utils::users::extract_user_id_with_no_error;
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
    feed_repo: FeedRepository,
    space_member_repo: SpaceMemberRepository,
    discussion_repo: DiscussionRepository,
    discussion_member_repo: DiscussionMemberRepository,
    elearning_repo: ElearningRepository,
    survey_repo: SurveyRepository,
    space_draft_repo: SpaceDraftRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
    notice_answer_repo: NoticeQuizAnswerRepository,
}

impl SpaceController {
    async fn get_space_by_id(&self, auth: Option<Authorization>, id: i64) -> Result<Space> {
        let user_id = extract_user_id_with_no_error(&self.pool, auth).await;
        tracing::debug!("user id: {:?}", user_id);
        // tracing::debug!("user: {:?}", user);

        let mut space = Space::query_builder(user_id)
            .id_equals(id)
            .discussions_builder(Discussion::query_builder())
            .comments_builder(SpaceComment::query_builder())
            .feed_comments_builder(SpaceComment::query_builder())
            .sprint_leagues_builder(
                SprintLeague::query_builder(user_id)
                    .players_builder(SprintLeaguePlayer::query_builder()),
            )
            .query()
            .map(Space::from)
            .fetch_one(&self.pool)
            .await?;

        // Access control for notice spaces
        if space.space_type == SpaceType::Notice {
            let is_owner = user_id == space.owner_id;

            match space.status {
                SpaceStatus::Draft => {
                    // Draft notice spaces: only owner has access
                    if !is_owner {
                        tracing::warn!(
                            "Access denied for user {} to draft notice space {} - only owner allowed",
                            user_id,
                            id
                        );
                        return Err(Error::Unauthorized);
                    }
                }
                _ => {
                    // published spaces
                    match space.publishing_scope {
                        PublishingScope::Private => {
                            // Private: only space members can access
                            let is_member = if user_id != 0 {
                                SpaceMember::query_builder()
                                    .space_id_equals(id)
                                    .user_id_equals(user_id)
                                    .query()
                                    .map(SpaceMember::from)
                                    .fetch_optional(&self.pool)
                                    .await?
                                    .is_some()
                            } else {
                                false
                            };

                            if !is_owner && !is_member {
                                tracing::warn!(
                                    "Access denied for user {} to private notice space {} - not owner or member",
                                    user_id,
                                    id
                                );
                                return Err(Error::Unauthorized);
                            }
                        }
                        PublishingScope::Public => {
                            // Public: all users can access (no additional checks needed)
                            tracing::debug!(
                                "Allowing access to public notice space {} for user {}",
                                id,
                                user_id
                            );
                        }
                    }
                }
            }
        }

        let user_response = if user_id != 0 {
            let mut res: Vec<SurveyResponse> = vec![];

            if space.space_type == SpaceType::Deliberation {
                res = SurveyResponse::query_builder()
                    .space_id_equals(id)
                    .user_id_equals(user_id)
                    .survey_type_equals(SurveyType::Survey)
                    .query()
                    .map(Into::into)
                    .fetch_optional(&self.pool)
                    .await?
                    .map_or_else(Vec::new, |res| vec![res]);
            } else {
                let response: Vec<SurveyResponse> = SurveyResponse::query_builder()
                    .space_id_equals(id)
                    .user_id_equals(user_id)
                    .order_by_created_at_desc()
                    .survey_type_equals(SurveyType::Sample)
                    .query()
                    .map(Into::into)
                    .fetch_optional(&self.pool)
                    .await?
                    .map_or_else(Vec::new, |res| vec![res]);

                if !response.is_empty() {
                    res = vec![response[0].clone()];
                }
            }

            res
        } else {
            Vec::new()
        };

        let responses: Vec<SurveyResponse> = if space.space_type == SpaceType::Deliberation {
            SurveyResponse::query_builder()
                .space_id_equals(id)
                .survey_type_equals(SurveyType::Survey)
                .query()
                .map(Into::into)
                .fetch_all(&self.pool)
                .await?
        } else {
            let sql = r#"
                SELECT DISTINCT ON (user_id) *
                FROM survey_responses
                WHERE space_id = $1 AND survey_type = $2
                ORDER BY user_id, created_at DESC
            "#;

            let response: Vec<SurveyResponse> = sqlx::query(sql)
                .bind(id)
                .bind(SurveyType::Sample)
                .map(SurveyResponse::from)
                .fetch_all(&self.pool)
                .await?;

            response
        };

        let discussions = space.discussions;

        let mut updated_discussions = Vec::with_capacity(discussions.len());
        let mut tx = self.pool.begin().await?;

        for mut discussion in discussions {
            let meeting_id = discussion.meeting_id.clone().unwrap_or_default();
            let pipeline_arn = discussion.media_pipeline_arn.clone().unwrap_or_default();
            let record = discussion.record.clone();

            let record = merge_recording_chunks(&meeting_id, pipeline_arn, record).await;

            if record.is_some() {
                match self
                    .discussion_repo
                    .update_with_tx(
                        &mut *tx,
                        discussion.id,
                        DiscussionRepositoryUpdateRequest {
                            record: record.clone(),
                            ..Default::default()
                        },
                    )
                    .await
                {
                    Ok(v) => tracing::debug!("success to update discussion record: {:?}", v),
                    Err(e) => {
                        tracing::error!("failed to update discussion record with error: {:?}", e)
                    }
                };

                discussion.record = record;
            }

            updated_discussions.push(discussion);
        }

        tx.commit().await?;

        space.user_responses = user_response;
        space.responses = responses;
        space.discussions = updated_discussions;

        Ok(space)
    }

    async fn like_space(&self, id: i64, auth: Option<Authorization>, value: bool) -> Result<Space> {
        let user_id = extract_user_id(&self.pool, auth).await?;
        let repo = SpaceLikeUser::get_repository(self.pool.clone());
        if !value {
            let space_user = SpaceLikeUser::query_builder()
                .space_id_equals(id)
                .user_id_equals(user_id)
                .query()
                .map(SpaceLikeUser::from)
                .fetch_optional(&self.pool)
                .await?;
            if let Some(space_user) = space_user {
                repo.delete(space_user.id).await?;
            }
        } else {
            repo.insert(id, user_id).await?;
        }

        Ok(Space::default())
    }

    async fn share_space(&self, id: i64, auth: Option<Authorization>) -> Result<Space> {
        let user_id = extract_user_id(&self.pool, auth).await?;
        let repo = SpaceShareUser::get_repository(self.pool.clone());

        let space_user = SpaceShareUser::query_builder()
            .space_id_equals(id)
            .user_id_equals(user_id)
            .query()
            .map(SpaceShareUser::from)
            .fetch_optional(&self.pool)
            .await?;

        if space_user.is_none() {
            repo.insert(id, user_id).await?;
        }

        Ok(Space::default())
    }

    async fn posting_space(&self, space_id: i64, auth: Option<Authorization>) -> Result<Space> {
        let user_id = extract_user_id(&self.pool, auth.clone())
            .await
            .unwrap_or_default();

        let space = Space::query_builder(user_id)
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
            publishing_scope,
            quiz,
        }: SpaceUpdateSpaceRequest,
    ) -> Result<Space> {
        let user_id = extract_user_id(&self.pool, auth.clone())
            .await
            .unwrap_or_default();
        // Validate quiz if provided
        if let Some(ref quiz_data) = quiz {
            if !quiz_data.questions.is_empty() {
                if let Err(e) = Self::validate_notice_quiz_request(&quiz_data.questions) {
                    tracing::error!("Quiz validation failed for space {}: {:?}", space_id, e);
                    return Err(e);
                }
            }
        }

        let space = Space::query_builder(user_id)
            .id_equals(space_id)
            .query()
            .map(Space::from)
            .fetch_one(&self.pool.clone())
            .await
            .map_err(|e| {
                tracing::error!("failed to get a space {space_id}: {e}");
                Error::FeedInvalidQuoteSpaceId
            })?;

        // Block quiz editing when space is InProgress
        let quiz = if space.status == SpaceStatus::InProgress {
            tracing::warn!(
                "Blocking quiz update for space {} - status is InProgress",
                space_id
            );
            None
        } else {
            quiz
        };

        // Prevent changing from Public back to Private
        if space.publishing_scope == PublishingScope::Public
            && publishing_scope == PublishingScope::Private
        {
            tracing::error!(
                "Cannot change space {} from Public to Private publishing scope",
                space_id
            );
            return Err(Error::InvalidInputValue);
        }

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

        // notice quiz
        // If quiz is provided, save the quiz with answers and convert to read-only version for space
        let notice_quiz_for_space = if let Some(ref quiz_data) = quiz {
            if !quiz_data.questions.is_empty() {
                // Convert new NoticeQuizRequest format to entities with UUID generation
                let (converted_quiz, answer_data) = convert_notice_quiz_request(quiz_data);

                // Check if answer already exists for this space
                let existing_answer = NoticeQuizAnswer::query_builder()
                    .space_id_equals(space_id)
                    .query()
                    .map(NoticeQuizAnswer::from)
                    .fetch_optional(&self.pool)
                    .await?;

                // Save or update answers in the notice_quiz_answers table
                let save_result = if let Some(existing) = existing_answer {
                    // For update, we need to create a new update request
                    let update_request = NoticeQuizAnswerRepositoryUpdateRequest {
                        answers: Some(answer_data.clone()),
                        ..Default::default()
                    };
                    self.notice_answer_repo
                        .update(existing.id, update_request)
                        .await
                        .map(|_| ())
                } else {
                    // For insert using manual parameter approach
                    self.notice_answer_repo
                        .insert(space_id, answer_data.clone())
                        .await
                        .map(|_| ())
                };

                match save_result {
                    Ok(_) => {
                        tracing::debug!(
                            "Successfully saved notice quiz answers for space {}",
                            space_id
                        );
                    }
                    Err(e) => {
                        tracing::error!(
                            "Failed to save notice quiz answers for space {}: {}",
                            space_id,
                            e
                        );
                        tx.rollback().await?;
                        return Err(Error::DatabaseException(e.to_string()));
                    }
                }

                // Return the converted quiz for the space (read-only version)
                converted_quiz
            } else {
                // Empty quiz provided, clear the quiz
                vec![]
            }
        } else {
            // No quiz provided, don't update quiz field (keep existing)
            space.notice_quiz
        };

        let res = match self
            .repo
            .update_with_tx(
                &mut *tx,
                space_id,
                SpaceRepositoryUpdateRequest {
                    title: title.clone(),
                    html_contents: Some(html_contents.clone()),
                    files: Some(files),
                    started_at,
                    ended_at,
                    publishing_scope: Some(publishing_scope),
                    notice_quiz: if quiz.is_some() {
                        Some(notice_quiz_for_space)
                    } else {
                        None
                    },
                    ..Default::default()
                },
            )
            .await
        {
            Ok(space) => space,
            Err(e) => {
                tracing::error!("Failed to update space {}: {}", space_id, e);
                tx.rollback().await?;
                return Err(Error::BadRequest);
            }
        };

        let feed_id = res.clone().unwrap_or_default().feed_id;

        let _ = self
            .feed_repo
            .update_with_tx(
                &mut *tx,
                feed_id,
                FeedRepositoryUpdateRequest {
                    title: title,
                    html_contents: Some(html_contents),
                    ..Default::default()
                },
            )
            .await?;

        // discussion
        let existing_discs = Discussion::query_builder()
            .space_id_equals(space_id)
            .query()
            .map(Discussion::from)
            .fetch_all(&self.pool.clone())
            .await?;

        let mut existing_map: HashMap<i64, Discussion> = HashMap::new();
        for disc in &existing_discs {
            existing_map.insert(disc.id, disc.clone());
        }

        let mut received_ids = HashSet::new();

        for new_disc in discussions {
            let participants = new_disc.participants;
            if let Some(id) = new_disc.discussion_id {
                received_ids.insert(id);

                let _ = match self
                    .discussion_repo
                    .update_with_tx(
                        &mut *tx,
                        id,
                        DiscussionRepositoryUpdateRequest {
                            name: Some(new_disc.name),
                            description: Some(new_disc.description),
                            started_at: Some(new_disc.started_at),
                            ended_at: Some(new_disc.ended_at),
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

                let ps = DiscussionParticipant::query_builder()
                    .discussion_id_equals(id)
                    .query()
                    .map(DiscussionParticipant::from)
                    .fetch_all(&self.pool.clone())
                    .await?;

                for p in ps {
                    self.discussion_member_repo
                        .delete_with_tx(&mut *tx, p.id)
                        .await?;
                }

                for pid in participants {
                    self.discussion_member_repo
                        .insert_with_tx(&mut *tx, id, pid)
                        .await?;
                }
            } else {
                let inserted = self
                    .discussion_repo
                    .insert_with_tx(
                        &mut *tx,
                        space_id,
                        user_id,
                        new_disc.started_at,
                        new_disc.ended_at,
                        new_disc.name,
                        new_disc.description,
                        None,
                        "".to_string(),
                        None,
                        None,
                    )
                    .await?
                    .unwrap_or_default();

                let new_id = inserted.id;

                for pid in participants {
                    self.discussion_member_repo
                        .insert_with_tx(&mut *tx, new_id, pid)
                        .await?;
                }
            }
        }

        for old_disc in existing_discs {
            if !received_ids.contains(&old_disc.id) {
                self.discussion_repo
                    .delete_with_tx(&mut *tx, old_disc.id)
                    .await?;
            }
        }

        //

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
            booster_type,
            started_at,
            ended_at,
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

        let mut tx = self.pool.begin().await?;

        let author_id = feed.author[0].id;

        let res = self
            .repo
            .insert_with_tx(
                &mut *tx,
                feed.title,
                feed.html_contents,
                space_type,
                author_id,
                feed.industry_id,
                started_at,
                ended_at,
                feed_id,
                SpaceStatus::Draft,
                feed.files,
                num_of_redeem_codes,
                Vec::new(),
                booster_type,
                PublishingScope::Private,
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

    /// Helper function to validate quiz requests (with correct answers)
    fn validate_notice_quiz_request(quiz: &[NoticeQuestionRequest]) -> Result<()> {
        for (question_index, question) in quiz.iter().enumerate() {
            // Check if question title is not empty
            let question_title = question.title.trim();
            if question_title.is_empty() {
                tracing::error!("Question {} title cannot be empty", question_index + 1);
                return Err(Error::InvalidInputValue);
            }

            // Check if each question has more than 2 options
            if question.options.len() < 2 {
                tracing::error!(
                    "Question {} has only {} options, minimum 2 required",
                    question_index + 1,
                    question.options.len()
                );
                return Err(Error::InvalidInputValue);
            }

            // Check if each question has no more than 4 options
            if question.options.len() > 4 {
                tracing::error!(
                    "Question {} has {} options, maximum 4 allowed",
                    question_index + 1,
                    question.options.len()
                );
                return Err(Error::InvalidInputValue);
            }

            // Check if all option contents are unique within the question
            let mut option_contents = HashSet::new();
            let mut selected_count = 0;
            for (option_index, option) in question.options.iter().enumerate() {
                let content = option.content.trim();

                // Check for empty content
                if content.is_empty() {
                    tracing::error!(
                        "Question {} option {} cannot be empty",
                        question_index + 1,
                        option_index + 1
                    );
                    return Err(Error::InvalidInputValue);
                }

                if !option_contents.insert(content.to_lowercase()) {
                    tracing::error!(
                        "Question {} has duplicate option content: '{}'",
                        question_index + 1,
                        content
                    );
                    return Err(Error::InvalidInputValue);
                }

                // Count correct options (is_correct)
                if option.is_correct {
                    selected_count += 1;
                }
            }

            // Check if exactly one option is selected
            if selected_count == 0 {
                tracing::error!(
                    "Question {} must have exactly one selected option, but has none",
                    question_index + 1
                );
                return Err(Error::InvalidInputValue);
            }

            if selected_count > 1 {
                tracing::error!(
                    "Question {} must have exactly one selected option, but has {}",
                    question_index + 1,
                    selected_count
                );
                return Err(Error::InvalidInputValue);
            }
        }

        Ok(())
    }
}

impl SpaceController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Space::get_repository(pool.clone());
        let feed_repo = Feed::get_repository(pool.clone());
        let space_member_repo = SpaceMember::get_repository(pool.clone());
        let space_draft_repo = SpaceDraft::get_repository(pool.clone());
        let discussion_repo = Discussion::get_repository(pool.clone());
        let discussion_member_repo = DiscussionMember::get_repository(pool.clone());
        let elearning_repo = Elearning::get_repository(pool.clone());
        let survey_repo = Survey::get_repository(pool.clone());
        let notice_answer_repo = NoticeQuizAnswer::get_repository(pool.clone());

        Self {
            repo,
            feed_repo,
            pool,
            discussion_repo,
            discussion_member_repo,
            elearning_repo,
            survey_repo,
            space_member_repo,
            space_draft_repo,
            notice_answer_repo,
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
                "/:space-id/responses",
                responses::SurveyResponseController::new(self.pool.clone())
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
            )
            .nest(
                "/:space-id/sprint-leagues",
                sprint_leagues::SprintLeagueController::new(self.pool.clone()).route(),
            )
            .nest(
                "/:space-id/notice-quiz-attempts",
                notice_quiz_attempts::SpaceNoticeQuizAttemptController::new(self.pool.clone())
                    .route(),
            )
            .nest(
                "/:space-id/notice-quiz-answers",
                notice_quiz_answers::SpaceNoticeQuizAnswersController::new(self.pool.clone())
                    .route(),
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
            SpaceByIdAction::Like(req) => ctrl.like_space(id, auth, req.value).await?,
            SpaceByIdAction::Share(_) => ctrl.share_space(id, auth).await?,
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
