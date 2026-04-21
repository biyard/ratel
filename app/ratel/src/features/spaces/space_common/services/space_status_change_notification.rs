use std::collections::HashSet;

use crate::common::models::auth::User;
use crate::common::models::notification::Notification;
use crate::common::models::space::{SpaceCommon, SpaceStatusChangeEvent};
use crate::common::types::{NotificationData, SpaceStatus};
use crate::common::*;
use crate::features::auth::UserTeam;
use crate::features::posts::models::{Post, TeamOwner};
use crate::features::spaces::space_common::types::SpaceStatusChangeError;

const PAGE_SIZE: i32 = 100;
const MAX_PAGES: usize = 10;
const EMAIL_CHUNK_SIZE: usize = 50;

/// Handle a space status transition by resolving the audience and creating
/// Notification rows to fan out via the existing SES pipeline.
pub async fn handle_space_status_change(event: SpaceStatusChangeEvent) -> Result<()> {
    tracing::info!(
        space_pk = %event.space_pk,
        old_status = ?event.old_status,
        new_status = ?event.new_status,
        "handle_space_status_change: received event",
    );

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    // 1. Resolve audience user_pks first — cheap early-exit for no-op transitions.
    let user_pks = match (&event.old_status, &event.new_status) {
        (_, SpaceStatus::Open) => {
            let space = match load_space(cli, &event.space_pk).await? {
                Some(s) => s,
                None => return Ok(()),
            };
            match &space.user_pk {
                Partition::Team(_) => resolve_team_member_user_pks(cli, &space.user_pk).await?,
                _ => return Ok(()),
            }
        }
        (Some(SpaceStatus::Open), SpaceStatus::Ongoing)
        | (Some(SpaceStatus::Ongoing), SpaceStatus::Finished) => {
            resolve_space_participant_user_pks(cli, &event.space_pk).await?
        }
        _ => return Ok(()),
    };

    if user_pks.is_empty() {
        tracing::info!("handle_space_status_change: no recipients, skipping");
        return Ok(());
    }

    // 2. Load space + post for content (title, URL).
    let space = match load_space(cli, &event.space_pk).await? {
        Some(s) => s,
        None => return Ok(()),
    };
    let post_pk = space.pk.clone().to_post_key()?;
    let post = Post::get(cli, &post_pk, Some(&EntityType::Post))
        .await?
        .ok_or_else(|| {
            tracing::error!(
                "handle_space_status_change: post not found for {}",
                post_pk
            );
            SpaceStatusChangeError::PostNotFound
        })?;

    // 3. Resolve emails via batch_get + dedupe.
    let emails = resolve_emails(cli, user_pks).await?;
    if emails.is_empty() {
        tracing::info!("handle_space_status_change: no emails resolved, skipping");
        return Ok(());
    }

    // 4. Pick copy and CTA URL.
    let (headline, body) = status_change_copy(&event.new_status, &post.title);
    let cta_url = build_space_url(&event.space_pk);

    tracing::info!(
        space_pk = %event.space_pk,
        recipient_count = emails.len(),
        "handle_space_status_change: fanning out notifications",
    );

    // 5. Fan out into Notification rows, EMAIL_CHUNK_SIZE per row.
    for chunk in emails.chunks(EMAIL_CHUNK_SIZE) {
        let notification = Notification::new(NotificationData::SendSpaceStatusUpdate {
            emails: chunk.to_vec(),
            headline: headline.clone(),
            body: body.clone(),
            cta_url: cta_url.clone(),
            space_title: post.title.clone(),
        });
        if let Err(e) = notification.create(cli).await {
            tracing::error!(
                "handle_space_status_change: failed to create notification row: {e}"
            );
            // Continue; don't abort fan-out on a single failed chunk.
        }
    }

    Ok(())
}

async fn load_space(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
) -> Result<Option<SpaceCommon>> {
    Ok(SpaceCommon::get(cli, space_pk, Some(&EntityType::SpaceCommon)).await?)
}

async fn resolve_team_member_user_pks(
    cli: &aws_sdk_dynamodb::Client,
    team_pk: &Partition,
) -> Result<Vec<Partition>> {
    let mut user_pks: HashSet<String> = HashSet::new();

    // Paginate through UserTeam::find_by_team.
    let user_team_sk = EntityType::UserTeam(team_pk.to_string());
    let mut bookmark: Option<String> = None;
    for page in 0..MAX_PAGES {
        let mut opt = crate::features::auth::UserTeamQueryOption::builder().limit(PAGE_SIZE);
        if let Some(bm) = bookmark.as_ref() {
            opt = opt.bookmark(bm.clone());
        }
        let (rows, next) = UserTeam::find_by_team(cli, &user_team_sk, opt).await?;
        for row in rows {
            user_pks.insert(row.pk.to_string());
        }
        match next {
            Some(b) => bookmark = Some(b),
            None => break,
        }
        if page + 1 == MAX_PAGES {
            tracing::warn!(
                team_pk = %team_pk,
                "resolve_team_member_user_pks: hit MAX_PAGES cap; additional members truncated"
            );
        }
    }

    // Always include the team owner.
    if let Ok(Some(owner)) = TeamOwner::get(cli, team_pk, Some(&EntityType::TeamOwner)).await {
        user_pks.insert(owner.user_pk.to_string());
    }

    Ok(user_pks
        .into_iter()
        .filter_map(|s| s.parse::<Partition>().ok())
        .collect())
}

async fn resolve_space_participant_user_pks(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
) -> Result<Vec<Partition>> {
    use crate::common::models::space::SpaceParticipant;

    let mut user_pks: HashSet<String> = HashSet::new();

    let mut bookmark: Option<String> = None;
    for page in 0..MAX_PAGES {
        let mut opt = SpaceParticipant::opt().limit(PAGE_SIZE);
        if let Some(bm) = bookmark.as_ref() {
            opt = opt.bookmark(bm.clone());
        }
        let (rows, next) = SpaceParticipant::find_by_space(cli, space_pk, opt).await?;
        for row in rows {
            user_pks.insert(row.user_pk.to_string());
        }
        match next {
            Some(b) => bookmark = Some(b),
            None => break,
        }
        if page + 1 == MAX_PAGES {
            tracing::warn!(
                space_pk = %space_pk,
                "resolve_space_participant_user_pks: hit MAX_PAGES cap; additional participants truncated"
            );
        }
    }

    Ok(user_pks
        .into_iter()
        .filter_map(|s| s.parse::<Partition>().ok())
        .collect())
}

async fn resolve_emails(
    cli: &aws_sdk_dynamodb::Client,
    user_pks: Vec<Partition>,
) -> Result<Vec<String>> {
    let keys: Vec<(Partition, EntityType)> = user_pks
        .into_iter()
        .map(|pk| (pk, EntityType::User))
        .collect();

    let users: Vec<User> = if keys.is_empty() {
        vec![]
    } else {
        User::batch_get(cli, keys).await?
    };

    let mut seen: HashSet<String> = HashSet::new();
    let mut out: Vec<String> = Vec::new();
    for u in users {
        if u.email.is_empty() {
            continue;
        }
        if seen.insert(u.email.clone()) {
            out.push(u.email);
        }
    }
    Ok(out)
}

fn status_change_copy(new_status: &SpaceStatus, space_title: &str) -> (String, String) {
    match new_status {
        SpaceStatus::Open => (
            format!("{space_title} is now live"),
            "Your team just published this space. You can invite participants and track activity from the dashboard.".to_string(),
        ),
        SpaceStatus::Ongoing => (
            format!("{space_title} is starting now"),
            "The space you joined has started. Head in to participate.".to_string(),
        ),
        SpaceStatus::Finished => (
            format!("{space_title} has ended"),
            "This space is now closed. Thank you for participating — you can still view results on the dashboard.".to_string(),
        ),
        _ => (
            format!("{space_title} status updated"),
            "The space's status has changed.".to_string(),
        ),
    }
}

fn build_space_url(space_pk: &Partition) -> String {
    let id = match space_pk {
        Partition::Space(id) => id.clone(),
        _ => String::new(),
    };
    format!("https://ratel.foundation/spaces/{}", id)
}
