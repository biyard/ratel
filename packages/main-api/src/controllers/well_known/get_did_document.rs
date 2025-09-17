use bdk::prelude::*;
use dto::{Result, by_axum::axum::Json};
use serde_json::Value;

use crate::config;

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    aide::OperationIo,
    JsonSchema,
)]
pub struct DidDocument {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(rename = "verificationMethod")]
    pub verification_method: Vec<Value>,
    pub authentication: Vec<String>,
    #[serde(rename = "assertionMethod")]
    pub assertion_method: Vec<String>,
    pub service: Vec<Value>,
}

pub async fn get_did_document_handler() -> Result<Json<DidDocument>> {
    let conf = config::get();
    let domain = conf.domain;
    let base = format!("https://{}", domain);

    let es256_x = conf.did.p256_x;
    let es256_y = conf.did.p256_y;

    //FIXME: remove this comment when conflicting is resolved about session
    // let bls_multibase = multibase::encode(
    //     multibase::Base::Base58Btc,
    //     JWK::generate_bls12381g2()
    //         .to_multicodec()
    //         .unwrap()
    //         .as_bytes(),
    // );

    let bls_multibase = "".to_string();

    let verification_method = vec![
        serde_json::json!({
            "id": format!("did:web:{}#es256-1", domain),
            "type": "JsonWebKey2020",
            "controller": format!("did:web:{}", domain),
            "publicKeyJwk": {
                "kty": "EC",
                "crv": "P-256",
                "x": es256_x,
                "y": es256_y
            }
        }),
        serde_json::json!({
            "id": format!("did:web:{}#bls-key-1", domain),
            "type": "Multikey",
            "controller": format!("did:web:{}", domain),
            "publicKeyMultibase": bls_multibase
        })
    ];

    let service = vec![
        serde_json::json!({
            "id": "#credential-issuer",
            "type": "CredentialIssuer",
            "serviceEndpoint": format!("{}/oid4vci", base)
        }),
        serde_json::json!({
            "id": "#issuer-metadata",
            "type": "OpenIDCredentialIssuer",
            "serviceEndpoint": format!("{}/.well-known/openid-credential-issuer", base)
        }),
        serde_json::json!({
            "id": "#status-list",
            "type": "StatusList",
            "serviceEndpoint": format!("{}/status/bitstring/1.json", base)
        })
    ];

    let did_doc = DidDocument {
        context: vec![
            "https://www.w3.org/ns/did/v1".to_string(),
            "https://w3id.org/security/suites/jws-2020/v1".to_string(),
            "https://w3id.org/security/multikey/v1".to_string(),
        ],
        id: format!("did:web:{}", domain),
        verification_method,
        authentication: vec![format!("did:web:{}#es256-1", domain)],
        assertion_method: vec![
            format!("did:web:{}#es256-1", domain),
            format!("did:web:{}#bls-key-1", domain),
        ],
        service,
    };

    Ok(Json(did_doc))
}
