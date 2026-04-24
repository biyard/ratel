use std::collections::HashSet;

use crate::common::*;
use crate::features::auth::UserTeam;
use crate::features::posts::models::{Team, TeamOwner};
use crate::features::sub_team::models::{
    SubTeamApplication, SubTeamApplicationStatus, SubTeamLink,
};
use crate::features::sub_team::types::SubTeamError;

const PAGE_SIZE: i32 = 100;
const MAX_PAGES: usize = 10;

/// Resolve admin+owner user pks for a team (owner record + UserTeam rows whose
/// role is Admin/Owner). Used by the application-lifecycle notifications to
/// address parent-side decision messages to the right set of recipients.
pub async fn resolve_team_admins(
    cli: &aws_sdk_dynamodb::Client,
    team_pk: &Partition,
) -> Result<Vec<Partition>> {
    use crate::features::social::pages::member::dto::TeamRole;

    let mut user_pks: HashSet<String> = HashSet::new();

    if let Ok(Some(owner)) = TeamOwner::get(cli, team_pk, Some(&EntityType::TeamOwner)).await {
        user_pks.insert(owner.user_pk.to_string());
    }

    let user_team_sk = EntityType::UserTeam(team_pk.to_string());
    let mut bookmark: Option<String> = None;
    for _ in 0..MAX_PAGES {
        let mut opt = crate::features::auth::UserTeamQueryOption::builder().limit(PAGE_SIZE);
        if let Some(bm) = bookmark.as_ref() {
            opt = opt.bookmark(bm.clone());
        }
        let (rows, next) = UserTeam::find_by_team(cli, &user_team_sk, opt).await?;
        for row in rows {
            if matches!(row.role, TeamRole::Admin | TeamRole::Owner) {
                user_pks.insert(row.pk.to_string());
            }
        }
        match next {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    Ok(user_pks
        .into_iter()
        .filter_map(|s| s.parse::<Partition>().ok())
        .collect())
}

/// Count members (user_team rows) currently on the applying team.
pub async fn count_team_members(
    cli: &aws_sdk_dynamodb::Client,
    team_pk: &Partition,
) -> Result<i64> {
    let user_team_sk = EntityType::UserTeam(team_pk.to_string());
    let mut count: i64 = 0;
    let mut bookmark: Option<String> = None;
    for _ in 0..MAX_PAGES {
        let mut opt = crate::features::auth::UserTeamQueryOption::builder().limit(PAGE_SIZE);
        if let Some(bm) = bookmark.as_ref() {
            opt = opt.bookmark(bm.clone());
        }
        let (rows, next) = UserTeam::find_by_team(cli, &user_team_sk, opt).await?;
        count += rows.len() as i64;
        match next {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }
    // Owner record is separate from UserTeam; count it once if distinct.
    if let Ok(Some(_)) = TeamOwner::get(cli, team_pk, Some(&EntityType::TeamOwner)).await {
        count += 1;
    }
    Ok(count)
}

/// Load an application by its id from the parent's queue.
pub async fn get_application(
    cli: &aws_sdk_dynamodb::Client,
    parent_pk: &Partition,
    application_id: &str,
) -> Result<SubTeamApplication> {
    let sk = EntityType::SubTeamApplication(application_id.to_string());
    SubTeamApplication::get(cli, parent_pk, Some(sk))
        .await
        .map_err(|e| {
            crate::error!("get_application query failed: {e}");
            Error::from(SubTeamError::ApplicationNotFound)
        })?
        .ok_or_else(|| SubTeamError::ApplicationNotFound.into())
}

/// Approve an application — transactional:
///   1. application.status = Approved, decided_at, decision_reason=None
///   2. child team.parent_team_id = parent.id, clear pending_parent_team_id
///   3. create SubTeamLink under parent
/// Then notify sub-team admins.
pub async fn approve_application(
    cli: &aws_sdk_dynamodb::Client,
    parent_team: &Team,
    app: SubTeamApplication,
    approver_user_pk: &Partition,
) -> Result<SubTeamApplication> {
    if !matches!(
        app.status,
        SubTeamApplicationStatus::Pending | SubTeamApplicationStatus::Returned
    ) {
        return Err(SubTeamError::ApplicationStateMismatch.into());
    }

    let now = crate::common::utils::time::get_now_timestamp_millis();
    let parent_team_id = match &parent_team.pk {
        Partition::Team(id) => id.clone(),
        _ => {
            crate::error!("approve_application: unexpected parent pk variant");
            return Err(SubTeamError::ApplicationStateMismatch.into());
        }
    };
    let sub_team_id = app.sub_team_id.clone();
    let sub_team_pk: Partition = Partition::Team(sub_team_id.clone());

    // 1. application update
    let app_update = SubTeamApplication::updater(&parent_team.pk, &app.sk)
        .with_status(SubTeamApplicationStatus::Approved)
        .with_decided_at(now)
        .with_updated_at(now)
        .transact_write_item();

    // 2. child team update — set parent_team_id, clear pending_parent_team_id
    let team_update = Team::updater(&sub_team_pk, EntityType::Team)
        .with_parent_team_id(parent_team_id.clone())
        .remove_pending_parent_team_id()
        .with_updated_at(now)
        .transact_write_item();

    // 3. create SubTeamLink
    let link = SubTeamLink::new(
        parent_team.pk.clone(),
        sub_team_id.clone(),
        approver_user_pk.to_string(),
        app.application_id.clone(),
    );
    let link_create = link.create_transact_write_item();

    cli.transact_write_items()
        .set_transact_items(Some(vec![app_update, team_update, link_create]))
        .send()
        .await
        .map_err(|e| {
            crate::error!("approve_application transact_write failed: {:?}", e);
            Error::from(SubTeamError::ApplicationStateMismatch)
        })?;

    // Notifications to sub-team admins
    let parent_team_name = parent_team.display_name.clone();
    let cta_url = build_sub_team_apply_status_url(&sub_team_id);
    let admins = resolve_team_admins(cli, &sub_team_pk)
        .await
        .unwrap_or_default();
    for u in admins {
        let payload = InboxPayload::SubTeamApplicationApproved {
            parent_team_id: parent_team_id.clone(),
            parent_team_name: parent_team_name.clone(),
            sub_team_id: sub_team_id.clone(),
            cta_url: cta_url.clone(),
        };
        if let Err(e) = crate::common::utils::inbox::create_inbox_row(u, payload).await {
            crate::error!("approve_application notify failed: {e}");
        }
    }

    let mut updated = app;
    updated.status = SubTeamApplicationStatus::Approved;
    updated.decided_at = Some(now);
    updated.updated_at = now;
    Ok(updated)
}

/// Reject an application. Updates application + clears child team's
/// pending_parent_team_id. Notifies sub-team admins with the reason.
pub async fn reject_application(
    cli: &aws_sdk_dynamodb::Client,
    parent_team: &Team,
    app: SubTeamApplication,
    reason: String,
) -> Result<SubTeamApplication> {
    if !matches!(
        app.status,
        SubTeamApplicationStatus::Pending | SubTeamApplicationStatus::Returned
    ) {
        return Err(SubTeamError::ApplicationStateMismatch.into());
    }
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let parent_team_id = match &parent_team.pk {
        Partition::Team(id) => id.clone(),
        _ => {
            crate::error!("reject_application: unexpected parent pk variant");
            return Err(SubTeamError::ApplicationStateMismatch.into());
        }
    };
    let sub_team_id = app.sub_team_id.clone();
    let sub_team_pk: Partition = Partition::Team(sub_team_id.clone());

    let app_update = SubTeamApplication::updater(&parent_team.pk, &app.sk)
        .with_status(SubTeamApplicationStatus::Rejected)
        .with_decision_reason(reason.clone())
        .with_decided_at(now)
        .with_updated_at(now)
        .transact_write_item();

    let team_update = Team::updater(&sub_team_pk, EntityType::Team)
        .remove_pending_parent_team_id()
        .with_updated_at(now)
        .transact_write_item();

    cli.transact_write_items()
        .set_transact_items(Some(vec![app_update, team_update]))
        .send()
        .await
        .map_err(|e| {
            crate::error!("reject_application transact_write failed: {:?}", e);
            Error::from(SubTeamError::ApplicationStateMismatch)
        })?;

    let parent_team_name = parent_team.display_name.clone();
    let cta_url = build_sub_team_apply_status_url(&sub_team_id);
    let admins = resolve_team_admins(cli, &sub_team_pk)
        .await
        .unwrap_or_default();
    for u in admins {
        let payload = InboxPayload::SubTeamApplicationRejected {
            parent_team_id: parent_team_id.clone(),
            parent_team_name: parent_team_name.clone(),
            sub_team_id: sub_team_id.clone(),
            reason: reason.clone(),
            cta_url: cta_url.clone(),
        };
        if let Err(e) = crate::common::utils::inbox::create_inbox_row(u, payload).await {
            crate::error!("reject_application notify failed: {e}");
        }
    }

    let mut updated = app;
    updated.status = SubTeamApplicationStatus::Rejected;
    updated.decision_reason = Some(reason);
    updated.decided_at = Some(now);
    updated.updated_at = now;
    Ok(updated)
}

/// Return an application to the applying team for revision. The applying
/// team remains in pending_parent_team_id. Notifies sub-team admins.
pub async fn return_application(
    cli: &aws_sdk_dynamodb::Client,
    parent_team: &Team,
    app: SubTeamApplication,
    comment: String,
) -> Result<SubTeamApplication> {
    if !matches!(app.status, SubTeamApplicationStatus::Pending) {
        return Err(SubTeamError::ApplicationStateMismatch.into());
    }
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let parent_team_id = match &parent_team.pk {
        Partition::Team(id) => id.clone(),
        _ => {
            crate::error!("return_application: unexpected parent pk variant");
            return Err(SubTeamError::ApplicationStateMismatch.into());
        }
    };
    let sub_team_id = app.sub_team_id.clone();
    let sub_team_pk: Partition = Partition::Team(sub_team_id.clone());

    SubTeamApplication::updater(&parent_team.pk, &app.sk)
        .with_status(SubTeamApplicationStatus::Returned)
        .with_decision_reason(comment.clone())
        .with_decided_at(now)
        .with_updated_at(now)
        .execute(cli)
        .await
        .map_err(|e| {
            crate::error!("return_application execute failed: {e}");
            Error::from(SubTeamError::ApplicationStateMismatch)
        })?;

    let parent_team_name = parent_team.display_name.clone();
    let cta_url = build_sub_team_apply_status_url(&sub_team_id);
    let admins = resolve_team_admins(cli, &sub_team_pk)
        .await
        .unwrap_or_default();
    for u in admins {
        let payload = InboxPayload::SubTeamApplicationReturned {
            parent_team_id: parent_team_id.clone(),
            parent_team_name: parent_team_name.clone(),
            sub_team_id: sub_team_id.clone(),
            comment: comment.clone(),
            cta_url: cta_url.clone(),
        };
        if let Err(e) = crate::common::utils::inbox::create_inbox_row(u, payload).await {
            crate::error!("return_application notify failed: {e}");
        }
    }

    let mut updated = app;
    updated.status = SubTeamApplicationStatus::Returned;
    updated.decision_reason = Some(comment);
    updated.decided_at = Some(now);
    updated.updated_at = now;
    Ok(updated)
}

/// Notify parent admins that a new application was submitted.
pub async fn notify_parent_of_submission(
    cli: &aws_sdk_dynamodb::Client,
    parent_pk: &Partition,
    parent_team_id: &str,
    application_id: &str,
    sub_team_id: &str,
    sub_team_name: &str,
) {
    let cta_url = build_sub_team_management_url(parent_team_id);
    let admins = resolve_team_admins(cli, parent_pk).await.unwrap_or_default();
    for u in admins {
        let payload = InboxPayload::SubTeamApplicationSubmitted {
            parent_team_id: parent_team_id.to_string(),
            application_id: application_id.to_string(),
            sub_team_id: sub_team_id.to_string(),
            sub_team_name: sub_team_name.to_string(),
            cta_url: cta_url.clone(),
        };
        if let Err(e) = crate::common::utils::inbox::create_inbox_row(u, payload).await {
            crate::error!("notify_parent_of_submission failed: {e}");
        }
    }
}

fn build_sub_team_apply_status_url(sub_team_id: &str) -> String {
    format!("/teams/{sub_team_id}/parent")
}

fn build_sub_team_management_url(parent_team_id: &str) -> String {
    format!("/teams/{parent_team_id}/sub-teams")
}
