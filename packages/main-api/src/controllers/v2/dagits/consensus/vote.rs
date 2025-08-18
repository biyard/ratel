use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    ArtworkCertification, CertificationVoter, Consensus, ConsensusRepositoryUpdateRequest,
    ConsensusResult, ConsensusVote, ConsensusVoteType, Result,
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

    let mut tx = pool.begin().await?;

    let oracle_query = r#"
        SELECT o.id as oracle_id
        FROM oracles o
        JOIN dagit_oracles dag_oracle ON o.id = dag_oracle.oracle_id
        WHERE o.user_id = $1 AND dag_oracle.space_id = $2
    "#;

    let oracle_row = sqlx::query(oracle_query)
        .bind(user_id)
        .bind(space_id)
        .fetch_one(&mut *tx)
        .await?;

    let oracle_id: i64 = oracle_row.get("oracle_id");
    tracing::debug!("Oracle ID: {}", oracle_id);

    let consensus_query = r#"
        SELECT 
            c.id,
            c.artwork_id,
            c.total_oracles,
            c.result,
            COALESCE(vote_counts.total_votes, 0) as current_votes,
            COALESCE(vote_counts.approved_votes, 0) as approved_votes
        FROM consensus c
        LEFT JOIN (
            SELECT 
                consensus_id,
                COUNT(*) as total_votes,
                COUNT(CASE WHEN vote_type = 1 THEN 1 END) as approved_votes
            FROM consensus_votes
            WHERE consensus_id IN (SELECT id FROM consensus WHERE artwork_id = $1)
            GROUP BY consensus_id
        ) vote_counts ON c.id = vote_counts.consensus_id
        WHERE c.artwork_id = $1
    "#;

    let consensus_row = sqlx::query(consensus_query)
        .bind(artwork_id)
        .fetch_one(&mut *tx)
        .await?;

    let consensus_id: i64 = consensus_row.get("id");
    let total_oracles: i64 = consensus_row.get("total_oracles");
    let result: Option<i32> = consensus_row.get("result");
    let current_votes: i64 = consensus_row.get("current_votes");
    let approved_votes: i64 = consensus_row.get("approved_votes");

    if result.is_some() {
        return Err(dto::Error::ServerError(
            "Consensus already completed".to_string(),
        ));
    }

    let repo = ConsensusVote::get_repository(pool.clone());
    let result = repo
        .insert_with_tx(
            &mut *tx,
            oracle_id,
            consensus_id,
            req.vote_type,
            req.description,
        )
        .await?
        .ok_or(dto::Error::ServerError("Failed to create vote".to_string()))?;

    let new_total_votes = current_votes + 1;
    let new_approved_votes = if req.vote_type == ConsensusVoteType::Approved {
        approved_votes + 1
    } else {
        approved_votes
    };

    if new_total_votes == total_oracles {
        let required_approvals = (total_oracles as f64 * 0.5).ceil() as i64;

        if new_approved_votes >= required_approvals {
            let voters_query = r#"
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

            let voters = sqlx::query(voters_query)
                .bind(consensus_id)
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
                .fetch_all(&mut *tx)
                .await?;

            ArtworkCertification::get_repository(pool.clone())
                .insert_with_tx(
                    &mut *tx,
                    artwork_id,
                    consensus_id,
                    total_oracles,
                    new_total_votes,
                    new_approved_votes,
                    new_total_votes - new_approved_votes,
                    voters,
                )
                .await?;

            Consensus::get_repository(pool.clone())
                .update_with_tx(
                    &mut *tx,
                    consensus_id,
                    ConsensusRepositoryUpdateRequest {
                        result: Some(ConsensusResult::Accepted),
                        ..Default::default()
                    },
                )
                .await?;
        } else {
            Consensus::get_repository(pool.clone())
                .update_with_tx(
                    &mut *tx,
                    consensus_id,
                    ConsensusRepositoryUpdateRequest {
                        result: Some(ConsensusResult::Rejected),
                        ..Default::default()
                    },
                )
                .await?;
        }
    }

    tx.commit().await?;
    Ok(Json(result))
}
