use crate::common::*;

#[cfg(feature = "server")]
use candid::{Decode, Encode};
#[cfg(feature = "server")]
use ic_agent::Agent;

#[cfg(feature = "server")]
pub use ratel_canister::types::*;

#[cfg(feature = "server")]
#[derive(Debug, Clone)]
pub struct IcpCanisterService {
    agent: Agent,
    canister_id: candid::Principal,
}

#[cfg(feature = "server")]
impl IcpCanisterService {
    pub async fn new(ic_url: &str, canister_id: &str) -> Result<Self> {
        let agent = Agent::builder()
            .with_url(ic_url)
            .build()
            .map_err(|e| Error::InternalServerError(format!("IC agent error: {}", e)))?;

        if ic_url.contains("localhost") || ic_url.contains("127.0.0.1") {
            agent
                .fetch_root_key()
                .await
                .map_err(|e| Error::InternalServerError(format!("IC root key error: {}", e)))?;
        }

        let canister_id = candid::Principal::from_text(canister_id.trim())
            .map_err(|e| Error::BadRequest(format!("Invalid RATEL_CANISTER_ID: {}", e)))?;

        Ok(Self {
            agent,
            canister_id,
        })
    }

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

    pub async fn health(&self) -> Result<String> {
        let args =
            Encode!().map_err(|e| Error::Unknown(format!("Candid encode error: {}", e)))?;

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
