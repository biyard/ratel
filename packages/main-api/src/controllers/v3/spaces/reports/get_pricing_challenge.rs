use crate::features::spaces::reports::dto::{GetPricingChallengeRequest, GetPricingChallengeResponse};
use crate::spaces::{SpacePath, SpacePathParam};
use crate::types::{Partition, TeamGroupPermission};
use crate::utils::time::get_now_timestamp_millis;
use crate::*;
use rand::Rng;

/// Generates a challenge message for wallet address verification.
/// The user must sign this message with their wallet to prove ownership.
pub async fn get_pricing_challenge_handler(
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<GetPricingChallengeRequest>,
) -> crate::Result<Json<GetPricingChallengeResponse>> {
    // Validate space_pk
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    // Check permissions - only space admin can set pricing
    if !permissions.contains(TeamGroupPermission::SpaceEdit) {
        return Err(Error::NoPermission);
    }

    // Validate recipient address format (EVM address)
    if !req.recipient_address.starts_with("0x") || req.recipient_address.len() != 42 {
        return Err(Error::BadRequest(
            "Invalid recipient address format. Must be a valid EVM address (0x...)".to_string(),
        ));
    }

    // Generate nonce (random hex string)
    let mut rng = rand::rng();
    let nonce_bytes: [u8; 16] = rng.random();
    let nonce = hex::encode(nonce_bytes);

    // Expiration time (5 minutes from now)
    let now = get_now_timestamp_millis();
    let expires_at = now + 5 * 60 * 1000; // 5 minutes

    // Create the message to be signed
    // Following EIP-191 standard format for personal_sign
    let message = format!(
        "Ratel Report Pricing Verification\n\nI confirm that I own the recipient address for report payments:\n\nAddress: {}\nNonce: {}\nExpires: {}",
        req.recipient_address,
        nonce,
        expires_at
    );

    Ok(Json(GetPricingChallengeResponse {
        message,
        nonce,
        expires_at,
    }))
}
