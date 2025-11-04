use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

use crate::api_main::api_main;

#[tokio::test]
async fn test_get_did_document() {
    let app = api_main().await.expect("Failed to create app");

    let response = app
        .oneshot(
            Request::builder()
                .uri("/.well-known/did.json")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let headers = response.headers();
    assert_eq!(
        headers.get("content-type").unwrap(),
        "application/did+json"
    );
    assert_eq!(
        headers.get("cache-control").unwrap(),
        "max-age=60, must-revalidate"
    );

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    let did_doc: serde_json::Value = serde_json::from_str(&body_str).unwrap();

    // Verify basic structure
    assert!(did_doc.get("@context").is_some());
    assert!(did_doc.get("id").is_some());
    assert!(did_doc.get("verificationMethod").is_some());
    assert!(did_doc.get("authentication").is_some());
    assert!(did_doc.get("assertionMethod").is_some());
    assert!(did_doc.get("service").is_some());

    // Verify DID format
    let id = did_doc.get("id").unwrap().as_str().unwrap();
    assert!(id.starts_with("did:web:"));

    // Verify verification methods
    let verification_methods = did_doc
        .get("verificationMethod")
        .unwrap()
        .as_array()
        .unwrap();
    assert_eq!(verification_methods.len(), 2);

    // Verify ES256 key
    let es256_key = &verification_methods[0];
    assert_eq!(es256_key.get("type").unwrap(), "JsonWebKey2020");
    let es256_jwk = es256_key.get("publicKeyJwk").unwrap();
    assert_eq!(es256_jwk.get("kty").unwrap(), "EC");
    assert_eq!(es256_jwk.get("crv").unwrap(), "P-256");
    assert!(es256_jwk.get("x").is_some());
    assert!(es256_jwk.get("y").is_some());

    // Verify BBS BLS key
    let bbs_key = &verification_methods[1];
    assert_eq!(bbs_key.get("type").unwrap(), "JsonWebKey2020");
    let bbs_jwk = bbs_key.get("publicKeyJwk").unwrap();
    assert_eq!(bbs_jwk.get("kty").unwrap(), "EC");
    assert_eq!(bbs_jwk.get("crv").unwrap(), "BLS12381G2");
    assert!(bbs_jwk.get("x").is_some());
    assert!(bbs_jwk.get("y").is_some());

    // Verify authentication array
    let authentication = did_doc.get("authentication").unwrap().as_array().unwrap();
    assert_eq!(authentication.len(), 1);
    assert!(authentication[0].as_str().unwrap().contains("#es256-1"));

    // Verify assertionMethod array
    let assertion_method = did_doc
        .get("assertionMethod")
        .unwrap()
        .as_array()
        .unwrap();
    assert_eq!(assertion_method.len(), 2);
    assert!(assertion_method[0].as_str().unwrap().contains("#es256-1"));
    assert!(assertion_method[1]
        .as_str()
        .unwrap()
        .contains("#bbs-bls-key-1"));

    // Verify services
    let services = did_doc.get("service").unwrap().as_array().unwrap();
    assert_eq!(services.len(), 3);

    // Verify credential issuer service
    let credential_issuer = &services[0];
    assert_eq!(credential_issuer.get("id").unwrap(), "#credential-issuer");
    assert_eq!(
        credential_issuer.get("type").unwrap(),
        "CredentialIssuer"
    );
    assert!(credential_issuer
        .get("serviceEndpoint")
        .unwrap()
        .as_str()
        .unwrap()
        .contains("/oid4vci"));
}
