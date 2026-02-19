use dioxus::prelude::*;
use common::{EntityType, Partition, SpaceUserRole};
#[cfg(feature = "server")]
use ratel_auth::models::user::SESSION_KEY_USER_ID;
#[cfg(not(feature = "server"))]
const SESSION_KEY_USER_ID: &str = "user_id";

#[get(
    "/api/spaces/:space_id/user-role",
    session: common::Extension<common::models::TowerSession>
)]
pub async fn get_user_role_in_space(space_id: String) -> Result<SpaceUserRole, ServerFnError> {
    let common::Extension(session) = session;
    let cli = crate::config::get().dynamodb();
    let space_pk = Partition::Space(space_id);

    let space = crate::models::SpaceCommonRef::get(cli, &space_pk, Some(EntityType::SpaceCommon))
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to query space common: {e}")))?
        .ok_or_else(|| ServerFnError::new("Space not found".to_string()))?;

    let user_pk: Option<Partition> = session
        .get(SESSION_KEY_USER_ID)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to read session user: {e}")))?;

    let Some(user_pk) = user_pk else {
        return Ok(SpaceUserRole::Viewer);
    };

    let is_creator = match &space.user_pk {
        Partition::User(_) => space.user_pk == user_pk,
        Partition::Team(_) => crate::models::is_team_admin(cli, &space.user_pk, &user_pk)
            .await
            .map_err(|e| {
                ServerFnError::new(format!("Failed to check team admin permission: {e}"))
            })?,
        _ => false,
    };

    Ok(if is_creator {
        SpaceUserRole::Creator
    } else {
        SpaceUserRole::Viewer
    })
}

#[get("/api/spaces/:space_id/dashboard-extensions")]
pub async fn fetch_dashboard_extensions(
    space_id: String,
) -> Result<Vec<crate::types::DashboardExtension>, ServerFnError> {
    let pk = common::Partition::Space(space_id);
    let sk_prefix = common::EntityType::SpaceDashboardExtension(String::new()).to_string();

    let cli = crate::config::get().dynamodb();
    let (items, _) = crate::models::DashboardExtensionEntity::query(
        cli,
        pk,
        crate::models::DashboardExtensionEntity::opt_all().sk(sk_prefix),
    )
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to query dashboard extensions: {e}")))?;

    let mut extensions: Vec<crate::types::DashboardExtension> = items
        .into_iter()
        .map(|item| {
            serde_json::from_str::<crate::types::DashboardExtension>(&item.data)
                .map_err(|e| ServerFnError::new(format!("Failed to parse dashboard extension data: {e}")))
        })
        .collect::<std::result::Result<_, _>>()?;

    extensions.sort_by_key(|ext| ext.order());
    Ok(extensions)
}
