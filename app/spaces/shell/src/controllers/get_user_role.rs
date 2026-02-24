use crate::*;

#[get("/api/spaces/{space_pk}/user-role", role: SpaceUserRole)]
pub async fn get_user_role(space_pk: SpacePartition) -> Result<SpaceUserRole> {
    return Ok(role);
}
