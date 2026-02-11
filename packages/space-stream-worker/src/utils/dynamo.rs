use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{config::Builder as DynamoConfigBuilder, Client as DynamoClient};
use lambda_runtime::Error as LambdaError;
use main_api::features::spaces::{
    boards::models::space_post::SpacePost,
    boards::models::space_post_comment::{SpacePostComment, SpacePostCommentQueryOption},
    panels::{SpacePanelParticipant, SpacePanelParticipantQueryOption},
    polls::{
        Poll, PollQueryOption, PollUserAnswer, PollUserAnswerQueryOption,
    },
};
use main_api::types::{EntityType, Partition};
use tracing::error;

pub async fn build_dynamo_client(endpoint: Option<&str>) -> Result<DynamoClient, LambdaError> {
    let aws_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let mut builder = DynamoConfigBuilder::from(&aws_config);
    if let Some(endpoint) = endpoint {
        builder = builder.endpoint_url(endpoint);
    }
    Ok(DynamoClient::from_conf(builder.build()))
}

pub fn map_model_error<E: std::fmt::Display + std::fmt::Debug>(
    ctx: &str,
    err: E,
) -> LambdaError {
    error!("model query failed ({ctx}): {err:?}");
    LambdaError::from("model query failed")
}

pub async fn fetch_all_polls(
    dynamo: &DynamoClient,
    space_pk: &Partition,
) -> Result<Vec<Poll>, LambdaError> {
    let mut all = Vec::new();
    let mut bookmark: Option<String> = None;

    loop {
        let mut opt = PollQueryOption::builder().sk("SPACE_POLL#".into()).limit(100);
        if let Some(b) = &bookmark {
            opt = opt.bookmark(b.clone());
        }

        let (items, next) = Poll::query(dynamo, space_pk.clone(), opt)
            .await
            .map_err(|e| map_model_error("polls", e))?;
        all.extend(items);

        match next {
            Some(next) => bookmark = Some(next),
            None => break,
        }
    }

    Ok(all)
}

pub async fn fetch_all_posts(
    dynamo: &DynamoClient,
    space_pk: &Partition,
) -> Result<Vec<SpacePost>, LambdaError> {
    let mut all = Vec::new();
    let mut bookmark: Option<String> = None;

    loop {
        let opt = SpacePost::opt_with_bookmark(bookmark.clone());
        let (items, next) = SpacePost::find_by_space_ordered(dynamo, space_pk.clone(), opt)
            .await
            .map_err(|e| map_model_error("space_posts", e))?;
        all.extend(items);

        match next {
            Some(next) => bookmark = Some(next),
            None => break,
        }
    }

    Ok(all)
}

pub async fn fetch_all_panel_participants(
    dynamo: &DynamoClient,
    space_pk: &Partition,
) -> Result<Vec<SpacePanelParticipant>, LambdaError> {
    let mut all = Vec::new();
    let mut bookmark: Option<String> = None;

    loop {
        let mut opt = SpacePanelParticipantQueryOption::builder()
            .sk("SPACE_PANEL_PARTICIPANT#".into())
            .limit(200);
        if let Some(b) = &bookmark {
            opt = opt.bookmark(b.clone());
        }

        let (items, next) = SpacePanelParticipant::query(dynamo, space_pk.clone(), opt)
            .await
            .map_err(|e| map_model_error("panel_participants", e))?;
        all.extend(items);

        match next {
            Some(next) => bookmark = Some(next),
            None => break,
        }
    }

    Ok(all)
}

pub async fn fetch_post_comments(
    dynamo: &DynamoClient,
    posts: &[SpacePost],
) -> Result<Vec<SpacePostComment>, LambdaError> {
    let mut all = Vec::new();
    for post in posts {
        let post_pk = match &post.sk {
            EntityType::SpacePost(id) => Partition::SpacePost(id.clone()),
            _ => continue,
        };
        match fetch_post_comments_for_post(dynamo, post_pk).await {
            Ok(mut items) => all.append(&mut items),
            Err(err) => error!("failed to fetch post comments: {err}"),
        }
    }

    Ok(all)
}

pub async fn fetch_poll_user_answers(
    dynamo: &DynamoClient,
    space_pk: &Partition,
    polls: &[Poll],
) -> Result<Vec<PollUserAnswer>, LambdaError> {
    let space_pk_string = space_pk.to_string();

    let mut all = Vec::new();
    for poll in polls {
        let poll_id = match &poll.sk {
            EntityType::SpacePoll(id) => id.clone(),
            _ => continue,
        };
        match fetch_poll_user_answers_for_poll(dynamo, &space_pk_string, poll_id).await {
            Ok(mut items) => all.append(&mut items),
            Err(err) => error!("failed to fetch poll answers: {err}"),
        }
    }

    Ok(all)
}

async fn fetch_post_comments_for_post(
    dynamo: &DynamoClient,
    post_pk: Partition,
) -> Result<Vec<SpacePostComment>, LambdaError> {
    let mut all = Vec::new();
    let mut bookmark: Option<String> = None;

    loop {
        let mut opt = SpacePostCommentQueryOption::builder().limit(100);
        if let Some(b) = &bookmark {
            opt = opt.bookmark(b.clone());
        }

        let (items, next) =
            SpacePostComment::find_by_post_order_by_likes(dynamo, post_pk.clone(), opt)
                .await
                .map_err(|e| map_model_error("post_comments", e))?;
        all.extend(items);

        match next {
            Some(next) => bookmark = Some(next),
            None => break,
        }
    }

    Ok(all)
}

async fn fetch_poll_user_answers_for_poll(
    dynamo: &DynamoClient,
    space_pk: &str,
    poll_id: String,
) -> Result<Vec<PollUserAnswer>, LambdaError> {
    let poll_pk = Partition::Poll(poll_id);
    let gsi1_pk = EntityType::SpacePollUserAnswer(space_pk.to_string(), poll_pk.to_string());

    let mut all = Vec::new();
    let mut bookmark: Option<String> = None;

    loop {
        let opt = if let Some(b) = &bookmark {
            PollUserAnswerQueryOption::builder().bookmark(b.clone())
        } else {
            PollUserAnswerQueryOption::builder()
        };

        let (items, next) = PollUserAnswer::find_by_space_pk(dynamo, &gsi1_pk, opt)
            .await
            .map_err(|e| map_model_error("poll_user_answers", e))?;
        all.extend(items);

        match next {
            Some(next) => bookmark = Some(next),
            None => break,
        }
    }

    Ok(all)
}
