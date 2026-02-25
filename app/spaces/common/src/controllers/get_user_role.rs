use crate::*;

#[get("/api/spaces/{space_id}/user-role", role: SpaceUserRole)]
pub async fn get_user_role(space_id: SpacePartition) -> Result<SpaceUserRole> {
    debug!("space_id: {:#?}, role: {:#?}", space_id, role);
    return Ok(role);
}
