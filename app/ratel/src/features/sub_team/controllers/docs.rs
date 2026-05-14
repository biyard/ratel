use crate::common::*;
use crate::features::posts::models::Team;
use crate::features::social::pages::member::dto::TeamRole;
use crate::features::sub_team::models::{
    version_sk_prefix, SubTeamDocument, SubTeamDocumentVersion, SUB_TEAM_DOCUMENT_MAX_BODY_BYTES,
};
use crate::features::sub_team::types::{
    CreateSubTeamDocumentRequest, ReorderDocumentsRequest, SubTeamDocumentResponse,
    SubTeamDocumentVersionResponse, SubTeamError, UpdateSubTeamDocumentRequest,
};

const DOC_SK_PREFIX: &str = "SUB_TEAM_DOCUMENT";
const LIST_PAGE_LIMIT: i32 = 100;
const VERSION_PAGE_LIMIT: i32 = 100;

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

    // Trailing `#` is critical — without it the prefix also matches
    // `SUB_TEAM_DOCUMENT_VERSION#…` snapshot rows.
    let opts = SubTeamDocument::opt()
        .sk(format!("{DOC_SK_PREFIX}#"))
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
    let attachments = body.attachments.unwrap_or_default();

    let mut doc = SubTeamDocument::new(
        team.pk.clone(),
        body.title.clone(),
        body.body.clone(),
        body.required,
        order,
        user.username.clone(),
        attachments.clone(),
    );

    // Bylaws / ClubBylaws docs are dual-written: the SubTeamDocument
    // is the canonical record, and a backing `Post` (with the same
    // category) carries likes/comments + the public detail page.
    // Plain (uncategorised) docs skip the backing post entirely — that
    // preserves the Documents tab "required reading" workflow.
    if let Some(cat) = body.category.as_ref() {
        let trimmed = cat.trim();
        if !trimmed.is_empty() {
            use crate::features::posts::models::Post;
            use crate::features::posts::types::{PostStatus, PostType, Visibility};
            doc.category = Some(trimmed.to_string());

            let now = crate::common::utils::time::get_now_timestamp_millis();
            let backing_post = Post {
                pk: Partition::Feed(uuid::Uuid::now_v7().to_string()),
                sk: EntityType::Post,
                created_at: now,
                updated_at: now,
                title: doc.title.clone(),
                body: ContentBody::html(doc.body.clone()),
                post_type: PostType::Post,
                status: PostStatus::Published,
                visibility: Some(Visibility::Public),
                shares: 0,
                likes: 0,
                comments: 0,
                reports: 0,
                user_pk: team.pk.clone(),
                author_display_name: team.display_name.clone(),
                author_profile_url: team.profile_url.clone(),
                author_username: team.username.clone(),
                author_type: crate::common::types::UserType::Team,
                space_pk: None,
                space_type: None,
                space_visibility: None,
                booster: None,
                rewards: None,
                urls: vec![],
                categories: vec![trimmed.to_string()],
                announcement_id: None,
                announcement_parent_team_id: None,
                pinned_as_announcement: false,
            };
            if let Err(e) = backing_post.create(cli).await {
                crate::error!(
                    "create_sub_team_doc backing post create failed: {e}"
                );
                return Err(SubTeamError::DocumentNotFound.into());
            }
            doc.backing_post_id = Some(backing_post.pk.to_string());
        }
    }

    doc.create(cli).await.map_err(|e| {
        crate::error!("create_sub_team_doc execute failed: {e}");
        SubTeamError::DocumentNotFound
    })?;

    // Immutable v1 snapshot. A failure here doesn't roll the doc back
    // — we log and continue; the doc itself is the canonical record
    // and a later edit will produce v2 snapshot regardless.
    let snapshot = SubTeamDocumentVersion::snapshot_of(team.pk.clone(), &doc);
    if let Err(e) = snapshot.create(cli).await {
        crate::error!("create_sub_team_doc snapshot write failed: {e}");
    }

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

    if let Some(attachments) = body.attachments {
        updater = updater.with_attachments(attachments.clone());
        existing.attachments = attachments;
        changed = true;
    }

    if changed {
        // Bump version and stamp editor. Treat legacy rows (`version
        // == 0`) as v1 so the first edit lands at v2 (rather than v1)
        // — matches the composer's `v{version}` rendering.
        let next_version = existing.version.max(1) + 1;
        let editor = user.username.clone();
        updater = updater
            .with_version(next_version)
            .with_editor_username(editor.clone());
        existing.version = next_version;
        existing.editor_username = editor;

        updater.execute(cli).await.map_err(|e| {
            crate::error!("update_sub_team_doc execute failed: {e}");
            SubTeamError::DocumentNotFound
        })?;

        // Immutable snapshot at the new version. Best-effort — see
        // the create handler for the same rationale.
        let snapshot = SubTeamDocumentVersion::snapshot_of(team.pk.clone(), &existing);
        if let Err(e) = snapshot.create(cli).await {
            crate::error!("update_sub_team_doc snapshot write failed: {e}");
        }
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
    let doc_id_for_versions = doc_id.clone();
    let sk = EntityType::SubTeamDocument(doc_id);

    SubTeamDocument::delete(cli, &team.pk, Some(sk))
        .await
        .map_err(|e| {
            crate::error!("delete_sub_team_doc execute failed: {e}");
            SubTeamError::DocumentNotFound
        })?;

    // Cascade-remove every snapshot row for this doc. Failures are
    // logged but do not surface — the canonical doc row is gone, and
    // orphan snapshots are harmless beyond storage.
    let prefix = version_sk_prefix(&doc_id_for_versions);
    let opts = SubTeamDocumentVersion::opt()
        .sk(prefix)
        .limit(VERSION_PAGE_LIMIT);
    if let Ok((versions, _)) =
        SubTeamDocumentVersion::query(cli, team.pk.clone(), opts).await
    {
        for v in versions {
            if let Err(e) =
                SubTeamDocumentVersion::delete(cli, &team.pk, Some(v.sk.clone())).await
            {
                crate::error!("delete_sub_team_doc snapshot cleanup failed: {e}");
            }
        }
    }

    Ok(String::new())
}

// ── GET version history ─────────────────────────────────────────
#[get("/api/teams/:team_pk/sub-teams/docs/:doc_id/versions", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn list_sub_team_doc_versions_handler(
    team_pk: TeamPartition,
    doc_id: String,
) -> Result<ListResponse<SubTeamDocumentVersionResponse>> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let opts = SubTeamDocumentVersion::opt()
        .sk(version_sk_prefix(&doc_id))
        .limit(VERSION_PAGE_LIMIT);
    let (mut items, next) = SubTeamDocumentVersion::query(cli, team.pk.clone(), opts)
        .await
        .map_err(|e| {
            crate::error!("list_sub_team_doc_versions query failed: {e}");
            SubTeamError::DocumentNotFound
        })?;
    // Highest version first — most recent at the top.
    items.sort_by(|a, b| b.version.cmp(&a.version));
    let items: Vec<SubTeamDocumentVersionResponse> =
        items.into_iter().map(Into::into).collect();
    Ok((items, next).into())
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

// ── GET public bylaws list (no admin gate) ───────────────────────
//
// Bylaws page entry point. Lists this team's documents filtered by
// category (`"Bylaws"` / `"ClubBylaws"`) and enriches each response
// with the backing post's likes/comments (single batch_get).
//
// Public — no role check — because the bylaws page is a read-only
// reader surface. Admin-only listing keeps using
// `list_sub_team_docs_handler` for the Documents tab.
#[get(
    "/api/teams/:team_pk/sub-teams/bylaws?category",
    user: crate::features::auth::OptionalUser
)]
pub async fn list_team_bylaws_handler(
    team_pk: TeamPartition,
    category: Option<String>,
) -> Result<ListResponse<SubTeamDocumentResponse>> {
    let _ = user;
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let pk: Partition = team_pk.into();

    let opts = SubTeamDocument::opt()
        .sk(format!("{DOC_SK_PREFIX}#"))
        .limit(LIST_PAGE_LIMIT);
    let (mut items, next) = SubTeamDocument::query(cli, pk, opts)
        .await
        .map_err(|e| {
            crate::error!("list_team_bylaws query failed: {e}");
            SubTeamError::DocumentNotFound
        })?;
    sort_docs(&mut items);

    let category_filter = category.as_deref().map(|c| c.trim().to_string());
    let filtered: Vec<SubTeamDocument> = items
        .into_iter()
        .filter(|d| match category_filter.as_deref() {
            Some(c) => d.category.as_deref() == Some(c),
            None => d.category.is_some(),
        })
        .collect();

    // Batch-get backing posts so likes/comments come from the canonical
    // engagement source.
    use crate::features::posts::models::Post;
    use std::collections::HashMap;
    let post_keys: Vec<(Partition, EntityType)> = filtered
        .iter()
        .filter_map(|d| {
            d.backing_post_id
                .as_ref()
                .and_then(|s| s.parse::<Partition>().ok())
                .map(|pk| (pk, EntityType::Post))
        })
        .collect();
    let engagement: HashMap<String, (i64, i64)> = if post_keys.is_empty() {
        HashMap::new()
    } else {
        Post::batch_get(cli, post_keys)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|p| (p.pk.to_string(), (p.likes, p.comments)))
            .collect()
    };

    let items: Vec<SubTeamDocumentResponse> = filtered
        .into_iter()
        .map(|d| {
            let backing_key = d.backing_post_id.clone();
            let mut resp: SubTeamDocumentResponse = d.into();
            if let Some(key) = backing_key {
                if let Some((likes, comments)) = engagement.get(&key) {
                    resp.likes = *likes;
                    resp.comments = *comments;
                }
            }
            resp
        })
        .collect();
    Ok((items, next).into())
}
