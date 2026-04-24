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

    Ok(ParentRelationshipResponse {
        status,
        parent_team_id: team.parent_team_id,
        pending_parent_team_id: team.pending_parent_team_id,
        latest_application_id,
    })
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

    let items: Vec<SubTeamApplicationResponse> = items.into_iter().map(Into::into).collect();
    Ok((items, next).into())
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

    // 3. No in-flight application under this applicant team.
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

    let now = crate::common::utils::time::get_now_timestamp_millis();
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
    let opts = SubTeamDocument::opt()
        .sk(DOC_SK_PREFIX.to_string())
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

