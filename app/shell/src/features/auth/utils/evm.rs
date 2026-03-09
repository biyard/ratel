use crate::features::auth::*;

pub fn generate_nonce() -> String {
    use rand::{Rng, RngExt};
    let mut rng = rand::rng();
    let nonce: u128 = rng.random();
    format!("{:032x}", nonce)
}

pub fn build_siwe_message(nonce: &str) -> String {
    let timestamp = chrono::Utc::now().to_rfc3339();
    format!(
        "ratel.foundation wants you to sign in with your Ethereum account.\n\n\
         Sign in to Ratel - Decentralized Legislative Platform\n\n\
         URI: https://ratel.foundation\n\
         Version: 1\n\
         Chain ID: 1\n\
         Nonce: {}\n\
         Issued At: {}",
        nonce, timestamp
    )
}

pub fn recover_address(message: &str, signature: &str) -> Result<String> {
    use sha3::{Digest, Keccak256};

    let prefixed = format!(
        "\x19Ethereum Signed Message:\n{}{}",
        message.len(),
        message
    );
    let msg_hash = Keccak256::digest(prefixed.as_bytes());

    let sig_bytes = hex::decode(signature.trim_start_matches("0x"))
        .map_err(|e| Error::BadRequest(format!("Invalid signature hex: {}", e)))?;

    if sig_bytes.len() != 65 {
        return Err(Error::BadRequest(format!(
            "Invalid signature length: expected 65, got {}",
            sig_bytes.len()
        )));
    }

    let r = &sig_bytes[0..32];
    let s = &sig_bytes[32..64];
    let v = sig_bytes[64];
    let recovery_id = if v >= 27 { v - 27 } else { v };

    use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};

    let recovery_id = RecoveryId::try_from(recovery_id)
        .map_err(|e| Error::BadRequest(format!("Invalid recovery id: {}", e)))?;

    let mut sig_bytes_64 = [0u8; 64];
    sig_bytes_64[..32].copy_from_slice(r);
    sig_bytes_64[32..].copy_from_slice(s);

    let signature = Signature::from_bytes((&sig_bytes_64).into())
        .map_err(|e| Error::BadRequest(format!("Invalid signature: {}", e)))?;

    let verifying_key =
        VerifyingKey::recover_from_prehash(msg_hash.as_slice(), &signature, recovery_id)
            .map_err(|e| Error::BadRequest(format!("Failed to recover public key: {}", e)))?;

    let public_key_bytes = verifying_key.to_encoded_point(false);
    let public_key_hash = Keccak256::digest(&public_key_bytes.as_bytes()[1..]);
    let address = format!("0x{}", hex::encode(&public_key_hash[12..]));

    Ok(address)
}
