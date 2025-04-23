use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, Query, State},
        routing::get,
    },
};
use by_types::QueryResponse;
use dto::*;
use sqlx::postgres::PgRow;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct PresidentialCandidatePath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct PresidentialCandidateController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl PresidentialCandidateController {
    async fn query(
        &self,
        _auth: Option<Authorization>,
        param: PresidentialCandidateQuery,
    ) -> Result<QueryResponse<PresidentialCandidateSummary>> {
        let mut total_count = 0;
        let items: Vec<PresidentialCandidateSummary> =
            PresidentialCandidateSummary::query_builder()
                .election_pledges_builder(ElectionPledge::query_builder())
                .limit(param.size())
                .page(param.page())
                .query()
                .map(|row: PgRow| {
                    use sqlx::Row;

                    total_count = row.try_get("total_count").unwrap_or_default();
                    row.into()
                })
                .fetch_all(&self.pool)
                .await?;

        Ok(QueryResponse { total_count, items })
    }
}

impl PresidentialCandidateController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/:id", get(Self::get_presidential_candidate_by_id))
            .route("/", get(Self::get_presidential_candidate))
            .with_state(self.clone()))
    }

    pub async fn get_presidential_candidate_by_id(
        State(ctrl): State<PresidentialCandidateController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(PresidentialCandidatePath { id }): Path<PresidentialCandidatePath>,
    ) -> Result<Json<PresidentialCandidate>> {
        tracing::debug!("get_presidential_candidate {:?}", id);

        Ok(Json(
            PresidentialCandidate::query_builder()
                .election_pledges_builder(ElectionPledge::query_builder())
                .id_equals(id)
                .query()
                .map(PresidentialCandidate::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn get_presidential_candidate(
        State(ctrl): State<PresidentialCandidateController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<PresidentialCandidateParam>,
    ) -> Result<Json<PresidentialCandidateGetResponse>> {
        tracing::debug!("list_presidential_candidate {:?}", q);

        match q {
            PresidentialCandidateParam::Query(param) => Ok(Json(
                PresidentialCandidateGetResponse::Query(ctrl.query(auth, param).await?),
            )),
            // PresidentialCandidateParam::Read(param)
            //     if param.action == Some(PresidentialCandidateReadActionType::ActionType) =>
            // {
            //     let res = ctrl.run_read_action(auth, param).await?;
            //     Ok(Json(PresidentialCandidateGetResponse::Read(res)))
            // }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{TestContext, setup};

    #[tokio::test]
    async fn test_list_candidates() {
        let TestContext {
            user,
            now,
            endpoint,
            pool,
            ..
        } = setup().await.unwrap();
        let repo = PresidentialCandidate::get_repository(pool.clone());
        let name = format!("list-candidates-{}", now);
        let image = format!("https://test.com/{now}.png");
        let crypto_stance = CryptoStance::StronglySupportive;
        let party = Party::PeoplePowerParty;
        let t1 = format!("test-pledge-{}", now);
        let t2 = format!("test-pledge2-{}", now);

        let doc = repo
            .insert(name.clone(), image.clone(), crypto_stance, party)
            .await
            .unwrap();

        let erepo = ElectionPledge::get_repository(pool.clone());
        let e1 = erepo.insert(doc.id, t1.clone()).await.unwrap();
        let e2 = erepo.insert(doc.id, t2.clone()).await.unwrap();

        let lrepo = ElectionPledgeLike::get_repository(pool.clone());
        lrepo.insert(e1.id, user.id).await.unwrap();

        let candidates = PresidentialCandidate::get_client(&endpoint)
            .query(PresidentialCandidateQuery::new(10))
            .await
            .unwrap();

        assert_eq!(candidates.items.len() > 0, true);
        let c = candidates
            .items
            .iter()
            .filter(|x| x.id == doc.id)
            .collect::<Vec<_>>()
            .clone();
        let c = c[0];
        assert_eq!(c.id, doc.id);
        assert_eq!(c.name, name);
        assert_eq!(c.image, image);
        assert_eq!(c.crypto_stance, crypto_stance);
        assert_eq!(c.party, party);
        assert_eq!(c.election_pledges.len(), 2);
        assert_eq!(c.election_pledges[0].id, e1.id);
        assert_eq!(c.election_pledges[0].promise, t1);
        assert_eq!(c.election_pledges[0].likes, 1);
        assert_eq!(c.election_pledges[1].id, e2.id);
        assert_eq!(c.election_pledges[1].promise, t2);
    }
}
