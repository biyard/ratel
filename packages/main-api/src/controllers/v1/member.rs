use by_axum::{
    axum::{
        extract::{Query, State}, 
        routing::get, 
        Json
    },
    log::root,
};
use dto::*;
use slog::o;
use crate::{
    models::openapi::member::Member, 
    utils::openapi::*
};

#[derive(Clone, Debug)]
pub struct MemberControllerV1 {
    log: slog::Logger,
}

#[derive(Debug, serde::Deserialize)]
pub struct ListMemberRequest {   
    _size: Option<usize>,
    _page: Option<usize>,
}

impl MemberControllerV1 {
    pub fn route() -> Result<by_axum::axum::Router> {
        let log = root().new(o!("api-controller" => "MemberControllerV1"));
        let ctrl = MemberControllerV1 { log };

        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::list_act_member))
            .with_state(ctrl.clone()))
    }

    pub async fn list_act_member(
        State(ctrl): State<MemberControllerV1>,
        Query(req): Query<ListMemberRequest>,
    ) -> Result<Json<CommonQueryResponse<Member>>> {
        let log = ctrl.log.new(o!("api" => "act_member"));
        slog::debug!(log, "list act member {:?}", req);

        let response =  OpenAPI::new().get_active_members(
            Some(req._page.unwrap_or(1).to_string()), // start from 1 not 0
            Some(req._size.unwrap_or(10).to_string()),
            None,
            None,
            None,
            None,
            None,
        ).await?;

        let mut ret = CommonQueryResponse {
            items: vec![],
            bookmark: None,
        };

        if let Some(row) = response.get("row") {
            for row in row.as_array().unwrap() {
                let member: Member = serde_json::from_value(row.clone())?;
                ret.items.push(member);
            }
        }

        Ok(Json(ret))
    }
}