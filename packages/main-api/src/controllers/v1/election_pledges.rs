use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, State},
        routing::post,
    },
};
use dto::*;

use crate::utils::users::extract_user_id;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct ElectionPledgePath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct ElectionPledgeController {
    repo: ElectionPledgeLikeRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl ElectionPledgeController {
    async fn like(
        &self,
        id: i64,
        auth: Option<Authorization>,
        _param: ElectionPledgeLikeRequest,
    ) -> Result<ElectionPledge> {
        let user_id = extract_user_id(&self.pool, auth).await?;

        self.repo.insert(id, user_id).await.map_err(|e| {
            tracing::error!("failed to insert like: {:?}", e);
            Error::AlreadyLiked
        })?;

        let res = ElectionPledge::query_builder(user_id)
            .id_equals(id)
            .query()
            .map(ElectionPledge::from)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("failed to get election pledge: {:?}", e);
                Error::NotFound
            })?;

        Ok(res)
    }
}

impl ElectionPledgeController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = ElectionPledgeLike::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/:id", post(Self::act_election_pledge_by_id))
            .with_state(self.clone()))
    }

    pub async fn act_election_pledge_by_id(
        State(ctrl): State<ElectionPledgeController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(ElectionPledgePath { id }): Path<ElectionPledgePath>,
        Json(body): Json<ElectionPledgeByIdAction>,
    ) -> Result<Json<ElectionPledge>> {
        tracing::debug!("act_election_pledge_by_id {:?} {:?}", id, body);
        match body {
            ElectionPledgeByIdAction::Like(param) => {
                let res = ctrl.like(id, auth, param).await?;
                Ok(Json(res))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{TestContext, setup};

    #[tokio::test]
    async fn test_like_election_pledge() {
        let TestContext {
            now,
            endpoint,
            pool,
            ..
        } = setup().await.unwrap();

        let repo = PresidentialCandidate::get_repository(pool.clone());
        let name = format!("like-election-pledge-{}", now);
        let image = format!("https://test.com/{now}.png");
        let crypto_stance = CryptoStance::StronglySupportive;
        let party = Party::PeoplePowerParty;
        let t1 = format!("test-pledge-{}", now);
        let t2 = format!("test-pledge2-{}", now);

        let doc = repo
            .insert(
                name.clone(),
                image.clone(),
                crypto_stance,
                party,
                "".to_string(),
                "".to_string(),
            )
            .await
            .unwrap();

        let erepo = ElectionPledge::get_repository(pool.clone());
        let e1 = erepo.insert(doc.id, t1.clone()).await.unwrap();
        erepo.insert(doc.id, t2.clone()).await.unwrap();

        ElectionPledge::get_client(&endpoint)
            .like(e1.id)
            .await
            .unwrap();

        let cand = PresidentialCandidate::get_client(&endpoint)
            .get(doc.id)
            .await
            .unwrap();

        let election_pledge = cand
            .election_pledges
            .into_iter()
            .find(|x| x.id == e1.id)
            .unwrap();

        assert_eq!(election_pledge.id, e1.id);
        assert_eq!(election_pledge.promise, t1);
        assert_eq!(election_pledge.likes, 1);
    }
}
