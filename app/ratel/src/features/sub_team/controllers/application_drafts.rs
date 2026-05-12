//! `SubTeamApplicationDraft` CRUD — lets the applicant save the form
//! state mid-flight and resume later. The row is keyed by
//! (applicant_team_pk, parent_team_id) so one applicant can hold one
//! draft per parent team they're considering.

use crate::common::*;
use crate::features::posts::models::Team;
use crate::features::social::pages::member::dto::TeamRole;
use crate::features::sub_team::models::{DocAgreementSnapshot, SubTeamApplicationDraft};
use crate::features::sub_team::types::{
    ApplicationDraftResponse, DocAgreementInput, SaveApplicationDraftRequest, SubTeamError,
};

// ── PUT /api/teams/:team_pk/sub-teams/application-draft ─────────
// `team_pk` is the **applicant** team. Admins of that team can write
// a draft for any parent.
#[post("/api/teams/:team_pk/sub-teams/application-draft", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn save_sub_team_application_draft_handler(
    team_pk: TeamPartition,
    body: SaveApplicationDraftRequest,
) -> Result<ApplicationDraftResponse> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let agreements: Vec<DocAgreementSnapshot> = body
        .doc_agreements
        .into_iter()
        .map(|a| DocAgreementSnapshot {
            doc_id: a.doc_id,
            body_hash: a.body_hash,
        })
        .collect();

    let draft = SubTeamApplicationDraft::new(
        team.pk.clone(),
        body.parent_team_id.clone(),
        body.form_values.clone(),
        agreements.clone(),
    );

    // Upsert — `DynamoEntity::create` puts a conditional `attribute_not_exists`
    // guard on pk/sk so it rejects repeats. Draft saves intentionally overwrite
    // every autosave tick, so use `upsert` (unconditional put).
    draft.upsert(cli).await.map_err(|e| {
        crate::error!("save_sub_team_application_draft upsert failed: {e}");
        SubTeamError::DraftOperationFailed
    })?;

    Ok(ApplicationDraftResponse {
        parent_team_id: draft.parent_team_id,
        form_values: draft.form_values,
        doc_agreements: agreements
            .into_iter()
            .map(|a| DocAgreementInput {
                doc_id: a.doc_id,
                body_hash: a.body_hash,
            })
            .collect(),
        updated_at: draft.updated_at,
    })
}

// ── GET /api/teams/:team_pk/sub-teams/application-draft/:parent_team_id ──
//
// Returns `Some(draft)` if one exists, `None` otherwise so the
// frontend can branch on resume-vs-start-fresh without a 404 round
// trip. The applicant team membership (via `team` extractor) is the
// only auth required — we don't gate on parent team eligibility
// because we're just returning the applicant's own scratchpad.
#[get("/api/teams/:team_pk/sub-teams/application-draft/:parent_team_id", user: crate::features::auth::User, team: Team)]
pub async fn get_sub_team_application_draft_handler(
    team_pk: TeamPartition,
    parent_team_id: String,
) -> Result<Option<ApplicationDraftResponse>> {
    let _ = team_pk;
    let _ = user;
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let sk = EntityType::SubTeamApplicationDraft(parent_team_id.clone());

    let row = SubTeamApplicationDraft::get(cli, &team.pk, Some(sk))
        .await
        .map_err(|e| {
            crate::error!("get_sub_team_application_draft get failed: {e}");
            SubTeamError::DraftOperationFailed
        })?;

    Ok(row.map(|d| ApplicationDraftResponse {
        parent_team_id: d.parent_team_id,
        form_values: d.form_values,
        doc_agreements: d
            .doc_agreements
            .into_iter()
            .map(|a| DocAgreementInput {
                doc_id: a.doc_id,
                body_hash: a.body_hash,
            })
            .collect(),
        updated_at: d.updated_at,
    }))
}

// ── DELETE /api/teams/:team_pk/sub-teams/application-draft/:parent_team_id
//
// Used by the submit handler after a successful POST so the draft
// doesn't linger after the application is sent.
#[delete("/api/teams/:team_pk/sub-teams/application-draft/:parent_team_id", user: crate::features::auth::User, team: Team)]
pub async fn delete_sub_team_application_draft_handler(
    team_pk: TeamPartition,
    parent_team_id: String,
) -> Result<String> {
    let _ = team_pk;
    let _ = user;
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let sk = EntityType::SubTeamApplicationDraft(parent_team_id);

    SubTeamApplicationDraft::delete(cli, &team.pk, Some(sk))
        .await
        .map_err(|e| {
            crate::error!("delete_sub_team_application_draft execute failed: {e}");
            SubTeamError::DraftOperationFailed
        })?;
    Ok(String::new())
}
