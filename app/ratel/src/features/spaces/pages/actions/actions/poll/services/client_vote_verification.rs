use crate::features::spaces::pages::actions::actions::poll::*;

const STORAGE_KEY_PREFIX: &str = "ratel.voteKey.";
const SESSION_SECRET_KEY: &str = "ratel.voteSecret.session";

fn quote(s: &str) -> String {
    serde_json::to_string(s).unwrap_or_else(|_| "\"\"".to_string())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoredVoterKey {
    pub version: u8,
    pub poll_id: String,
    pub voter_tag: String,
    pub key_bundle_json: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClientVoteVerification {
    pub voter_tag: String,
    pub ciphertext_hash: String,
    pub decrypted_choice: String,
    pub decrypted_metadata: Option<serde_json::Value>,
}

pub fn voter_key_storage_key(poll_id: &str) -> String {
    format!("{STORAGE_KEY_PREFIX}{poll_id}")
}

pub fn build_stored_voter_key(
    material: &VoteVerificationMaterialResponse,
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

pub fn save_stored_voter_key(record: &StoredVoterKey) -> Result<(), String> {
    let json = serde_json::to_string(record).map_err(|e| e.to_string())?;
    crate::common::utils::storage::save(&voter_key_storage_key(&record.poll_id), &json);
    Ok(())
}

pub async fn load_stored_voter_key(poll_id: &str) -> Option<StoredVoterKey> {
    let raw = crate::common::utils::storage::load(&voter_key_storage_key(poll_id)).await?;
    serde_json::from_str(&raw).ok()
}

pub async fn load_session_vote_secret() -> Option<String> {
    #[cfg(feature = "server")]
    {
        None
    }

    #[cfg(not(feature = "server"))]
    {
        use dioxus::prelude::*;

        let script = format!(
            r#"try {{
                var v = window.sessionStorage.getItem({k});
                dioxus.send(v === null ? null : v);
            }} catch (_e) {{
                dioxus.send(null);
            }}"#,
            k = quote(SESSION_SECRET_KEY),
        );
        let mut runner = document::eval(&script);
        runner.recv::<Option<String>>().await.ok().flatten()
    }
}

pub fn save_session_vote_secret(user_secret: &str) {
    #[cfg(feature = "server")]
    {
        let _ = user_secret;
    }

    #[cfg(not(feature = "server"))]
    {
        use dioxus::prelude::*;

        let script = format!(
            r#"try {{ window.sessionStorage.setItem({k}, {v}); }} catch (_e) {{}}"#,
            k = quote(SESSION_SECRET_KEY),
            v = quote(user_secret),
        );
        let _ = document::eval(&script);
    }
}

pub fn verify_client_vote_material(
    material: &VoteVerificationMaterialResponse,
    stored_key: &StoredVoterKey,
    user_secret: &str,
) -> Result<ClientVoteVerification, String> {
    if material.poll_id != stored_key.poll_id || material.voter_tag != stored_key.voter_tag {
        return Err("Stored key does not match this vote".to_string());
    }

    let computed_hash = {
        use sha2::Digest;
        hex::encode(sha2::Sha256::digest(
            material.encrypted_vote_json.as_bytes(),
        ))
    };
    if computed_hash != material.ciphertext_hash {
        return Err("Ciphertext hash mismatch".to_string());
    }

    let voter_secret_key_json =
        attr_voting::key_vault::unwrap_secret_key(user_secret, &stored_key.key_bundle_json)
            .map_err(|e| e.to_string())?;
    let payload = attr_voting::decrypt_vote_json_with_key_json(
        &voter_secret_key_json,
        &material.encrypted_vote_json,
    )
    .map_err(|e| e.to_string())?;

    Ok(ClientVoteVerification {
        voter_tag: material.voter_tag.clone(),
        ciphertext_hash: material.ciphertext_hash.clone(),
        decrypted_choice: payload.choice,
        decrypted_metadata: payload.metadata,
    })
}
