use aide::OperationIo;
use bdk::prelude::*;
use serde::{Deserialize, Serialize};

use crate::types::{ReportPublishState, SpacePartition};

/// Request to get a challenge message for address verification
#[derive(Debug, Deserialize, JsonSchema, OperationIo)]
pub struct GetPricingChallengeRequest {
    /// BASE network recipient address (0x...)
    pub recipient_address: String,
}

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct GetPricingChallengeResponse {
    /// Message to sign with wallet
    pub message: String,
    /// Challenge nonce (for replay protection)
    pub nonce: String,
    /// Expiration timestamp (Unix timestamp in milliseconds)
    pub expires_at: i64,
}

/// Request to set pricing with signature verification
#[derive(Debug, Deserialize, JsonSchema, OperationIo)]
pub struct SetPricingRequest {
    /// Price in dollars (atomic units, 6 decimals). e.g., 1500000 = $1.50
    pub price_dollars: i64,
    /// BASE network recipient address (0x...)
    pub recipient_address: String,
    /// Signature of the challenge message (hex string with 0x prefix)
    pub signature: String,
    /// The challenge nonce that was signed
    pub nonce: String,
}

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct SetPricingResponse {
    pub space_id: SpacePartition,
    pub price_dollars: i64,
    pub recipient_address: String,
    pub address_verified: bool,
    pub revenue_split: RevenueSplitInfo,
    pub publish_state: ReportPublishState,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct RevenueSplitInfo {
    pub treasury_percent: u32,
    pub platform_percent: u32,
    pub creator_percent: u32,
    pub creator_earning: i64,
    pub treasury_amount: i64,
    pub platform_fee: i64,
}

impl RevenueSplitInfo {
    pub fn new(
        price_dollars: i64,
        treasury_percent: u32,
        platform_percent: u32,
        creator_percent: u32,
    ) -> Self {
        let treasury_amount = price_dollars * treasury_percent as i64 / 100;
        let platform_fee = price_dollars * platform_percent as i64 / 100;
        let creator_earning = price_dollars - treasury_amount - platform_fee;

        Self {
            treasury_percent,
            platform_percent,
            creator_percent,
            creator_earning,
            treasury_amount,
            platform_fee,
        }
    }
}
