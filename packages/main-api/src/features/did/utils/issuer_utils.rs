use crate::*;
use ssi::JWK;

pub fn sign(message: &[u8]) {
    let key: JWK = config::get().did.bbs_bls_key.into();

    key.sign(payload)
}
