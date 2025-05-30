use bdk::prelude::*;

use gloo_storage::{LocalStorage, Storage};
use ring::rand::SystemRandom;

use ring::signature::{Ed25519KeyPair, KeyPair};

use simple_asn1::{
    ASN1Block::{BitString, ObjectIdentifier, Sequence},
    oid, to_der,
};

const IDENTITY_KEY: &str = "anonymous-identity";

#[derive(Clone, Copy, DioxusController)]
pub struct AnonymouseService {
    pub private_key: Signal<Vec<u8>>,
}

impl AnonymouseService {
    pub fn init() {
        let private_key = match LocalStorage::get(IDENTITY_KEY) {
            Ok(key) => key,
            Err(_) => {
                let rng = SystemRandom::new();
                let key_pair = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
                let key = key_pair.as_ref().to_vec();
                LocalStorage::set(IDENTITY_KEY, key.clone()).unwrap();
                key
            }
        };

        let srv = Self {
            private_key: use_signal(move || private_key),
        };

        rest_api::set_signer(Box::new(srv));
        use_context_provider(move || srv);
    }

    pub fn set_signer(&self) {
        rest_api::set_signer(Box::new(*self));
    }

    pub fn get_identity(&self) -> Ed25519KeyPair {
        ring::signature::Ed25519KeyPair::from_pkcs8(&self.private_key())
            .expect("Could not read the key pair.")
    }

    pub fn get_public_key(&self) -> Vec<u8> {
        let key_pair = self.get_identity();

        key_pair.public_key().as_ref().to_vec()
    }

    pub fn get_principal(&self) -> String {
        let key_pair = self.get_identity();

        let public_key = key_pair.public_key().as_ref().to_vec();
        let id_ed25519 = oid!(1, 3, 101, 112);
        let algorithm = Sequence(0, vec![ObjectIdentifier(0, id_ed25519)]);
        let subject_public_key = BitString(0, public_key.len() * 8, public_key);
        let subject_public_key_info = Sequence(0, vec![algorithm, subject_public_key]);
        let der_public_key = to_der(&subject_public_key_info).unwrap();
        let wallet_address = candid::Principal::self_authenticating(der_public_key);
        wallet_address.to_text()
    }
}

impl rest_api::Signer for AnonymouseService {
    fn signer(&self) -> String {
        self.get_principal()
    }

    fn sign(
        &self,
        msg: &str,
    ) -> std::result::Result<rest_api::Signature, Box<dyn std::error::Error>> {
        tracing::debug!("AnonymousService::sign: msg={}", msg);
        let key_pair = self.get_identity();

        let sig = key_pair.sign(msg.as_bytes());

        let sig = rest_api::Signature {
            signature: sig.as_ref().to_vec(),
            public_key: self.get_public_key(),
            algorithm: rest_api::signature::SignatureAlgorithm::EdDSA,
        };

        return Ok(sig);
    }
}
