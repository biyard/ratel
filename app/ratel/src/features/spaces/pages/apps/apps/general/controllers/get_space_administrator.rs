use crate::features::spaces::pages::apps::apps::general::*;
#[cfg(feature = "server")]
use crate::common::SpaceUserRole;
#[cfg(feature = "server")]
use crate::features::auth::User;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpaceAdministratorResponse {
    pub user_id: String,
    pub username: String,
    pub display_name: String,
    pub profile_url: String,
}

#[get("/api/spaces/{space_id}/administrator", role: SpaceUserRole)]
pub async fn get_space_administrator(
    space_id: SpacePartition,
) -> crate::common::Result<SpaceAdministratorResponse> {
    use crate::common::models::space::SpaceCommon;
    use crate::common::types::Partition;

    if role != SpaceUserRole::Creator {
        return Err(Error::NoPermission);
    }

    let common_config = crate::common::CommonConfig::default();
    let dynamo = common_config.dynamodb();
    let space_pk: Partition = space_id.into();
    let space = SpaceCommon::get(dynamo, &space_pk, Some(&EntityType::SpaceCommon))
        .await?
        .ok_or(Error::SpaceNotFound)?;

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
