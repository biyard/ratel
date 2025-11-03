use crate::*;

use super::{ChannelResponse, IdentifyResponse, VerifiedCustomer, VerifiedGender};

#[derive(Debug, Clone)]
pub struct PortOne {}

impl PortOne {
    pub fn new(_api_secret: &str) -> Self {
        Self {}
    }

    pub async fn identify(&self, _id: &str) -> Result<IdentifyResponse> {
        // Return mock data for testing
        Ok(IdentifyResponse {
            channel: ChannelResponse {
                id: "channel-test".to_string(),
                name: "Test Channel".to_string(),
                key: "test-key".to_string(),
                pg_merchant_id: "test-merchant".to_string(),
                pg_provider: "test-provider".to_string(),
                r#type: "identity".to_string(),
            },
            id: "test-identity-id".to_string(),
            pg_raw_response: "{}".to_string(),
            pg_tx_id: "test-tx-id".to_string(),
            requested_at: "2024-01-01T00:00:00Z".to_string(),
            status: "verified".to_string(),
            status_changed_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            verified_at: "2024-01-01T00:00:00Z".to_string(),
            verified_customer: VerifiedCustomer {
                birth_date: "1990-01-15".to_string(),
                gender: VerifiedGender::Male,
                id: "test-customer-id".to_string(),
                is_foreigner: false,
                name: "Test User".to_string(),
                phone_number: "+821012345678".to_string(),
            },
            version: "1".to_string(),
        })
    }
}
