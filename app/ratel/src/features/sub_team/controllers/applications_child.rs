use crate::common::*;
use crate::features::posts::models::Team;
use crate::features::social::pages::member::dto::TeamRole;
use crate::features::sub_team::types::{
    ApplicationDecisionReasonRequest, ApplicationReturnCommentRequest, ParentRelationshipResponse,
    ParentRelationshipStatus, SubTeamApplicationDetailResponse, SubTeamApplicationResponse,
    SubTeamDocAgreementResponse, SubTeamError, SubmitApplicationRequest, UpdateApplicationRequest,
};

#[cfg(feature = "server")]
use crate::features::sub_team::models::{
    SubTeamApplication, SubTeamApplicationStatus, SubTeamDocAgreement, SubTeamDocument,
    SubTeamFormField, SubTeamFormFieldSnapshot, SubTeamLink,
};
#[cfg(feature = "server")]
use crate::features::sub_team::services::application_lifecycle::{
    count_team_members, notify_parent_of_submission,
};

#[cfg(feature = "server")]
const APPLICATION_SK_PREFIX: &str = "SUB_TEAM_APPLICATION";
#[cfg(feature = "server")]
const DOC_SK_PREFIX: &str = "SUB_TEAM_DOCUMENT";
#[cfg(feature = "server")]
const FIELD_SK_PREFIX: &str = "SUB_TEAM_FORM_FIELD";
#[cfg(feature = "server")]
const DOC_AGREEMENT_SK_PREFIX: &str = "SUB_TEAM_DOC_AGREEMENT";
#[cfg(feature = "server")]
const LINK_SK_PREFIX: &str = "SUB_TEAM_LINK";
#[cfg(feature = "server")]
const PAGE_LIMIT: i32 = 50;
#[cfg(feature = "server")]
const MAX_PAGES: usize = 5;

// ── GET /api/teams/:team_pk/parent — summary ───────────────────────
#[get("/api/teams/:team_pk/parent", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn get_parent_relationship_handler(
    team_pk: TeamPartition,
) -> Result<ParentRelationshipResponse> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let status = if team.parent_team_id.is_some() {
        ParentRelationshipStatus::RecognizedSubTeam
    } else if team.pending_parent_team_id.is_some() {
        ParentRelationshipStatus::PendingSubTeam
    } else {
        ParentRelationshipStatus::Standalone
    };

    let latest_application_id = match find_latest_application(cli, &team.pk).await {
        Ok(app) => app.map(|a| a.application_id),
        Err(e) => {
            crate::error!("get_parent_relationship latest app lookup failed: {e}");
            None
        }
    };

    // Resolve the parent (or pending-parent) team's display info so
    // the HUD panel can show the name + handle without a client-side
    // re-fetch. `recognized_at` is set only when the team is fully
    // recognized — pulled from the matching SubTeamLink row on the
    // parent's pk.
    let parent_uuid_for_join = team
        .parent_team_id
        .clone()
        .or_else(|| team.pending_parent_team_id.clone());
    let (parent_display_name, parent_username, recognized_at) = match parent_uuid_for_join {
        Some(uuid) => {
            let parent_pk = Partition::Team(uuid.clone());
            let display = Team::get(cli, &parent_pk, Some(EntityType::Team))
                .await
                .ok()
                .flatten();
            let (name, username) = match display {
                Some(t) => (Some(t.display_name), Some(t.username)),
                None => (None, None),
            };
            let recognized_at = if matches!(status, ParentRelationshipStatus::RecognizedSubTeam) {
                find_recognition_timestamp(cli, &parent_pk, &team.pk)
                    .await
                    .ok()
                    .flatten()
            } else {
                None
            };
            (name, username, recognized_at)
        }
        None => (None, None, None),
    };

    Ok(ParentRelationshipResponse {
        status,
        parent_team_id: team.parent_team_id,
        pending_parent_team_id: team.pending_parent_team_id,
        latest_application_id,
        parent_team_display_name: parent_display_name,
        parent_team_username: parent_username,
        recognized_at,
    })
}

// Walks SubTeamLink rows under `parent_pk` and returns the
// `created_at` of the link whose child team matches `child_pk`. Used by
// the HUD panel to show "인증 YYYY-MM-DD ~" for recognized sub-teams.
#[cfg(feature = "server")]
async fn find_recognition_timestamp(
    cli: &aws_sdk_dynamodb::Client,
    parent_pk: &Partition,
    child_pk: &Partition,
) -> Result<Option<i64>> {
    let prefix = format!("{LINK_SK_PREFIX}#");
    let opts = SubTeamLink::opt().sk(prefix).limit(PAGE_LIMIT);
    let (links, _) = SubTeamLink::query(cli, parent_pk.clone(), opts).await?;
    let child_uuid = match child_pk {
        Partition::Team(id) => id.clone(),
        _ => return Ok(None),
    };
    Ok(links
        .into_iter()
        .find(|l| l.child_team_id == child_uuid)
        .map(|l| l.approved_at))
}

// ── GET /api/teams/:team_pk/parent/applications ────────────────────
#[get("/api/teams/:team_pk/parent/applications?bookmark", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn list_child_applications_handler(
    team_pk: TeamPartition,
    bookmark: Option<String>,
) -> Result<ListResponse<SubTeamApplicationResponse>> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let opts = SubTeamApplication::opt_with_bookmark(bookmark)
        .limit(PAGE_LIMIT)
        .scan_index_forward(false);

    let (items, next) = SubTeamApplication::find_by_applicant(cli, team.pk.clone(), opts)
        .await
        .map_err(|e| {
            crate::error!("list_child_applications find_by_applicant failed: {e}");
            SubTeamError::ApplicationNotFound
        })?;

    // Join each application to its parent team so the status page can
    // render the feedback card's author as the parent (not the
    // applicant team). N joins, but pages are capped to PAGE_LIMIT.
    let mut response: Vec<SubTeamApplicationResponse> = Vec::new();
    for app in items.into_iter() {
        let mut dto: SubTeamApplicationResponse = app.clone().into();
        // Applicant is the caller's `team` already — fill those fields
        // from the extractor instead of an extra ddb read.
        dto.applicant_team_display_name = team.display_name.clone();
        dto.applicant_team_username = team.username.clone();
        let parent_pk = crate::common::types::Partition::Team(app.parent_team_id.clone());
        if let Ok(Some(parent)) = crate::features::posts::models::Team::get(
            cli,
            &parent_pk,
            Some(crate::common::types::EntityType::Team),
        )
        .await
        {
            dto.parent_team_display_name = parent.display_name;
            dto.parent_team_username = parent.username;
        }
        response.push(dto);
    }
    Ok((response, next).into())
}

// ── POST /api/teams/:team_pk/parent/applications — submit ──────────
#[post("/api/teams/:team_pk/parent/applications", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn submit_application_handler(
    team_pk: TeamPartition,
    body: SubmitApplicationRequest,
) -> Result<SubTeamApplicationResponse> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    // 1. Applying team pulled from extractor `team` — already exists and user is admin/owner.

    // 2. Parent must exist AND be parent-eligible.
    let parent_pk: Partition = Partition::Team(body.parent_team_id.clone());
    let parent = Team::get(cli, &parent_pk, Some(EntityType::Team))
        .await
        .map_err(|e| {
            crate::error!("submit_application parent load failed: {e}");
            SubTeamError::ParentNotEligible
        })?
        .ok_or(SubTeamError::ParentNotEligible)?;
    if !parent.is_parent_eligible {
        return Err(SubTeamError::ParentNotEligible.into());
    }

    // 4. Cycle: cannot apply to self; cannot apply to own existing sub-team.
    if parent.pk == team.pk {
        return Err(SubTeamError::CycleDetected.into());
    }
    if is_parent_already_sub_team_of_child(cli, &team.pk, &parent.pk)
        .await
        .map_err(|e| {
            crate::error!("cycle detection failed: {e}");
            Error::from(SubTeamError::CycleDetected)
        })?
    {
        return Err(SubTeamError::CycleDetected.into());
    }

    // 3. Resubmit path — if this applicant already has a Returned
    //    application for THIS parent, update it in place instead of
    //    creating a new one. Keeps `application_id` stable so the
    //    parent's reviewer sees one continuous timeline. Doc
    //    agreements + form_snapshot from the original submission are
    //    preserved (snapshot at v1 stays immutable).
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let returned_existing = find_returned_application(cli, &team.pk, &body.parent_team_id)
        .await
        .map_err(|e| {
            crate::error!("returned lookup failed: {e}");
            Error::from(SubTeamError::ApplicationStateMismatch)
        })?;

    if let Some(existing) = returned_existing {
        // Member count is the only gate we re-check on update; form &
        // doc snapshots stay frozen at v1.
        let member_count = count_team_members(cli, &team.pk).await.map_err(|e| {
            crate::error!("member count failed: {e}");
            Error::from(SubTeamError::MemberCountBelowMinimum)
        })?;
        if member_count < parent.min_sub_team_members as i64 {
            return Err(SubTeamError::MemberCountBelowMinimum.into());
        }
        validate_form_values(&[], &body.form_values).ok();

        SubTeamApplication::updater(&existing.pk, existing.sk.clone())
            .with_status(SubTeamApplicationStatus::Pending)
            .with_submitted_at(now)
            .with_updated_at(now)
            .with_decision_reason(String::new())
            .with_form_values(body.form_values.clone())
            .execute(cli)
            .await
            .map_err(|e| {
                crate::error!("resubmit application update failed: {:?}", e);
                Error::from(SubTeamError::ApplicationStateMismatch)
            })?;

        notify_parent_of_submission(
            cli,
            &parent.pk,
            &body.parent_team_id,
            &existing.application_id,
            &existing.sub_team_id,
            &team.display_name,
        )
        .await;

        // Load fresh row so the response carries the updated fields.
        let refreshed = SubTeamApplication::get(cli, &existing.pk, Some(existing.sk.clone()))
            .await
            .map_err(|e| {
                crate::error!("resubmit refresh failed: {:?}", e);
                Error::from(SubTeamError::ApplicationStateMismatch)
            })?
            .unwrap_or(existing);
        return Ok(refreshed.into());
    }

    // 4. New-submit path — block when any in-flight app exists.
    if has_in_flight_application(cli, &team.pk).await.map_err(|e| {
        crate::error!("in-flight lookup failed: {e}");
        Error::from(SubTeamError::ApplicationInFlight)
    })? {
        return Err(SubTeamError::ApplicationInFlight.into());
    }

    // 5. Member count ≥ parent.min_sub_team_members.
    let member_count = count_team_members(cli, &team.pk).await.map_err(|e| {
        crate::error!("member count failed: {e}");
        Error::from(SubTeamError::MemberCountBelowMinimum)
    })?;
    if member_count < parent.min_sub_team_members as i64 {
        return Err(SubTeamError::MemberCountBelowMinimum.into());
    }

    // 6+7. Load parent's form fields and docs.
    let parent_form_fields = load_form_fields(cli, &parent.pk).await.map_err(|e| {
        crate::error!("form fields load failed: {e}");
        Error::from(SubTeamError::MissingRequiredFormField)
    })?;
    let parent_docs = load_docs(cli, &parent.pk).await.map_err(|e| {
        crate::error!("docs load failed: {e}");
        Error::from(SubTeamError::MissingRequiredDocAgreement)
    })?;

    validate_form_values(&parent_form_fields, &body.form_values)?;
    let agreed_docs = validate_doc_agreements(&parent_docs, &body.doc_agreements)?;

    // 8. Transact-write: application + doc agreements + pending_parent on child.
    let parent_team_id = match &parent.pk {
        Partition::Team(id) => id.clone(),
        _ => {
            crate::error!("submit_application: parent pk is not Team");
            return Err(SubTeamError::ApplicationStateMismatch.into());
        }
    };
    let sub_team_id = match &team.pk {
        Partition::Team(id) => id.clone(),
        _ => {
            crate::error!("submit_application: applying team pk is not Team");
            return Err(SubTeamError::ApplicationStateMismatch.into());
        }
    };

    let mut application = SubTeamApplication::new(
        parent.pk.clone(),
        team.pk.clone(),
        parent_team_id.clone(),
        sub_team_id.clone(),
        user.pk.to_string(),
    );
    application.status = SubTeamApplicationStatus::Pending;
    application.submitted_at = Some(now);
    application.updated_at = now;
    application.form_values = body.form_values.clone();
    application.form_snapshot = parent_form_fields
        .iter()
        .map(|f| SubTeamFormFieldSnapshot {
            field_id: sub_team_form_field_id(f),
            label: f.label.clone(),
            field_type: f.field_type,
            required: f.required,
            order: f.order,
            options: f.options.clone(),
            locked: f.locked,
        })
        .collect();

    let mut items = vec![application.create_transact_write_item()];
    for (doc, hash) in agreed_docs {
        let doc_id = sub_team_document_id(&doc);
        let agreement = SubTeamDocAgreement::new(
            parent.pk.clone(),
            application.application_id.clone(),
            doc_id,
            doc.title.clone(),
            hash,
            user.pk.to_string(),
        );
        items.push(agreement.create_transact_write_item());
    }

    items.push(
        Team::updater(&team.pk, EntityType::Team)
            .with_pending_parent_team_id(parent_team_id.clone())
            .with_updated_at(now)
            .transact_write_item(),
    );

    cli.transact_write_items()
        .set_transact_items(Some(items))
        .send()
        .await
        .map_err(|e| {
            crate::error!("submit_application transact_write failed: {:?}", e);
            Error::from(SubTeamError::ApplicationStateMismatch)
        })?;

    // 9. Notify parent admins.
    notify_parent_of_submission(
        cli,
        &parent.pk,
        &parent_team_id,
        &application.application_id,
        &sub_team_id,
        &team.display_name,
    )
    .await;

    Ok(application.into())
}

// ── GET /api/teams/:team_pk/parent/applications/:application_id ────
#[get("/api/teams/:team_pk/parent/applications/:application_id", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn get_child_application_handler(
    team_pk: TeamPartition,
    application_id: String,
) -> Result<SubTeamApplicationDetailResponse> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let app = find_application_for_child(cli, &team.pk, &application_id).await?;
    let parent_pk = app.pk.clone();
    let agreements = list_agreements(cli, &parent_pk, &application_id)
        .await
        .unwrap_or_default();
    let application: SubTeamApplicationResponse = app.into();
    let doc_agreements: Vec<SubTeamDocAgreementResponse> =
        agreements.into_iter().map(Into::into).collect();
    Ok(SubTeamApplicationDetailResponse {
        application,
        doc_agreements,
    })
}

// ── PATCH /api/teams/:team_pk/parent/applications/:application_id ──
#[patch("/api/teams/:team_pk/parent/applications/:application_id", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn update_child_application_handler(
    team_pk: TeamPartition,
    application_id: String,
    body: UpdateApplicationRequest,
) -> Result<SubTeamApplicationResponse> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let app = find_application_for_child(cli, &team.pk, &application_id).await?;
    if app.status != SubTeamApplicationStatus::Returned {
        return Err(SubTeamError::ApplicationStateMismatch.into());
    }
    let parent_pk = app.pk.clone();
    let parent = Team::get(cli, &parent_pk, Some(EntityType::Team))
        .await
        .map_err(|e| {
            crate::error!("update_child_application parent load failed: {e}");
            SubTeamError::ParentNotEligible
        })?
        .ok_or(SubTeamError::ParentNotEligible)?;

    let form_values = body.form_values.clone().unwrap_or_else(|| app.form_values.clone());
    let doc_agreements = body.doc_agreements.clone();

    let member_count = count_team_members(cli, &team.pk).await.map_err(|e| {
        crate::error!("member count failed: {e}");
        Error::from(SubTeamError::MemberCountBelowMinimum)
    })?;
    if member_count < parent.min_sub_team_members as i64 {
        return Err(SubTeamError::MemberCountBelowMinimum.into());
    }

    let parent_form_fields = load_form_fields(cli, &parent.pk).await.map_err(|e| {
        crate::error!("form fields load failed: {e}");
        Error::from(SubTeamError::MissingRequiredFormField)
    })?;
    validate_form_values(&parent_form_fields, &form_values)?;

    let mut items: Vec<aws_sdk_dynamodb::types::TransactWriteItem> = Vec::new();

    let now = crate::common::utils::time::get_now_timestamp_millis();
    if let Some(new_agreements) = doc_agreements.as_ref() {
        let parent_docs = load_docs(cli, &parent.pk).await.map_err(|e| {
            crate::error!("docs load failed: {e}");
            Error::from(SubTeamError::MissingRequiredDocAgreement)
        })?;
        let agreed_docs = validate_doc_agreements(&parent_docs, new_agreements)?;

        // Remove old agreements, then create new.
        let old_agreements = list_agreements(cli, &parent.pk, &application_id)
            .await
            .unwrap_or_default();
        for old in old_agreements {
            items.push(
                SubTeamDocAgreement::delete_transact_write_item(&parent.pk, old.sk),
            );
        }
        for (doc, hash) in agreed_docs {
            let doc_id = sub_team_document_id(&doc);
            let agreement = SubTeamDocAgreement::new(
                parent.pk.clone(),
                application_id.clone(),
                doc_id,
                doc.title.clone(),
                hash,
                user.pk.to_string(),
            );
            items.push(agreement.create_transact_write_item());
        }
    }

    // Application update: back to Pending with submitted_at=now.
    let mut app_updater = SubTeamApplication::updater(&parent.pk, &app.sk)
        .with_status(SubTeamApplicationStatus::Pending)
        .with_submitted_at(now)
        .with_updated_at(now)
        .remove_decision_reason();
    app_updater = app_updater.with_form_values(form_values.clone());
    items.push(app_updater.transact_write_item());

    cli.transact_write_items()
        .set_transact_items(Some(items))
        .send()
        .await
        .map_err(|e| {
            crate::error!("update_child_application transact_write failed: {:?}", e);
            Error::from(SubTeamError::ApplicationStateMismatch)
        })?;

    // Notify parent admins of resubmission.
    let parent_team_id = match &parent.pk {
        Partition::Team(id) => id.clone(),
        _ => String::new(),
    };
    let sub_team_id = match &team.pk {
        Partition::Team(id) => id.clone(),
        _ => String::new(),
    };
    notify_parent_of_submission(
        cli,
        &parent.pk,
        &parent_team_id,
        &application_id,
        &sub_team_id,
        &team.display_name,
    )
    .await;

    let mut updated = app;
    updated.status = SubTeamApplicationStatus::Pending;
    updated.submitted_at = Some(now);
    updated.updated_at = now;
    updated.form_values = form_values;
    updated.decision_reason = None;
    Ok(updated.into())
}

// ── POST /api/teams/:team_pk/parent/applications/:application_id/cancel ──
#[post("/api/teams/:team_pk/parent/applications/:application_id/cancel", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn cancel_child_application_handler(
    team_pk: TeamPartition,
    application_id: String,
) -> Result<SubTeamApplicationResponse> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let app = find_application_for_child(cli, &team.pk, &application_id).await?;
    if !matches!(
        app.status,
        SubTeamApplicationStatus::Pending | SubTeamApplicationStatus::Returned
    ) {
        return Err(SubTeamError::ApplicationStateMismatch.into());
    }
    let parent_pk = app.pk.clone();

    let now = crate::common::utils::time::get_now_timestamp_millis();

    let mut items: Vec<aws_sdk_dynamodb::types::TransactWriteItem> = Vec::new();
    items.push(
        SubTeamApplication::updater(&parent_pk, &app.sk)
            .with_status(SubTeamApplicationStatus::Cancelled)
            .with_decided_at(now)
            .with_updated_at(now)
            .transact_write_item(),
    );
    items.push(
        Team::updater(&team.pk, EntityType::Team)
            .remove_pending_parent_team_id()
            .with_updated_at(now)
            .transact_write_item(),
    );

    cli.transact_write_items()
        .set_transact_items(Some(items))
        .send()
        .await
        .map_err(|e| {
            crate::error!("cancel_child_application transact_write failed: {:?}", e);
            Error::from(SubTeamError::ApplicationStateMismatch)
        })?;

    let mut updated = app;
    updated.status = SubTeamApplicationStatus::Cancelled;
    updated.decided_at = Some(now);
    updated.updated_at = now;
    Ok(updated.into())
}

// Avoid triggering unused-import warnings when body type is only used above.
#[cfg(feature = "server")]
fn _unused_silencer() {
    let _ = ApplicationDecisionReasonRequest::default();
    let _ = ApplicationReturnCommentRequest::default();
}

// ── Helpers ────────────────────────────────────────────────────────

#[cfg(feature = "server")]
fn sub_team_form_field_id(f: &SubTeamFormField) -> String {
    match &f.sk {
        EntityType::SubTeamFormField(id) => id.clone(),
        _ => String::new(),
    }
}

#[cfg(feature = "server")]
fn sub_team_document_id(d: &SubTeamDocument) -> String {
    match &d.sk {
        EntityType::SubTeamDocument(id) => id.clone(),
        _ => String::new(),
    }
}

#[cfg(feature = "server")]
async fn load_form_fields(
    cli: &aws_sdk_dynamodb::Client,
    parent_pk: &Partition,
) -> Result<Vec<SubTeamFormField>> {
    let opts = SubTeamFormField::opt()
        .sk(FIELD_SK_PREFIX.to_string())
        .limit(PAGE_LIMIT);
    let (items, _) = SubTeamFormField::query(cli, parent_pk.clone(), opts).await?;
    Ok(items)
}

#[cfg(feature = "server")]
async fn load_docs(
    cli: &aws_sdk_dynamodb::Client,
    parent_pk: &Partition,
) -> Result<Vec<SubTeamDocument>> {
    // Trailing `#` keeps `SUB_TEAM_DOCUMENT_VERSION#…` snapshot rows
    // out of the result set.
    let opts = SubTeamDocument::opt()
        .sk(format!("{DOC_SK_PREFIX}#"))
        .limit(PAGE_LIMIT);
    let (items, _) = SubTeamDocument::query(cli, parent_pk.clone(), opts).await?;
    Ok(items)
}

#[cfg(feature = "server")]
fn validate_form_values(
    fields: &[SubTeamFormField],
    values: &std::collections::HashMap<String, serde_json::Value>,
) -> Result<()> {
    for f in fields {
        if !f.required {
            continue;
        }
        let id = sub_team_form_field_id(f);
        let Some(v) = values.get(&id) else {
            return Err(SubTeamError::MissingRequiredFormField.into());
        };
        if is_empty_value(v) {
            return Err(SubTeamError::MissingRequiredFormField.into());
        }
    }
    Ok(())
}

#[cfg(feature = "server")]
fn is_empty_value(v: &serde_json::Value) -> bool {
    match v {
        serde_json::Value::Null => true,
        serde_json::Value::String(s) => s.trim().is_empty(),
        serde_json::Value::Array(a) => a.is_empty(),
        serde_json::Value::Object(o) => o.is_empty(),
        _ => false,
    }
}

#[cfg(feature = "server")]
fn validate_doc_agreements(
    parent_docs: &[SubTeamDocument],
    agreements: &[crate::features::sub_team::types::DocAgreementInput],
) -> Result<Vec<(SubTeamDocument, String)>> {
    let mut out: Vec<(SubTeamDocument, String)> = Vec::new();
    for doc in parent_docs {
        if !doc.required {
            continue;
        }
        let doc_id = sub_team_document_id(doc);
        let Some(ag) = agreements.iter().find(|a| a.doc_id == doc_id) else {
            return Err(SubTeamError::MissingRequiredDocAgreement.into());
        };
        if ag.body_hash != doc.body_hash {
            return Err(SubTeamError::DocAgreementStale.into());
        }
        out.push((doc.clone(), ag.body_hash.clone()));
    }
    Ok(out)
}

#[cfg(feature = "server")]
async fn has_in_flight_application(
    cli: &aws_sdk_dynamodb::Client,
    applicant_pk: &Partition,
) -> Result<bool> {
    let mut bookmark: Option<String> = None;
    for _ in 0..MAX_PAGES {
        let opts = SubTeamApplication::opt_with_bookmark(bookmark.clone()).limit(PAGE_LIMIT);
        let (items, next) =
            SubTeamApplication::find_by_applicant(cli, applicant_pk.clone(), opts).await?;
        for a in &items {
            if matches!(
                a.status,
                SubTeamApplicationStatus::Pending | SubTeamApplicationStatus::Returned
            ) {
                return Ok(true);
            }
        }
        match next {
            Some(b) => bookmark = Some(b),
            None => return Ok(false),
        }
    }
    Ok(false)
}

/// Looks for the most recent application under `applicant_pk` whose
/// status is `Returned` and parent matches `parent_team_id`. When such
/// an application exists, "Edit and resubmit" UPDATES it back to
/// `Pending` instead of creating a brand-new row — the original
/// application id stays stable so reviewers see one continuous timeline
/// rather than a fresh queue entry.
#[cfg(feature = "server")]
async fn find_returned_application(
    cli: &aws_sdk_dynamodb::Client,
    applicant_pk: &Partition,
    parent_team_id: &str,
) -> Result<Option<SubTeamApplication>> {
    let mut bookmark: Option<String> = None;
    for _ in 0..MAX_PAGES {
        let opts = SubTeamApplication::opt_with_bookmark(bookmark.clone()).limit(PAGE_LIMIT);
        let (items, next) =
            SubTeamApplication::find_by_applicant(cli, applicant_pk.clone(), opts).await?;
        for a in items {
            if matches!(a.status, SubTeamApplicationStatus::Returned)
                && a.parent_team_id == parent_team_id
            {
                return Ok(Some(a));
            }
        }
        match next {
            Some(b) => bookmark = Some(b),
            None => return Ok(None),
        }
    }
    Ok(None)
}

#[cfg(feature = "server")]
async fn is_parent_already_sub_team_of_child(
    cli: &aws_sdk_dynamodb::Client,
    child_team_pk: &Partition,
    candidate_parent_pk: &Partition,
) -> Result<bool> {
    // Look under child's pk for SubTeamLink rows — if the candidate parent is
    // among those, we'd create a cycle.
    let mut bookmark: Option<String> = None;
    let candidate_id = match candidate_parent_pk {
        Partition::Team(id) => id.clone(),
        _ => return Ok(false),
    };
    for _ in 0..MAX_PAGES {
        let opts = SubTeamLink::opt_with_bookmark(bookmark.clone())
            .sk(LINK_SK_PREFIX.to_string())
            .limit(PAGE_LIMIT);
        let (items, next) = SubTeamLink::query(cli, child_team_pk.clone(), opts).await?;
        for link in &items {
            if link.child_team_id == candidate_id {
                return Ok(true);
            }
        }
        match next {
            Some(b) => bookmark = Some(b),
            None => return Ok(false),
        }
    }
    Ok(false)
}

#[cfg(feature = "server")]
async fn find_latest_application(
    cli: &aws_sdk_dynamodb::Client,
    applicant_pk: &Partition,
) -> Result<Option<SubTeamApplication>> {
    let opts = SubTeamApplication::opt().limit(1).scan_index_forward(false);
    let (items, _) = SubTeamApplication::find_by_applicant(cli, applicant_pk.clone(), opts).await?;
    Ok(items.into_iter().next())
}

#[cfg(feature = "server")]
async fn find_application_for_child(
    cli: &aws_sdk_dynamodb::Client,
    applicant_pk: &Partition,
    application_id: &str,
) -> Result<SubTeamApplication> {
    // Walk the GSI1 entries for this applicant and find the one with matching id.
    let mut bookmark: Option<String> = None;
    for _ in 0..MAX_PAGES {
        let opts = SubTeamApplication::opt_with_bookmark(bookmark.clone()).limit(PAGE_LIMIT);
        let (items, next) =
            SubTeamApplication::find_by_applicant(cli, applicant_pk.clone(), opts).await?;
        for a in items {
            if a.application_id == application_id {
                return Ok(a);
            }
        }
        match next {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }
    Err(SubTeamError::ApplicationNotFound.into())
}

#[cfg(feature = "server")]
async fn list_agreements(
    cli: &aws_sdk_dynamodb::Client,
    parent_pk: &Partition,
    application_id: &str,
) -> Result<Vec<SubTeamDocAgreement>> {
    let prefix = format!("{DOC_AGREEMENT_SK_PREFIX}#{application_id}#");
    let opts = SubTeamDocAgreement::opt().sk(prefix).limit(PAGE_LIMIT);
    let (items, _) = SubTeamDocAgreement::query(cli, parent_pk.clone(), opts).await?;
    Ok(items)
}

// ── GET /api/parent-teams/:parent_team_pk/my-application ──────────
//
// "Given this parent team, find MY application as one of its
// would-be sub-teams." Walks the viewer's admin/owner teams and
// returns the most recent application (Pending / Returned /
// Approved / Rejected) targeting `parent_team_pk`. Used by the
// `/{parent_username}/sub-teams/application` status page so the URL
// stays parent-centric ("status of my application TO this parent")
// while the data is the applicant team's row.
#[get(
    "/api/parent-teams/:parent_team_pk/my-application",
    user: crate::features::auth::User
)]
pub async fn find_my_application_for_parent_handler(
    parent_team_pk: TeamPartition,
) -> Result<Option<SubTeamApplicationResponse>> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let user_pk = user.pk.clone();
    let parent_pk_full: Partition = parent_team_pk.clone().into();
    let parent_uuid = match &parent_pk_full {
        Partition::Team(id) => id.clone(),
        _ => return Ok(None),
    };

    // 1. Enumerate viewer's admin/owner teams (UserTeam rows under
    //    `user_pk` with sk prefix `USER_TEAM#`). Pagination capped —
    //    in practice users belong to 1–2 admin teams.
    let sk_prefix = crate::common::types::EntityType::UserTeam(String::new()).to_string();
    let ut_opts = crate::features::auth::UserTeam::opt()
        .sk(sk_prefix)
        .limit(PAGE_LIMIT);
    let (user_teams, _): (Vec<crate::features::auth::UserTeam>, _) =
        crate::features::auth::UserTeam::query(cli, &user_pk, ut_opts)
            .await
            .map_err(|e| {
                crate::error!("find_my_application_for_parent UserTeam query failed: {e}");
                SubTeamError::ApplicationStateMismatch
            })?;

    // 2. For each admin team, walk its application history (GSI1,
    //    DESC by created_at) and find the first row whose parent
    //    matches. Returning the freshest match overall handles the
    //    edge case where the user has multiple admin teams that all
    //    applied to the same parent.
    let mut best: Option<SubTeamApplication> = None;
    for ut in user_teams {
        if !ut.role.is_admin_or_owner() {
            continue;
        }
        let applicant_pk = match &ut.sk {
            crate::common::types::EntityType::UserTeam(s) => match s.parse::<Partition>() {
                Ok(pk) => pk,
                Err(_) => continue,
            },
            _ => continue,
        };
        let opts = SubTeamApplication::opt()
            .limit(PAGE_LIMIT)
            .scan_index_forward(false);
        let Ok((apps, _)) =
            SubTeamApplication::find_by_applicant(cli, applicant_pk.clone(), opts).await
        else {
            continue;
        };
        for app in apps {
            if app.parent_team_id != parent_uuid {
                continue;
            }
            // Skip applications that have no meaningful "status view".
            // `Draft` was never submitted and `Cancelled` is a closed
            // record (the team left or withdrew). Leaving them in would
            // make the applicant's status page keep showing
            // "Approved · 정식 하위팀으로 등록되었습니다" even after
            // they left the parent team.
            if matches!(
                app.status,
                SubTeamApplicationStatus::Draft | SubTeamApplicationStatus::Cancelled
            ) {
                continue;
            }
            let candidate_ts = app.submitted_at.unwrap_or(app.created_at);
            let beat = match &best {
                Some(b) => {
                    let b_ts = b.submitted_at.unwrap_or(b.created_at);
                    candidate_ts > b_ts
                }
                None => true,
            };
            if beat {
                best = Some(app);
            }
            break;
        }
    }

    let Some(app) = best else { return Ok(None) };

    // 3. Join the applicant + parent teams for display metadata
    //    (avatar / name / @username) so the status page can render
    //    "fddffd → subteam" without extra client fetches.
    let mut dto: SubTeamApplicationResponse = app.clone().into();
    if let Ok(Some(applicant)) =
        Team::get(cli, &app.applicant_team_pk, Some(EntityType::Team)).await
    {
        dto.applicant_team_display_name = applicant.display_name.clone();
        dto.applicant_team_username = applicant.username.clone();
    }
    if let Ok(Some(parent)) = Team::get(cli, &parent_pk_full, Some(EntityType::Team)).await {
        dto.parent_team_display_name = parent.display_name.clone();
        dto.parent_team_username = parent.username.clone();
    }
    Ok(Some(dto))
}

