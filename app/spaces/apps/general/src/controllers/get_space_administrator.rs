use crate::*;
#[cfg(feature = "server")]
use ratel_auth::User;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpaceAdministratorResponse {
    pub user_id: String,
    pub username: String,
    pub display_name: String,
    pub profile_url: String,
}

#[get("/api/spaces/{space_id}/administrator", user: User)]
pub async fn get_space_administrator(
    space_id: SpacePartition,
) -> common::Result<SpaceAdministratorResponse> {
    use common::types::Partition;

    let dynamo = crate::config::get().common.dynamodb();
    let space = super::get_space_and_ensure_admin(&space_id, &user).await?;

    // TODO: Replace this with an actual team admin lookup.
    let admin_pk = match &space.user_pk {
        Partition::User(user_id) => Partition::User(user_id.clone()),
        _ => return Err(Error::NotFound("Space admin user not found".to_string())),
    };

    let admin = User::get(dynamo, admin_pk, Some(EntityType::User))
        .await?
        .ok_or_else(|| Error::NotFound("Space admin user not found".to_string()))?;

    let user_id = match admin.pk {
        Partition::User(id) => id,
        _ => admin.pk.to_string(),
    };

    Ok(SpaceAdministratorResponse {
        user_id,
        username: admin.username,
        display_name: admin.display_name,
        profile_url: admin.profile_url,
    })
}
