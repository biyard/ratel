use std::collections::HashMap;

use crate::{
    features::spaces::{
        SpaceParticipant, SpaceParticipantQueryOption,
        members::{InvitationStatus, SpaceInvitationMember, SpaceInvitationMemberQueryOption},
    },
    models::{Post, SpaceCommon},
};

use super::*;

use crate::*;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct GetSpaceResponse {
    #[serde(flatten)]
    pub space_common: SpaceCommon,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(tag = "invitation_status", rename_all = "snake_case")]
pub enum MySpace {
    Pending(GetSpaceResponse),
    Participating(GetSpaceResponse),
}

pub async fn list_my_spaces_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Query(Pagination { mut bookmark }): ListItemsQuery,
) -> Result<Json<ListItemsResponse<MySpace>>> {
    let mut items = Vec::new();
    let mut limit = 10;

    if should_list_invited_spaces(&bookmark) {
        let status = InvitationStatus::Invited.to_string();
        tracing::debug!("Listing invited spaces with status: {:?}", status);
        let opt = SpaceInvitationMember::opt_with_bookmark(bookmark)
            .limit(limit)
            .sk(status);

        let (invited_spaces, posts, bm) =
            list_invited_spaces(&dynamo.client, &user.pk, opt).await?;

        let post_titles: HashMap<String, String> = posts
            .iter()
            .map(|p| (p.pk.to_string(), p.title.clone()))
            .collect();

        items.extend(invited_spaces.into_iter().map(|space| {
            let title = space
                .clone()
                .pk
                .to_post_key()
                .ok()
                .and_then(|post_pk| post_titles.get(&post_pk.to_string()).cloned())
                .unwrap_or_default();

            MySpace::Pending(GetSpaceResponse {
                space_common: space,
                title,
            })
        }));
        tracing::info!("Listed invited spaces, total items: {}", items.len());

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
    let bookmark = if let Some(bm) = bookmark {
        let bm = bm.replacen("SP-", "", 1); // "SP-" -> ""
        tracing::debug!(
            "Continuing listing participating spaces with bookmark: {:?}",
            bm
        );
        if bm.is_empty() { None } else { Some(bm) }
    } else {
        return Err(Error::InvalidBookmark);
    };
    tracing::debug!("Listing participating spaces with bookmark: {:?}", bookmark);

    let (participating_spaces, posts, bookmark) = list_participating_spaces(
        &dynamo.client,
        &user.pk,
        SpaceParticipant::opt_with_bookmark(bookmark).limit(limit),
    )
    .await?;

    let post_titles: HashMap<String, String> = posts
        .iter()
        .map(|p| (p.pk.to_string(), p.title.clone()))
        .collect();

    items.extend(participating_spaces.clone().into_iter().map(|space| {
        let title = space
            .clone()
            .pk
            .to_post_key()
            .ok()
            .and_then(|post_pk| post_titles.get(&post_pk.to_string()).cloned())
            .unwrap_or_default();

        MySpace::Participating(GetSpaceResponse {
            space_common: space,
            title,
        })
    }));

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
) -> Result<(Vec<SpaceCommon>, Vec<Post>, Option<String>)> {
    let (sps, bookmark) = match SpaceParticipant::find_by_user(cli, user_pk, opt).await {
        Ok(v) => (v.0, v.1),
        Err(e) => {
            tracing::error!("not exists error: {:?}", e);
            (vec![], None)
        }
    };

    let keys = sps
        .into_iter()
        .map(|sp| (sp.space_pk, EntityType::SpaceCommon))
        .collect::<Vec<(Partition, EntityType)>>();

    let spaces: Vec<SpaceCommon> = SpaceCommon::batch_get(cli, keys.clone()).await?;

    let post_keys: Vec<(Partition, EntityType)> = keys
        .iter()
        .filter_map(|(raw_pk, _)| raw_pk.clone().to_post_key().ok())
        .map(|pk| (pk, EntityType::Post))
        .collect();

    let posts: Vec<Post> = if post_keys.is_empty() {
        Vec::new()
    } else {
        Post::batch_get(cli, post_keys).await?
    };

    Ok((spaces, posts, bookmark))
}

pub async fn list_invited_spaces(
    cli: &aws_sdk_dynamodb::Client,
    user_pk: &Partition,
    opt: SpaceInvitationMemberQueryOption,
) -> Result<(Vec<SpaceCommon>, Vec<Post>, Option<String>)> {
    let (si, bookmark) =
        SpaceInvitationMember::find_user_invitations_by_status(cli, user_pk, opt).await?;

    let space_keys: Vec<(Partition, EntityType)> = si
        .iter()
        .map(|sp| (sp.pk.clone(), EntityType::SpaceCommon))
        .collect();

    let spaces: Vec<SpaceCommon> = SpaceCommon::batch_get(cli, space_keys.clone()).await?;

    let post_keys: Vec<(Partition, EntityType)> = space_keys
        .iter()
        .filter_map(|(raw_pk, _)| raw_pk.clone().to_post_key().ok())
        .map(|pk| (pk, EntityType::Post))
        .collect();

    let posts: Vec<Post> = if post_keys.is_empty() {
        Vec::new()
    } else {
        Post::batch_get(cli, post_keys).await?
    };

    Ok((spaces, posts, bookmark))
}
