use ssi::{
    bbs::BBSplusSecretKey,
    crypto::{
        ed25519::SigningKey,
        p256::{self, NistP256},
        signatures::bbs::bls_generate_blinded_g1_key,
    },
};

#[derive(Debug, Clone, Copy)]
pub struct DidConfig {
    pub bbs_bls_key: &'static BBSplusSecretKey,
    pub p256_key: &'static ssi::crypto::p256::ecdsa::SigningKey,
}

impl Default for DidConfig {
    fn default() -> Self {
        let bbs_bls_key: &'static BBSplusSecretKey = if let Some(encoded_key) =
            option_env!("BBS_BLS_KEY")
        {
            let key_bytes = hex::decode(encoded_key).expect("Failed to decode BBS+ key");
            let key = BBSplusSecretKey::from_bytes(key_bytes.as_ref()).expect("Invalid BBS+ key");

            Box::leak(Box::new(key))
        } else {
            let mut rng = ssi::crypto::rand::rngs::OsRng {};
            let key = ssi::bbs::generate_secret_key(&mut rng);
            let encoded_key = key.encode();
            tracing::debug!("Generated BBS+ key: {}", encoded_key);

            Box::leak(Box::new(key))
        };

        let p256_key =
            ssi::crypto::p256::ecdsa::SigningKey::random(&mut ssi::crypto::rand::rngs::OsRng {});
        let p256_key: &'static ssi::crypto::p256::ecdsa::SigningKey = Box::leak(Box::new(p256_key));

        Self {
            bbs_bls_key,
            p256_key,
        }
    }
}
