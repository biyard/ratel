use by_axum::{
    axum::{
        extract::{Path, Query, State},
        routing::get,
        Json,
    },
    log::root,
};
use dto::*;
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
            .route("/:id", get(Self::get_topic))
            .with_state(ctrl.clone())
            .route("/", get(Self::list_topics))
            .with_state(ctrl))
    }

    pub async fn get_topic(
        State(ctrl): State<TopicControllerV1>,

        Path(id): Path<String>,
    ) -> Result<Json<Topic>> {
        let log = ctrl.log.new(o!("api" => "get_topic"));
        slog::debug!(log, "get topic {:?}", id);
        Ok(Json(Topic::default()))
    }

    pub async fn list_topics(
        State(ctrl): State<TopicControllerV1>,

        Query(req): Query<TopicQuery>,
    ) -> Result<Json<CommonQueryResponse<TopicSummery>>> {
        let log = ctrl.log.new(o!("api" => "list_topics"));
        slog::debug!(log, "list topics {:?}", req);
        let started_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let day = 60 * 60 * 24;
        let ended_at = started_at + day * 7;
        let status = req.status.unwrap_or(TopicStatus::Draft);

        let ret = CommonQueryResponse {
            items: vec![
                TopicSummery {
                    id: "1".to_string(),
                    r#type: "type".to_string(),
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
                TopicSummery {
                    id: "1".to_string(),
                    r#type: "type".to_string(),
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
                TopicSummery {
                    id: "1".to_string(),
                    r#type: "type".to_string(),
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
            bookmark: None,
        };
        Ok(Json(ret))
    }
}
