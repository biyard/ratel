use ssi::{
    dids::{DIDBuf, Document, document::Service},
    json_ld::iref::UriBuf,
    verification_methods::ssi_core::OneOrMany,
};

use crate::*;

pub fn generate_did_by_username(username: &str) -> Result<Document> {
    let id = get_did(username)?;

    let mut document = Document::new(id.clone());
    document.controller = Some(OneOrMany::One(id));

    Ok(document)
}

pub fn get_did(username: &str) -> Result<DIDBuf> {
    Ok(DIDBuf::from_string(format!(
        "did:web:{}:{}",
        crate::config::get().domain,
        username
    ))?)
}
