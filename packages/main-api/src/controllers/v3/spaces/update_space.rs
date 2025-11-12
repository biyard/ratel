use crate::controllers::v3::spaces::dto::*;
use crate::features::spaces::members::{
    SpaceEmailVerification, SpaceInvitationMember, SpaceInvitationMemberQueryOption,
};
use crate::features::telegrams::{TelegramChannel, get_space_created_message};
use crate::models::space::SpaceCommon;

use crate::models::Post;
use crate::models::user::User;
use crate::utils::aws::DynamoClient;
use crate::utils::aws::SesClient;
use crate::utils::telegram::ArcTelegramBot;
use crate::utils::time::get_now_timestamp;
use crate::*;

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
    File {
        files: Vec<File>,
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
        #[serde(default)]
        block_participate: bool,
    },
    Finish {
        finished: bool,
    },
}

pub async fn update_space_handler(
    State(AppState { dynamo, ses, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    NoApi(permissions): NoApi<Permissions>,
    Extension(telegram_bot): Extension<Option<ArcTelegramBot>>,
    Extension(space): Extension<SpaceCommon>,
    Path(SpacePathParam { space_pk }): Path<SpacePathParam>,
    Json(req): Json<UpdateSpaceRequest>,
) -> Result<Json<SpaceCommonResponse>> {
    if !permissions.is_admin() {
        tracing::error!(
            "User {} does not have admin permissions {:?}",
            user.pk,
            permissions
        );
        return Err(Error::NoPermission);
    }

    let mut space = space.clone();

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

            SpaceInvitationMember::send_email(&dynamo, &ses, &space, post.title).await?;

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
        UpdateSpaceRequest::File { files } => {
            su = su.with_files(files.clone());

            space.files = Some(files);
        }
        UpdateSpaceRequest::Title { title } => {
            pu = pu.with_title(title.clone());
        }
        UpdateSpaceRequest::Start {
            start,
            block_participate,
        } => {
            if space.status != Some(SpaceStatus::InProgress) {
                return Err(Error::NotSupported(
                    "Start is not available for the current status.".into(),
                ));
            }

            if !start {
                return Err(Error::NotSupported("it does not support start now".into()));
            }

            su = su
                .with_status(SpaceStatus::Started)
                .with_block_participate(block_participate);

            space.status = Some(SpaceStatus::Started);
            space.block_participate = block_participate;
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
        UpdateSpaceRequest::ChangeVisibility { change_visibility } => {
            su = su.with_change_visibility(change_visibility);

            space.change_visibility = change_visibility;
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
