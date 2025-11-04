use bdk::prelude::*;
use by_axum::axum::http::{HeaderMap, HeaderValue, StatusCode};
use by_axum::axum::response::IntoResponse;
use serde_json::json;

use crate::config;

pub async fn get_did_document_handler() -> impl IntoResponse {
    let conf = config::get();
    let domain = conf.domain;
    let base = format!("https://{}", domain);

    let es256_x = conf.did.p256_x;
    let es256_y = conf.did.p256_y;

    // BBS BLS keys from config for selective disclosure
    let bbs_bls_x = conf.did.bbs_bls_x;
    let bbs_bls_y = conf.did.bbs_bls_y;
    let bbs_bls_crv = conf.did.bbs_bls_crv;

    let did_doc = json!({
        "@context": [
            "https://www.w3.org/ns/did/v1",
            "https://w3id.org/security/suites/jws-2020/v1",
            "https://w3id.org/security/data-integrity/v2"
        ],
        "id": format!("did:web:{}", domain),
        "verificationMethod": [
            {
                "id": format!("did:web:{}#es256-1", domain),
                "type": "JsonWebKey2020",
                "controller": format!("did:web:{}", domain),
                "publicKeyJwk": {
                    "kty": "EC",
                    "crv": "P-256",
                    "x": es256_x,
                    "y": es256_y
                }
            },
            {
                "id": format!("did:web:{}#bbs-bls-key-1", domain),
                "type": "JsonWebKey2020",
                "controller": format!("did:web:{}", domain),
                "publicKeyJwk": {
                    "kty": "EC",
                    "crv": bbs_bls_crv,
                    "x": bbs_bls_x,
                    "y": bbs_bls_y
                }
            }
        ],
        "authentication": [
            format!("did:web:{}#es256-1", domain)
        ],
        "assertionMethod": [
            format!("did:web:{}#es256-1", domain),
            format!("did:web:{}#bbs-bls-key-1", domain)
        ],
        "service": [
            {
                "id": "#credential-issuer",
                "type": "CredentialIssuer",
                "serviceEndpoint": format!("{}/oid4vci", base)
            },
            {
                "id": "#issuer-metadata",
                "type": "OpenIDCredentialIssuer",
                "serviceEndpoint": format!("{}/.well-known/openid-credential-issuer", base)
            },
            {
                "id": "#status-list",
                "type": "StatusList",
                "serviceEndpoint": format!("{}/status/bitstring/1.json", base)
            }
        ]
    });

    did_json(did_doc).await
}

async fn did_json(doc: serde_json::Value) -> (StatusCode, HeaderMap, String) {
    let mut headers = HeaderMap::new();
    headers.insert(
        "content-type",
        HeaderValue::from_static("application/did+json"),
    );

    headers.insert(
        "cache-control",
        HeaderValue::from_static("max-age=60, must-revalidate"),
    );

    let body = serde_json::to_string_pretty(&doc).unwrap();
    (StatusCode::OK, headers, body)
}
