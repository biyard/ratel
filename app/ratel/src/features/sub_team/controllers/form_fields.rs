use crate::common::*;
use crate::features::posts::models::Team;
use crate::features::social::pages::member::dto::TeamRole;
use crate::features::sub_team::models::{SubTeamFormField, SubTeamFormFieldType};
use crate::features::sub_team::types::{
    CreateSubTeamFormFieldRequest, ReorderFormFieldsRequest, SubTeamError, SubTeamFormFieldResponse,
    TeamProfileLink, UpdateSubTeamFormFieldRequest,
};

const FORM_FIELD_SK_PREFIX: &str = "SUB_TEAM_FORM_FIELD";
const LIST_PAGE_LIMIT: i32 = 100;

fn sort_fields(items: &mut [SubTeamFormField]) {
    // Locked defaults (팀 이름 / 설립 목적) always render before any
    // custom fields, regardless of their stored `order` value, so
    // admins can't accidentally bury them with a higher-priority
    // row. Among locked rows we still respect `order`, and ditto for
    // the unlocked tail.
    items.sort_by(|a, b| {
        b.locked
            .cmp(&a.locked)
            .then(a.order.cmp(&b.order))
            .then(a.created_at.cmp(&b.created_at))
    });
}

// ── GET list ─────────────────────────────────────────────────────
//
// First-call seeding: if the team's form is empty we lazily lay down
// two `locked: true` defaults — "팀 이름" linked to `team.display_name`
// and "설립 목적" linked to `team.description`. These are baseline
// fields every sub-team application needs, so we hard-code them
// rather than asking the admin to recreate them by hand. Subsequent
// calls return them just like any other row.
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
    let (mut items, next) = SubTeamFormField::query(cli, parent_pk.clone(), opts)
        .await
        .map_err(|e| {
            crate::error!("list_sub_team_form_fields query failed: {e}");
            SubTeamError::FormFieldNotFound
        })?;

    // Backfill any missing locked defaults — runs on every list call
    // so a team that pre-dates the seeding logic still ends up with
    // both rows. Each default is gated on its own `linked_to` so we
    // never duplicate a row that already exists.
    let seeded = ensure_default_fields(cli, &parent_pk, &items).await;
    items.extend(seeded);

    sort_fields(&mut items);
    let items: Vec<SubTeamFormFieldResponse> = items.into_iter().map(Into::into).collect();
    Ok((items, next).into())
}

#[cfg(feature = "server")]
async fn ensure_default_fields(
    cli: &aws_sdk_dynamodb::Client,
    parent_pk: &Partition,
    existing: &[SubTeamFormField],
) -> Vec<SubTeamFormField> {
    let defaults: [(&str, SubTeamFormFieldType, TeamProfileLink, i32); 2] = [
        (
            "제안하는 팀 이름",
            SubTeamFormFieldType::ShortText,
            TeamProfileLink::TeamName,
            0,
        ),
        (
            "설립 목적",
            SubTeamFormFieldType::LongText,
            TeamProfileLink::TeamBio,
            1,
        ),
    ];
    let mut out: Vec<SubTeamFormField> = Vec::new();
    for (label, field_type, link, order) in defaults {
        // Skip if a locked row with the same link already exists.
        if existing
            .iter()
            .any(|f| f.locked && f.linked_to == Some(link))
        {
            continue;
        }
        let field = SubTeamFormField::new_with_lock(
            parent_pk.clone(),
            label.to_string(),
            field_type,
            true,
            order,
            Vec::new(),
            Some(link),
            true,
        );
        if let Err(e) = field.create(cli).await {
            crate::error!("ensure_default_fields create failed: {e}");
            continue;
        }
        out.push(field);
    }
    out
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
        body.linked_to,
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

    // Locked defaults are immutable from the admin form-builder. The
    // controller still loads the row so we can return a clean error.
    if existing.locked {
        return Err(Error::UnauthorizedAccess);
    }

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
    if let Some(linked_to) = body.linked_to {
        updater = updater.with_linked_to(linked_to);
        updated.linked_to = Some(linked_to);
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

    // Refuse to delete locked defaults — admins should never lose
    // the seed rows (team name / 설립 목적).
    if let Ok(Some(existing)) =
        SubTeamFormField::get(cli, &team.pk, Some(sk.clone())).await
    {
        if existing.locked {
            return Err(Error::UnauthorizedAccess);
        }
    }

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
