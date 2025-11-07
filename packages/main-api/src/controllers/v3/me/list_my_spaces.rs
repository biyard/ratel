use crate::{
    features::spaces::{SpaceParticipant, members::SpaceInvitationMember},
    models::SpaceCommon,
};

use super::*;

pub async fn list_my_spaces_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Query(Pagination { bookmark }): ListItemsQuery,
) -> Result<Json<ListItemsResponse<SpaceCommon>>> {
    let (sps, bookmark) = SpaceParticipant::find_by_user(
        &dynamo.client,
        &user.pk,
        SpaceParticipant::opt_with_bookmark(bookmark).limit(10),
    )
    .await?;

    let keys = sps
        .into_iter()
        .map(|sp| (sp.space_pk, EntityType::SpaceCommon))
        .collect::<Vec<(Partition, EntityType)>>();

    let items = SpaceCommon::batch_get(&dynamo.client, keys).await?;

    Ok(Json(ListItemsResponse { items, bookmark }))
}
