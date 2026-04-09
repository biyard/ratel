use crate::common::models::space::SpaceAdmin;
use crate::features::spaces::pages::apps::apps::general::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddSpaceAdminRequest {
    /// Either a username or an email address. The handler decides which
    /// lookup to perform based on the presence of `@`.
    pub target: String,
}

#[post("/api/spaces/{space_id}/admins", role: SpaceUserRole)]
pub async fn add_space_admin(space_id: SpacePartition, body: AddSpaceAdminRequest) -> Result<()> {
    use crate::common::models::auth::User;

    if role != SpaceUserRole::Creator {
        return Err(Error::NoPermission);
    }

    let common_config = crate::common::CommonConfig::default();
    let dynamo = common_config.dynamodb();
    let space_pk: Partition = space_id.into();

    // Load space to check owner
    use crate::common::models::space::SpaceCommon;
    let space = SpaceCommon::get(dynamo, &space_pk, Some(&EntityType::SpaceCommon))
        .await?
        .ok_or(Error::SpaceNotFound)?;

    // Look the user up by email if the input contains `@`, otherwise by
    // username. The Administrators panel allows admins to add new admins
    // by either identifier.
    let identifier = body.target.trim();
    let user = if identifier.contains('@') {
        let email = identifier.to_ascii_lowercase();
        let (users, _) = User::find_by_email(dynamo, &email, User::opt().limit(1)).await?;
        users
            .into_iter()
            .find(|u| u.email.eq_ignore_ascii_case(&email))
            .ok_or(Error::NotFound("User not found".to_string()))?
    } else {
        let (users, _) = User::find_by_username(dynamo, identifier, User::opt().limit(1)).await?;
        users
            .into_iter()
            .find(|u| u.username == identifier)
            .ok_or(Error::NotFound("User not found".to_string()))?
    };

    // Prevent adding the space owner as a Space Admin (they already have Creator role)
    if space.user_pk == user.pk {
        return Ok(());
    }

    // Check if already a Space Admin
    let (pk, sk) = SpaceAdmin::keys(&space_pk, &user.pk);
    if SpaceAdmin::get(dynamo, &pk, Some(&sk)).await?.is_some() {
        return Ok(());
    }

    let space_admin = SpaceAdmin::new(
        space_pk.clone(),
        user.pk.clone(),
        user.display_name.clone(),
        user.username.clone(),
        user.profile_url.clone(),
    );

    space_admin.create(dynamo).await?;

    // Also create SpaceParticipant if not already present (so the space appears in the user's MySpace list)
    use crate::common::models::space::SpaceParticipant;
    let (sp_pk, sp_sk) = SpaceParticipant::keys(space_pk.clone(), user.pk.clone());
    if SpaceParticipant::get(dynamo, &sp_pk, Some(&sp_sk))
        .await?
        .is_none()
    {
        let participant = SpaceParticipant::new_non_anonymous(space_pk.clone(), user);
        participant.create(dynamo).await?;

        SpaceCommon::updater(&space_pk, &EntityType::SpaceCommon)
            .increase_participants(1)
            .execute(dynamo)
            .await?;
    }

    Ok(())
}
