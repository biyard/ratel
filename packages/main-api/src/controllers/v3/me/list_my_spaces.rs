use crate::{
    features::spaces::{
        SpaceParticipant, SpaceParticipantQueryOption,
        members::{SpaceInvitationMember, SpaceInvitationMemberQueryOption},
    },
    models::SpaceCommon,
};

use super::*;

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(tag = "invitation_status", rename_all = "snake_case")]
pub enum MySpace {
    Pending { spaces: SpaceCommon },
    Participating { spaces: SpaceCommon },
}

pub async fn list_my_spaces_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Query(Pagination { mut bookmark }): ListItemsQuery,
) -> Result<Json<ListItemsResponse<MySpace>>> {
    let mut items = Vec::new();
    let mut limit = 10;

    if should_list_invited_spaces(&bookmark) {
        let opt = SpaceInvitationMember::opt_with_bookmark(bookmark).limit(limit);

        let (invited_spaces, bm) = list_invited_spaces(&dynamo.client, &user.pk, opt).await?;
        items.extend(
            invited_spaces
                .into_iter()
                .map(|space| MySpace::Pending { spaces: space }),
        );

        if let Some(b) = &bm {
            bookmark = Some(b.clone());
        } else if bm.is_none() && items.len() == 10 {
            bookmark = Some("SP-".to_string());

            return Ok(Json(ListItemsResponse { items, bookmark }));
        } else {
            // bookmark.is_none && items.len() < 10
            limit = limit - items.len() as i32;
            bookmark = Some("SP-".to_string());
        }
    }

    // NOTE: Continue listing participating spaces if we still have limit
    let bookmark = if let Some(mut bm) = bookmark {
        bm.truncate(3); // "SP-" -> ""
        if bm.is_empty() { None } else { Some(bm) }
    } else {
        return Err(Error::InvalidBookmark);
    };

    let (participating_spaces, bookmark) = list_participating_spaces(
        &dynamo.client,
        &user.pk,
        SpaceParticipant::opt_with_bookmark(bookmark).limit(limit),
    )
    .await?;

    items.extend(
        participating_spaces
            .into_iter()
            .map(|space| MySpace::Participating { spaces: space }),
    );

    let bookmark = if let Some(b) = &bookmark {
        Some(format!("SP-{}", b))
    } else {
        None
    };

    Ok(Json(ListItemsResponse { items, bookmark }))
}

pub fn should_list_invited_spaces(bookmark: &Option<String>) -> bool {
    if let Some(b) = bookmark {
        !b.starts_with("SP-")
    } else {
        true
    }
}

pub async fn list_participating_spaces(
    cli: &aws_sdk_dynamodb::Client,
    user_pk: &Partition,
    opt: SpaceParticipantQueryOption,
) -> Result<(Vec<SpaceCommon>, Option<String>)> {
    let (sps, bookmark) = SpaceParticipant::find_by_user(cli, user_pk, opt).await?;

    let keys = sps
        .into_iter()
        .map(|sp| (sp.space_pk, EntityType::SpaceCommon))
        .collect::<Vec<(Partition, EntityType)>>();

    Ok((SpaceCommon::batch_get(cli, keys).await?, bookmark))
}

pub async fn list_invited_spaces(
    cli: &aws_sdk_dynamodb::Client,
    user_pk: &Partition,
    opt: SpaceInvitationMemberQueryOption,
) -> Result<(Vec<SpaceCommon>, Option<String>)> {
    let (si, mut bookmark) = SpaceInvitationMember::find_user_invitations_by_status(
        cli,
        user_pk,
        SpaceInvitationMember::opt().limit(10),
    )
    .await?;

    let keys = si
        .into_iter()
        .map(|sp| (sp.space_pk, EntityType::SpaceCommon))
        .collect::<Vec<(Partition, EntityType)>>();

    Ok((SpaceCommon::batch_get(cli, keys).await?, bookmark))
}
