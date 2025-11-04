use crate::controllers::v3::spaces::dto::*;
use crate::features::spaces::members::{
    SpaceEmailVerification, SpaceInvitationMember, SpaceInvitationMemberQueryOption,
};
use crate::features::telegrams::{TelegramChannel, get_space_created_message};
use crate::models::space::SpaceCommon;

use crate::models::Post;
use crate::models::user::User;
use crate::types::{
    BoosterType, EntityType, SpacePublishState, SpaceStatus, SpaceVisibility, TeamGroupPermission,
};
use crate::types::{File, Partition};
use crate::utils::aws::DynamoClient;
use crate::utils::aws::SesClient;
use crate::utils::telegram::ArcTelegramBot;
use crate::utils::time::get_now_timestamp;
use crate::{AppState, Error, transact_write};
use aide::NoApi;
use axum::extract::{Json, Path, State};
use bdk::prelude::axum::Extension;
use bdk::prelude::*;

use serde::Deserialize;
#[derive(Debug, Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
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
    Start {
        start: bool,
    },
    Finish {
        finished: bool,
    },
}

pub async fn update_space_handler(
    State(AppState { dynamo, ses, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Extension(telegram_bot): Extension<Option<ArcTelegramBot>>,
    Path(SpacePathParam { space_pk }): Path<SpacePathParam>,
    Json(req): Json<UpdateSpaceRequest>,
) -> Result<Json<SpaceCommonResponse>, Error> {
    let (mut space, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        Some(&user.pk),
        TeamGroupPermission::PostEdit,
    )
    .await?;
    if !has_perm {
        return Err(Error::NoPermission);
    }

    let now = chrono::Utc::now().timestamp_millis();
    let mut su = SpaceCommon::updater(&space.pk, &space.sk).with_updated_at(now);
    let mut pu =
        Post::updater(space.pk.clone().to_post_key()?, EntityType::Post).with_updated_at(now);
    let mut should_notify_space = false;
    match req {
        UpdateSpaceRequest::Publish {
            publish,
            visibility,
        } => {
            let post_pk = space_pk.clone().to_post_key()?;
            let post = Post::get(&dynamo.client, &post_pk, Some(&EntityType::Post))
                .await?
                .unwrap_or_default();

            if !publish {
                return Err(Error::NotSupported(
                    "it does not support unpublished now".into(),
                ));
            }
            // FIXME: check validation if it is well designed to be published.
            su = su
                .with_publish_state(SpacePublishState::Published)
                .with_status(SpaceStatus::InProgress)
                .with_visibility(visibility.clone());

            pu = pu
                .with_space_visibility(visibility.clone())
                .with_visibility(visibility.clone().into())
                .with_status(crate::types::PostStatus::Published);

            should_notify_space = space.booster != BoosterType::NoBoost
                && (space.publish_state == SpacePublishState::Draft && publish)
                && visibility == SpaceVisibility::Public;

            let _ = send_space_verification_code_handler(
                &dynamo,
                &ses,
                &space.clone(),
                post.title.clone(),
            )
            .await?;

            space.publish_state = SpacePublishState::Published;
            space.visibility = visibility;
        }
        UpdateSpaceRequest::Visibility { visibility } => {
            su = su.with_visibility(visibility.clone());

            should_notify_space = space.booster != BoosterType::NoBoost
                && space.publish_state == SpacePublishState::Published
                && space.visibility != SpaceVisibility::Public
                && visibility == SpaceVisibility::Public;

            space.visibility = visibility;
        }
        UpdateSpaceRequest::Content { content } => {
            su = su.with_content(content.clone());

            space.content = content;
        }
        UpdateSpaceRequest::Title { title } => {
            pu = pu.with_title(title.clone());
        }
        UpdateSpaceRequest::Start { start } => {
            if space.status != Some(SpaceStatus::InProgress) {
                return Err(Error::NotSupported(
                    "Start is not available for the current status.".into(),
                ));
            }

            if !start {
                return Err(Error::NotSupported("it does not support start now".into()));
            }

            su = su.with_status(SpaceStatus::Started);

            space.status = Some(SpaceStatus::Started);
            let _ = SpaceEmailVerification::expire_verifications(&dynamo, space_pk.clone()).await?;
        }
        UpdateSpaceRequest::Finish { finished } => {
            if space.status != Some(SpaceStatus::Started) {
                return Err(Error::NotSupported(
                    "Finish is not available for the current status.".into(),
                ));
            }

            if !finished {
                return Err(Error::NotSupported("it does not support end now".into()));
            }

            su = su.with_status(SpaceStatus::Finished);

            space.status = Some(SpaceStatus::Finished);
        }
        UpdateSpaceRequest::Anonymous {
            anonymous_participation,
        } => {
            su = su.with_anonymous_participation(anonymous_participation);

            space.anonymous_participation = anonymous_participation;
        }
    }

    transact_write!(
        dynamo.client,
        su.transact_write_item(),
        pu.transact_write_item()
    )?;

    if should_notify_space {
        if let Some(bot) = telegram_bot {
            let dynamo_client = dynamo.client.clone();
            let post_pk = space_pk.clone().to_post_key()?;
            let space_pk_clone = space_pk.clone();
            tokio::spawn(async move {
                if let Ok(post) = Post::get(&dynamo_client, &post_pk, Some(EntityType::Post)).await
                {
                    if let Some(post) = post {
                        let (content, button) = get_space_created_message(
                            &bot.bot_name,
                            &space_pk_clone,
                            space.space_type,
                            &post.title,
                        );

                        if let Err(err) = TelegramChannel::send_message_to_channels(
                            &dynamo_client,
                            &bot,
                            content,
                            Some(button),
                        )
                        .await
                        {
                            tracing::error!("Failed to send Telegram message: {}", err);
                        }
                    }
                }
            });
        }
    }

    Ok(Json(SpaceCommonResponse::from(space)))
}

async fn send_space_verification_code_handler(
    dynamo: &DynamoClient,
    ses: &SesClient,
    space: &SpaceCommon,
    title: String,
) -> Result<Json<()>, Error> {
    let mut bookmark = None::<String>;
    loop {
        let (responses, new_bookmark) = SpaceInvitationMember::query(
            &dynamo.client,
            space.pk.clone(),
            if let Some(b) = &bookmark {
                SpaceInvitationMemberQueryOption::builder()
                    .sk("SPACE_INVITATION_MEMBER#".into())
                    .bookmark(b.clone())
            } else {
                SpaceInvitationMemberQueryOption::builder().sk("SPACE_INVITATION_MEMBER#".into())
            },
        )
        .await?;

        for response in responses {
            let user_email = response.email;
            let _ = SpaceEmailVerification::send_email(
                &dynamo,
                &ses,
                user_email,
                space.clone(),
                title.clone(),
            )
            .await?;
        }

        match new_bookmark {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    Ok(Json(()))
}
