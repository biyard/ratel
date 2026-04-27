use crate::features::spaces::pages::actions::actions::poll::*;

/// Output of client-side ABE encryption — paired with `RespondPollRequest`'s
/// `client_ciphertext_json` and `client_voter_tag` fields.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClientEncryptedVote {
    pub voter_tag: String,
    pub ciphertext_json: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct ClientVotePayloadMetadata {
    poll_id: String,
    submitted_at_ms: i64,
}

/// Encrypt poll answers in the browser using the voter's ABE secret key.
///
/// `material` carries the voter SK + the authority public key fetched from the
/// `/encryption-material` endpoint. `answers` is serialized into the choice
/// field of the ABE payload. The result is sent back to the server, which
/// uploads it verbatim to the canister.
pub fn encrypt_answers_for_canister(
    material: &VoteEncryptionMaterialResponse,
    answers: &[Answer],
    submitted_at_ms: i64,
) -> Result<ClientEncryptedVote, String> {
    use attr_voting::types::VotePayload;
    use attr_voting::vote::encrypt_vote_json_with_pk_json;

    let choice = serde_json::to_string(answers).map_err(|e| e.to_string())?;
    let metadata = serde_json::to_value(ClientVotePayloadMetadata {
        poll_id: material.poll_id.clone(),
        submitted_at_ms,
    })
    .ok();

    let payload = VotePayload { choice, metadata };

    let ciphertext_json = encrypt_vote_json_with_pk_json(
        &material.authority_public_key_json,
        &material.voter_tag,
        &payload,
    )
    .map_err(|e| format!("ABE encrypt failed: {e}"))?;

    Ok(ClientEncryptedVote {
        voter_tag: material.voter_tag.clone(),
        ciphertext_json,
    })
}

/// Build a `StoredVoterKey` for an encryption-material payload.
///
/// Mirrors `build_stored_voter_key` for the verification material so the same
/// localStorage record can be re-used by the verification panel after submit.
pub fn build_stored_voter_key_from_encryption_material(
    material: &VoteEncryptionMaterialResponse,
    user_secret: &str,
) -> Result<StoredVoterKey, String> {
    if user_secret.is_empty() {
        return Err("Secret is required".to_string());
    }

    let key_bundle_json =
        attr_voting::key_vault::wrap_secret_key(user_secret, &material.voter_secret_key_json)
            .map_err(|e| e.to_string())?;

    Ok(StoredVoterKey {
        version: 1,
        poll_id: material.poll_id.clone(),
        voter_tag: material.voter_tag.clone(),
        key_bundle_json,
    })
}
