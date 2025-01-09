use std::str::FromStr;

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

#[derive(Debug, serde::Deserialize)]
pub struct ListTopicsRequest {
    _size: Option<usize>,
    _bookmark: Option<String>,
    status: Option<String>,
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

        Query(req): Query<ListTopicsRequest>,
    ) -> Result<Json<CommonQueryResponse<Topic>>> {
        let log = ctrl.log.new(o!("api" => "list_topics"));
        slog::debug!(log, "list topics {:?}", req);
        let started_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let day = 60 * 60 * 24;
        let ended_at = started_at + day * 7;
        let status = TopicStatus::from_str(&req.status.unwrap_or("ongoing".to_string()))
            .unwrap_or(TopicStatus::Draft);

        let ret = CommonQueryResponse {
            items: vec![
                Topic {
                    id: "1".to_string(),
                    r#type: "type".to_string(),
                    created_at: 0,
                    updated_at: 0,
                    deleted_at: None,
                    author: "author".to_string(),

                    title: "윤대통령 2차 탄핵안 절차 게시될까?".to_string(),
                    content: "민주당과 조국혁신당, 개혁신당 등 야 6당이 함께 윤석열 대통령에 대한 두 번째 탄핵소추안을 국회에 제출했습니다.  지난 7일, 국민의힘 의원 대부분이 표결에 불참해 1차 탄핵소추안이 의결정족수 미달로...".to_string(),
                    images: vec!["https://dev.democrasee.me/images/sample.png".to_string()],
                    votes: vec![Vote::Yes(30), Vote::No(20)],
                    donations: vec![Donation::Yes(30), Donation::No(20)],
                    started_at,
                    ended_at,
                    voters: 100,
                    replies: 100,
                    status: status.clone(),
                    result: None,
                },
                Topic {
                    id: "1".to_string(),
                    r#type: "type".to_string(),
                    created_at: 0,
                    updated_at: 0,
                    deleted_at: None,
                    author: "author".to_string(),

                    title: "윤대통령 2차 탄핵안 절차 게시될까?".to_string(),
                    content: "민주당과 조국혁신당, 개혁신당 등 야 6당이 함께 윤석열 대통령에 대한 두 번째 탄핵소추안을 국회에 제출했습니다.  지난 7일, 국민의힘 의원 대부분이 표결에 불참해 1차 탄핵소추안이 의결정족수 미달로...".to_string(),
                    images: vec!["https://dev.democrasee.me/images/sample.png".to_string()],
                    votes: vec![Vote::Yes(30), Vote::No(20)],
                    donations: vec![Donation::Yes(30), Donation::No(20)],
                    started_at,
                    ended_at,
                    voters: 100,
                    replies: 100,
                    status: status.clone(),
                    result: None,
                },
                Topic {
                    id: "1".to_string(),
                    r#type: "type".to_string(),
                    created_at: 0,
                    updated_at: 0,
                    deleted_at: None,
                    author: "author".to_string(),

                    title: "윤대통령 2차 탄핵안 절차 게시될까?".to_string(),
                    content: "민주당과 조국혁신당, 개혁신당 등 야 6당이 함께 윤석열 대통령에 대한 두 번째 탄핵소추안을 국회에 제출했습니다.  지난 7일, 국민의힘 의원 대부분이 표결에 불참해 1차 탄핵소추안이 의결정족수 미달로...".to_string(),
                    images: vec!["https://dev.democrasee.me/images/sample.png".to_string()],
                    votes: vec![Vote::Yes(30), Vote::No(20)],
                    donations: vec![Donation::Yes(30), Donation::No(20)],
                    started_at,
                    ended_at,
                    voters: 100,
                    replies: 100,
                    status: status.clone(),
                    result: None,
                }
            ],
            bookmark: None,
        };
        Ok(Json(ret))
    }
}
