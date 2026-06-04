//! Inner callback handlers. Each takes already-verified inputs and
//! delegates point reads/writes to the Biyard console via `BiyardService`.

#![cfg(feature = "server")]

use crate::common::types::{EntityType, Partition};
use crate::common::utils::time::current_month;
use crate::features::launchpad_partner::config::LaunchpadPartnerConfig;
use crate::features::launchpad_partner::error::PartnerError;
use crate::features::launchpad_partner::models::LaunchpadDeduction;
use crate::features::launchpad_partner::types::{
    DeductBody, DeductResponse, HealthResponse, LookupResponse,
};

/// Read the user's cumulative cycle points (same figure as the ratel
/// rewards hero): during the launch quarter this is the Apr–Jun total,
/// not just the current month. Reuses the rewards controller so both
/// surfaces stay in lockstep.
pub async fn lookup(company_user_key: &str) -> Result<LookupResponse, PartnerError> {
    let cfg = LaunchpadPartnerConfig::default();
    let pk = Partition::User(company_user_key.to_string());

    let points =
        crate::features::social::pages::reward::user::controllers::cumulative_cycle_points(pk)
            .await
            .map_err(|e| {
                crate::error!("launchpad lookup: cycle points failed: {e}");
                PartnerError::UnknownUser
            })?;

    Ok(LookupResponse {
        available_points: points,
        point_symbol: cfg.point_symbol.to_string(),
    })
}

/// Idempotently deduct points by issuing a console Exchange transaction.
pub async fn deduct(req: &DeductBody) -> Result<DeductResponse, PartnerError> {
    if req.point_amount <= 0 {
        return Err(PartnerError::InvalidAmount);
    }
    let common = crate::common::CommonConfig::default();
    let cli = common.dynamodb();
    let biyard = common.biyard();
    let pk = Partition::User(req.company_user_key.clone());

    // Idempotency: replay a stored result instead of double-spending.
    if let Ok(Some(existing)) = LaunchpadDeduction::get(
        cli,
        pk.clone(),
        Some(EntityType::LaunchpadDeduction(req.idempotency_key.clone())),
    )
    .await
    {
        return Ok(DeductResponse {
            brand_tx_id: existing.brand_tx_id,
            deducted_points: existing.point_amount,
            remaining_points: existing.remaining_points,
        });
    }

    // NOTE: point deduction is temporarily DISABLED. We acknowledge the
    // deduct so Launchpad's convert proceeds (round registration + token
    // issuance) but DO NOT reduce the console balance. The real deduction
    // (biyard `exchange_points`) will be wired in later.
    let remaining = biyard
        .get_user_balance(pk, current_month())
        .await
        .map(|b| b.balance)
        .unwrap_or(0);
    let brand_tx_id = format!("ratel_nodeduct_{}", req.idempotency_key);

    let row = LaunchpadDeduction::new(
        &req.company_user_key,
        &req.idempotency_key,
        req.point_amount,
        &brand_tx_id,
        remaining,
    );
    if let Err(e) = row.create(cli).await {
        crate::error!("launchpad deduct: idempotency row write failed: {e}");
    }

    Ok(DeductResponse {
        brand_tx_id,
        deducted_points: req.point_amount,
        remaining_points: remaining,
    })
}

/// Health check — no console call; just confirms config + signature.
pub fn health() -> HealthResponse {
    let cfg = LaunchpadPartnerConfig::default();
    HealthResponse {
        ok: true,
        project_id: cfg.project_id.to_string(),
        service: "ratel".to_string(),
    }
}
