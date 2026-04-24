use crate::common::*;
use crate::features::posts::models::Team;
use crate::features::social::pages::member::dto::TeamRole;
use crate::features::sub_team::models::SubTeamFormField;
use crate::features::sub_team::types::{
    CreateSubTeamFormFieldRequest, ReorderFormFieldsRequest, SubTeamError, SubTeamFormFieldResponse,
    UpdateSubTeamFormFieldRequest,
};

const FORM_FIELD_SK_PREFIX: &str = "SUB_TEAM_FORM_FIELD";
const LIST_PAGE_LIMIT: i32 = 100;

fn sort_fields(items: &mut [SubTeamFormField]) {
    items.sort_by(|a, b| a.order.cmp(&b.order).then(a.created_at.cmp(&b.created_at)));
}

// ── GET list ─────────────────────────────────────────────────────
#[get("/api/teams/:team_pk/sub-teams/form-fields", user: crate::features::auth::User)]
pub async fn list_sub_team_form_fields_handler(
    team_pk: TeamPartition,
) -> Result<ListResponse<SubTeamFormFieldResponse>> {
    let _ = user;
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let parent_pk: Partition = team_pk.into();

    let opts = SubTeamFormField::opt()
        .sk(FORM_FIELD_SK_PREFIX.to_string())
        .limit(LIST_PAGE_LIMIT);
    let (mut items, next) =
        SubTeamFormField::query(cli, parent_pk, opts).await.map_err(|e| {
            crate::error!("list_sub_team_form_fields query failed: {e}");
            SubTeamError::FormFieldNotFound
        })?;
    sort_fields(&mut items);
    let items: Vec<SubTeamFormFieldResponse> = items.into_iter().map(Into::into).collect();
    Ok((items, next).into())
}

// ── POST create ─────────────────────────────────────────────────
#[post("/api/teams/:team_pk/sub-teams/form-fields", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn create_sub_team_form_field_handler(
    team_pk: TeamPartition,
    body: CreateSubTeamFormFieldRequest,
) -> Result<SubTeamFormFieldResponse> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let order = body.order.unwrap_or(0);
    let options = body.options.unwrap_or_default();

    let field = SubTeamFormField::new(
        team.pk.clone(),
        body.label,
        body.field_type,
        body.required,
        order,
        options,
    );

    field.create(cli).await.map_err(|e| {
        crate::error!("create_sub_team_form_field execute failed: {e}");
        SubTeamError::FormFieldNotFound
    })?;

    Ok(field.into())
}

// ── PATCH update ────────────────────────────────────────────────
#[patch("/api/teams/:team_pk/sub-teams/form-fields/:field_id", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn update_sub_team_form_field_handler(
    team_pk: TeamPartition,
    field_id: String,
    body: UpdateSubTeamFormFieldRequest,
) -> Result<SubTeamFormFieldResponse> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let sk = EntityType::SubTeamFormField(field_id);

    let existing = SubTeamFormField::get(cli, &team.pk, Some(sk.clone()))
        .await
        .map_err(|e| {
            crate::error!("update_sub_team_form_field get failed: {e}");
            SubTeamError::FormFieldNotFound
        })?
        .ok_or(SubTeamError::FormFieldNotFound)?;

    let mut updated = existing;
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let mut updater = SubTeamFormField::updater(&team.pk, &sk).with_updated_at(now);
    updated.updated_at = now;
    let mut changed = false;

    if let Some(label) = body.label {
        updater = updater.with_label(label.clone());
        updated.label = label;
        changed = true;
    }
    if let Some(field_type) = body.field_type {
        updater = updater.with_field_type(field_type);
        updated.field_type = field_type;
        changed = true;
    }
    if let Some(required) = body.required {
        updater = updater.with_required(required);
        updated.required = required;
        changed = true;
    }
    if let Some(order) = body.order {
        updater = updater.with_order(order);
        updated.order = order;
        changed = true;
    }
    if let Some(options) = body.options {
        updater = updater.with_options(options.clone());
        updated.options = options;
        changed = true;
    }

    if changed {
        updater.execute(cli).await.map_err(|e| {
            crate::error!("update_sub_team_form_field execute failed: {e}");
            SubTeamError::FormFieldNotFound
        })?;
    }

    Ok(updated.into())
}

// ── DELETE ──────────────────────────────────────────────────────
#[delete("/api/teams/:team_pk/sub-teams/form-fields/:field_id", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn delete_sub_team_form_field_handler(
    team_pk: TeamPartition,
    field_id: String,
) -> Result<String> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let sk = EntityType::SubTeamFormField(field_id);

    SubTeamFormField::delete(cli, &team.pk, Some(sk))
        .await
        .map_err(|e| {
            crate::error!("delete_sub_team_form_field execute failed: {e}");
            SubTeamError::FormFieldNotFound
        })?;

    Ok(String::new())
}

// ── POST reorder ────────────────────────────────────────────────
#[post("/api/teams/:team_pk/sub-teams/form-fields/reorder", user: crate::features::auth::User, team: Team, role: TeamRole)]
pub async fn reorder_sub_team_form_fields_handler(
    team_pk: TeamPartition,
    body: ReorderFormFieldsRequest,
) -> Result<String> {
    let _ = team_pk;
    let _ = user;
    if !role.is_admin_or_owner() {
        return Err(Error::UnauthorizedAccess);
    }
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let now = crate::common::utils::time::get_now_timestamp_millis();

    for (idx, field_id) in body.field_ids.iter().enumerate() {
        let sk = EntityType::SubTeamFormField(field_id.clone());
        // Silently skip non-existent ids per spec.
        let existing = SubTeamFormField::get(cli, &team.pk, Some(sk.clone()))
            .await
            .ok()
            .flatten();
        if existing.is_none() {
            continue;
        }
        let _ = SubTeamFormField::updater(&team.pk, &sk)
            .with_order(idx as i32)
            .with_updated_at(now)
            .execute(cli)
            .await
            .map_err(|e| {
                crate::error!("reorder_sub_team_form_fields per-row failed: {e}");
                SubTeamError::FormFieldNotFound
            });
    }

    Ok(String::new())
}
