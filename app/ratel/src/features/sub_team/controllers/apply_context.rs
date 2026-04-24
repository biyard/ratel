use crate::common::*;
use crate::features::sub_team::types::{
    ApplyContextDocument, ApplyContextResponse, SubTeamError, SubTeamFormFieldResponse,
};

#[cfg(feature = "server")]
use crate::features::posts::models::Team;
#[cfg(feature = "server")]
use crate::features::sub_team::models::{
    SubTeamApplication, SubTeamApplicationStatus, SubTeamDocument, SubTeamFormField, SubTeamLink,
};

#[cfg(feature = "server")]
const FIELD_SK_PREFIX: &str = "SUB_TEAM_FORM_FIELD";
#[cfg(feature = "server")]
const DOC_SK_PREFIX: &str = "SUB_TEAM_DOCUMENT";
#[cfg(feature = "server")]
const LINK_SK_PREFIX: &str = "SUB_TEAM_LINK";
#[cfg(feature = "server")]
const APPLICATION_SK_PREFIX: &str = "SUB_TEAM_APPLICATION";
#[cfg(feature = "server")]
const PAGE_LIMIT: i32 = 100;
#[cfg(feature = "server")]
const MAX_PAGES: usize = 5;

#[get("/api/teams/:team_pk/sub-teams/apply-context")]
pub async fn get_sub_team_apply_context_handler(
    team_pk: TeamPartition,
) -> Result<ApplyContextResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let parent_pk: Partition = team_pk.into();

    // 1. Load parent team for settings flags.
    let team = Team::get(cli, &parent_pk, Some(EntityType::Team))
        .await
        .map_err(|e| {
            crate::error!("apply_context team load failed: {e}");
            SubTeamError::ApplicationStateMismatch
        })?
        .ok_or(SubTeamError::ParentNotEligible)?;

    // 2. Form fields (ordered by order ASC then created_at ASC).
    let field_opts = SubTeamFormField::opt()
        .sk(FIELD_SK_PREFIX.to_string())
        .limit(PAGE_LIMIT);
    let (mut fields, _) = SubTeamFormField::query(cli, parent_pk.clone(), field_opts)
        .await
        .map_err(|e| {
            crate::error!("apply_context form fields query failed: {e}");
            SubTeamError::FormFieldNotFound
        })?;
    fields.sort_by(|a, b| a.order.cmp(&b.order).then(a.created_at.cmp(&b.created_at)));
    let form_fields: Vec<SubTeamFormFieldResponse> = fields.into_iter().map(Into::into).collect();

    // 3. Required docs only.
    let doc_opts = SubTeamDocument::opt()
        .sk(DOC_SK_PREFIX.to_string())
        .limit(PAGE_LIMIT);
    let (mut docs, _) = SubTeamDocument::query(cli, parent_pk.clone(), doc_opts)
        .await
        .map_err(|e| {
            crate::error!("apply_context docs query failed: {e}");
            SubTeamError::DocumentNotFound
        })?;
    docs.retain(|d| d.required);
    docs.sort_by(|a, b| a.order.cmp(&b.order).then(a.created_at.cmp(&b.created_at)));
    let required_docs: Vec<ApplyContextDocument> = docs.into_iter().map(Into::into).collect();

    // 4. Recognized count — query SubTeamLink rows under parent pk.
    let recognized_count = count_links(cli, &parent_pk).await.map_err(|e| {
        crate::error!("apply_context recognized count failed: {e}");
        SubTeamError::ApplicationStateMismatch
    })?;

    // 5. Pending count — SubTeamApplication rows whose status is Pending.
    let pending_count = count_pending_applications(cli, &parent_pk).await.map_err(|e| {
        crate::error!("apply_context pending count failed: {e}");
        SubTeamError::ApplicationStateMismatch
    })?;

    Ok(ApplyContextResponse {
        is_parent_eligible: team.is_parent_eligible,
        min_sub_team_members: team.min_sub_team_members,
        recognized_count,
        pending_count,
        form_fields,
        required_docs,
    })
}

#[cfg(feature = "server")]
async fn count_links(cli: &aws_sdk_dynamodb::Client, parent_pk: &Partition) -> Result<i64> {
    let mut count: i64 = 0;
    let mut bookmark: Option<String> = None;
    for _ in 0..MAX_PAGES {
        let opts = SubTeamLink::opt_with_bookmark(bookmark.clone())
            .sk(LINK_SK_PREFIX.to_string())
            .limit(PAGE_LIMIT);
        let (items, next) = SubTeamLink::query(cli, parent_pk.clone(), opts).await?;
        count += items.len() as i64;
        match next {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }
    Ok(count)
}

#[cfg(feature = "server")]
async fn count_pending_applications(
    cli: &aws_sdk_dynamodb::Client,
    parent_pk: &Partition,
) -> Result<i64> {
    let mut count: i64 = 0;
    let mut bookmark: Option<String> = None;
    for _ in 0..MAX_PAGES {
        let opts = SubTeamApplication::opt_with_bookmark(bookmark.clone())
            .sk(APPLICATION_SK_PREFIX.to_string())
            .limit(PAGE_LIMIT);
        let (items, next) = SubTeamApplication::query(cli, parent_pk.clone(), opts).await?;
        for a in &items {
            if a.status == SubTeamApplicationStatus::Pending {
                count += 1;
            }
        }
        match next {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }
    Ok(count)
}
