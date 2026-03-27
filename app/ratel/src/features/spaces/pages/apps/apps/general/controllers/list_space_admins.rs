use crate::common::models::space::SpaceAdmin;
use crate::features::spaces::pages::apps::apps::general::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpaceAdminListItem {
    pub user_id: String,
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
    pub is_owner: bool,
    pub created_at: i64,
}

#[get("/api/spaces/{space_id}/admins", role: SpaceUserRole)]
pub async fn list_space_admins(
    space_id: SpacePartition,
) -> Result<Vec<SpaceAdminListItem>> {
    use crate::common::models::auth::User;
    use crate::common::models::space::SpaceCommon;

    if role != SpaceUserRole::Creator {
        return Err(Error::NoPermission);
    }

    let common_config = crate::common::CommonConfig::default();
    let dynamo = common_config.dynamodb();
    let space_pk: Partition = space_id.into();

    let space = SpaceCommon::get(dynamo, &space_pk, Some(&EntityType::SpaceCommon))
        .await?
        .ok_or(Error::SpaceNotFound)?;

    let mut admins: Vec<SpaceAdminListItem> = Vec::new();

    // Add the space owner as the first admin
    if let Ok(Some(owner)) = User::get(dynamo, space.user_pk.clone(), Some(EntityType::User)).await
    {
        let user_id = match &owner.pk {
            Partition::User(id) => id.clone(),
            _ => owner.pk.to_string(),
        };
        admins.push(SpaceAdminListItem {
            user_id,
            display_name: owner.display_name,
            profile_url: owner.profile_url,
            username: owner.username,
            is_owner: true,
            created_at: owner.created_at,
        });
    }

    // Add explicit Space Admins
    let sk_prefix = EntityType::SpaceAdmin(String::default()).to_string();
    let opt = SpaceAdmin::opt().sk(sk_prefix).limit(100);
    let (space_admins, _) = SpaceAdmin::query(dynamo, &space_pk, opt).await?;

    for sa in space_admins {
        let user_id = match &sa.user_pk {
            Partition::User(id) => id.clone(),
            _ => sa.user_pk.to_string(),
        };
        admins.push(SpaceAdminListItem {
            user_id,
            display_name: sa.display_name,
            profile_url: sa.profile_url,
            username: sa.username,
            is_owner: false,
            created_at: sa.created_at,
        });
    }

    Ok(admins)
}
