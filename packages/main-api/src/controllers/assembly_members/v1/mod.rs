use by_axum::{
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
    log::root,
};
use dto::*;
use rest_api::Signature;
use slog::o;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct AssemblyMemberControllerV1 {
    log: slog::Logger,
}

impl AssemblyMemberControllerV1 {
    pub fn route() -> Result<by_axum::axum::Router> {
        let log = root().new(o!("api-controller" => "AssemblyMemberControllerV1"));
        let ctrl = AssemblyMemberControllerV1 { log };

        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::list_assembly_members))
            .route("/parties", get(Self::list_parties))
            .route("/districts", get(Self::list_districts))
            .route("/:id", post(Self::act_assembly_member_by_id))
            .with_state(ctrl.clone()))
    }

    pub async fn act_assembly_member_by_id(
        State(ctrl): State<AssemblyMemberControllerV1>,
        Path(_id): Path<String>,
        Extension(_sig): Extension<Option<Signature>>,
        Json(body): Json<AssemblyMemberByIdActionRequest>,
    ) -> Result<Json<AssemblyMemberByIdActionResponse>> {
        let log = ctrl.log.new(o!("api" => "act_assembly_member_by_id"));
        slog::debug!(log, "act_assembly_member_by_id: {:?}", body);
        Ok(Json(AssemblyMemberByIdActionResponse::default()))
    }

    pub async fn list_assembly_members(
        State(ctrl): State<AssemblyMemberControllerV1>,
        Extension(_sig): Extension<Option<Signature>>,
        Query(req): Query<AssemblyMembersQuery>,
    ) -> Result<Json<CommonQueryResponse<AssemblyMember>>> {
        let log = ctrl.log.new(o!("api" => "list_assembly_members"));
        slog::debug!(log, "list assembly members {:?}", req);

        let lang = req.lang.unwrap_or_default();
        let res: CommonQueryResponse<AssemblyMember> = CommonQueryResponse::query(
            &log,
            "gsi1-index",
            req.bookmark,
            req.size.map(|s| s as i32),
            vec![("gsi1", format!("assembly_member#{}", lang))],
        )
        .await?;

        Ok(Json(res))
    }

    pub async fn list_parties(
        State(ctrl): State<AssemblyMemberControllerV1>,
        Extension(_sig): Extension<Option<Signature>>,
        Query(req): Query<PartiesQuery>,
    ) -> Result<Json<Vec<String>>> {
        let log = ctrl.log.new(o!("api" => "list_parties"));
        slog::debug!(log, "list parties: {req}");

        let lang = req.lang.unwrap_or_default();
        let res: CommonQueryResponse<AssemblyMember> = CommonQueryResponse::query(
            &log,
            "gsi1-index",
            None,
            Some(300),
            vec![("gsi1", format!("assembly_member#{}", lang))],
        )
        .await?;

        let mut party_counts: HashMap<String, usize> = HashMap::new();
        for member in res.items.iter() {
            *party_counts.entry(member.party.to_string()).or_insert(0) += 1;
        }

        let mut parties: Vec<(String, usize)> = party_counts.into_iter().collect();
        parties.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

        let sorted_parties: Vec<String> = parties.into_iter().map(|(party, _)| party).collect();

        Ok(Json(sorted_parties))
    }

    pub async fn list_districts(
        State(ctrl): State<AssemblyMemberControllerV1>,
        Extension(_sig): Extension<Option<Signature>>,
        Query(req): Query<DistrictQuery>,
    ) -> Result<Json<Vec<String>>> {
        let log = ctrl.log.new(o!("api" => "list_district"));
        slog::debug!(log, "list district: {req}");

        let lang = req.lang.unwrap_or_default();
        let res: CommonQueryResponse<AssemblyMember> = CommonQueryResponse::query(
            &log,
            "gsi1-index",
            None,
            Some(10),
            vec![("gsi1", format!("assembly_member#{}", lang))],
        )
        .await?;

        let district: HashSet<String> = res
            .items
            .iter()
            .map(|member| member.party.to_string())
            .collect();

        Ok(Json(district.into_iter().collect()))
    }
}
