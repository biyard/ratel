#[cfg(feature = "server")]
use crate::features::spaces::space_common::models::SpaceInvitationMember;
use crate::features::spaces::space_common::*;
#[cfg(feature = "server")]
use crate::common::SpaceUserRole;
#[cfg(feature = "server")]
use crate::common::models::space::SpaceCommon;
#[cfg(feature = "server")]
use crate::features::posts::models::Post;
#[cfg(feature = "server")]
use crate::features::posts::types::PostStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSpaceRequest {
    Publish {
        publish: bool,
        visibility: SpaceVisibility,
    },
    Content {
        content: String,
    },
    Title {
        title: String,
    },
    Visibility {
        visibility: SpaceVisibility,
    },
    Anonymous {
        anonymous_participation: bool,
    },
    ChangeVisibility {
        change_visibility: bool,
    },
    Start {
        start: bool,
    },
    Finish {
        finished: bool,
    },
    Quota {
        quotas: i64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateSpaceResponse {
    pub pk: Partition,
    pub sk: EntityType,
    pub created_at: i64,
    pub updated_at: i64,
    pub status: Option<SpaceStatus>,
    pub publish_state: SpacePublishState,
    pub visibility: SpaceVisibility,
    pub content: String,
    pub anonymous_participation: bool,
    pub quota: i64,
    pub remains: i64,
}

#[cfg(feature = "server")]
impl From<SpaceCommon> for UpdateSpaceResponse {
    fn from(s: SpaceCommon) -> Self {
        Self {
            pk: s.pk,
            sk: s.sk,
            created_at: s.created_at,
            updated_at: s.updated_at,
            status: s.status,
            publish_state: s.publish_state,
            visibility: s.visibility,
            content: s.content,
            anonymous_participation: s.anonymous_participation,
            quota: s.quota,
            remains: s.remains,
        }
    }
}

#[patch("/api/spaces/{space_id}", role: SpaceUserRole, space: SpaceCommon)]
pub async fn update_space(
    space_id: SpacePartition,
    req: UpdateSpaceRequest,
) -> Result<UpdateSpaceResponse> {
    if role != SpaceUserRole::Creator {
        return Err(Error::NoPermission);
    }

    let conf = ServerConfig::default();
    let dynamo = conf.dynamodb();
    let ses = conf.ses();

    let space_pk: Partition = space_id.into();

    let now = chrono::Utc::now().timestamp_millis();
    let mut su = SpaceCommon::updater(&space.pk, &space.sk).with_updated_at(now);
    let mut pu: Option<_> = None;
    let mut should_send_invitation = false;
    let mut updated_space = space.clone();

    match req {
        UpdateSpaceRequest::Publish {
            publish,
            visibility,
        } => {
            let post_pk = space_pk.clone().to_post_key()?;

            if !publish {
                return Err(Error::BadRequest(
                    "it does not support unpublished now".into(),
                ));
            }

            su = su
                .with_publish_state(SpacePublishState::Published)
                .with_status(SpaceStatus::InProgress)
                .with_visibility(visibility.clone());

            let mut post_updater = Post::updater(post_pk, EntityType::Post).with_updated_at(now);

            post_updater = post_updater
                .with_space_visibility(visibility.clone())
                .with_visibility(visibility.clone().into())
                .with_status(PostStatus::Published);

            pu = Some(post_updater);
            should_send_invitation = true;

            updated_space.publish_state = SpacePublishState::Published;
            updated_space.visibility = visibility;
        }
        UpdateSpaceRequest::Visibility { visibility } => {
            su = su.with_visibility(visibility.clone());

            updated_space.visibility = visibility;
        }
        UpdateSpaceRequest::Content { content } => {
            su = su.with_content(content.clone());

            updated_space.content = content;
        }
        UpdateSpaceRequest::Title { title } => {
            let post_pk = space_pk.clone().to_post_key()?;
            let mut post_updater = Post::updater(post_pk, EntityType::Post).with_updated_at(now);

            post_updater = post_updater.with_title(title);
            pu = Some(post_updater);
        }
        UpdateSpaceRequest::Start { start } => {
            use crate::features::spaces::space_common::models::SpaceEmailVerification;

            if updated_space.status != Some(SpaceStatus::InProgress) {
                return Err(Error::BadRequest(
                    "Start is not available for the current status.".into(),
                ));
            }

            if !start {
                return Err(Error::BadRequest("it does not support start now".into()));
            }

            su = su.with_status(SpaceStatus::Started);

            updated_space.status = Some(SpaceStatus::Started);

            let _ = SpaceEmailVerification::expire_verifications(dynamo, space_pk.clone()).await?;
        }
        UpdateSpaceRequest::Finish { finished } => {
            if updated_space.status != Some(SpaceStatus::Started) {
                return Err(Error::BadRequest(
                    "Finish is not available for the current status.".into(),
                ));
            }

            if !finished {
                return Err(Error::BadRequest("it does not support finish now".into()));
            }

            su = su.with_status(SpaceStatus::Finished);

            updated_space.status = Some(SpaceStatus::Finished);
        }
        UpdateSpaceRequest::Anonymous {
            anonymous_participation,
        } => {
            su = su.with_anonymous_participation(anonymous_participation);

            updated_space.anonymous_participation = anonymous_participation;
        }
        UpdateSpaceRequest::ChangeVisibility { .. } => {
            tracing::error!("ChangeVisibility is deprecated");
            return Err(Error::InternalServerError(
                "ChangeVisibility is deprecated".to_string(),
            ));
        }
        UpdateSpaceRequest::Quota { quotas } => {
            let remains = updated_space.remains + (quotas - updated_space.quota);

            if remains < 0 {
                return Err(Error::BadRequest("Invalid panel quota".into()));
            }

            su = su.with_quota(quotas).with_remains(remains);

            updated_space.quota = quotas;
            updated_space.remains = remains;
        }
    }

    if let Some(pu) = pu {
        crate::transact_write!(dynamo, su.transact_write_item(), pu.transact_write_item())?;
    } else {
        su.execute(dynamo).await?;
    }

    if should_send_invitation {
        let post_pk = space_pk.clone().to_post_key()?;
        let post = Post::get(dynamo, &post_pk, Some(&EntityType::Post))
            .await?
            .ok_or_else(|| Error::InternalServerError("Failed to get post".to_string()))?;

        let ses = conf.ses();
        SpaceInvitationMember::send_email(dynamo, ses, &updated_space, post.title).await?;
    }

    Ok(UpdateSpaceResponse::from(updated_space))
}
