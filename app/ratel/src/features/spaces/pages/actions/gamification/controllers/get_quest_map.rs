use super::*;
#[cfg(feature = "server")]
use crate::common::models::auth::OptionalUser;
#[cfg(feature = "server")]
use crate::common::models::space::SpaceCommon;
#[cfg(feature = "server")]
use crate::features::spaces::pages::actions::actions::quiz::{SpaceQuiz, SpaceQuizAttempt};
#[cfg(feature = "server")]
use crate::features::spaces::pages::actions::actions::poll::SpacePollUserAnswer;

/// Returns the Quest Map for the requesting user in a given space.
///
/// One response includes every chapter (sorted by `order`) together with
/// their quest nodes and the user's per-node status so the UI can render
/// the interactive DAG without additional round-trips.
#[get(
    "/api/spaces/{space_id}/quest-map",
    role: SpaceUserRole,
    user: OptionalUser,
    space: SpaceCommon
)]
pub async fn get_quest_map(space_id: SpacePartition) -> Result<QuestMapResponse> {
    let config = crate::common::CommonConfig::default();
    let cli = config.dynamodb();
    let space_pk: Partition = space_id.clone().into();

    // ── 1. Load all chapters ──────────────────────────────────────────────────
    let chapter_sk_prefix = EntityType::SpaceChapter(String::new()).to_string();
    let (raw_chapters, _) = SpaceChapter::query(
        cli,
        &space_pk,
        SpaceChapter::opt()
            .sk(chapter_sk_prefix)
            .limit(1_000_000)
            .scan_index_forward(true),
    )
    .await
    .map_err(|e| {
        crate::error!("get_quest_map: failed to load chapters: {e}");
        Error::InternalServerError("failed to load chapters".into())
    })?;

    // Sort chapters by order ascending (the query should already do this but
    // we sort defensively since `scan_index_forward` applies to the sort key).
    let mut chapters = raw_chapters;
    chapters.sort_by_key(|c| c.order);

    // ── 2. Load all space actions ──────────────────────────────────────────────
    let (space_actions, _) = SpaceAction::find_by_space(cli, &space_pk, SpaceAction::opt())
        .await
        .map_err(|e| {
            crate::error!("get_quest_map: failed to load actions: {e}");
            Error::InternalServerError("failed to load actions".into())
        })?;

    // ── 3. Determine current user ──────────────────────────────────────────────
    let current_user = user.0;

    // ── 4. Load user gamification state (combo & streak) ──────────────────────
    let (combo_multiplier, streak_days) = if let Some(ref u) = current_user {
        let combo_mult = match UserSpaceCombo::get(
            cli,
            &space_pk,
            Some(EntityType::UserSpaceCombo(u.pk.to_string())),
        )
        .await
        .ok()
        .flatten()
        {
            Some(combo) => combo.combo_multiplier,
            None => 1.0_f32,
        };

        let streak = match UserStreak::get(cli, &u.pk, Some(EntityType::UserStreak))
            .await
            .ok()
            .flatten()
        {
            Some(s) => s.current_streak,
            None => 0_u32,
        };

        (combo_mult, streak)
    } else {
        (1.0_f32, 0_u32)
    };

    let streak_mult = UserStreak::streak_multiplier(streak_days);

    // ── 5. Participant count: quota − remains ──────────────────────────────────
    let participant_count = (space.quota - space.remains).max(1) as i64;

    // ── 6. Build a set of cleared action ids for the current user ─────────────
    // We compute this once, then reuse for every node.
    let cleared_ids: std::collections::HashSet<String> = if let Some(ref u) = current_user {
        collect_cleared_action_ids(cli, &space_pk, &u.pk, &space_actions).await?
    } else {
        std::collections::HashSet::new()
    };

    // ── 7. Build QuestNodeView for each action ────────────────────────────────
    // Index cleared ids for fast lookup.
    let mut chapter_views: Vec<ChapterView> = Vec::with_capacity(chapters.len());

    // Track which chapter order indices are fully cleared (for prior-chapter gate).
    let mut chapter_completion: Vec<bool> = Vec::with_capacity(chapters.len());

    for (chapter_idx, chapter) in chapters.iter().enumerate() {
        // All chapters with lower order must be cleared for this chapter to be reachable.
        let prior_complete = chapter_completion[..chapter_idx].iter().all(|&c| c);

        // Extract chapter id string from EntityType::SpaceChapter(id).
        let chapter_id_str: String = match &chapter.sk {
            EntityType::SpaceChapter(id) => id.clone(),
            _ => continue,
        };

        // Actions belonging to this chapter.
        let chapter_actions: Vec<&SpaceAction> = space_actions
            .iter()
            .filter(|a| {
                a.chapter_id
                    .as_ref()
                    .map(|cid| cid.0 == chapter_id_str)
                    .unwrap_or(false)
            })
            .collect();

        let mut nodes: Vec<QuestNodeView> = Vec::with_capacity(chapter_actions.len());

        for action in &chapter_actions {
            let action_id = action.pk.1.clone();
            let is_cleared = cleared_ids.contains(&action_id);

            // DAG parent gate: all depends_on entries must be cleared.
            let deps_met = action
                .depends_on
                .iter()
                .all(|dep| cleared_ids.contains(dep));

            // Determine node status.
            let status = if is_cleared {
                QuestNodeStatus::Cleared
            } else if !deps_met || !prior_complete {
                QuestNodeStatus::Locked
            } else if !role_meets_requirement(role, chapter.actor_role) {
                QuestNodeStatus::RoleGated
            } else {
                QuestNodeStatus::Active
            };

            // Projected XP = base × participants × combo × streak.
            let base = action.total_points as i64;
            let projected_xp =
                (base as f64 * participant_count as f64 * combo_multiplier as f64 * streak_mult as f64)
                    .round() as i64;

            // Quiz result (only for cleared quiz nodes).
            let quiz_result = if is_cleared
                && action.space_action_type == SpaceActionType::Quiz
            {
                load_quiz_result(cli, &space_pk, &action_id, current_user.as_ref())
                    .await
                    .ok()
                    .flatten()
            } else {
                None
            };

            nodes.push(QuestNodeView {
                id: action_id,
                action_type: action.space_action_type.clone(),
                title: action.title.clone(),
                base_points: base,
                projected_xp,
                status,
                depends_on: action.depends_on.clone(),
                chapter_id: chapter_id_str.clone(),
                started_at: Some(action.started_at),
                ended_at: Some(action.ended_at),
                quiz_result,
            });
        }

        let is_complete = !nodes.is_empty() && nodes.iter().all(|n| n.status == QuestNodeStatus::Cleared);
        let total_xp_earned = nodes
            .iter()
            .filter(|n| n.status == QuestNodeStatus::Cleared)
            .map(|n| n.projected_xp)
            .sum();

        chapter_completion.push(is_complete);

        chapter_views.push(ChapterView {
            id: chapter_id_str,
            order: chapter.order,
            name: chapter.name.clone(),
            description: chapter.description.clone(),
            actor_role: chapter.actor_role,
            completion_benefit: chapter.completion_benefit.clone(),
            nodes,
            is_complete,
            total_xp_earned,
        });
    }

    // ── 8. Compute current_chapter_id ─────────────────────────────────────────
    let current_chapter_id = chapter_views
        .iter()
        .find(|c| !c.is_complete)
        .map(|c| c.id.clone());

    let current_user_state = UserQuestState {
        role,
        current_chapter_id,
        current_combo_multiplier: combo_multiplier,
        current_streak_days: streak_days,
    };

    Ok(QuestMapResponse {
        chapters: chapter_views,
        current_user_state,
    })
}

/// Check whether `role` meets the `required` minimum role for a chapter.
fn role_meets_requirement(role: SpaceUserRole, required: SpaceUserRole) -> bool {
    match (role, required) {
        (SpaceUserRole::Creator, _) => true,
        (
            SpaceUserRole::Participant,
            SpaceUserRole::Participant | SpaceUserRole::Candidate | SpaceUserRole::Viewer,
        ) => true,
        (SpaceUserRole::Candidate, SpaceUserRole::Candidate | SpaceUserRole::Viewer) => true,
        (SpaceUserRole::Viewer, SpaceUserRole::Viewer) => true,
        _ => false,
    }
}

/// Collect the set of action ids that the current user has completed.
#[cfg(feature = "server")]
async fn collect_cleared_action_ids(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    user_pk: &Partition,
    space_actions: &[SpaceAction],
) -> Result<std::collections::HashSet<String>> {
    use crate::features::spaces::pages::actions::actions::discussion::SpacePostComment;
    use crate::features::spaces::pages::actions::actions::follow::SpaceFollowUser;
    use crate::common::models::space::SpaceCommon as SpaceCommonModel;
    use crate::common::models::auth::UserFollow;

    let mut cleared = std::collections::HashSet::new();

    for action in space_actions {
        let action_id = action.pk.1.clone();

        let is_cleared = match action.space_action_type {
            SpaceActionType::Quiz => {
                let quiz_id = SpaceQuizEntityType::from(action_id.clone());
                if let Some(quiz) =
                    SpaceQuiz::get(cli, space_pk, Some(EntityType::SpaceQuiz(action_id.clone())))
                        .await
                        .ok()
                        .flatten()
                {
                    SpaceQuizAttempt::find_latest_by_quiz_user(cli, &quiz_id, user_pk)
                        .await
                        .ok()
                        .flatten()
                        .map(|attempt| attempt.score >= quiz.pass_score)
                        .unwrap_or(false)
                } else {
                    false
                }
            }
            SpaceActionType::Poll => {
                let poll_sk = EntityType::SpacePoll(action_id.clone());
                SpacePollUserAnswer::find_one(cli, space_pk, &poll_sk, user_pk)
                    .await
                    .ok()
                    .flatten()
                    .is_some()
            }
            SpaceActionType::TopicDiscussion => {
                has_discussion_completed(cli, &action_id, user_pk)
                    .await
                    .unwrap_or(false)
            }
            SpaceActionType::Follow => {
                has_follow_completed(cli, space_pk, user_pk).await.unwrap_or(false)
            }
        };

        if is_cleared {
            cleared.insert(action_id);
        }
    }

    Ok(cleared)
}

#[cfg(feature = "server")]
async fn has_discussion_completed(
    cli: &aws_sdk_dynamodb::Client,
    action_id: &str,
    user_pk: &Partition,
) -> Result<bool> {
    use crate::features::spaces::pages::actions::actions::discussion::SpacePostComment;

    let discussion_pk = Partition::SpacePost(action_id.to_string());
    let prefixes = [
        EntityType::SpacePostComment(String::default()).to_string(),
        EntityType::SpacePostCommentReply(String::default(), String::default()).to_string(),
    ];

    for sk_prefix in prefixes {
        let mut bookmark: Option<String> = None;
        loop {
            let opt = if let Some(next_bookmark) = bookmark.clone() {
                SpacePostComment::opt()
                    .sk(sk_prefix.clone())
                    .bookmark(next_bookmark)
                    .limit(100)
            } else {
                SpacePostComment::opt().sk(sk_prefix.clone()).limit(100)
            };

            let (comments, next_bookmark) =
                SpacePostComment::find_by_user_pk(cli, user_pk, opt)
                    .await
                    .map_err(|e| {
                        Error::InternalServerError(format!(
                            "quest_map: failed to check discussion: {e}"
                        ))
                    })?;

            if comments.iter().any(|c| c.pk == discussion_pk) {
                return Ok(true);
            }

            if next_bookmark.is_none() {
                break;
            }
            bookmark = next_bookmark;
        }
    }

    Ok(false)
}

#[cfg(feature = "server")]
async fn has_follow_completed(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    user_pk: &Partition,
) -> Result<bool> {
    use crate::common::models::auth::UserFollow;
    use crate::common::models::space::SpaceCommon as SpaceCommonModel;
    use crate::features::spaces::pages::actions::actions::follow::SpaceFollowUser;
    use std::collections::HashSet;

    let space = SpaceCommonModel::get(cli, space_pk, Some(EntityType::SpaceCommon))
        .await?
        .ok_or_else(|| Error::NotFound("space not found".to_string()))?;

    let mut target_user_pks = Vec::new();
    let mut bookmark: Option<String> = None;

    loop {
        let opt = if let Some(next_bookmark) = bookmark.clone() {
            SpaceFollowUser::opt()
                .sk(EntityType::SpaceSubscriptionUser(String::default()).to_string())
                .bookmark(next_bookmark)
                .limit(100)
        } else {
            SpaceFollowUser::opt()
                .sk(EntityType::SpaceSubscriptionUser(String::default()).to_string())
                .limit(100)
        };

        let (users, next_bookmark) = SpaceFollowUser::query(cli, space_pk.clone(), opt)
            .await
            .map_err(|e| {
                Error::InternalServerError(format!("quest_map: failed to load follow targets: {e}"))
            })?;

        target_user_pks.extend(
            users
                .into_iter()
                .filter(|u| u.user_pk != Partition::None)
                .map(|u| u.user_pk),
        );

        if next_bookmark.is_none() {
            break;
        }
        bookmark = next_bookmark;
    }

    if !target_user_pks.iter().any(|t| t == &space.user_pk) {
        target_user_pks.push(space.user_pk);
    }

    let mut seen = HashSet::new();
    let deduped: Vec<Partition> = target_user_pks
        .into_iter()
        .filter(|t| seen.insert(t.to_string()))
        .collect();

    let keys: Vec<_> = deduped
        .iter()
        .map(|target| UserFollow::follower_keys(target, user_pk))
        .collect();

    if keys.is_empty() {
        return Ok(true);
    }

    let follows = UserFollow::batch_get(cli, keys).await.map_err(|e| {
        Error::InternalServerError(format!("quest_map: failed to verify follows: {e}"))
    })?;

    Ok(follows.len() == deduped.len())
}

/// Load the quiz result for a cleared quiz node.
#[cfg(feature = "server")]
async fn load_quiz_result(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    action_id: &str,
    user: Option<&crate::common::models::auth::User>,
) -> Result<Option<QuestQuizResult>> {
    let Some(user) = user else {
        return Ok(None);
    };

    let quiz_sk = EntityType::SpaceQuiz(action_id.to_string());
    let quiz = match SpaceQuiz::get(cli, space_pk, Some(quiz_sk)).await? {
        Some(q) => q,
        None => return Ok(None),
    };

    let quiz_id = SpaceQuizEntityType::from(action_id.to_string());
    let attempt = SpaceQuizAttempt::find_latest_by_quiz_user(cli, &quiz_id, &user.pk).await?;

    Ok(attempt.map(|a| QuestQuizResult {
        score: a.score,
        total: quiz.questions.len() as i64,
        passed: a.score >= quiz.pass_score,
    }))
}
