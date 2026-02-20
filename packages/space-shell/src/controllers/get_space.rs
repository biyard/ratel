use crate::models::*;
use crate::*;
use common::types::{Partition, SpacePartition};
use ratel_auth::models::user::{OptionalUser, User};
use ratel_post::models::Post;

#[cfg(feature = "server")]
use async_trait::async_trait;
#[cfg(feature = "server")]
use common::axum::{
    extract::{FromRequestParts, Path},
    http::request::Parts,
};

struct GetSpaceRequest {
    user: OptionalUser,
}

#[get("/api/spaces/{space_pk}", user: OptionalUser)]
pub async fn get_space(space_pk: SpacePartition) -> Result<dto::GetSpaceResponse> {
    let config = crate::config::get();
    let dynamo = config.common.dynamodb();

    let space_pk_partition: Partition = space_pk.clone().into();
    let space =
        SpaceCommon::get(dynamo, &space_pk_partition, Some(&EntityType::SpaceCommon)).await?;
    let space = space.ok_or_else(|| Error::NotFound("Space Not Found".to_string()))?;

    let post_pk = space.pk.clone().to_post_key()?;
    let post = Post::get(dynamo, &post_pk, Some(EntityType::Post)).await?;
    let post = post.ok_or_else(|| Error::NotFound("Post Not Found".to_string()))?;
    let user: Option<ratel_auth::User> = user.into();

    let permissions = post.get_permissions(dynamo, user.clone()).await?;

    let (user_participant, can_participate) = if let Some(ref user) = user {
        let (participant_pk, participant_sk) =
            SpaceParticipant::keys(space.pk.clone(), user.pk.clone());
        let participant =
            SpaceParticipant::get(dynamo, &participant_pk, Some(&participant_sk)).await?;
        let can_participate = participant.is_none();
        (participant, can_participate)
    } else {
        (None, false)
    };

    let (participated, participant_display_name, participant_profile_url, participant_username) =
        if let Some(participant) = user_participant {
            (
                true,
                Some(participant.display_name),
                Some(participant.profile_url),
                Some(participant.username),
            )
        } else {
            (false, None, None, None)
        };

    Ok(dto::GetSpaceResponse {
        pk: space.pk.clone(),
        sk: space.sk,
        title: post.title,
        content: if space.content.is_empty() {
            post.html_contents
        } else {
            space.content
        },
        created_at: space.created_at,
        updated_at: space.updated_at,
        urls: post.urls,
        space_type: space.space_type,
        features: vec![],
        status: space.status,
        permissions: permissions.into(),
        author_type: post.author_type,
        author_display_name: post.author_display_name,
        author_username: post.author_username,
        author_profile_url: post.author_profile_url,
        certified: false,
        likes: post.likes,
        comments: post.comments,
        shares: post.shares,
        reports: space.reports,
        rewards: space.rewards,
        visibility: space.visibility,
        publish_state: space.publish_state,
        booster: space.booster,
        files: space.files,
        anonymous_participation: space.anonymous_participation,
        can_participate,
        participated,
        participant_display_name,
        participant_profile_url,
        participant_username,
        remains: space.remains,
        quota: space.quota,
        is_report: false,
    })
}
