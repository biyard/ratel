use crate::controllers::v3::spaces::dto::*;
use crate::features::spaces::SpaceDao;
use crate::features::spaces::members::{
    SpaceEmailVerification, SpaceInvitationMember, SpaceInvitationMemberQueryOption,
};
use crate::features::spaces::polls::{Poll, PollQueryOption};
use crate::features::telegrams::{TelegramChannel, get_space_created_message};
use crate::models::space::SpaceCommon;

use crate::models::Post;
use crate::models::user::User;
use crate::services::fcm_notification::FCMService;
use crate::utils::aws::DynamoClient;
use crate::utils::aws::PollScheduler;
use crate::utils::aws::SesClient;
use crate::utils::aws::get_aws_config;
use crate::utils::telegram::ArcTelegramBot;
use crate::utils::time::get_now_timestamp;
use crate::*;

use crate::utils::space_dao_sampling::sample_space_dao_participants;
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
        #[serde(default)]
        block_participate: bool,
    },
    Quota {
        quotas: i64,
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

    let sdk_config = get_aws_config();
    let scheduler = PollScheduler::new(&sdk_config);

    let mut space = space.clone();

    let now = chrono::Utc::now().timestamp_millis();
    let mut su = SpaceCommon::updater(&space.pk, &space.sk).with_updated_at(now);
    let mut pu: Option<_> = None;
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

            let mut post_updater = Post::updater(post_pk, EntityType::Post).with_updated_at(now);

            post_updater = post_updater
                .with_space_visibility(visibility.clone())
                .with_visibility(visibility.clone().into())
                .with_status(crate::types::PostStatus::Published);

            pu = Some(post_updater);

            should_notify_space = space.booster != BoosterType::NoBoost
                && (space.publish_state == SpacePublishState::Draft && publish)
                && visibility == SpaceVisibility::Public;

            SpaceInvitationMember::send_email(&dynamo, &ses, &space, post.title.clone()).await?;

            // FIXME: fix to one call code
            if let Ok(mut fcm) = FCMService::new().await {
                SpaceInvitationMember::send_notification(&dynamo, &mut fcm, &space, post.title)
                    .await?;
            } else {
                warn!("Failed to initialize FCMService, skipping notifications.");
            }

            let mut bookmark: Option<String> = None;

            loop {
                let mut query_options = PollQueryOption::builder()
                    .sk("SPACE_POLL#".into())
                    .limit(10);

                if let Some(b) = bookmark.clone() {
                    query_options = query_options.bookmark(b);
                }

                let (responses, next_bookmark) =
                    Poll::query(&dynamo.client, space_pk.clone(), query_options).await?;

                for response in responses {
                    response
                        .schedule_start_notification(&scheduler, response.started_at)
                        .await?;
                }

                match next_bookmark {
                    Some(b) => bookmark = Some(b),
                    None => break,
                }
            }

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
            let post_pk = space_pk.clone().to_post_key()?;
            let mut post_updater = Post::updater(post_pk, EntityType::Post).with_updated_at(now);

            post_updater = post_updater.with_title(title.clone());
            pu = Some(post_updater);
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
        UpdateSpaceRequest::Finish {
            finished,
            block_participate,
        } => {
            if space.status != Some(SpaceStatus::Started) {
                return Err(Error::NotSupported(
                    "Finish is not available for the current status.".into(),
                ));
            }

            if !finished {
                return Err(Error::NotSupported("it does not support finish now".into()));
            }

            su = su
                .with_status(SpaceStatus::Finished)
                .with_block_participate(block_participate);

            space.status = Some(SpaceStatus::Finished);
            space.block_participate = block_participate;

            // FIXME: This architecture should be changed to a event fetcher structure in the future.
            let dao = SpaceDao::get(&dynamo.client, &space_pk, Some(&EntityType::SpaceDao)).await?;
            if let Some(dao) = dao {
                if dao.sampling_count > 0 {
                    let sampled = sample_space_dao_participants(
                        &dynamo.client,
                        &space_pk,
                        dao.sampling_count,
                    )
                    .await?;
                    let sampled_users: Vec<String> =
                        sampled.iter().map(|p| p.user_pk.to_string()).collect();
                    tracing::info!(
                        "Finish sampling: space={}, count={}, sampled={:?}",
                        space_pk,
                        sampled_users.len(),
                        sampled_users
                    );
                }
            }
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
        UpdateSpaceRequest::Quota { quotas } => {
            let remains = space.remains + (quotas - space.quota);

            if remains < 0 {
                return Err(Error::InvalidPanelQuota);
            }

            su = su.with_quota(quotas).with_remains(remains);

            space.quota = quotas;
            space.remains = remains;
        }
    }

    if let Some(pu) = pu {
        transact_write!(
            dynamo.client,
            su.transact_write_item(),
            pu.transact_write_item()
        )?;
    } else {
        transact_write!(dynamo.client, su.transact_write_item())?;
    }

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
