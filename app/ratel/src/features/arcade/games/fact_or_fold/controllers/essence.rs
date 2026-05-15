//! Essence opt-in endpoint (PR6 step 4).
//!
//! Surface:
//!   POST /api/arcade/games/fact-or-fold/rounds/{round_id}/essence
//!
//! Body:
//!   { "register": bool }
//!
//! Roadmap §FR-34~38: after settlement, the user is shown a review
//! screen with their rationale + decisive-quote-marked debate
//! utterances. Items ≥50 chars (§FR-37) are on by default; toggling
//! posts here. v1 surface registers the caller's *rationale* only
//! — the decisive-quote chat-message variant lands when chat is
//! moderated/curatable (deferred).
//!
//! Eligibility:
//!   - Caller must own a `FactFoldRationale` row for this round
//!   - `rationale.essence_eligible == true` (≥50 chars, set at
//!     submit time per §FR-37)
//!
//! On register:
//!   - Calls `essence::services::index_fact_fold_rationale(...)` —
//!     idempotent, just upserts the Essence row
//!   - Marks `rationale.essence_registered = true` for audit /
//!     dedup (the UI uses this to disable the toggle)
//!
//! On unregister:
//!   - v1 doesn't support unregistration; flagged unless explicit
//!     spec extension lands. roadmap §FR-37 only describes
//!     opt-in registration.

use crate::common::*;
use crate::features::arcade::games::fact_or_fold::types::*;

#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[cfg(feature = "server")]
use crate::common::models::auth::User;
#[cfg(feature = "server")]
use crate::features::arcade::games::fact_or_fold::models::FactFoldRationale;

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RegisterEssenceRequest {
    /// v1: only `true` is supported (opt-in registration). `false`
    /// is rejected — there is no unregister flow yet.
    pub register: bool,
}

#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RegisterEssenceResponse {
    pub registered: bool,
    pub rationale_text: String,
}

#[post(
    "/api/arcade/games/fact-or-fold/rounds/{round_id}/essence",
    user: User
)]
pub async fn register_essence_handler(
    round_id: FactFoldRoundEntityType,
    req: RegisterEssenceRequest,
) -> Result<RegisterEssenceResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let inner_round_id = round_id.0.clone();

    if !req.register {
        // Unregister isn't part of v1 (roadmap §FR-37 frames the
        // flow as one-time opt-in). Fail closed.
        return Err(FactOrFoldError::RationaleInvalid.into());
    }

    let user_id = user
        .pk
        .to_string()
        .strip_prefix("USER#")
        .unwrap_or(&user.pk.to_string())
        .to_string();
    let (pk, sk) = FactFoldRationale::keys(&inner_round_id, &user_id);
    let mut rationale = FactFoldRationale::get(cli, &pk, Some(sk.clone()))
        .await
        .map_err(|e| {
            crate::error!("register_essence_handler rationale read failed: {e}");
            FactOrFoldError::StorageFailure
        })?
        .ok_or(FactOrFoldError::RationaleInvalid)?;

    if !rationale.essence_eligible {
        // §FR-38: short rationales auto-excluded.
        return Err(FactOrFoldError::RationaleInvalid.into());
    }

    crate::features::essence::services::indexer::index_fact_fold_rationale(cli, &rationale)
        .await
        .map_err(|e| {
            crate::error!("register_essence_handler index failed: {e}");
            FactOrFoldError::StorageFailure
        })?;

    if !rationale.essence_registered {
        rationale.essence_registered = true;
        rationale.updated_at = crate::common::utils::time::get_now_timestamp_millis();
        rationale.upsert(cli).await.map_err(|e| {
            crate::error!("register_essence_handler rationale upsert failed: {e}");
            FactOrFoldError::StorageFailure
        })?;
    }

    Ok(RegisterEssenceResponse {
        registered: true,
        rationale_text: rationale.text,
    })
}
