use crate::*;
use common::SpaceUserRole;

#[get("/api/spaces/{space_id}/apps/access", role: SpaceUserRole)]
pub async fn get_apps_access(space_id: SpacePartition) -> Result<bool> {
    let _ = space_id;
    Ok(role == SpaceUserRole::Creator)
}

pub(super) fn ensure_space_admin(role: SpaceUserRole) -> Result<()> {
    if role != SpaceUserRole::Creator {
        return Err(Error::NoPermission);
    }

    Ok(())
}
