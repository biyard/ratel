use crate::common::*;

#[cfg(feature = "server")]
use candid::{Decode, Encode};
#[cfg(feature = "server")]
use ic_agent::Agent;

#[cfg(feature = "server")]
pub use ratel_canister::types::*;

#[cfg(feature = "server")]
#[derive(Debug, Clone)]
pub struct CanisterService {
    agent: Agent,
    canister_id: candid::Principal,
}

#[cfg(feature = "server")]
impl CanisterService {
    pub async fn new(
        ic_url: &str,
        canister_id: &str,
        identity: Option<ic_agent::identity::Secp256k1Identity>,
    ) -> Result<Self> {
        let mut builder = Agent::builder().with_url(ic_url);

        if let Some(identity) = identity {
            builder = builder.with_identity(identity);
        } else {
            tracing::warn!("ICP identity not configured. Using anonymous identity.");
        }

        let agent = builder
            .build()
            .map_err(|e| Error::InternalServerError(format!("IC agent error: {}", e)))?;

        if !ic_url.contains("ic0.app") {
            agent
                .fetch_root_key()
                .await
                .map_err(|e| Error::InternalServerError(format!("IC root key error: {}", e)))?;
        }

        let canister_id = candid::Principal::from_text(canister_id.trim())
            .map_err(|e| Error::BadRequest(format!("Invalid RATEL_CANISTER_ID: {}", e)))?;

        Ok(Self { agent, canister_id })
    }

    pub async fn health(&self) -> Result<String> {
        let args = Encode!().map_err(|e| Error::Unknown(format!("Candid encode error: {}", e)))?;

        let response = self
            .agent
            .query(&self.canister_id, "health")
            .with_arg(args)
            .call()
            .await
            .map_err(|e| Error::Unknown(format!("IC query error: {}", e)))?;

        Decode!(response.as_slice(), String)
            .map_err(|e| Error::Unknown(format!("Candid decode error: {}", e)))
    }
}

// Vote

#[cfg(feature = "server")]
impl CanisterService {
    pub async fn upsert_vote(
        &self,
        vote_key: &str,
        voter_tag: &str,
        ballot: VoteBallot,
    ) -> Result<SubmitVoteResult> {
        let args = Encode!(&vote_key.to_string(), &voter_tag.to_string(), &ballot)
            .map_err(|e| Error::InternalServerError(format!("Candid encode error: {}", e)))?;

        let response = self
            .agent
            .update(&self.canister_id, "upsert_vote")
            .with_arg(args)
            .call_and_wait()
            .await
            .map_err(|e| Error::InternalServerError(format!("IC call error: {}", e)))?;

        Decode!(response.as_slice(), SubmitVoteResult)
            .map_err(|e| Error::InternalServerError(format!("Candid decode error: {}", e)))
    }

    pub async fn get_vote_counts(&self, vote_key: &str) -> Result<Vec<QuestionOptionCount>> {
        let args = Encode!(&vote_key.to_string())
            .map_err(|e| Error::InternalServerError(format!("Candid encode error: {}", e)))?;

        let response = self
            .agent
            .query(&self.canister_id, "get_vote_counts")
            .with_arg(args)
            .call()
            .await
            .map_err(|e| Error::InternalServerError(format!("IC query error: {}", e)))?;

        Decode!(response.as_slice(), Vec<QuestionOptionCount>)
            .map_err(|e| Error::InternalServerError(format!("Candid decode error: {}", e)))
    }

    pub async fn get_ballot_by_tag(
        &self,
        vote_key: &str,
        voter_tag: &str,
    ) -> Result<Option<VoteBallot>> {
        let args = Encode!(&vote_key.to_string(), &voter_tag.to_string())
            .map_err(|e| Error::InternalServerError(format!("Candid encode error: {}", e)))?;

        let response = self
            .agent
            .query(&self.canister_id, "get_ballot_by_tag")
            .with_arg(args)
            .call()
            .await
            .map_err(|e| Error::InternalServerError(format!("IC query error: {}", e)))?;

        Decode!(response.as_slice(), Option<VoteBallot>)
            .map_err(|e| Error::InternalServerError(format!("Candid decode error: {}", e)))
    }
}

// Sampling

#[cfg(feature = "server")]
impl CanisterService {
    pub async fn run_sampling(&self, input: SamplingInput) -> Result<SamplingResult> {
        let args =
            Encode!(&input).map_err(|e| Error::Unknown(format!("Candid encode error: {}", e)))?;

        let response = self
            .agent
            .update(&self.canister_id, "run_sampling")
            .with_arg(args)
            .call_and_wait()
            .await
            .map_err(|e| Error::Unknown(format!("IC call error: {}", e)))?;

        Decode!(response.as_slice(), SamplingResult)
            .map_err(|e| Error::Unknown(format!("Candid decode error: {}", e)))
    }

    pub async fn get_model(&self, id: &str) -> Result<Option<ModelParams>> {
        let args = Encode!(&id.to_string())
            .map_err(|e| Error::Unknown(format!("Candid encode error: {}", e)))?;

        let response = self
            .agent
            .query(&self.canister_id, "get_model")
            .with_arg(args)
            .call()
            .await
            .map_err(|e| Error::Unknown(format!("IC query error: {}", e)))?;

        Decode!(response.as_slice(), Option<ModelParams>)
            .map_err(|e| Error::Unknown(format!("Candid decode error: {}", e)))
    }
}
