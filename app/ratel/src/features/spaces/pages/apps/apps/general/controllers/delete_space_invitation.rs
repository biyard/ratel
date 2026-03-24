use crate::features::spaces::pages::apps::apps::general::*;
use crate::spaces::SpaceInvitationMember;

#[delete(
    "/api/spaces/{space_id}/participants/invitations/{user_id}",
    role: SpaceUserRole
)]
pub async fn delete_space_invitation(
    space_id: SpacePartition,
    user_id: UserPartition,
) -> Result<()> {
    if role != SpaceUserRole::Creator {
        return Err(Error::NoPermission);
    }

    let common_config = crate::common::CommonConfig::default();
    let dynamo = common_config.dynamodb();
    let space_pk: Partition = space_id.into();
    let user_pk: Partition = user_id.into();
    let (pk, sk) = SpaceInvitationMember::keys(&space_pk, &user_pk);

    SpaceInvitationMember::get(dynamo, &pk, Some(&sk))
        .await?
        .ok_or(Error::InvitationNotFound)?;

    SpaceInvitationMember::delete(dynamo, &pk, Some(sk)).await?;

    Ok(())
}
