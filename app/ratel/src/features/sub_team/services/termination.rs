use crate::common::*;
use crate::features::posts::models::Team;
use crate::features::sub_team::models::SubTeamLink;
use crate::features::sub_team::services::application_lifecycle::resolve_team_admins;
use crate::features::sub_team::types::SubTeamError;

#[cfg(feature = "server")]
const LINK_SK_PREFIX: &str = "SUB_TEAM_LINK";
#[cfg(feature = "server")]
const PAGE_SIZE: i32 = 50;
#[cfg(feature = "server")]
const MAX_PAGES: usize = 5;

/// Sever a single parent↔child relationship in a single transact-write:
///   1. Delete the SubTeamLink row under parent_pk / sk=SUB_TEAM_LINK#child_id
///   2. Clear child Team.parent_team_id
/// Leaves all sub-team content (posts, spaces, members, messages) untouched.
#[cfg(feature = "server")]
pub async fn detach_sub_team_link(
    cli: &aws_sdk_dynamodb::Client,
    parent_pk: &Partition,
    child_pk: &Partition,
    child_id: &str,
) -> Result<()> {
    let now = crate::common::utils::time::get_now_timestamp_millis();

    let link_sk = EntityType::SubTeamLink(child_id.to_string());
    let delete_link = SubTeamLink::delete_transact_write_item(parent_pk.clone(), link_sk);

    let clear_parent = Team::updater(child_pk, EntityType::Team)
        .remove_parent_team_id()
        .with_updated_at(now)
        .transact_write_item();

    cli.transact_write_items()
        .set_transact_items(Some(vec![delete_link, clear_parent]))
        .send()
        .await
        .map_err(|e| {
            crate::error!("detach_sub_team_link transact_write failed: {:?}", e);
            Error::from(SubTeamError::ApplicationStateMismatch)
        })?;

    Ok(())
}

/// List all sub-team links under a given parent's pk. Used by the parent-delete
/// cascade to enumerate every child that needs detaching before Team removal.
#[cfg(feature = "server")]
pub async fn list_sub_team_links_for_cascade(
    cli: &aws_sdk_dynamodb::Client,
    parent_pk: &Partition,
) -> Result<Vec<SubTeamLink>> {
    let mut out: Vec<SubTeamLink> = Vec::new();
    let mut bookmark: Option<String> = None;
    for _ in 0..MAX_PAGES {
        let opts = SubTeamLink::opt_with_bookmark(bookmark.clone())
            .sk(LINK_SK_PREFIX.to_string())
            .limit(PAGE_SIZE);
        let (items, next) = SubTeamLink::query(cli, parent_pk.clone(), opts).await?;
        out.extend(items);
        match next {
            Some(b) => bookmark = Some(b),
            None => return Ok(out),
        }
    }
    Ok(out)
}

/// Notify every admin/owner of `target_pk` with the provided payload.
/// Best-effort — per-recipient failures are logged and swallowed.
#[cfg(feature = "server")]
pub async fn notify_team_admins(
    cli: &aws_sdk_dynamodb::Client,
    target_pk: &Partition,
    payload_for: impl Fn() -> InboxPayload,
) {
    let admins = resolve_team_admins(cli, target_pk).await.unwrap_or_default();
    for u in admins {
        let payload = payload_for();
        if let Err(e) = crate::common::utils::inbox::create_inbox_row(u, payload).await {
            crate::error!("notify_team_admins failed: {e}");
        }
    }
}

/// URL builder for the child-side "parent relationship" page (sub-team admin
/// landing point for termination notifications).
pub fn build_sub_team_parent_url(sub_team_id: &str) -> String {
    format!("/teams/{sub_team_id}/parent")
}

/// URL builder for the parent-side sub-teams management dashboard.
pub fn build_parent_sub_teams_url(parent_team_id: &str) -> String {
    format!("/teams/{parent_team_id}/sub-teams")
}

/// Parent-delete cascade: enumerate every SubTeamLink under `parent_pk`,
/// detach each one (delete link + clear child parent_team_id), and notify
/// the former sub-team admins. Sub-team CONTENT (posts, spaces, members,
/// messages) is NEVER touched here — this only severs the relationship.
///
/// Called from `delete_team_handler` before the Team row is removed so the
/// cascade happens synchronously in the same user-facing request.
#[cfg(feature = "server")]
pub async fn cascade_parent_delete_to_sub_teams(
    cli: &aws_sdk_dynamodb::Client,
    parent_pk: &Partition,
    parent_display_name: &str,
) -> Result<()> {
    let former_parent_team_id = match parent_pk {
        Partition::Team(id) => id.clone(),
        _ => return Ok(()),
    };
    let former_parent_team_name = parent_display_name.to_string();

    let links = list_sub_team_links_for_cascade(cli, parent_pk).await?;
    if links.is_empty() {
        return Ok(());
    }

    for link in links {
        let child_id = link.child_team_id.clone();
        let child_pk: Partition = Partition::Team(child_id.clone());

        if let Err(e) = detach_sub_team_link(cli, parent_pk, &child_pk, &child_id).await {
            crate::error!(
                "cascade_parent_delete: detach failed for child={}: {e}",
                child_id
            );
            continue;
        }

        // Notify former sub-team admins.
        let fpt_id = former_parent_team_id.clone();
        let fpt_name = former_parent_team_name.clone();
        let cta_url = build_sub_team_parent_url(&child_id);
        notify_team_admins(cli, &child_pk, move || {
            InboxPayload::SubTeamParentDeleted {
                former_parent_team_id: fpt_id.clone(),
                former_parent_team_name: fpt_name.clone(),
                cta_url: cta_url.clone(),
            }
        })
        .await;
    }

    Ok(())
}
