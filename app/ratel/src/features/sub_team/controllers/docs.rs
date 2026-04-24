use crate::common::*;
use crate::features::posts::models::Team;
use crate::features::social::pages::member::dto::TeamRole;
use crate::features::sub_team::models::{SubTeamDocument, SUB_TEAM_DOCUMENT_MAX_BODY_BYTES};
use crate::features::sub_team::types::{
    CreateSubTeamDocumentRequest, ReorderDocumentsRequest, SubTeamDocumentResponse, SubTeamError,
    UpdateSubTeamDocumentRequest,
};

const DOC_SK_PREFIX: &str = "SUB_TEAM_DOCUMENT";
const LIST_PAGE_LIMIT: i32 = 100;

fn sort_docs(items: &mut [SubTeamDocument]) {
    items.sort_by(|a, b| a.order.cmp(&b.order).then(a.created_at.cmp(&b.created_at)));
}

// ── GET list (admin view) ───────────────────────────────────────
#[get("/api/teams/:team_pk/sub-teams/docs", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn list_sub_team_docs_handler(
    team_pk: TeamPartition,
) -> Result<ListResponse<SubTeamDocumentResponse>> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let opts = SubTeamDocument::opt()
        .sk(DOC_SK_PREFIX.to_string())
        .limit(LIST_PAGE_LIMIT);
    let (mut items, next) = SubTeamDocument::query(cli, team.pk.clone(), opts)
        .await
        .map_err(|e| {
            crate::error!("list_sub_team_docs query failed: {e}");
            SubTeamError::DocumentNotFound
        })?;
    sort_docs(&mut items);
    let items: Vec<SubTeamDocumentResponse> = items.into_iter().map(Into::into).collect();
    Ok((items, next).into())
}

// ── POST create ─────────────────────────────────────────────────
#[post("/api/teams/:team_pk/sub-teams/docs", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn create_sub_team_doc_handler(
    team_pk: TeamPartition,
    body: CreateSubTeamDocumentRequest,
) -> Result<SubTeamDocumentResponse> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    if body.body.as_bytes().len() > SUB_TEAM_DOCUMENT_MAX_BODY_BYTES {
        return Err(SubTeamError::DocumentBodyTooLarge.into());
    }

    let order = body.order.unwrap_or(0);

    let doc = SubTeamDocument::new(team.pk.clone(), body.title, body.body, body.required, order);
    doc.create(cli).await.map_err(|e| {
        crate::error!("create_sub_team_doc execute failed: {e}");
        SubTeamError::DocumentNotFound
    })?;

    Ok(doc.into())
}

// ── PATCH update ────────────────────────────────────────────────
#[patch("/api/teams/:team_pk/sub-teams/docs/:doc_id", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn update_sub_team_doc_handler(
    team_pk: TeamPartition,
    doc_id: String,
    body: UpdateSubTeamDocumentRequest,
) -> Result<SubTeamDocumentResponse> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let sk = EntityType::SubTeamDocument(doc_id);

    let mut existing = SubTeamDocument::get(cli, &team.pk, Some(sk.clone()))
        .await
        .map_err(|e| {
            crate::error!("update_sub_team_doc get failed: {e}");
            SubTeamError::DocumentNotFound
        })?
        .ok_or(SubTeamError::DocumentNotFound)?;

    let now = crate::common::utils::time::get_now_timestamp_millis();
    let mut updater = SubTeamDocument::updater(&team.pk, &sk).with_updated_at(now);
    existing.updated_at = now;
    let mut changed = false;

    if let Some(title) = body.title {
        updater = updater.with_title(title.clone());
        existing.title = title;
        changed = true;
    }

    if let Some(new_body) = body.body {
        if new_body.as_bytes().len() > SUB_TEAM_DOCUMENT_MAX_BODY_BYTES {
            return Err(SubTeamError::DocumentBodyTooLarge.into());
        }
        // Use the model helper to recompute body_hash + updated_at.
        existing.update_body(new_body.clone());
        updater = updater
            .with_body(new_body)
            .with_body_hash(existing.body_hash.clone())
            .with_updated_at(existing.updated_at);
        changed = true;
    }

    if let Some(required) = body.required {
        updater = updater.with_required(required);
        existing.required = required;
        changed = true;
    }

    if let Some(order) = body.order {
        updater = updater.with_order(order);
        existing.order = order;
        changed = true;
    }

    if changed {
        updater.execute(cli).await.map_err(|e| {
            crate::error!("update_sub_team_doc execute failed: {e}");
            SubTeamError::DocumentNotFound
        })?;
    }

    Ok(existing.into())
}

// ── DELETE ──────────────────────────────────────────────────────
#[delete("/api/teams/:team_pk/sub-teams/docs/:doc_id", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn delete_sub_team_doc_handler(
    team_pk: TeamPartition,
    doc_id: String,
) -> Result<String> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let sk = EntityType::SubTeamDocument(doc_id);

    SubTeamDocument::delete(cli, &team.pk, Some(sk))
        .await
        .map_err(|e| {
            crate::error!("delete_sub_team_doc execute failed: {e}");
            SubTeamError::DocumentNotFound
        })?;

    Ok(String::new())
}

// ── POST reorder ────────────────────────────────────────────────
#[post("/api/teams/:team_pk/sub-teams/docs/reorder", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn reorder_sub_team_docs_handler(
    team_pk: TeamPartition,
    body: ReorderDocumentsRequest,
) -> Result<String> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let now = crate::common::utils::time::get_now_timestamp_millis();

    for (idx, doc_id) in body.doc_ids.iter().enumerate() {
        let sk = EntityType::SubTeamDocument(doc_id.clone());
        let existing = SubTeamDocument::get(cli, &team.pk, Some(sk.clone()))
            .await
            .ok()
            .flatten();
        if existing.is_none() {
            continue;
        }
        let _ = SubTeamDocument::updater(&team.pk, &sk)
            .with_order(idx as i32)
            .with_updated_at(now)
            .execute(cli)
            .await
            .map_err(|e| {
                crate::error!("reorder_sub_team_docs per-row failed: {e}");
                SubTeamError::DocumentNotFound
            });
    }

    Ok(String::new())
}
