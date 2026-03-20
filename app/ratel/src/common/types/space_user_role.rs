use crate::common::*;

#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    Default,
    // DynamoEnum,
    // JsonSchema,
    // OperationIo,
    Translate,
    PartialEq,
    Eq,
)]
pub enum SpaceUserRole {
    #[default]
    #[translate(en = "Viewer", ko = "뷰어")]
    Viewer,
    #[translate(en = "Participant", ko = "참가자")]
    Participant,
    #[translate(en = "Candidate", ko = "참가후보")]
    Candidate,
    #[translate(en = "Admin", ko = "관리자")]
    Creator,
}

impl SpaceUserRole {
    pub fn is_admin(&self) -> bool {
        matches!(self, SpaceUserRole::Creator)
    }

    pub fn can_edit(&self) -> bool {
        matches!(self, SpaceUserRole::Creator)
    }

    pub fn can_act(&self) -> bool {
        matches!(self, SpaceUserRole::Participant | SpaceUserRole::Creator)
    }
}

#[cfg(feature = "server")]
fn prerequisite_check_error(context: &str, err: impl std::fmt::Display) -> Error {
    error!("{context}: {err}");
    Error::InternalServerError(format!("{err}"))
}

#[cfg(feature = "server")]
async fn has_completed_prerequisite_actions(
    cli: &aws_sdk_dynamodb::Client,
    space: &crate::common::models::space::SpaceCommon,
    user: &crate::common::models::auth::User,
) -> Result<bool> {
    use crate::features::spaces::pages::actions::models::SpaceAction;

    let (actions, _) = SpaceAction::find_by_space(cli, &space.pk, SpaceAction::opt())
        .await
        .map_err(|err| prerequisite_check_error("Failed to load prerequisite actions", err))?;

    let prerequisite_actions: Vec<_> = actions
        .into_iter()
        .filter(|action| action.prerequisite)
        .collect();

    if prerequisite_actions.is_empty() {
        return Ok(true);
    }

    for action in prerequisite_actions {
        if !has_completed_prerequisite_action(cli, space, &action, &user.pk).await? {
            return Ok(false);
        }
    }

    Ok(true)
}

#[cfg(feature = "server")]
async fn has_completed_prerequisite_action(
    cli: &aws_sdk_dynamodb::Client,
    space: &crate::common::models::space::SpaceCommon,
    action: &crate::features::spaces::pages::actions::models::SpaceAction,
    user_pk: &Partition,
) -> Result<bool> {
    use crate::features::spaces::pages::actions::types::SpaceActionType;

    match action.space_action_type {
        SpaceActionType::Poll => {
            has_completed_poll_action(cli, &space.pk, &action.pk.1, user_pk).await
        }
        SpaceActionType::Quiz => has_completed_quiz_action(cli, &action.pk.1, user_pk).await,
        SpaceActionType::TopicDiscussion => {
            has_completed_discussion_action(cli, &action.pk.1, user_pk).await
        }
        SpaceActionType::Follow => has_completed_follow_action(cli, space, user_pk).await,
    }
}

#[cfg(feature = "server")]
async fn has_completed_poll_action(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    action_id: &str,
    user_pk: &Partition,
) -> Result<bool> {
    use crate::features::spaces::pages::actions::actions::poll::SpacePollUserAnswer;

    let poll_id: SpacePollEntityType = action_id.to_string().into();
    let poll_sk: EntityType = poll_id.into();

    SpacePollUserAnswer::find_one(cli, space_pk, &poll_sk, user_pk)
        .await
        .map(|answer| answer.is_some())
        .map_err(|err| prerequisite_check_error("Failed to verify poll prerequisite", err))
}

#[cfg(feature = "server")]
async fn has_completed_quiz_action(
    cli: &aws_sdk_dynamodb::Client,
    action_id: &str,
    user_pk: &Partition,
) -> Result<bool> {
    use crate::features::spaces::pages::actions::actions::quiz::SpaceQuizAttempt;

    let quiz_id: SpaceQuizEntityType = action_id.to_string().into();

    SpaceQuizAttempt::find_latest_by_quiz_user(cli, &quiz_id, user_pk)
        .await
        .map(|attempt| attempt.is_some())
        .map_err(|err| prerequisite_check_error("Failed to verify quiz prerequisite", err))
}

#[cfg(feature = "server")]
async fn has_completed_discussion_action(
    cli: &aws_sdk_dynamodb::Client,
    action_id: &str,
    user_pk: &Partition,
) -> Result<bool> {
    use crate::features::spaces::pages::actions::actions::discussion::SpacePostComment;

    let discussion_pk = Partition::SpacePost(action_id.to_string());
    let mut bookmark: Option<String> = None;

    loop {
        let opt = if let Some(next_bookmark) = bookmark {
            SpacePostComment::opt().bookmark(next_bookmark).limit(100)
        } else {
            SpacePostComment::opt().limit(100)
        };

        let (comments, next_bookmark) =
            SpacePostComment::find_by_post_order_by_likes(cli, discussion_pk.clone(), opt)
                .await
                .map_err(|err| {
                    prerequisite_check_error("Failed to verify discussion prerequisite", err)
                })?;

        if comments.iter().any(|comment| &comment.author_pk == user_pk) {
            return Ok(true);
        }

        if next_bookmark.is_none() {
            return Ok(false);
        }

        bookmark = next_bookmark;
    }
}

#[cfg(feature = "server")]
async fn has_completed_follow_action(
    cli: &aws_sdk_dynamodb::Client,
    space: &crate::common::models::space::SpaceCommon,
    user_pk: &Partition,
) -> Result<bool> {
    use std::collections::HashSet;

    use crate::common::models::auth::UserFollow;
    use crate::features::spaces::pages::actions::actions::follow::{
        SpaceFollowUser, SpaceFollowUserQueryOption,
    };

    let mut target_user_pks = Vec::new();
    let mut bookmark: Option<String> = None;

    loop {
        let opt = if let Some(next_bookmark) = bookmark.clone() {
            SpaceFollowUserQueryOption::builder()
                .sk(EntityType::SpaceSubscriptionUser(String::default()).to_string())
                .bookmark(next_bookmark)
                .limit(100)
        } else {
            SpaceFollowUserQueryOption::builder()
                .sk(EntityType::SpaceSubscriptionUser(String::default()).to_string())
                .limit(100)
        };

        let (users, next_bookmark) = SpaceFollowUser::query(cli, space.pk.clone(), opt)
            .await
            .map_err(|err| {
                prerequisite_check_error("Failed to load follow prerequisite targets", err)
            })?;

        target_user_pks.extend(
            users
                .into_iter()
                .filter(|user| user.user_pk != Partition::None)
                .map(|user| user.user_pk),
        );

        if next_bookmark.is_none() {
            break;
        }

        bookmark = next_bookmark;
    }

    if !target_user_pks
        .iter()
        .any(|target| target == &space.user_pk)
    {
        target_user_pks.push(space.user_pk.clone());
    }

    let mut deduped_targets = Vec::new();
    let mut seen = HashSet::new();
    for target_user_pk in target_user_pks {
        let target_key = target_user_pk.to_string();
        if seen.insert(target_key) {
            deduped_targets.push(target_user_pk);
        }
    }

    let keys: Vec<_> = deduped_targets
        .iter()
        .map(|target_user_pk| UserFollow::follower_keys(target_user_pk, user_pk))
        .collect();

    if keys.is_empty() {
        return Ok(true);
    }

    let follows = UserFollow::batch_get(cli, keys)
        .await
        .map_err(|err| prerequisite_check_error("Failed to verify follow prerequisite", err))?;
    let followed_targets: HashSet<_> = follows
        .into_iter()
        .map(|follow| follow.target_user_pk.to_string())
        .collect();

    Ok(deduped_targets
        .iter()
        .all(|target_user_pk| followed_targets.contains(&target_user_pk.to_string())))
}

#[cfg(feature = "server")]
impl<S> FromRequestParts<S> for SpaceUserRole
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        use crate::common::models::auth::User;
        use crate::common::models::space::{SpaceCommon, SpaceParticipant};
        use crate::common::types::{CompositePartition, EntityType};
        tracing::debug!("extracting space from request parts. Path: {:?}", parts.uri);

        if let Some(space_role) = parts.extensions.get::<SpaceUserRole>() {
            return Ok(space_role.clone());
        }

        let space = SpaceCommon::from_request_parts(parts, state).await?;

        let user = User::from_request_parts(parts, state).await.ok();

        let public_space = space.is_public();

        if user.is_none() {
            if public_space {
                parts.extensions.insert(SpaceUserRole::Viewer);
                return Ok(SpaceUserRole::Viewer);
            } else {
                return Err(Error::UnauthorizedAccess);
            }
        }

        let user = user.unwrap();

        // Individual creator check
        if user.pk == space.user_pk {
            parts.extensions.insert(SpaceUserRole::Creator);
            return Ok(SpaceUserRole::Creator);
        }

        let conf = config::ServerConfig::default();
        let cli = conf.dynamodb();

        // Team admin check: if the space is owned by a team, check if the user
        // has TeamAdmin permission within that team.
        if matches!(&space.user_pk, Partition::Team(_)) {
            use crate::features::posts::models::Team;
            use crate::features::posts::types::TeamGroupPermission;

            if Team::has_permission(
                cli,
                &space.user_pk,
                &user.pk,
                TeamGroupPermission::TeamAdmin,
            )
            .await
            .unwrap_or(false)
            {
                parts.extensions.insert(SpaceUserRole::Creator);
                return Ok(SpaceUserRole::Creator);
            }
        }

        // Check participant
        let participant = SpaceParticipant::get(
            cli,
            CompositePartition(space.pk.clone(), user.pk.clone()),
            Some(EntityType::SpaceParticipant),
        )
        .await
        .ok()
        .flatten();

        if participant.is_some() {
            let role = if matches!(space.status, Some(crate::common::SpaceStatus::InProgress)) {
                SpaceUserRole::Candidate
            } else if matches!(
                space.status,
                Some(crate::common::SpaceStatus::Started | crate::common::SpaceStatus::Finished)
            ) {
                if has_completed_prerequisite_actions(cli, &space, &user).await? {
                    SpaceUserRole::Participant
                } else {
                    SpaceUserRole::Candidate
                }
            } else {
                SpaceUserRole::Candidate
            };
            parts.extensions.insert(role);
            return Ok(role);
        }

        // For public spaces, unauthenticated users are Viewers (handled above),
        // but authenticated non-participants are also Viewers.
        if public_space {
            parts.extensions.insert(SpaceUserRole::Viewer);
            return Ok(SpaceUserRole::Viewer);
        }

        Err(Error::UnauthorizedAccess)
    }
}
