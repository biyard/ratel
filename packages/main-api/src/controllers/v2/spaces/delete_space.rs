use bdk::prelude::*;
use by_axum::axum::{extract::State, Extension, Json};
use dto::{
    by_axum::axum::extract::Path, Authorization, Discussion, DiscussionMember, Elearning,
    Error, Feed, GroupPermission, NoticeQuizAnswer, NoticeQuizAttempt, Result, Space,
    SpaceComment, SpaceDeleteConfirmation, SpaceDraft, SpaceGroup, SpaceLikeUser, SpaceMember,
    SpaceShareUser, sqlx::{Pool, Postgres},
};

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default, aide::OperationIo, JsonSchema,
)]
pub struct DeleteSpacePathParams {
    #[schemars(description = "Space ID")]
    pub space_id: i64,
}

/// Delete a space (and all related resources) after confirmation
///
/// Permissions: ManageSpace on RatelResource::Space { space_id }
pub async fn delete_space_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<Pool<Postgres>>,
    Path(DeleteSpacePathParams { space_id }): Path<DeleteSpacePathParams>,
    Json(req): Json<SpaceDeleteConfirmation>,
) -> Result<Json<()>> {
    // Fetch the space first
    let space = Space::query_builder(0)
        .id_equals(space_id)
        .query()
        .map(Space::from)
        .fetch_one(&pool)
        .await?;

    // Validate confirmation and name
    if !req.confirmation {
        return Err(Error::BadRequest);
    }
    if space.title.unwrap_or_default() != req.space_name {
        return Err(Error::BadRequest);
    }

    // Permission check: ManageSpace for the space
    crate::security::check_perm(
        &pool,
        auth,
        dto::RatelResource::Space { space_id },
        GroupPermission::ManageSpace,
    )
    .await?;

    // Start transaction
    let mut tx = pool.begin().await?;

    // Delete discussions and their participants
    let discussions = Discussion::query_builder()
        .space_id_equals(space_id)
        .query()
        .map(Discussion::from)
        .fetch_all(&pool)
        .await?;
    let discussion_repo = Discussion::get_repository(pool.clone());
    let discussion_member_repo = dto::DiscussionMember::get_repository(pool.clone());
    for discussion in discussions {
        let participants = DiscussionMember::query_builder()
            .discussion_id_equals(discussion.id)
            .query()
            .map(DiscussionMember::from)
            .fetch_all(&pool)
            .await?;
        for participant in participants {
            discussion_member_repo.delete_with_tx(&mut *tx, participant.id).await?;
        }
        discussion_repo.delete_with_tx(&mut *tx, discussion.id).await?;
    }

    // Delete surveys and responses
    let surveys = dto::Survey::query_builder()
        .space_id_equals(space_id)
        .query()
        .map(dto::Survey::from)
        .fetch_all(&pool)
        .await?;
    let survey_repo = dto::Survey::get_repository(pool.clone());
    let response_repo = dto::SurveyResponse::get_repository(pool.clone());
    for survey in &surveys {
        let responses = dto::SurveyResponse::query_builder()
            .survey_id_equals(survey.id)
            .query()
            .map(dto::SurveyResponse::from)
            .fetch_all(&pool)
            .await?;
        for resp in responses {
            response_repo.delete_with_tx(&mut *tx, resp.id).await?;
        }
    }
    for survey in surveys {
        survey_repo.delete_with_tx(&mut *tx, survey.id).await?;
    }

    // Delete elearnings
    let elearnings = Elearning::query_builder()
        .space_id_equals(space_id)
        .query()
        .map(Elearning::from)
        .fetch_all(&pool)
        .await?;
    let elearning_repo = Elearning::get_repository(pool.clone());
    for elearning in elearnings {
        elearning_repo.delete_with_tx(&mut *tx, elearning.id).await?;
    }

    // Delete space members
    let members = SpaceMember::query_builder()
        .space_id_equals(space_id)
        .query()
        .map(SpaceMember::from)
        .fetch_all(&pool)
        .await?;
    let space_member_repo = SpaceMember::get_repository(pool.clone());
    for member in members {
        space_member_repo.delete_with_tx(&mut *tx, member.id).await?;
    }

    // Delete drafts
    let drafts = SpaceDraft::query_builder()
        .space_id_equals(space_id)
        .query()
        .map(SpaceDraft::from)
        .fetch_all(&pool)
        .await?;
    let space_draft_repo = SpaceDraft::get_repository(pool.clone());
    for draft in drafts {
        space_draft_repo.delete_with_tx(&mut *tx, draft.id).await?;
    }

    // Delete likes and shares
    let like_repo = SpaceLikeUser::get_repository(pool.clone());
    let likes = SpaceLikeUser::query_builder()
        .space_id_equals(space_id)
        .query()
        .map(SpaceLikeUser::from)
        .fetch_all(&pool)
        .await?;
    for like in likes {
        like_repo.delete_with_tx(&mut *tx, like.id).await?;
    }

    let share_repo = SpaceShareUser::get_repository(pool.clone());
    let shares = SpaceShareUser::query_builder()
        .space_id_equals(space_id)
        .query()
        .map(SpaceShareUser::from)
        .fetch_all(&pool)
        .await?;
    for share in shares {
        share_repo.delete_with_tx(&mut *tx, share.id).await?;
    }

    // Delete groups
    let groups = SpaceGroup::query_builder()
        .space_id_equals(space_id)
        .query()
        .map(SpaceGroup::from)
        .fetch_all(&pool)
        .await?;
    let space_group_repo = SpaceGroup::get_repository(pool.clone());
    for group in groups {
        space_group_repo.delete_with_tx(&mut *tx, group.id).await?;
    }

    // Delete comments (feed comments parented by space.feed_id)
    let feed = Feed::query_builder(0)
        .id_equals(space.feed_id)
        .query()
        .map(Feed::from)
        .fetch_one(&pool)
        .await?;
    let comment_repo = SpaceComment::get_repository(pool.clone());
    let comments = SpaceComment::query_builder()
        .parent_id_equals(feed.id)
        .query()
        .map(SpaceComment::from)
        .fetch_all(&pool)
        .await?;
    for comment in comments {
        comment_repo.delete_with_tx(&mut *tx, comment.id).await?;
    }

    // Delete notice quiz answers and attempts
    let quiz_answer_repo = NoticeQuizAnswer::get_repository(pool.clone());
    let quiz_answers = NoticeQuizAnswer::query_builder()
        .space_id_equals(space_id)
        .query()
        .map(NoticeQuizAnswer::from)
        .fetch_all(&pool)
        .await?;
    for ans in quiz_answers {
        quiz_answer_repo.delete_with_tx(&mut *tx, ans.id).await?;
    }

    let quiz_attempt_repo = NoticeQuizAttempt::get_repository(pool.clone());
    let quiz_attempts = NoticeQuizAttempt::query_builder()
        .space_id_equals(space_id)
        .query()
        .map(NoticeQuizAttempt::from)
        .fetch_all(&pool)
        .await?;
    for att in quiz_attempts {
        quiz_attempt_repo.delete_with_tx(&mut *tx, att.id).await?;
    }

    // Finally, delete the space itself
    let space_repo = Space::get_repository(pool.clone());
    space_repo.delete_with_tx(&mut *tx, space_id).await?;

    tx.commit().await?;

    Ok(Json(()))
}
