use by_axum::{
    axum::{
        extract::{Path, Query, State},
        routing::get,
        Extension, Json,
    },
    log::root,
};
use dto::*;
use rest_api::Signature;
use slog::o;

#[derive(Clone, Debug)]
pub struct TopicControllerV1 {
    log: slog::Logger,
}

impl TopicControllerV1 {
    pub fn route() -> Result<by_axum::axum::Router> {
        let log = root().new(o!("api-controller" => "TopicControllerV1"));
        let ctrl = TopicControllerV1 { log };

        Ok(by_axum::axum::Router::new()
            .route("/:id", get(Self::get_topic).post(Self::act_topic_by_id))
            .with_state(ctrl.clone())
            .route("/", get(Self::list_topics).post(Self::act_topic))
            .route("/legislations", get(Self::list_legislations))
            .with_state(ctrl))
    }

    pub async fn list_legislations(
        State(ctrl): State<TopicControllerV1>,
        Extension(_sig): Extension<Option<Signature>>,
        Query(req): Query<LegislationsQuery>,
    ) -> Result<Json<CommonQueryResponse<Legislation>>> {
        let log = ctrl.log.new(o!("api" => "list_legislations"));
        slog::debug!(log, "list legislations: {req}");
        let filter = req
            .lang
            .map(|lang| vec![("gsi1", format!("legislation#{}", lang))]);

        let res: CommonQueryResponse<Legislation> = CommonQueryResponse::query(
            &log,
            "gsi1-index",
            req.bookmark,
            req.size.map(|s| s as i32),
            filter.unwrap_or_default(),
        )
        .await?;

        Ok(Json(res))
    }

    pub async fn act_topic_by_id(
        State(ctrl): State<TopicControllerV1>,
        Extension(_sig): Extension<Option<Signature>>,
        Json(body): Json<TopicByIdAction>,
    ) -> Result<Json<Topic>> {
        let log = ctrl.log.new(o!("api" => "act_topic_by_id"));
        slog::debug!(log, "act_topic_by_id: {:?}", body);
        Ok(Json(Topic::default()))
    }

    pub async fn act_topic(
        State(ctrl): State<TopicControllerV1>,
        Extension(_sig): Extension<Option<Signature>>,
        Json(body): Json<TopicAction>,
    ) -> Result<Json<Topic>> {
        let log = ctrl.log.new(o!("api" => "act_topic"));
        slog::debug!(log, "act_topic: {:?}", body);
        Ok(Json(Topic::default()))
    }

    pub async fn get_topic(
        State(ctrl): State<TopicControllerV1>,
        Extension(_sig): Extension<Option<Signature>>,

        Path(id): Path<String>,
    ) -> Result<Json<Topic>> {
        let log = ctrl.log.new(o!("api" => "get_topic"));
        slog::debug!(log, "get topic {:?}", id);
        Ok(Json(Topic::default()))
    }

    pub async fn list_topics(
        State(ctrl): State<TopicControllerV1>,
        Extension(_sig): Extension<Option<Signature>>,

        Query(req): Query<TopicQuery>,
    ) -> Result<Json<QueryResponse<TopicSummary>>> {
        let log = ctrl.log.new(o!("api" => "list_topics"));
        slog::debug!(log, "list topics {:?}", req);
        let started_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let day = 60 * 60 * 24;
        let ended_at = started_at + day * 7;
        let status = req.status.unwrap_or(TopicStatus::Draft);

        let ret = QueryResponse {
            total_count: 3,
            items: vec![
                TopicSummary {
                    id: "1".to_string(),
                    created_at: 0,
                    updated_at: 0,
                    deleted_at: None,
                    author: "author".to_string(),

                    // name of legislation
                    title: "Digital Heritage Preservation and Utilization Act".to_string(),
                    content: "This Act aims to preserve cultural heritage and promote its sustainable utilization through digital innovation and advanced technologies, ensuring the protection of valuable resources while fostering public access and engagement.".to_string(),
                    images: vec!["https://dev.democrasee.me/images/sample.png".to_string()],
                    votes: vec![Vote::Supportive(30), Vote::Against(20)],
                    donations: vec![Donation::Yes(30), Donation::No(20)],
                    started_at,
                    ended_at,
                    voters: 100,
                    replies: 100,
                    status: status.clone(),
                    result: None,
                    weekly_replies: 100,
                    weekly_volume: 100,
                    weekly_votes: 100,
                    volume: 1000,
                },
                TopicSummary {
                    id: "1".to_string(),
                    created_at: 0,
                    updated_at: 0,
                    deleted_at: None,
                    author: "author".to_string(),

                    title: "Digital Heritage Preservation and Utilization Act".to_string(),
                    content: "This Act aims to preserve cultural heritage and promote its sustainable utilization through digital innovation and advanced technologies, ensuring the protection of valuable resources while fostering public access and engagement.".to_string(),
                    images: vec!["https://dev.democrasee.me/images/sample.png".to_string()],
                    votes: vec![Vote::Supportive(30), Vote::Against(20)],
                    donations: vec![Donation::Yes(30), Donation::No(20)],
                    started_at,
                    ended_at,
                    voters: 100,
                    replies: 100,
                    status: status.clone(),
                    result: None,
                    weekly_replies: 100,
                    weekly_volume: 100,
                    weekly_votes: 100,
                    volume: 1000,
                },
                TopicSummary {
                    id: "1".to_string(),
                    created_at: 0,
                    updated_at: 0,
                    deleted_at: None,
                    author: "author".to_string(),

                    title: "Digital Heritage Preservation and Utilization Act".to_string(),
                    content: "This Act aims to preserve cultural heritage and promote its sustainable utilization through digital innovation and advanced technologies, ensuring the protection of valuable resources while fostering public access and engagement.".to_string(),
                    images: vec!["https://dev.democrasee.me/images/sample.png".to_string()],
                    votes: vec![Vote::Supportive(30), Vote::Against(20)],
                    donations: vec![Donation::Yes(30), Donation::No(20)],
                    started_at,
                    ended_at,
                    voters: 100,
                    replies: 100,
                    status: status.clone(),
                    result: None,
                    weekly_replies: 100,
                    weekly_volume: 100,
                    weekly_votes: 100,
                    volume: 1000,
                }
            ],
        };
        Ok(Json(ret))
    }
}
