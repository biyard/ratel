//! Admin headline CRUD endpoints for *Fact or Fold*.
//!
//! Surface mirrored from the design doc (PR1 in §"Scope & PR slicing"):
//!  - POST   /api/fact-or-fold/admin/headlines          create draft or scheduled
//!  - GET    /api/fact-or-fold/admin/headlines          list with status filter
//!  - GET    /api/fact-or-fold/admin/headlines/:id      single headline
//!  - PATCH  /api/fact-or-fold/admin/headlines/:id      edit (locked once Live)
//!  - DELETE /api/fact-or-fold/admin/headlines/:id      soft-delete
//!  - POST   /api/fact-or-fold/admin/headlines/:id/publish  schedule or publish-now
//!
//! All endpoints are gated by `AdminUser` — only the operator role per
//! roadmap §FR-39.

use crate::common::*;
use crate::features::fact_or_fold::types::*;

#[cfg(feature = "server")]
use crate::common::models::auth::AdminUser;
#[cfg(feature = "server")]
use crate::features::fact_or_fold::models::FactFoldHeadline;

#[cfg(feature = "server")]
const PAGE_LIMIT: i32 = 50;

// ── Helpers ───────────────────────────────────────────────────────

#[cfg(feature = "server")]
fn validate_headline_fields(
    headline_text: &str,
    body_excerpt: &str,
    difficulty: i32,
    reveal_sources_len: usize,
) -> Result<()> {
    if headline_text.trim().is_empty() || headline_text.len() > HEADLINE_TEXT_MAX {
        return Err(FactOrFoldError::HeadlineInvalid.into());
    }
    let body_len = body_excerpt.chars().count();
    if body_len < HEADLINE_BODY_MIN || body_len > HEADLINE_BODY_MAX {
        return Err(FactOrFoldError::HeadlineInvalid.into());
    }
    if !(HEADLINE_DIFFICULTY_MIN..=HEADLINE_DIFFICULTY_MAX).contains(&difficulty) {
        return Err(FactOrFoldError::HeadlineInvalid.into());
    }
    if reveal_sources_len > REVEAL_SOURCES_MAX {
        return Err(FactOrFoldError::HeadlineInvalid.into());
    }
    Ok(())
}

#[cfg(feature = "server")]
fn validate_scheduled_at(scheduled_at: Option<i64>) -> Result<()> {
    if let Some(ts) = scheduled_at {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        if ts < now {
            return Err(FactOrFoldError::PublishInvariantViolation.into());
        }
    }
    Ok(())
}

#[cfg(feature = "server")]
fn to_response(row: FactFoldHeadline) -> HeadlineResponse {
    let id = row.id().unwrap_or_default();
    HeadlineResponse {
        id: FactFoldHeadlineEntityType(id),
        status: row.status,
        headline_text: row.headline_text,
        body_excerpt: row.body_excerpt,
        verdict: row.verdict,
        difficulty: row.difficulty,
        category_tags: row.category_tags,
        source_label: row.source_label,
        insider_statement: row.insider_statement,
        reveal_summary: row.reveal_summary,
        reveal_sources: row.reveal_sources,
        scheduled_at: row.scheduled_at,
        created_at: row.created_at,
        updated_at: row.updated_at,
    }
}

// ── POST /api/fact-or-fold/admin/headlines ───────────────────────

#[post("/api/fact-or-fold/admin/headlines", user: AdminUser)]
pub async fn create_headline_handler(
    req: CreateHeadlineRequest,
) -> Result<HeadlineResponse> {
    validate_headline_fields(
        &req.headline_text,
        &req.body_excerpt,
        req.difficulty,
        req.reveal_sources.len(),
    )?;
    validate_scheduled_at(req.scheduled_at)?;

    if req.insider_statement.trim().is_empty() || req.source_label.trim().is_empty() {
        return Err(FactOrFoldError::HeadlineInvalid.into());
    }

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let headline_id = uuid::Uuid::new_v4().to_string();
    let row = FactFoldHeadline::new_draft(
        headline_id,
        user.pk.clone(),
        req.headline_text,
        req.body_excerpt,
        req.verdict,
        req.difficulty,
        req.category_tags,
        req.source_label,
        req.insider_statement,
        req.reveal_summary,
        req.reveal_sources,
        req.scheduled_at,
    );

    row.create(cli).await.map_err(|e| {
        crate::error!("create_headline_handler create failed: {e}");
        FactOrFoldError::StorageFailure
    })?;

    Ok(to_response(row))
}

// ── GET /api/fact-or-fold/admin/headlines ────────────────────────

const HEADLINE_SK_PREFIX: &str = "FACT_FOLD_HEADLINE";

#[get("/api/fact-or-fold/admin/headlines?bookmark&status", _user: AdminUser)]
pub async fn list_headlines_handler(
    bookmark: Option<String>,
    status: Option<HeadlineStatus>,
) -> Result<ListResponse<HeadlineResponse>> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    // All headlines share a single anchor pk so a sk-prefix query lists
    // them in one round-trip.
    let opts = FactFoldHeadline::opt_with_bookmark(bookmark)
        .sk(HEADLINE_SK_PREFIX.to_string())
        .limit(PAGE_LIMIT);
    let (rows, next) = FactFoldHeadline::query(cli, FactFoldHeadline::anchor_pk(), opts)
        .await
        .map_err(|e| {
            crate::error!("list_headlines_handler query failed: {e}");
            FactOrFoldError::StorageFailure
        })?;

    let exclude_deleted = status.is_none();
    let mut items: Vec<_> = rows
        .into_iter()
        .filter(|h| match status {
            Some(s) => h.status == s,
            None => true,
        })
        .filter(|h| {
            if exclude_deleted {
                !matches!(h.status, HeadlineStatus::Deleted)
            } else {
                true
            }
        })
        .collect();
    items.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    let responses: Vec<HeadlineResponse> = items.into_iter().map(to_response).collect();
    Ok((responses, next).into())
}

// ── GET /api/fact-or-fold/admin/headlines/{headline_id} ───────────

#[get("/api/fact-or-fold/admin/headlines/{headline_id}", _user: AdminUser)]
pub async fn get_headline_handler(
    headline_id: FactFoldHeadlineEntityType,
) -> Result<HeadlineResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let sk: EntityType = headline_id.into();
    let row = FactFoldHeadline::get(cli, &FactFoldHeadline::anchor_pk(), Some(sk))
        .await
        .map_err(|e| {
            crate::error!("get_headline_handler get failed: {e}");
            FactOrFoldError::StorageFailure
        })?
        .ok_or(FactOrFoldError::HeadlineNotFound)?;

    Ok(to_response(row))
}

// ── PATCH /api/fact-or-fold/admin/headlines/{headline_id} ─────────

#[patch("/api/fact-or-fold/admin/headlines/{headline_id}", _user: AdminUser)]
pub async fn update_headline_handler(
    headline_id: FactFoldHeadlineEntityType,
    req: UpdateHeadlineRequest,
) -> Result<HeadlineResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let pk = FactFoldHeadline::anchor_pk();
    let sk: EntityType = headline_id.into();

    let mut existing = FactFoldHeadline::get(cli, &pk, Some(sk.clone()))
        .await
        .map_err(|e| {
            crate::error!("update_headline_handler get failed: {e}");
            FactOrFoldError::StorageFailure
        })?
        .ok_or(FactOrFoldError::HeadlineNotFound)?;

    let locked = existing.is_locked();

    if locked {
        // Once a round is Live or Settled, only reveal_sources may grow
        // (roadmap §FR-43). Any other field change is rejected.
        let only_reveal_sources_change = req.headline_text.is_none()
            && req.body_excerpt.is_none()
            && req.verdict.is_none()
            && req.difficulty.is_none()
            && req.category_tags.is_none()
            && req.source_label.is_none()
            && req.insider_statement.is_none()
            && req.reveal_summary.is_none()
            && req.scheduled_at.is_none();
        if !only_reveal_sources_change {
            return Err(FactOrFoldError::HeadlineLocked.into());
        }
    }

    if let Some(ts) = req.scheduled_at {
        validate_scheduled_at(Some(ts))?;
    }

    let now = crate::common::utils::time::get_now_timestamp_millis();
    let mut updater = FactFoldHeadline::updater(&pk, &sk).with_updated_at(now);
    existing.updated_at = now;
    let mut changed = false;

    if let Some(v) = req.headline_text {
        if v.trim().is_empty() || v.len() > HEADLINE_TEXT_MAX {
            return Err(FactOrFoldError::HeadlineInvalid.into());
        }
        updater = updater.with_headline_text(v.clone());
        existing.headline_text = v;
        changed = true;
    }
    if let Some(v) = req.body_excerpt {
        let body_len = v.chars().count();
        if body_len < HEADLINE_BODY_MIN || body_len > HEADLINE_BODY_MAX {
            return Err(FactOrFoldError::HeadlineInvalid.into());
        }
        updater = updater.with_body_excerpt(v.clone());
        existing.body_excerpt = v;
        changed = true;
    }
    if let Some(v) = req.verdict {
        updater = updater.with_verdict(v);
        existing.verdict = v;
        changed = true;
    }
    if let Some(v) = req.difficulty {
        if !(HEADLINE_DIFFICULTY_MIN..=HEADLINE_DIFFICULTY_MAX).contains(&v) {
            return Err(FactOrFoldError::HeadlineInvalid.into());
        }
        updater = updater.with_difficulty(v);
        existing.difficulty = v;
        changed = true;
    }
    if let Some(v) = req.category_tags {
        updater = updater.with_category_tags(v.clone());
        existing.category_tags = v;
        changed = true;
    }
    if let Some(v) = req.source_label {
        if v.trim().is_empty() {
            return Err(FactOrFoldError::HeadlineInvalid.into());
        }
        updater = updater.with_source_label(v.clone());
        existing.source_label = v;
        changed = true;
    }
    if let Some(v) = req.insider_statement {
        if v.trim().is_empty() {
            return Err(FactOrFoldError::HeadlineInvalid.into());
        }
        updater = updater.with_insider_statement(v.clone());
        existing.insider_statement = v;
        changed = true;
    }
    if let Some(v) = req.reveal_summary {
        updater = updater.with_reveal_summary(v.clone());
        existing.reveal_summary = v;
        changed = true;
    }
    if let Some(v) = req.reveal_sources {
        if v.len() > REVEAL_SOURCES_MAX {
            return Err(FactOrFoldError::HeadlineInvalid.into());
        }
        updater = updater.with_reveal_sources(v.clone());
        existing.reveal_sources = v;
        changed = true;
    }
    if let Some(v) = req.scheduled_at {
        updater = updater.with_scheduled_at(v);
        existing.scheduled_at = Some(v);
        if matches!(existing.status, HeadlineStatus::Draft) {
            updater = updater.with_status(HeadlineStatus::Scheduled);
            existing.status = HeadlineStatus::Scheduled;
        }
        changed = true;
    }

    if changed {
        updater.execute(cli).await.map_err(|e| {
            crate::error!("update_headline_handler execute failed: {e}");
            FactOrFoldError::StorageFailure
        })?;
    }

    Ok(to_response(existing))
}

// ── DELETE /api/fact-or-fold/admin/headlines/{headline_id} ────────

#[delete("/api/fact-or-fold/admin/headlines/{headline_id}", _user: AdminUser)]
pub async fn delete_headline_handler(
    headline_id: FactFoldHeadlineEntityType,
) -> Result<HeadlineResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let pk = FactFoldHeadline::anchor_pk();
    let sk: EntityType = headline_id.into();

    let mut existing = FactFoldHeadline::get(cli, &pk, Some(sk.clone()))
        .await
        .map_err(|e| {
            crate::error!("delete_headline_handler get failed: {e}");
            FactOrFoldError::StorageFailure
        })?
        .ok_or(FactOrFoldError::HeadlineNotFound)?;

    if existing.is_locked() {
        return Err(FactOrFoldError::HeadlineLocked.into());
    }

    let now = crate::common::utils::time::get_now_timestamp_millis();
    FactFoldHeadline::updater(&pk, &sk)
        .with_status(HeadlineStatus::Deleted)
        .with_updated_at(now)
        .execute(cli)
        .await
        .map_err(|e| {
            crate::error!("delete_headline_handler execute failed: {e}");
            FactOrFoldError::StorageFailure
        })?;

    existing.status = HeadlineStatus::Deleted;
    existing.updated_at = now;
    Ok(to_response(existing))
}

// ── POST /api/fact-or-fold/admin/headlines/{headline_id}/publish ──

/// Drafts must satisfy the full validation (insider statement, body
/// length, sources) before they can leave Draft. Mode (schedule vs
/// publish-now) is encoded in `PublishHeadlineRequest::scheduled_at`.
#[post("/api/fact-or-fold/admin/headlines/{headline_id}/publish", _user: AdminUser)]
pub async fn publish_headline_handler(
    headline_id: FactFoldHeadlineEntityType,
    req: PublishHeadlineRequest,
) -> Result<HeadlineResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let pk = FactFoldHeadline::anchor_pk();
    let sk: EntityType = headline_id.into();

    let mut existing = FactFoldHeadline::get(cli, &pk, Some(sk.clone()))
        .await
        .map_err(|e| {
            crate::error!("publish_headline_handler get failed: {e}");
            FactOrFoldError::StorageFailure
        })?
        .ok_or(FactOrFoldError::HeadlineNotFound)?;

    if existing.is_locked() {
        return Err(FactOrFoldError::HeadlineLocked.into());
    }

    validate_headline_fields(
        &existing.headline_text,
        &existing.body_excerpt,
        existing.difficulty,
        existing.reveal_sources.len(),
    )
    .map_err(|_| FactOrFoldError::PublishInvariantViolation)?;

    if existing.insider_statement.trim().is_empty()
        || existing.source_label.trim().is_empty()
        || existing.reveal_summary.trim().is_empty()
    {
        return Err(FactOrFoldError::PublishInvariantViolation.into());
    }

    let now = crate::common::utils::time::get_now_timestamp_millis();
    let (next_status, next_scheduled_at) = match req.scheduled_at {
        Some(ts) => {
            if ts < now {
                return Err(FactOrFoldError::PublishInvariantViolation.into());
            }
            (HeadlineStatus::Scheduled, Some(ts))
        }
        None => (HeadlineStatus::Live, None),
    };

    let updater = FactFoldHeadline::updater(&pk, &sk)
        .with_status(next_status)
        .with_updated_at(now);
    let updater = match next_scheduled_at {
        Some(ts) => updater.with_scheduled_at(ts),
        // Publish-now clears any prior schedule so the row no longer
        // surfaces in queue/upcoming queries.
        None => updater.remove_scheduled_at(),
    };
    updater.execute(cli).await.map_err(|e| {
        crate::error!("publish_headline_handler execute failed: {e}");
        FactOrFoldError::StorageFailure
    })?;

    existing.status = next_status;
    existing.scheduled_at = next_scheduled_at;
    existing.updated_at = now;
    Ok(to_response(existing))
}
