mod config;

use bdk::prelude::*;
use lambda_runtime::{Error as LambdaError, LambdaEvent};
use main_api::{
    features::spaces::members::{SpaceInvitationMember, SpaceInvitationMemberQueryOption},
    features::spaces::polls::Poll,
    models::{Post, SpaceCommon},
    types::{EntityType, Partition},
    utils::aws::{DynamoClient, SesClient, get_aws_config},
};
use serde::Deserialize;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
struct StartSurveyEvent {
    pub space_id: String,
    pub survey_id: String,
}

#[derive(Clone)]
struct AppState {
    dynamo: DynamoClient,
    ses: SesClient,
}

#[cfg(not(feature = "local-run"))]
#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    use lambda_runtime::service_fn;

    init_tracing();

    let cfg = config::get();
    let is_local = cfg.env == "local" || cfg.env == "test";
    let aws_config = get_aws_config();
    let dynamo = DynamoClient::new(Some(aws_config.clone()));
    let ses = SesClient::new(aws_config, is_local);

    let state = AppState { dynamo, ses };

    lambda_runtime::run(service_fn(move |event| handler(event, state.clone()))).await
}

#[cfg(feature = "local-run")]
#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    use lambda_runtime::Context;
    init_tracing();

    let cfg = config::get();
    let is_local = cfg.env == "local" || cfg.env == "test";
    let aws_config = get_aws_config();
    let dynamo = DynamoClient::new(Some(aws_config.clone()));
    let ses = SesClient::new(aws_config, is_local);

    let state = AppState { dynamo, ses };
    let payload = StartSurveyEvent {
        space_id: "5a383702-d617-4f4f-ad14-b7daf7ead42e".into(),
        survey_id: "5a383702-d617-4f4f-ad14-b7daf7ead42e".into(),
    };

    let ctx = Context::default();
    handler(LambdaEvent::new(payload, ctx), state).await
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_max_level(tracing::Level::INFO)
        .without_time()
        .init();
}

async fn handler(event: LambdaEvent<StartSurveyEvent>, state: AppState) -> Result<(), LambdaError> {
    let (payload, ctx) = event.into_parts();
    info!(
        "start-survey invoked: request_id={}, space_id={}, survey_id={}",
        ctx.request_id, payload.space_id, payload.survey_id
    );

    if let Err(e) = start_survey(&state, &payload).await {
        error!("failed to start survey: {e:?}");
        return Err(e);
    }

    Ok(())
}

async fn start_survey(state: &AppState, evt: &StartSurveyEvent) -> Result<(), LambdaError> {
    let pk = evt.space_id.clone();
    let sk = evt.survey_id.clone();

    let space_pk = Partition::Space(pk);
    let poll_sk = EntityType::SpacePoll(sk);

    let space = SpaceCommon::get(
        &state.dynamo.client,
        &space_pk,
        Some(EntityType::SpaceCommon),
    )
    .await?
    .unwrap_or_default();
    let post_pk = space.pk.clone().to_post_key()?;
    let post = Post::get(&state.dynamo.client, &post_pk, Some(&EntityType::Post))
        .await?
        .unwrap_or_default();
    let poll = Poll::get(&state.dynamo.client, space_pk, Some(poll_sk))
        .await?
        .unwrap_or_default();

    let mut emails: Vec<String> = Vec::new();
    let mut bookmark: Option<String> = None;

    loop {
        let (responses, new_bookmark) = SpaceInvitationMember::query(
            &state.dynamo.client,
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
            emails.push(response.email);
        }

        match new_bookmark {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    let _ = Poll::send_email(
        &state.dynamo,
        &state.ses,
        space,
        post.title,
        emails,
        poll.is_default_poll(),
    )
    .await?;

    info!("survey status updated to Started");

    Ok(())
}
