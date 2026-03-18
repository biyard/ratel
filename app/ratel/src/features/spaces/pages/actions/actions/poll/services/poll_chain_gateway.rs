use crate::common::types::Error;
use async_trait::async_trait;
use ratel_canister::types::poll::{QuestionOptionCount, QuestionVote, SubmitVoteResult};
use std::sync::Arc;

/// Receipt returned after successfully submitting a vote to the canister.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PollChainReceipt {
    pub record_id: String,
    pub canister_id: String,
    pub ciphertext_hash: String,
    pub voter_tag: String,
    pub submitted_at_ms: i64,
}

/// Abstraction for on-chain poll vote operations.
#[async_trait]
pub trait PollChainGateway: Send + Sync {
    async fn submit_poll_vote(
        &self,
        poll_sk: &str,
        votes: Vec<QuestionVote>,
    ) -> Result<PollChainReceipt, Error>;

    async fn get_vote_counts(
        &self,
        poll_sk: &str,
    ) -> Result<Vec<QuestionOptionCount>, Error>;

    async fn get_vote_by_tag(
        &self,
        poll_sk: &str,
        voter_tag: &str,
    ) -> Result<Vec<QuestionVote>, Error>;
}

/// ICP canister implementation using ic-agent.
pub struct IcpPollChainGateway {
    agent: ic_agent::Agent,
    canister_id: candid::Principal,
}

impl IcpPollChainGateway {
    pub async fn new() -> Result<Self, Error> {
        let ic_url = std::env::var("IC_URL").unwrap_or_else(|_| {
            tracing::warn!("IC_URL not set, using local default");
            "http://127.0.0.1:4943".to_string()
        });

        let canister_id_str =
            std::env::var("ICP_POLL_CANISTER_ID").or_else(|_| std::env::var("RATEL_CANISTER_ID"))
                .map_err(|_| {
                    Error::InternalServerError(
                        "ICP_POLL_CANISTER_ID or RATEL_CANISTER_ID not configured".to_string(),
                    )
                })?;

        let agent = ic_agent::Agent::builder()
            .with_url(&ic_url)
            .build()
            .map_err(|e| Error::InternalServerError(format!("IC agent error: {}", e)))?;

        if ic_url.contains("localhost") || ic_url.contains("127.0.0.1") {
            agent
                .fetch_root_key()
                .await
                .map_err(|e| Error::InternalServerError(format!("IC root key error: {}", e)))?;
        }

        let canister_id = candid::Principal::from_text(canister_id_str.trim())
            .map_err(|e| Error::BadRequest(format!("Invalid canister ID: {}", e)))?;

        Ok(Self {
            agent,
            canister_id,
        })
    }

    fn canister_id_str(&self) -> String {
        self.canister_id.to_text()
    }
}

#[async_trait]
impl PollChainGateway for IcpPollChainGateway {
    async fn submit_poll_vote(
        &self,
        poll_sk: &str,
        votes: Vec<QuestionVote>,
    ) -> Result<PollChainReceipt, Error> {
        use candid::{Decode, Encode};

        let ciphertext_hash = votes
            .first()
            .map(|v| v.ciphertext_hash.clone())
            .unwrap_or_default();
        let voter_tag = votes
            .first()
            .map(|v| v.voter_tag.clone())
            .unwrap_or_default();
        let submitted_at_ms = votes
            .first()
            .map(|v| v.submitted_at_ms)
            .unwrap_or_default();

        let args = Encode!(&poll_sk.to_string(), &votes)
            .map_err(|e| Error::InternalServerError(format!("Candid encode error: {}", e)))?;

        let response = self
            .agent
            .update(&self.canister_id, "submit_poll_vote")
            .with_arg(args)
            .call_and_wait()
            .await
            .map_err(|e| Error::InternalServerError(format!("IC call error: {}", e)))?;

        let result = Decode!(response.as_slice(), SubmitVoteResult)
            .map_err(|e| Error::InternalServerError(format!("Candid decode error: {}", e)))?;

        Ok(PollChainReceipt {
            record_id: result.record_id,
            canister_id: self.canister_id_str(),
            ciphertext_hash,
            voter_tag,
            submitted_at_ms,
        })
    }

    async fn get_vote_counts(
        &self,
        poll_sk: &str,
    ) -> Result<Vec<QuestionOptionCount>, Error> {
        use candid::{Decode, Encode};

        let args = Encode!(&poll_sk.to_string())
            .map_err(|e| Error::InternalServerError(format!("Candid encode error: {}", e)))?;

        let response = self
            .agent
            .query(&self.canister_id, "get_poll_vote_counts")
            .with_arg(args)
            .call()
            .await
            .map_err(|e| Error::InternalServerError(format!("IC query error: {}", e)))?;

        Decode!(response.as_slice(), Vec<QuestionOptionCount>)
            .map_err(|e| Error::InternalServerError(format!("Candid decode error: {}", e)))
    }

    async fn get_vote_by_tag(
        &self,
        poll_sk: &str,
        voter_tag: &str,
    ) -> Result<Vec<QuestionVote>, Error> {
        use candid::{Decode, Encode};

        let args = Encode!(&poll_sk.to_string(), &voter_tag.to_string())
            .map_err(|e| Error::InternalServerError(format!("Candid encode error: {}", e)))?;

        let response = self
            .agent
            .query(&self.canister_id, "get_poll_vote_by_tag")
            .with_arg(args)
            .call()
            .await
            .map_err(|e| Error::InternalServerError(format!("IC query error: {}", e)))?;

        Decode!(response.as_slice(), Vec<QuestionVote>)
            .map_err(|e| Error::InternalServerError(format!("Candid decode error: {}", e)))
    }
}

// Global accessor with test override support
static POLL_CHAIN_GATEWAY: std::sync::OnceLock<Arc<dyn PollChainGateway>> =
    std::sync::OnceLock::new();

/// Get or initialize the poll chain gateway.
pub async fn poll_chain_gateway() -> Result<Arc<dyn PollChainGateway>, Error> {
    if let Some(gw) = POLL_CHAIN_GATEWAY.get() {
        return Ok(gw.clone());
    }

    let gw = Arc::new(IcpPollChainGateway::new().await?);
    let _ = POLL_CHAIN_GATEWAY.set(gw.clone());
    Ok(gw)
}

/// Override the gateway for testing.
#[cfg(test)]
pub fn set_poll_chain_gateway_for_test(gw: Arc<dyn PollChainGateway>) {
    let _ = POLL_CHAIN_GATEWAY.set(gw);
}
