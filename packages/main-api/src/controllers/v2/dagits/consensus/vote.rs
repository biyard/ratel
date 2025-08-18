use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    ArtworkCertification, CertificationVoter, Consensus, ConsensusRepositoryUpdateRequest,
    ConsensusResult, ConsensusVote, ConsensusVoteType, DagitOracle, Oracle, Result,
    by_axum::{auth::Authorization, axum::extract::Path},
    sqlx::{Pool, Postgres},
};
use sqlx::Row;

use crate::utils::users::extract_user_id;

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]

pub struct VoteConsensusPathParams {
    #[schemars(description = "Space ID")]
    pub space_id: i64,
    #[schemars(description = "Artwork ID")]
    pub artwork_id: i64,
}

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct VoteConsensusRequest {
    #[schemars(description = "Vote Type")]
    pub vote_type: ConsensusVoteType,

    #[schemars(description = "Additional Description of the vote")]
    pub description: Option<String>,
}

pub async fn consensus_vote_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<Pool<Postgres>>,
    Path(VoteConsensusPathParams {
        space_id,
        artwork_id,
    }): Path<VoteConsensusPathParams>,
    Json(req): Json<VoteConsensusRequest>,
) -> Result<Json<ConsensusVote>> {
    let user_id = extract_user_id(&pool, auth).await?;
    let oracle = Oracle::query_builder()
        .user_id_equals(user_id)
        .query()
        .map(Oracle::from)
        .fetch_one(&pool)
        .await?;
    DagitOracle::query_builder()
        .space_id_equals(space_id)
        .oracle_id_equals(oracle.id)
        .query()
        .map(DagitOracle::from)
        .fetch_one(&pool)
        .await?;

    let mut tx = pool.begin().await?;
    let consensus = Consensus::query_builder()
        .artwork_id_equals(artwork_id)
        .query()
        .map(Consensus::from)
        .fetch_one(&pool)
        .await?;
    if consensus.result.is_some() {
        return Err(dto::Error::ServerError(
            "Consensus already completed".to_string(),
        ));
    }

    let repo = ConsensusVote::get_repository(pool.clone());
    let result = repo
        .insert_with_tx(
            &mut *tx,
            oracle.id,
            consensus.id,
            req.vote_type,
            req.description,
        )
        .await?
        .ok_or(dto::Error::ServerError("Failed to create vote".to_string()))?;

    // Consensus Logic
    let consensus = Consensus::query_builder()
        .id_equals(consensus.id)
        .query()
        .map(Consensus::from)
        .fetch_one(&pool)
        .await?;

    let votes = ConsensusVote::query_builder()
        .consensus_id_equals(consensus.id)
        .vote_type_equals(ConsensusVoteType::Approved)
        .query()
        .map(ConsensusVote::from)
        .fetch_all(&pool)
        .await?;
    if votes.len() >= (consensus.total_oracles as f64 * 0.5).ceil() as usize {
        let votes_query = r#"
        SELECT
            cv.vote_type,
            cv.description,
            u.nickname
        FROM consensus_votes cv
        JOIN oracles o ON cv.oracle_id = o.id
        JOIN users u ON o.user_id = u.id
        WHERE cv.consensus_id = $1
        ORDER BY cv.created_at
    "#;

        let voters = sqlx::query(votes_query)
            .bind(consensus.id)
            .map(|row: sqlx::postgres::PgRow| {
                let vote_type_int: Option<i32> = row.try_get("vote_type").ok();
                let vote_type = match vote_type_int {
                    Some(1) => ConsensusVoteType::Approved,
                    _ => ConsensusVoteType::Rejected,
                };

                let description: Option<String> = row.try_get("description").ok();
                let nickname: String = row
                    .try_get("nickname")
                    .unwrap_or_else(|_| "Unknown".to_string());

                CertificationVoter {
                    nickname,
                    vote_type,
                    description,
                }
            })
            .fetch_all(&pool)
            .await?;

        ArtworkCertification::get_repository(pool.clone())
            .insert_with_tx(
                &mut *tx,
                consensus.artwork_id,
                consensus.id,
                consensus.total_oracles,
                voters.len() as i64,
                votes.len() as i64, // Approved votes
                (voters.len() - votes.len()) as i64,
                voters,
            )
            .await?;

        Consensus::get_repository(pool.clone())
            .update_with_tx(
                &mut *tx,
                consensus.id,
                ConsensusRepositoryUpdateRequest {
                    result: Some(ConsensusResult::Accepted),
                    ..Default::default()
                },
            )
            .await?;
    } else if votes.len() == consensus.total_oracles as usize {
        Consensus::get_repository(pool.clone())
            .update_with_tx(
                &mut *tx,
                consensus.id,
                ConsensusRepositoryUpdateRequest {
                    result: Some(ConsensusResult::Rejected),
                    ..Default::default()
                },
            )
            .await?;
    }
    tx.commit().await?;
    Ok(Json(result))
}
