mod config;

#[cfg(feature = "local-run")]
use bdk::prelude::*;
use lambda_runtime::{Error as LambdaError, LambdaEvent};
use main_api::{
    features::spaces::{
        analyzes::SpaceAnalyze,
        boards::models::{space_post::SpacePost, space_post_comment::SpacePostComment},
        members::{SpaceInvitationMember, SpaceInvitationMemberQueryOption},
        polls::Poll,
    },
    models::{Post, SpaceCommon},
    types::{EntityType, Partition},
    utils::{
        aws::{DynamoClient, SesClient},
        reports::{LdaConfigV1, NetworkConfigV1, TfidfConfigV1, run_lda, run_network, run_tfidf},
    },
};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize, Serialize)]
struct EventBridgeEnvelope {
    pub detail: JsonValue,
}

#[derive(Debug, Deserialize, Serialize)]
struct StartSurveyEvent {
    pub space_id: String,
    pub survey_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct UpsertAnalyzeEvent {
    pub space_id: String,
    pub lda_topics: usize,
    pub tf_idf_keywords: usize,
    pub network_top_nodes: usize,
    pub remove_keywords: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
enum WorkerEvent {
    StartSurvey(StartSurveyEvent),
    UpsertAnalyze(UpsertAnalyzeEvent),
}

#[derive(Clone)]
struct AppState {
    dynamo: DynamoClient,
    ses: SesClient,
}

#[cfg(not(feature = "local-run"))]
#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    use aws_config::BehaviorVersion;
    use lambda_runtime::service_fn;

    init_tracing();

    let cfg = config::get();
    let is_local = cfg.env == "local" || cfg.env == "test";
    let aws_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let dynamo = DynamoClient::new(Some(aws_config.clone()));
    let ses = SesClient::new(aws_config, is_local);

    let state = AppState { dynamo, ses };

    lambda_runtime::run(service_fn(move |event| handler(event, state.clone()))).await
}

#[cfg(feature = "local-run")]
#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    use lambda_runtime::Context;
    use main_api::utils::aws::get_aws_config;

    init_tracing();

    let cfg = config::get();
    let is_local = cfg.env == "local" || cfg.env == "test";
    let aws_config = get_aws_config();
    let dynamo = DynamoClient::new(Some(aws_config.clone()));
    let ses = SesClient::new(aws_config, is_local);

    let state = AppState { dynamo, ses };

    let payload = EventBridgeEnvelope {
        detail: serde_json::to_value(UpsertAnalyzeEvent {
            space_id: "019b914a-0a9b-7911-baa3-f673afd776ee".into(),
            lda_topics: 5,
            tf_idf_keywords: 100,
            network_top_nodes: 30,
            remove_keywords: vec![],
        })?,
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

fn classify_event(detail: &JsonValue) -> Result<WorkerEvent, LambdaError> {
    let obj = detail
        .as_object()
        .ok_or_else(|| LambdaError::from("detail must be a JSON object"))?;

    if obj.contains_key("survey_id") {
        let evt: StartSurveyEvent = serde_json::from_value(detail.clone())?;
        return Ok(WorkerEvent::StartSurvey(evt));
    }

    if obj.contains_key("lda_topics")
        || obj.contains_key("tf_idf_keywords")
        || obj.contains_key("network_top_nodes")
    {
        let evt: UpsertAnalyzeEvent = serde_json::from_value(detail.clone())?;
        return Ok(WorkerEvent::UpsertAnalyze(evt));
    }

    Err(LambdaError::from(
        "unknown event: cannot classify by detail fields",
    ))
}

async fn handler(
    event: LambdaEvent<EventBridgeEnvelope>,
    state: AppState,
) -> Result<(), LambdaError> {
    let (payload, ctx) = event.into_parts();

    info!("worker invoked: request_id={}", ctx.request_id);

    match classify_event(&payload.detail)? {
        WorkerEvent::StartSurvey(evt) => {
            info!(
                "StartSurvey(detail-based): request_id={}, space_id={}, survey_id={}",
                ctx.request_id, evt.space_id, evt.survey_id
            );

            if let Err(e) = start_survey(&state, &evt).await {
                error!("failed to start survey: {e:?}");
                return Err(e);
            }
        }
        WorkerEvent::UpsertAnalyze(evt) => {
            info!(
                "UpsertAnalyze(detail-based): request_id={}, space_id={}, lda_topics={}, tf_idf_keywords={}, network_top_nodes={}, remove_keywords={}",
                ctx.request_id,
                evt.space_id,
                evt.lda_topics,
                evt.tf_idf_keywords,
                evt.network_top_nodes,
                evt.remove_keywords.len()
            );

            if let Err(e) = upsert_analyze(&state, &evt).await {
                error!("failed to upsert analyze: {e:?}");
                return Err(e);
            }
        }
    }

    Ok(())
}

async fn start_survey(state: &AppState, evt: &StartSurveyEvent) -> Result<(), LambdaError> {
    let pk = evt.space_id.clone();
    let sk = evt.survey_id.clone();

    let space_pk = Partition::Space(pk);
    let poll_sk = EntityType::SpacePoll(sk.clone());

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

    if let Err(e) = Poll::send_email(
        &state.dynamo,
        &state.ses,
        sk.clone(),
        space,
        post.title,
        emails,
        poll.is_default_poll(),
    )
    .await
    {
        error!("failed to send survey email: {e:?}");
    }

    info!("survey status updated to Started");

    Ok(())
}

async fn upsert_analyze(state: &AppState, evt: &UpsertAnalyzeEvent) -> Result<(), LambdaError> {
    use futures::future::try_join_all;

    let space_pk = Partition::Space(evt.space_id.clone());

    let posts = SpacePost::find_by_space_ordered(
        &state.dynamo.client,
        space_pk.clone(),
        SpacePost::opt_all(),
    )
    .await?
    .0;

    let comment_futs = posts.iter().filter_map(|post| {
        let space_post_pk = match &post.sk {
            EntityType::SpacePost(pk) => Partition::SpacePost(pk.to_string()),
            _ => return None,
        };

        Some(async {
            let (comments, _) = SpacePostComment::find_by_post_order_by_likes(
                &state.dynamo.client,
                space_post_pk,
                SpacePostComment::opt_all(),
            )
            .await?;
            Ok::<Vec<SpacePostComment>, main_api::Error>(comments)
        })
    });

    let comments_per_post: Vec<Vec<SpacePostComment>> = try_join_all(comment_futs).await?;
    let mut post_comments: Vec<String> = Vec::new();
    for comments in comments_per_post {
        for c in comments {
            post_comments.push(c.content);
        }
    }

    let mut lda_config = LdaConfigV1::default();
    lda_config.num_topics = evt.lda_topics;
    let lda = run_lda(&post_comments, lda_config)?;

    let mut tfidf_config = TfidfConfigV1::default();
    tfidf_config.max_features = evt.tf_idf_keywords;
    let tf_idf = run_tfidf(&post_comments, tfidf_config)?;

    let mut network_config = NetworkConfigV1::default();
    network_config.top_nodes = evt.network_top_nodes;
    let network = run_network(&post_comments, network_config)?;

    let analyze = SpaceAnalyze::new(space_pk, lda, network, tf_idf);
    analyze.upsert(&state.dynamo.client).await?;

    info!("analyze update successed!");

    Ok(())
}
