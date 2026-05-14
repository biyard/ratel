//! Migration 002 — backfill PendingReward rows for the 2026-04-19/20 Biyard
//! outage. The retry endpoint (`run_pending_reward_retry`) drains the rows
//! into Biyard with the original month preserved so April-earned points are
//! billed against April even when retried later.
//!
//! Idempotent via `PendingReward::create` (conditional PutItem with
//! `attribute_not_exists(pk) AND attribute_not_exists(sk)`); a re-run skips
//! rows already inserted by a previous run or by the retry pipeline.
//!
//! Source data: a 4-column CSV (`user_id, amount, created_at_ms, reward_key`)
//! generated from the reconciliation audit (`MISSING_IN_BIYARD` rows only,
//! PII stripped). The path is supplied at runtime via the
//! `M002_CSV_PATH` env var so the file never enters source control or the
//! deployed binary; absent the env var, this migration aborts before
//! advancing the version gate.

use crate::common::models::reward::PendingReward;
use crate::common::types::{PendingRewardKey, PendingRewardStatus, RewardKey};
use crate::common::utils::aws::error::AwsError;
use crate::common::*;

pub const REQUIRED_VERSION: i64 = 2;

const SPACE_ID: &str = "019d70df-dfc0-7222-be71-e55c2bd8121a";
const OWNER_TEAM_ID: &str = "840";
const DESCRIPTION: &str = "outage-backfill";

const CSV_PATH_ENV: &str = "M002_CSV_PATH";
const EXPECTED_HEADER: &str = "user_id,amount,created_at_ms,reward_key";

pub async fn run(cli: &aws_sdk_dynamodb::Client) -> crate::common::Result<()> {
    let space_pk = Partition::Space(SPACE_ID.to_string());
    let owner_pk = Partition::Team(OWNER_TEAM_ID.to_string());

    let csv_path = std::env::var(CSV_PATH_ENV).map_err(|_| {
        tracing::error!(
            env = CSV_PATH_ENV,
            "m002: env var not set; refusing to advance migration version",
        );
        Error::InvalidFormat
    })?;
    let csv_data = std::fs::read_to_string(&csv_path).map_err(|e| {
        tracing::error!(path = %csv_path, error = %e, "m002: failed to read CSV");
        Error::InvalidFormat
    })?;

    let mut lines = csv_data.lines();
    let header = lines.next().ok_or(Error::InvalidFormat)?;
    if header.trim() != EXPECTED_HEADER {
        tracing::error!(actual = header, expected = EXPECTED_HEADER, "m002: header mismatch");
        return Err(Error::InvalidFormat);
    }

    let mut written = 0usize;
    let mut already_present = 0usize;
    let mut total = 0usize;

    for (lineno, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        total += 1;

        // splitn(4) so the reward_key (which contains commas in nothing we've
        // seen, but `##`-delimited identifiers in general) survives intact.
        let parts: Vec<&str> = line.splitn(4, ',').collect();
        if parts.len() != 4 {
            tracing::error!(line = lineno + 2, raw = line, "m002: malformed row");
            return Err(Error::InvalidFormat);
        }

        let user_id = parts[0].trim();
        let amount: i64 = parts[1].trim().parse().map_err(|_| Error::InvalidFormat)?;
        let created_at_ms: i64 = parts[2].trim().parse().map_err(|_| Error::InvalidFormat)?;
        let reward_key: RewardKey = parts[3].trim().parse()?;

        let target_pk = Partition::User(user_id.to_string());

        let row = PendingReward {
            pk: Partition::PendingReward,
            sk: PendingRewardKey {
                created_at: created_at_ms,
                target_pk: target_pk.clone(),
                reward_key: reward_key.clone(),
            },
            created_at: created_at_ms,
            target_pk,
            owner_pk: Some(owner_pk.clone()),
            space_pk: space_pk.clone(),
            reward_key,
            amount,
            description: DESCRIPTION.to_string(),
            status: PendingRewardStatus::Pending,
            updated_at: created_at_ms,
            retry_count: 0,
            last_error: String::new(),
        };

        match row.create(cli).await {
            Ok(_) => written += 1,
            Err(Error::Aws(AwsError::DynamoDb(
                aws_sdk_dynamodb::Error::ConditionalCheckFailedException(_),
            ))) => already_present += 1,
            Err(e) => {
                tracing::error!(
                    line = lineno + 2,
                    error = %e,
                    "m002: failed to create pending reward",
                );
                return Err(e);
            }
        }
    }

    tracing::info!(
        total,
        written,
        already_present,
        "m002: backfill_pending_rewards complete",
    );
    Ok(())
}
