use bdk::prelude::*;
use by_axum::axum::http::{HeaderMap, HeaderValue, StatusCode};
use by_axum::axum::response::IntoResponse;
use serde_json::json;
// use ssi::JWK;

use crate::config;

// use crate::config;

pub async fn get_did_document_handler() -> impl IntoResponse {
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

    let did_doc = json!({
        "@context": [
            "https://www.w3.org/ns/did/v1",
            "https://w3id.org/security/suites/jws-2020/v1",
            "https://w3id.org/security/multikey/v1"
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
                "id": format!("did:web:{}#bls-key-1", domain),
                "type": "Multikey",
                "controller": format!("did:web:{}", domain),
                "publicKeyMultibase": bls_multibase
            }
        ],
        "authentication": [
            format!("did:web:{}#es256-1", domain)
        ],
        "assertionMethod": [
            format!("did:web:{}#es256-1", domain),
            format!("did:web:{}#bls-key-1", domain)
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
