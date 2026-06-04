//! Wire DTOs for Launchpad point callbacks. Field names match Launchpad's
//! `demo_preview/server.rs` exactly — do not rename.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct LookupBody {
    pub project_id: String,
    pub company_user_key: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LookupResponse {
    pub available_points: i64,
    pub point_symbol: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeductBody {
    pub project_id: String,
    pub company_user_key: String,
    pub point_amount: i64,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeductResponse {
    pub brand_tx_id: String,
    pub deducted_points: i64,
    pub remaining_points: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HealthBody {
    pub project_id: String,
    pub check: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct HealthResponse {
    pub ok: bool,
    pub project_id: String,
    pub service: String,
}
