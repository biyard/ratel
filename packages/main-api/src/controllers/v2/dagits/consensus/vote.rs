#![allow(unused)]
use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    Artwork, ArtworkCertification, Consensus, ConsensusRepositoryUpdateRequest, ConsensusResult,
    ConsensusVote, ConsensusVoteType, DagitOracle, Oracle, Result,
    by_axum::{auth::Authorization, axum::extract::Path},
    sqlx::{Pool, Postgres},
};
use tracing_subscriber::filter::combinator::Or;

use crate::{controllers::v2::dagits::consensus, utils::users::extract_user_id};

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
    #[schemars(description = "Dagit ID")]
    pub dagit_id: i64,
    #[schemars(description = "Consensus ID")]
    pub consensus_id: i64,
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
        dagit_id,
        consensus_id,
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
        .dagit_id_equals(dagit_id)
        .oracle_id_equals(oracle.id)
        .query()
        .map(DagitOracle::from)
        .fetch_one(&pool)
        .await?;

    let mut tx = pool.begin().await?;
    let repo = ConsensusVote::get_repository(pool.clone());
    let result = repo
        .insert_with_tx(
            &mut *tx,
            oracle.id,
            consensus_id,
            req.vote_type,
            req.description,
        )
        .await?
        .ok_or(dto::Error::ServerError("Failed to create vote".to_string()))?;

    // Consensus Logic
    let consensus = Consensus::query_builder()
        .id_equals(consensus_id)
        .query()
        .map(Consensus::from)
        .fetch_one(&pool)
        .await?;

    let votes = ConsensusVote::query_builder()
        .consensus_id_equals(consensus_id)
        .vote_type_equals(ConsensusVoteType::Approved)
        .query()
        .map(ConsensusVote::from)
        .fetch_all(&pool)
        .await?;
    if votes.len() >= (consensus.total_oracles as f64 * 0.5).ceil() as usize {
        ArtworkCertification::get_repository(pool.clone())
            .insert_with_tx(&mut *tx, consensus.artwork_id, consensus_id)
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
